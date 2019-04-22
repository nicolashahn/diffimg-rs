extern crate clap;
extern crate image;

use clap::ArgMatches;

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

pub fn run(config: Config) -> Result<(), String> {
    println!("{:?}", config);
    match image::open(config.image1) {
        Ok(image1) => println!("{:?}", image1.color()),
        Err(msg) => return Err(format!("{:?}", msg)),
    }
    match image::open(config.image2) {
        Ok(image2) => println!("{:?}", image2.color()),
        Err(msg) => return Err(format!("{:?}", msg)),
    }

    Ok(())
}
