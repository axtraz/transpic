use base64::{engine::general_purpose, Engine as _};
use image::{DynamicImage, ImageFormat};
use serde::Deserialize;
use std::collections::HashMap;
use std::io::Cursor;

use transpic_core::{
    blur, brighten, clamp_for_format, grayscale, huerotate, invert, pixelate, resize, rotate,
    unsharpen,
};

#[derive(Deserialize)]
struct PipelineOp {
    id: String,
    params: HashMap<String, f64>,
}

fn decode_image(data_url: &str) -> Result<DynamicImage, String> {
    let b64 = data_url.split(',').last().ok_or("invalid data url")?;
    let bytes = general_purpose::STANDARD
        .decode(b64)
        .map_err(|e| e.to_string())?;
    image::load_from_memory(&bytes).map_err(|e| e.to_string())
}

fn format_and_mime(format: &str) -> Result<(ImageFormat, &'static str), String> {
    match format.to_lowercase().as_str() {
        "avif" => Ok((ImageFormat::Avif, "image/avif")),
        "bmp" => Ok((ImageFormat::Bmp, "image/bmp")),
        "exr" => Ok((ImageFormat::OpenExr, "image/x-exr")),
        "ff" | "farbfeld" => Ok((ImageFormat::Farbfeld, "application/octet-stream")),
        "gif" => Ok((ImageFormat::Gif, "image/gif")),
        "hdr" => Ok((ImageFormat::Hdr, "image/vnd.radiance")),
        "ico" => Ok((ImageFormat::Ico, "image/x-icon")),
        "jpeg" | "jpg" => Ok((ImageFormat::Jpeg, "image/jpeg")),
        "png" => Ok((ImageFormat::Png, "image/png")),
        "pnm" => Ok((ImageFormat::Pnm, "image/x-portable-anymap")),
        "qoi" => Ok((ImageFormat::Qoi, "image/qoi")),
        "tga" => Ok((ImageFormat::Tga, "image/x-tga")),
        "tiff" | "tif" => Ok((ImageFormat::Tiff, "image/tiff")),
        "webp" => Ok((ImageFormat::WebP, "image/webp")),
        unsupported => Err(format!("unsupported output format: {unsupported}")),
    }
}

fn encode_image(img: DynamicImage, format: &str) -> Result<String, String> {
    let (img_format, mime) = format_and_mime(format)?;

    let img = clamp_for_format(img, img_format);

    let mut buf = Cursor::new(Vec::new());
    img.write_to(&mut buf, img_format)
        .map_err(|e| format!("failed to encode as {format}: {e}"))?;
    Ok(format!(
        "data:{mime};base64,{}",
        general_purpose::STANDARD.encode(buf.into_inner())
    ))
}

#[tauri::command]
fn process_image(
    image_data: String,
    operations: Vec<PipelineOp>,
    format: String,
) -> Result<String, String> {
    let mut img = decode_image(&image_data)?;

    for op in operations {
        img = match op.id.as_str() {
            "blur" => {
                let sigma = *op.params.get("radius").unwrap_or(&8.0) as f32;
                blur::blur(img, sigma)
            }
            "brighten" => {
                let amount = *op.params.get("amount").unwrap_or(&0.0) as i32;
                brighten::brighten(img, amount).map_err(|e| e.to_string())?
            }
            "grayscale" => {
                if *op.params.get("enabled").unwrap_or(&0.0) >= 1.0 {
                    grayscale::grayscale(img)
                } else {
                    img
                }
            }
            "huerotate" => {
                let degrees = *op.params.get("degrees").unwrap_or(&0.0) as i32;
                huerotate::huerotate(img, degrees)?
            }
            "invert" => {
                if *op.params.get("enabled").unwrap_or(&0.0) >= 1.0 {
                    invert::invert(img)
                } else {
                    img
                }
            }
            "pixelate" => {
                let block_size = *op.params.get("blockSize").unwrap_or(&8.0) as u32;
                pixelate::pixelate(img, block_size)?
            }
            "resize" => {
                let width = *op.params.get("width").unwrap_or(&1024.0) as u32;
                let height = *op.params.get("height").unwrap_or(&1024.0) as u32;
                let resize_str = format!("{width}x{height}");
                resize::resize(img, &resize_str).map_err(|e| e.to_string())?
            }
            "rotate" => {
                let degrees = *op.params.get("degrees").unwrap_or(&90.0) as u32;
                rotate::rotate(img, degrees).map_err(|e| e.to_string())?
            }
            "unsharpen" => {
                let sigma = *op.params.get("sigma").unwrap_or(&1.5) as f32;
                let threshold = *op.params.get("threshold").unwrap_or(&5.0) as i32;
                unsharpen::unsharpen(img, sigma, threshold).map_err(|e| e.to_string())?
            }
            other => return Err(format!("unknown operation: {other}")),
        };
    }

    encode_image(img, &format)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![process_image])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
