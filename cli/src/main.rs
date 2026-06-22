use clap::Parser;
use std::path::PathBuf;
use std::time::Instant;
use transpic_core::{ImageOptions, process_image};

#[derive(Parser, Debug)]
#[command(name = "transpic-cli")]
#[command(version)]
#[command(about = "A fast image manipulation CLI tool", long_about = None)]
struct Cli {
    #[arg(
        short,
        long,
        help = "Path to the input image, example: --path assets/image.png"
    )]
    path: PathBuf,

    #[arg(long, help = "Output format, example: --format png")]
    format: Option<String>,

    #[arg(long, help = "Blur intensity, example: --blur 1")]
    blur: Option<f32>,

    #[arg(
        long,
        help = "Brighten the image by an integer (positive to increase, negative to decrease). Example: --brighten 20"
    )]
    brighten: Option<i32>,

    #[arg(long, help = "Grayscale the image, example: --grayscale")]
    grayscale: Option<bool>,

    #[arg(long, help = "Hue rotation in degrees, example: --huerotate 90")]
    huerotate: Option<i32>,

    #[arg(long, help = "Invert the image colors, example: --invert")]
    invert: Option<bool>,

    #[arg(
        long,
        help = "Pixelation block size in pixels, example: --pixelate 10"
    )]
    pixelate: Option<u32>,

    #[arg(long, help = "Resize the image, example: --resize 800x600")]
    resize: Option<String>,

    #[arg(
        long,
        help = "Rotation angle in degrees (90, 180, or 270), example: --rotate 180"
    )]
    rotate: Option<u32>,

    #[arg(
        long,
        help = "Sharpen the image using unsharp mask, example: --unsharpen 2.0,5"
    )]
    unsharpen: Option<String>,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("\x1b[31mError: {}\x1b[0m", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let start = Instant::now();

    if cli.format.is_none()
        && cli.blur.is_none()
        && cli.brighten.is_none()
        && !cli.grayscale.unwrap_or(false)
        && cli.huerotate.is_none()
        && !cli.invert.unwrap_or(false)
        && cli.pixelate.is_none()
        && cli.resize.is_none()
        && cli.rotate.is_none()
        && cli.unsharpen.is_none()
    {
        return Err(
            "You must specify at least one action (e.g., --grayscale, --blur 2, etc.).".into(),
        );
    }

    let unsharpen = cli.unsharpen.map(|s| {
        let parts: Vec<f64> = s.split(',')
            .filter_map(|v| v.parse().ok())
            .collect();
        parts
    });

    let options = ImageOptions {
        blur: cli.blur.map(|b| b as f64),
        brighten: cli.brighten,
        grayscale: cli.grayscale,
        huerotate: cli.huerotate,
        invert: cli.invert,
        pixelate: cli.pixelate,
        resize: cli.resize,
        rotate: cli.rotate,
        unsharpen: unsharpen,
        output_format: cli.format,
    };

    let saved_filename = process_image(cli.path.to_string_lossy().to_string(), options)?;

    let duration = start.elapsed();
    println!(
        "\x1b[32m✓ Image edited successfully in {:.2?} and saved as '{}'\x1b[0m",
        duration, saved_filename
    );

    Ok(())
}