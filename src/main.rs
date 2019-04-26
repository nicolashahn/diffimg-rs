extern crate clap;

use std::process::exit;

use clap::{App, Arg};

use diffimg_rs::Config;

fn main() {
    let matches = App::new("diffimg")
        .version("1.0")
        .author("Nicolas Hahn <nicolas@stonespring.org>")
        .about("Calculate the ratio difference of an image, or generate a diff image.")
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
            Arg::with_name("filename")
                .help("If present, save a diff image to this filename. Currently, only .png is supported.")
                .short("f")
                .long("filename")
                .takes_value(true),
        )
        .get_matches();

    // We're relying on clap to correctly validate all args,
    // so we shouldn't need to use Result here
    let config = Config::from_clap_matches(&matches);
    if let Err(msg) = diffimg_rs::run(config) {
        println!("Error: {:?}", msg);
        exit(1);
    };
}
