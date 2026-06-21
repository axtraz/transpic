use image::{DynamicImage, ImageFormat, ImageReader};
use std::fs::File;
use std::io::{Cursor, Write};
use std::path::PathBuf;

use napi::{Error as NapiError, Status};
use napi_derive::napi;

pub mod blur;
pub mod brighten;
pub mod grayscale;
pub mod huerotate;
pub mod invert;
pub mod pixelate;
pub mod resize;
pub mod rotate;

/// Some output formats impose hard dimension caps (e.g. .ico is limited to
/// 256x256 per frame). This downscales (never upscales) the image to fit
/// within the format's limit, preserving aspect ratio. No-op if the image
/// already fits or the format has no cap.
pub fn clamp_for_format(img: DynamicImage, fmt: ImageFormat) -> DynamicImage {
  match fmt {
    ImageFormat::Ico if img.width() > 256 || img.height() > 256 => {
      img.resize(256, 256, image::imageops::FilterType::Lanczos3)
    }
    _ => img,
  }
}

/// Options for a single `processImage` call. All fields are optional —
/// omitted operations are skipped entirely rather than applied with a
/// default/no-op value. Operations are applied in a fixed order
/// (blur, brighten, grayscale, huerotate, invert, pixelate, resize, rotate),
/// regardless of the order fields are set in JS.
#[napi(object)]
pub struct ImageOptions {
  /// Gaussian blur sigma. Larger values blur more; cost scales with sigma.
  pub blur: Option<f64>,
  /// Brightness delta applied per-pixel. Positive brightens, negative darkens.
  pub brighten: Option<i32>,
  /// If `true`, converts the image to grayscale.
  pub grayscale: Option<bool>,
  /// Hue rotation in degrees (0-360, can wrap).
  pub huerotate: Option<i32>,
  /// If `true`, inverts all pixel colors.
  pub invert: Option<bool>,
  /// Pixelation block size in pixels. Larger values produce a chunkier,
  /// more pixelated result. Values `<= 1` have no visible effect.
  pub pixelate: Option<u32>,
  /// Target size as a `"WIDTHxHEIGHT"` string, e.g. `"512x512"`.
  pub resize: Option<String>,
  /// Rotation in degrees. Implementations typically only support
  /// multiples of 90.
  pub rotate: Option<u32>,
  /// Output format identifier (e.g. `"png"`, `"ico"`, `"webp"`). Falls back
  /// to the source image's detected format if omitted.
  pub output_format: Option<String>,
}

/// Loads an image from `input_path`, applies the requested operations from
/// `options` in sequence, encodes the result in the requested (or original)
/// format, and writes it to a sibling file named `<original_stem>.<ext>` in
/// the current working directory.
///
/// Returns the filename that was written on success.
///
/// # Errors
/// Returns a `NapiError` if the file can't be opened/decoded, an operation
/// fails (e.g. invalid resize string), the output format is unsupported, or
/// the encoded result can't be written to disk.
#[napi(js_name = "processImage")]
pub fn process_image(input_path: String, options: ImageOptions) -> Result<String, NapiError> {
  let input_path = PathBuf::from(input_path);

  let reader = ImageReader::open(&input_path)
    .map_err(|e| {
      NapiError::new(
        Status::GenericFailure,
        format!("Failed to open image '{}': {}", input_path.display(), e),
      )
    })?
    .with_guessed_format()
    .map_err(|e| {
      NapiError::new(
        Status::GenericFailure,
        format!("Failed to guess image format: {}", e),
      )
    })?;

  let original_format = reader.format();
  let mut img = reader.decode().map_err(|e| {
    NapiError::new(
      Status::GenericFailure,
      format!("Failed to decode image: {}", e),
    )
  })?;

  if let Some(sigma) = options.blur {
    img = blur::blur(img, sigma as f32);
  }

  if let Some(intensity) = options.brighten {
    img = brighten::brighten(img, intensity)
      .map_err(|e| NapiError::new(Status::GenericFailure, e.to_string()))?;
  }

  if options.grayscale.unwrap_or(false) {
    img = grayscale::grayscale(img);
  }

  if let Some(degrees) = options.huerotate {
    img =
      huerotate::huerotate(img, degrees).map_err(|e| NapiError::new(Status::GenericFailure, e))?;
  }

  if options.invert.unwrap_or(false) {
    img = invert::invert(img);
  }

  if let Some(block_size) = options.pixelate {
    img =
      pixelate::pixelate(img, block_size).map_err(|e| NapiError::new(Status::GenericFailure, e))?;
  }

  if let Some(ref resize_str) = options.resize {
    img = resize::resize(img, resize_str)
      .map_err(|e| NapiError::new(Status::GenericFailure, e.to_string()))?;
  }

  if let Some(rotation) = options.rotate {
    img = rotate::rotate(img, rotation)
      .map_err(|e| NapiError::new(Status::GenericFailure, e.to_string()))?;
  }

  let out_fmt = if let Some(fmt_str) = &options.output_format {
    match fmt_str.to_lowercase().as_str() {
      "avif" => ImageFormat::Avif,
      "bmp" => ImageFormat::Bmp,
      "exr" => ImageFormat::OpenExr,
      "ff" | "farbfeld" => ImageFormat::Farbfeld,
      "gif" => ImageFormat::Gif,
      "hdr" => ImageFormat::Hdr,
      "ico" => ImageFormat::Ico,
      "jpeg" | "jpg" => ImageFormat::Jpeg,
      "png" => ImageFormat::Png,
      "pnm" => ImageFormat::Pnm,
      "qoi" => ImageFormat::Qoi,
      "tga" => ImageFormat::Tga,
      "tiff" | "tif" => ImageFormat::Tiff,
      "webp" => ImageFormat::WebP,
      unsupported => {
        return Err(NapiError::new(
          Status::GenericFailure,
          format!("Unsupported output format: {}", unsupported),
        ));
      }
    }
  } else {
    original_format.ok_or_else(|| {
      NapiError::new(
        Status::GenericFailure,
        "Could not determine original image format.",
      )
    })?
  };

  img = clamp_for_format(img, out_fmt);

  let mut bytes: Vec<u8> = Vec::new();
  img
    .write_to(&mut Cursor::new(&mut bytes), out_fmt)
    .map_err(|e| {
      NapiError::new(
        Status::GenericFailure,
        format!("Failed to encode image to requested format: {}", e),
      )
    })?;

  let stem = input_path
    .file_stem()
    .ok_or_else(|| {
      NapiError::new(
        Status::GenericFailure,
        "Could not read the source file name",
      )
    })?
    .to_string_lossy()
    .into_owned();

  let ext = out_fmt.extensions_str().first().copied().unwrap_or("bin");
  let filename = format!("{}.{}", stem, ext);

  let mut file = File::create(&filename).map_err(|e| {
    NapiError::new(
      Status::GenericFailure,
      format!("Failed to create file '{}': {}", filename, e),
    )
  })?;

  file.write_all(&bytes).map_err(|e| {
    NapiError::new(
      Status::GenericFailure,
      format!("Failed to write data to file: {}", e),
    )
  })?;

  Ok(filename)
}
