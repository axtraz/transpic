use image::{ImageFormat, ImageReader};
use std::fs::File;
use std::io::{Cursor, Write};
use std::path::Path;

use napi::{Error as NapiError, Status};
use napi_derive::napi;

pub mod blur;
pub mod grayscale;
pub mod invert;
pub mod resize;
pub mod rotate;

pub struct ImageOptions {
  pub blur: Option<f32>,
  pub grayscale: bool,
  pub invert: bool,
  pub resize: Option<String>,
  pub rotate: Option<u32>,
  pub output_format: Option<String>,
}

pub fn process_image(
  input_path: &Path,
  options: ImageOptions,
) -> Result<String, Box<dyn std::error::Error>> {
  let reader = ImageReader::open(input_path)
    .map_err(|e| format!("Failed to open image '{}': {}", input_path.display(), e))?
    .with_guessed_format()
    .map_err(|e| format!("Failed to guess image format: {}", e))?;

  let original_format = reader.format();
  let mut img = reader
    .decode()
    .map_err(|e| format!("Failed to decode image: {}", e))?;

  if let Some(sigma) = options.blur {
    img = blur::blur(img, sigma);
  }

  if options.grayscale {
    img = grayscale::grayscale(img);
  }

  if options.invert {
    img = invert::invert(img);
  }

  if let Some(ref resize_str) = options.resize {
    img = resize::resize(img, resize_str)?;
  }

  if let Some(rotation) = options.rotate {
    img = rotate::rotate(img, rotation)?;
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
      unsupported => return Err(format!("Unsupported output format: {}", unsupported).into()),
    }
  } else {
    original_format.ok_or("Could not determine original image format.")?
  };

  let mut bytes: Vec<u8> = Vec::new();
  img
    .write_to(&mut Cursor::new(&mut bytes), out_fmt)
    .map_err(|e| format!("Failed to encode image to requested format: {}", e))?;

  let stem = input_path
    .file_stem()
    .to_owned()
    .ok_or("Could not read the source file name")?
    .to_string_lossy();

  let ext = out_fmt.extensions_str().first().copied().unwrap_or("bin");
  let filename = format!("{}.{}", stem, ext);

  let mut file =
    File::create(&filename).map_err(|e| format!("Failed to create file '{}': {}", filename, e))?;
  file
    .write_all(&bytes)
    .map_err(|e| format!("Failed to write data to file: {}", e))?;

  Ok(filename)
}

#[napi(object)]
pub struct NapiImageOptions {
  pub blur: Option<f64>,
  pub grayscale: bool,
  pub invert: bool,
  pub resize: Option<String>,
  pub rotate: Option<u32>,
  pub output_format: Option<String>,
}

#[napi(js_name = "processImage")]
pub fn napi_process_image(input_path: String, options: NapiImageOptions) -> Result<String, NapiError> {
  let path = Path::new(&input_path);

  let rust_options = ImageOptions {
    blur: options.blur.map(|b| b as f32),
    grayscale: options.grayscale,
    invert: options.invert,
    resize: options.resize,
    rotate: options.rotate,
    output_format: options.output_format,
  };

  process_image(path, rust_options)
    .map_err(|e| NapiError::new(Status::GenericFailure, e.to_string()))
}
