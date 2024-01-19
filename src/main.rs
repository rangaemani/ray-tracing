use vector::{Point3, Vec3};

use crate::color::{write_color, Color};
use crate::ray::Ray;
use std::fs::File;
use std::io::prelude::*;

mod color;
mod ray;
mod vector;

fn main() {
    // image setup
    let image_dimensions = set_image_configuration();

    // camera setup
    let focal_length = 1.0;
    let camera_center = Point3::new(0.0, 0.0, 0.0);
    let viewport_dimensions = set_camera_configuration(image_dimensions);

    // vector calculation [0 index -> u, 1 index -> v]
    let vectors = set_vectors(viewport_dimensions, image_dimensions);
    let viewport_vectors = vectors.0;
    let pixel_deltas = vectors.1;
    // calculate pixel origin
    let viewport_origin = camera_center
        - Vec3::new(0.0, 0.0, focal_length)
        - viewport_vectors.0 / 2.0
        - viewport_vectors.1 / 2.0;
    let pixel_origin = viewport_origin + 0.5 * pixel_deltas.0 + pixel_deltas.1;

    // render logic
    render(pixel_deltas, pixel_origin, camera_center);
}

fn set_image_configuration() -> (usize, usize) {
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 400;
    let image_height: usize = (image_width as f32 / aspect_ratio) as usize;
    let image_height = if image_height < 1 { 1 } else { image_height };
    (image_width, image_height)
}

fn set_camera_configuration(dimensions: (usize, usize)) -> (f32, f32) {
    let image_width = dimensions.0;
    let image_height = dimensions.1;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width / image_height) as f32;
    (viewport_width, viewport_height)
}

fn set_vectors(
    viewport_dimensions: (f32, f32),
    image_dimensions: (usize, usize),
) -> ((Vec3, Vec3), (Vec3, Vec3)) {
    // calculate viewport vectors
    let viewport_u_vector = Vec3::new(viewport_dimensions.0 as f64, 0.0, 0.0);
    let viewport_v_vector = Vec3::new(0.0, -viewport_dimensions.1 as f64, 0.0);

    // calculate pixel delta vectors
    let pixel_delta_u = viewport_u_vector / image_dimensions.0 as f64;
    let pixel_delta_v = viewport_v_vector / image_dimensions.1 as f64;
    (
        (viewport_u_vector, viewport_v_vector),
        (pixel_delta_u, pixel_delta_v),
    )
}

fn get_ray_color(ray: &Ray) -> Color {
    ray.unit_vector();
    let offset = 0.5 * (ray.direction().y() / 2.0 + 1.0);
    return (1.0 - offset) * Color::new(1.0, 1.0, 1.0) + offset * Color::new(0.5, 0.7, 1.0);
}

// creates a 256 x 256 ppm file
fn render(pixel_deltas: (Vec3, Vec3), pixel_origin: Vec3, camera_center: Vec3) {
    let mut file = match File::create("image.ppm") {
        Ok(file) => file,
        Err(e) => panic!("could not create file: {}", e),
    };
    // image dimensions
    const IMAGE_WIDTH: u16 = 512;
    const IMAGE_HEIGHT: u16 = 512;

    // data formatting
    let ppm_buffer = format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    // image logic
    match file.write_all(ppm_buffer.as_bytes()) {
        Ok(_t) => _t,
        Err(e) => panic!("failed to write to file: {}", e),
    };
    // force flush file buffer to ensure consistent read/write
    match file.flush() {
        Ok(_f) => _f,
        Err(e) => panic!("failed to flush file: {}", e),
    }
    // iteratively create gradient
    for j in 0..IMAGE_HEIGHT {
        println!("\rscanlines remaining: {} ", IMAGE_HEIGHT - j);
        for i in 0..IMAGE_WIDTH {
            let pixel_center =
                pixel_origin + (i as f64 * pixel_deltas.0) + (j as f64 * pixel_deltas.1);
            let ray_direction = pixel_center - camera_center;
            let r: Ray = Ray::new(camera_center, ray_direction);
            let pixel_color = get_ray_color(&r);

            let rgb_buffer = write_color(pixel_color);
            // file write
            match file.write_all(rgb_buffer.as_bytes()) {
                Ok(_t) => _t,
                Err(e) => panic!("failed to write to file: {}", e),
            };
            // buffer flush
            match file.flush() {
                Ok(_f) => _f,
                Err(e) => panic!("failed to flush file buffer: {}", e),
            }
        }
    }
    println!("image complete");
}
