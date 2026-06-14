use image::{ImageFormat, ImageReader};
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
pub mod resize;
pub mod rotate;

#[napi(object)]
pub struct ImageOptions {
  pub blur: Option<f64>,
  pub brighten: Option<i32>,
  pub grayscale: Option<bool>,
  pub huerotate: Option<i32>,
  pub invert: Option<bool>,
  pub resize: Option<String>,
  pub rotate: Option<u32>,
  pub output_format: Option<String>,
}

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
