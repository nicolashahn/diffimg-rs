extern crate clap;

use std::process::exit;

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

    // We're relying on clap to correctly validate all args,
    // so we shouldn't need to use Result here
    let config = Config::from_clap_matches(&matches);

    match diffimg_rs::calculate_diff(config) {
        Ok(ratio) => {
            println!("{}", ratio);
            exit(0)
        }
        Err(msg) => {
            println!("Error: {}", msg);
            exit(1)
        }
    }
}
