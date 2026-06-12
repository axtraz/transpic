use image::DynamicImage;

pub fn huerotate(img: DynamicImage, degrees: i32) -> Result<DynamicImage, String> {
  let rotated = image::imageops::huerotate(&img, degrees);
  
  Ok(DynamicImage::ImageRgba8(rotated))
}