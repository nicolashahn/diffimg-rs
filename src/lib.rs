extern crate clap;
extern crate image;

use std::path::Path;

use clap::ArgMatches;
use image::{DynamicImage, GenericImageView, RgbImage, RgbaImage};

#[derive(Debug)]
pub struct Config<'a> {
    pub image1: &'a str,
    pub image2: &'a str,
    pub filename: Option<&'a str>,
}

impl<'a> Config<'a> {
    pub fn from_clap_matches(matches: &'a ArgMatches) -> Config<'a> {
        // unwrap() should be safe here because clap does argument validation
        let image1 = matches.value_of("image1").unwrap();
        let image2 = matches.value_of("image2").unwrap();
        let filename = matches.value_of("filename");

        Config {
            image1,
            image2,
            filename,
        }
    }
}

/// abs(x - y) for u8
fn abs_diff(x: u8, y: u8) -> u8 {
    if x > y {
        return x - y;
    }
    return y - x;
}

#[test]
fn test_abs_diff() {
    assert_eq!(abs_diff(5, 8), 3);
    assert_eq!(abs_diff(8, 5), 3);
    assert_eq!(abs_diff(11, 11), 0);
    assert_eq!(abs_diff(0, 255), 255);
}

/// Return the image from the file path, or throw an error
fn safe_load_image(raw_path: &str) -> Result<DynamicImage, String> {
    let path = &Path::new(raw_path);
    if !Path::exists(path) {
        return Err(format!("Path \"{}\" does not exist", raw_path));
    }

    image::open(path).map_err(|msg| format!("{:?}", msg))
}

/// Check if two images are the same size and color mode
fn validate_image_compatibility(
    image1: &DynamicImage,
    image2: &DynamicImage,
) -> Result<(), String> {
    if image1.dimensions() != image2.dimensions() {
        return Err("images must have the same dimensions".to_string());
    }
    if image1.color() != image2.color() {
        return Err("images must have the same color mode".to_string());
    }

    Ok(())
}

/// Return a difference ratio between 0 and 1 for the two images
pub fn calculate_diff_ratio(image1: DynamicImage, image2: DynamicImage) -> f64 {
    use std::u8;
    // All color types wrap an 8-bit value for each channel
    let total_possible = (u8::MAX as usize * image1.raw_pixels().len()) as f64;

    image1.raw_pixels().into_iter().zip(image2.raw_pixels())
	.map(|(a, b)| abs_diff(a, b) as u64).sum::<u64>() as f64 / total_possible
}

/// Create an image that is the difference of the two images given, and write to the given filename
pub fn create_diff_image(
    image1: DynamicImage,
    image2: DynamicImage,
    filename: &str,
) -> Result<(), String> {
    use image::ColorType;

    let w = image1.width();
    let h = image1.height();

    let pix_data : Vec<u8> = image1
        .raw_pixels()
        .into_iter()
        .zip(image2.raw_pixels())
        .map(|(p1, p2)| abs_diff(p1, p2))
        .collect();

    let diff = match image1.color() {
        ColorType::RGB(_) => DynamicImage::ImageRgb8(RgbImage::from_raw(w, h, pix_data).unwrap()),
        ColorType::RGBA(_) => DynamicImage::ImageRgba8(RgbaImage::from_raw(w, h, pix_data).unwrap()),
        _ => return Err(format!("color mode {:?} not yet supported", image1.color())),
    };

    if let Err(msg) = diff.save(filename) {
        return Err(msg.to_string());
    }

    Ok(())
}

/// Run the appropriate diffing process given the configuration settings
pub fn run(config: Config) -> Result<(), String> {
    let image1 = safe_load_image(&config.image1)?;
    let image2 = safe_load_image(&config.image2)?;
    validate_image_compatibility(&image1, &image2)?;

    match config.filename {
        Some(filename) => {
            create_diff_image(image1, image2, filename)?;
            println!("Wrote diff image to {}", filename);
        },
        None => {
            let ratio = calculate_diff_ratio(image1, image2);
            println!("{}", ratio);
        }
    }
    Ok(())
}
