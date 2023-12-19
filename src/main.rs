use std::fs::File;
use std::io::prelude::*;

use crate::color::{write_color, Color};

mod color;
mod ray;
mod vector;
fn main() {
    create_ppm();
}

// creates a 256 x 256 ppm file
fn create_ppm() {
    let mut file = match File::create("image.ppm") {
        Ok(file) => file,
        Err(e) => panic!("could not create file: {}", e),
    };
    // image dimensions
    const IMAGE_WIDTH: u16 = 512;
    const IMAGE_HEIGHT: u16 = 512;

    let ppm_buffer = format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    // image logic
    match file.write_all(ppm_buffer.as_bytes()) {
        Ok(_t) => _t,
        Err(e) => panic!("failed to write to file: {}", e),
    };

    match file.flush() {
        Ok(_f) => _f,
        Err(e) => panic!("failed to flush file: {}", e),
    }

    for j in 0..IMAGE_HEIGHT {
        println!("\rscanlines remaining: {} ", IMAGE_HEIGHT - j);
        for i in 0..IMAGE_WIDTH {
            let pixel_color = Color::new(
                i as f64 / (IMAGE_WIDTH - 1) as f64,
                j as f64 / (IMAGE_HEIGHT - 1) as f64,
                0.0,
            );

            let rgb_buffer = write_color(pixel_color);

            match file.write_all(rgb_buffer.as_bytes()) {
                Ok(_t) => _t,
                Err(e) => panic!("failed to write to file: {}", e),
            };

            match file.flush() {
                Ok(_f) => _f,
                Err(e) => panic!("failed to flush file: {}", e),
            }
        }
    }
    println!("image complete");
}
