extern crate clap;
extern crate image;

use std::path::Path;

use clap::ArgMatches;
use image::{DynamicImage, GenericImageView};

#[derive(Debug)]
pub struct Config<'a> {
    pub image1: &'a str,
    pub image2: &'a str,
    pub ratio: bool,
    pub delete: bool,
    pub filename: Option<&'a str>,
}

impl<'a> Config<'a> {
    pub fn from_clap_matches(matches: &'a ArgMatches) -> Config<'a> {
        // unwrap() should be safe here because clap does argument validation
        let image1 = matches.value_of("image1").unwrap();
        let image2 = matches.value_of("image2").unwrap();
        let ratio = matches.is_present("ratio");
        let delete = matches.is_present("delete");
        let filename = matches.value_of("filename");

        Config {
            image1,
            image2,
            ratio,
            delete,
            filename,
        }
    }
}

fn safe_load_image(raw_path: &str) -> Result<DynamicImage, String> {
    let path = &Path::new(raw_path);
    if !Path::exists(path) {
        return Err(format!("Path \"{}\" does not exist", raw_path));
    }

    match image::open(path) {
        Ok(image) => Ok(image),
        Err(msg) => Err(format!("{:?}", msg)),
    }
}

pub fn calculate_diff(config: Config) -> Result<f64, String> {
    let image1 = safe_load_image(&config.image1)?;
    let image2 = safe_load_image(&config.image2)?;

    if image1.dimensions() != image2.dimensions() {
        return Err("images must have the same dimensions".to_string());
    }
    if image1.color() != image2.color() {
        return Err("images must have the same color mode".to_string());
    }
    // TODO make this work with all color types, without having to match on all
    let bits: u32 = match image1.color() {
        image::ColorType::RGB(n) => u32::from(n),
        image::ColorType::RGBA(n) => u32::from(n),
        _ => return Err("color mode not supported".to_string()),
    };

    // u32 can handle 2^16 x 2^16 pixel images
    let mut diffsum: u32 = 0;
    for (p1, p2) in image1.raw_pixels().iter().zip(image2.raw_pixels().iter()) {
        let large: u8;
        let small: u8;
        if p1 > p2 {
            large = *p1;
            small = *p2;
        } else {
            large = *p2;
            small = *p1;
        }
        diffsum += u32::from(large - small);
    }
    let total_possible = bits * image1.raw_pixels().len() as u32;
    let ratio = diffsum as f64 / total_possible as f64;

    Ok(ratio)
}
