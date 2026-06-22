use image::DynamicImage;

/// Sharpens an image using an unsharp mask.
///
/// This technique subtracts a blurred version of the image from the original,
/// enhancing edges and fine details.
///
/// # Arguments
/// * `img` - The image to sharpen
/// * `sigma` - Blur radius used for the mask. Higher values affect broader areas.
/// * `threshold` - Minimum brightness difference required to apply sharpening.
///   Higher values limit sharpening to stronger edges only.
pub fn unsharpen(img: DynamicImage, sigma: f32, threshold: i32) -> Result<DynamicImage, String> {
  if sigma <= 0.0 {
    return Err(format!(
      "Invalid sigma value: {}. Must be greater than 0.",
      sigma
    ));
  }

  if threshold < 0 {
    return Err(format!(
      "Invalid threshold value: {}. Must be >= 0.",
      threshold
    ));
  }

  Ok(img.unsharpen(sigma, threshold))
}
