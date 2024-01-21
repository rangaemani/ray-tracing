use vector::{Point3, Vec3};

use crate::color::{write_color, Color};
use crate::ray::Ray;
use crate::vector::dot;
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
    render(pixel_deltas, pixel_origin, camera_center, image_dimensions);
}

fn set_image_configuration() -> (usize, usize) {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1920;
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
    if detect_sphere_intersection(Point3::new(0.0, 0.0, -1.0), 0.5, ray) {
        return Color::new(1.0, 0.0, 0.0);
    }
    // normalize ray direction
    let unit_direction = ray.direction().unit_vector();
    let blend_factor = 0.5 * (unit_direction.y() + 1.0); // interpolates value between two colors. in this case, blue & white
    return (1.0 - blend_factor) * Color::new(1.0, 1.0, 1.0) // standard lerp
        + blend_factor * Color::new(0.5, 0.7, 1.0);
}

/// Determines whether a ray intersects a sphere.
///
/// This function implements the sphere intersection test in ray tracing.
/// Given a ray and a sphere (defined by its center and radius), it computes
/// whether the ray intersects the sphere.
///
/// The intersection test is based on solving the quadratic equation derived from
/// the geometric relationship between the ray and the sphere. The equation is:
///
/// `(ray.origin() - center)^2 = radius^2`
///
/// Expanding and rearranging terms gives the quadratic equation:
///
/// `a*t^2 + b*t + c = 0`
///
/// where:
/// - `a` is the square of the length of the ray direction vector,
/// - `b` is twice the dot product of the ray direction vector and the vector from the ray origin to the sphere center,
/// - `c` is the square of the distance from the ray origin to the sphere center minus the square of the sphere radius.
///
/// The roots of this equation represent the distances from the ray origin to the intersection points.
/// If the discriminant `b^2 - 4ac` is greater than or equal to zero, the ray intersects the sphere.
///
/// # Arguments
///
/// * `center` - The center point of the sphere.
/// * `radius` - The radius of the sphere.
/// * `ray` - The ray to check for intersection with the sphere.
///
/// # Returns
///
/// `true` if the ray intersects the sphere, `false` otherwise.
/// Determines whether a ray intersects a sphere.
///
/// This function implements the sphere intersection test in ray tracing.
/// Given a ray and a sphere (defined by its center and radius), it computes
/// whether the ray intersects the sphere.
///
/// The intersection test is based on solving the quadratic equation derived from
/// the geometric relationship between the ray and the sphere. The equation is:
///
/// `(ray.origin() - center)^2 = radius^2`
///
/// Expanding and rearranging terms gives the quadratic equation:
///
/// `a*t^2 + b*t + c = 0`
///
/// where:
/// - `a` is the square of the length of the ray direction vector,
/// - `b` is twice the dot product of the ray direction vector and the vector from the ray origin to the sphere center,
/// - `c` is the square of the distance from the ray origin to the sphere center minus the square of the sphere radius.
///
/// The roots of this equation represent the distances from the ray origin to the intersection points.
/// If the discriminant `b^2 - 4ac` is greater than or equal to zero, the ray intersects the sphere.
///
/// # Arguments
///
/// * `center` - The center point of the sphere.
/// * `radius` - The radius of the sphere.
/// * `ray` - The ray to check for intersection with the sphere.
///
/// # Returns
///
/// `true` if the ray intersects the sphere, `false` otherwise.
fn detect_sphere_intersection(center: Point3, radius: f64, ray: &Ray) -> bool {
    let origin_offset: Vec3 = ray.origin() - center;
    let a = dot(&ray.direction(), &ray.direction());
    let b = 2.0 * dot(&origin_offset, &ray.direction());
    let c = dot(&origin_offset, &origin_offset) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    return discriminant >= 0.0;
}

// creates a 256 x 256 ppm file
fn render(
    pixel_deltas: (Vec3, Vec3),
    pixel_origin: Vec3,
    camera_center: Vec3,
    image_dimensions: (usize, usize),
) {
    let mut file = match File::create("image.ppm") {
        Ok(file) => file,
        Err(e) => panic!("could not create file: {}", e),
    };
    // image dimensions
    let (image_width, image_height) = image_dimensions;

    // data formatting
    let ppm_buffer = format!("P3\n{} {}\n255\n", image_width, image_height);

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
    for j in 0..image_height {
        println!("\rscanlines remaining: {} ", image_height - j);
        for i in 0..image_width {
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
