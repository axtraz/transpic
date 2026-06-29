use image::{DynamicImage, imageops::FilterType};

/// Applies a pixelation (mosaic) effect to the image.
///
/// `block_size` controls the size of each "pixel" block in the output.
/// A larger value produces a chunkier, more pixelated result.
///
/// # Errors
/// Returns an error if `block_size` is `0`.
pub fn pixelate(img: DynamicImage, block_size: u32) -> Result<DynamicImage, String> {
  if block_size == 0 {
    return Err("pixelate: block_size must be greater than 0".to_string());
  }

  let (width, height) = (img.width(), img.height());
  let small_w = (width / block_size).max(1);
  let small_h = (height / block_size).max(1);
  let small = img.resize_exact(small_w, small_h, FilterType::Triangle);
  let result = small.resize_exact(width, height, FilterType::Nearest);

  Ok(result)
}
