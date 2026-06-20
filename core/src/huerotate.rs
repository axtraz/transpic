use image::DynamicImage;

/// Rotates the hue of every pixel by `degrees` around the color wheel.
/// Values outside `0..360` wrap around as expected (e.g. `400` behaves
/// like `40`, `-90` behaves like `270`).
///
/// Always converts the result to RGBA8 internally, so output images will
/// gain an alpha channel even if the source didn't have one.
///
/// # Errors
/// Never actually fails — kept as `Result` for API consistency with the
/// other operations and to allow future fallible behavior without a
/// breaking signature change.
pub fn huerotate(img: DynamicImage, degrees: i32) -> Result<DynamicImage, String> {
  let rotated = image::imageops::huerotate(&img, degrees);

  Ok(DynamicImage::ImageRgba8(rotated))
}