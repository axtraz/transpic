use image::DynamicImage;

/// Converts the image to grayscale. Color channels are collapsed to
/// luminance; the image's pixel format (e.g. RGBA) is otherwise preserved.
pub fn grayscale(img: DynamicImage) -> DynamicImage {
  img.grayscale()
}