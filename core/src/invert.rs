use image::DynamicImage;

pub fn invert(mut img: DynamicImage) -> DynamicImage {
    img.invert();
    img
}
