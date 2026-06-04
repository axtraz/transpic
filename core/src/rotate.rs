use image::DynamicImage;

pub fn rotate(img: DynamicImage, angle: u32) -> Result<DynamicImage, String> {
    match angle {
        90 => Ok(img.rotate90()),
        180 => Ok(img.rotate180()),
        270 => Ok(img.rotate270()),
        0 => Ok(img),
        _ => Err(format!(
            "Rotation angle {}° not supported. Use 90, 180, or 270.",
            angle
        )),
    }
}
