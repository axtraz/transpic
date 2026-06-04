use image::DynamicImage;

pub fn blur(img: DynamicImage, sigma: f32) -> DynamicImage {
    img.blur(sigma)
}
