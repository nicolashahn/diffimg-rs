extern crate image;

use std::fs;

static MARIO_NODE: &str = "images/mario-circle-node.png";
static MARIO_CS: &str = "images/mario-circle-cs.png";
static MARIO_DIFF: &str = "images/mario-diff.png";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_diff_mario() {
        let image1 = image::open(MARIO_CS).unwrap();
        let image2 = image::open(MARIO_NODE).unwrap();
        assert_eq!(
            0.007319618135968298,
            diffimg::calculate_diff_ratio(image1, image2)
        )
    }

    #[test]
    fn test_create_diff_image_mario() {
        let mario_diff_test = "tests/mario-diff.png";
        let image1 = image::open(MARIO_CS).unwrap();
        let image2 = image::open(MARIO_NODE).unwrap();
        diffimg::create_diff_image(image1, image2, mario_diff_test);
        assert_eq!(
            image::open(mario_diff_test).unwrap().raw_pixels(),
            image::open(MARIO_DIFF).unwrap().raw_pixels()
        );
        if let Err(_) = fs::remove_file(mario_diff_test) {
            panic!("Could not remove test file");
        };
    }

}
