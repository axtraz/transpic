use image::DynamicImage;

pub fn grayscale(img: DynamicImage) -> DynamicImage {
    img.grayscale()
}
