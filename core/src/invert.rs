use image::DynamicImage;

/// Inverts the image's colors in place (each channel becomes `255 - value`;
/// alpha is left untouched).
pub fn invert(mut img: DynamicImage) -> DynamicImage {
  img.invert();
  img
}