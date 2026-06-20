use image::DynamicImage;

/// Adjusts the image's brightness by adding `intensity` to each pixel
/// channel. Positive values brighten, negative values darken.
///
/// # Errors
/// Returns `Err` if `intensity` is outside the `-255..=255` range.
pub fn brighten(img: DynamicImage, intensity: i32) -> Result<DynamicImage, String> {
  if !(-255..=255).contains(&intensity) {
    return Err(format!(
      "Brightness intensity {} out of range. Use a value between -255 and 255.",
      intensity
    ));
  }

  Ok(img.brighten(intensity))
}