extern crate clap;
use clap::{App, Arg};

use diffimg_rs::Config;

fn main() {
    let matches = App::new("diffimg")
        .version("1.0")
        .author("Nicolas Hahn <nicolas@stonespring.org>")
        .about("Calculate the percent/ratio difference of an image, or generate a diff image.")
        .arg(
            Arg::with_name("image1")
                .help("First image to diff")
                .index(1)
                .required(true),
        )
        .arg(
            Arg::with_name("image2")
                .help("Second image to diff")
                .index(2)
                .required(true),
        )
        .arg(
            Arg::with_name("ratio")
                .help("Output a ratio instead of a percentage")
                .short("r")
                .long("ratio"),
        )
        .arg(
            Arg::with_name("delete")
                .help("Output a ratio instead of a percentage")
                .short("d")
                .long("delete"),
        )
        .arg(
            Arg::with_name("filename")
                .help("Filename to save the diff file as")
                .short("f")
                .long("filename")
                .takes_value(true),
        )
        .get_matches();

    // We're relying on clap to correctly validate all args so we don't need to use Result here
    let config = Config::from_clap_matches(&matches);

    if let Err(msg) = diffimg_rs::run(config) {
        println!("Failed to diff images, error: {}", msg);
    }
}

// using docopt instead of clap
/*
use docopt::Docopt;
use serde::Deserialize;

const USAGE: &'static str = "
diffimg_rs - Calculate the percent/ratio difference of an image, or generate a diff image.

Usage:
    diffimg_rs <im1> <im2> [-r/--ratio] [-d/--delete] [--filename=<name>]
    diffimg_rs (-h | --help)
    diffimg_rs --version

Options:
    -h --help               Show this screen.
    --version               Show version.
    -r --ratio              Output a ratio from 0 to 1 instead of a percentage.
    -d --delete             Delete the diff image produced for the calculation.
    --filename=<filename>   The filename to save the diff image as (unless --delete)
                            is passed. (default: diff_img.png)

";

#[derive(Debug, Deserialize)]
pub struct Args {
    pub arg_im1: String,
    pub arg_im2: String,
    pub flag_ratio: bool,
    pub flag_delete: bool,
    pub flag_filename: String,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
    println!("{:?}", args);
}
*/
