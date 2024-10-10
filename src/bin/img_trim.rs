extern crate image;

use std::fs;
use image::{GenericImageView, imageops::FilterType};
use log::{debug, info};

const TARGET_WIDTH: u32 = 500;
const TARGET_HEIGHT: u32 = 380;
const TARGET_ASPECT_RATIO: f32 = TARGET_WIDTH as f32 / TARGET_HEIGHT as f32;

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    
    let paths = fs::read_dir("./data/original").unwrap();

    for path in paths {
        let path = path.unwrap().path();
        if path.extension().unwrap() == "png" {
            let img = image::io::Reader::open(&path).unwrap()
                .with_guessed_format()
                .unwrap()
                .decode()
                .unwrap();
            let (width, height) = img.dimensions();
            let aspect_ratio = width as f32 / height as f32;

            if width == TARGET_WIDTH && height == TARGET_HEIGHT {
                debug!("Skipping {:?} because it is already 500x380", &path);
                continue
            }
            let (new_width, new_height) = if aspect_ratio > TARGET_ASPECT_RATIO {
                (height as f32 * TARGET_ASPECT_RATIO, height as f32)
            } else {
                (width as f32, width as f32 / TARGET_ASPECT_RATIO)
            };

            let x = (width / 2) - (new_width as u32 / 2);
            let y = (height / 2) - (new_height as u32 / 2);

            let sub_image = img.view(x, y, new_width as u32, new_height as u32).to_image();

            let resized = image::imageops::resize(&sub_image, TARGET_WIDTH, TARGET_HEIGHT, FilterType::CatmullRom);

            resized.save(&path).unwrap();
            info!("Resized {:?} from {}x{} to 500x380", path, width, height);
        }
    }
}
