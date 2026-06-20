use image::{DynamicImage, imageops::FilterType};

/// Resizes the image to an exact target size, ignoring aspect ratio.
///
/// `size_str` must be in `"WIDTHxHEIGHT"` form, e.g. `"800x600"`. Uses
/// Lanczos3 filtering for high-quality results on both up- and downscale.
///
/// # Errors
/// Returns `Err` if `size_str` isn't in the expected `WIDTHxHEIGHT` form,
/// either dimension fails to parse as `u32`, or either dimension is `0`.
pub fn resize(img: DynamicImage, size_str: &str) -> Result<DynamicImage, String> {
  let parts: Vec<&str> = size_str.split('x').collect();
  if parts.len() != 2 {
    return Err("Resize format must be 'WIDTHxHEIGHT' (e.g., 800x600)".to_string());
  }

  let width = parts[0]
    .parse::<u32>()
    .map_err(|e| format!("Invalid width: {}", e))?;
  let height = parts[1]
    .parse::<u32>()
    .map_err(|e| format!("Invalid height: {}", e))?;

  if width == 0 {
    return Err("Width must be greater than 0.".to_string());
  }
  if height == 0 {
    return Err("Height must be greater than 0.".to_string());
  }

  Ok(img.resize_exact(width, height, FilterType::Lanczos3))
}
