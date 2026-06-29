use image::DynamicImage;

/// Applies a Gaussian blur to the image.
///
/// `sigma` controls the blur radius — larger values produce a stronger
/// blur and take proportionally longer to compute. A `sigma` of `0.0`
/// effectively leaves the image unchanged. Negative values are accepted
/// by the underlying implementation but have no meaningful effect; pass
/// `0.0` instead of a negative value if no blur is desired.
pub fn blur(img: DynamicImage, sigma: f32) -> DynamicImage {
  img.blur(sigma)
}
