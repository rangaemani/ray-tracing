#![allow(dead_code)]
use crate::color::{write_color, Color};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::traceable::{Traceable, Traceables};
use rt_math::INFINITY;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::sync::Arc;
use traceable::HitRecord;
use vector::{Point3, Vec3};

mod color;
mod ray;
mod rt_math;
mod sphere;
mod traceable;
mod vector;

fn main() {
    // Set up image properties such as aspect ratio and dimensions.
    let image_dimensions = set_image_configuration();

    // Setup  World
    let mut world: Traceables = Traceables::new();

    world.add(Arc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Configure the camera with its position and compute viewport dimensions.
    let focal_length = 1.0;
    let camera_center = Point3::new(0.0, 0.0, 0.0);
    let viewport_dimensions = set_camera_configuration(image_dimensions);

    // Calculate the viewport and pixel vectors for the camera.
    let vectors = set_vectors(viewport_dimensions, image_dimensions);
    let viewport_vectors = vectors.0;
    let pixel_deltas = vectors.1;

    // Calculate the origin position for the viewport and initial pixel.
    let viewport_origin = camera_center
        - Vec3::new(0.0, 0.0, focal_length)
        - viewport_vectors.0 / 2.0
        - viewport_vectors.1 / 2.0;
    let pixel_origin = viewport_origin + 0.5 * pixel_deltas.0 + pixel_deltas.1;

    // Commence the rendering process for the created scene.
    render(
        pixel_deltas,
        pixel_origin,
        camera_center,
        image_dimensions,
        world,
    );
}

/// Sets up and returns the image configuration, which includes the image width
/// and height based on a fixed aspect ratio to ensure that the generated image
/// maintains a correct proportion.
///
/// # Returns
///
/// A tuple containing the image width and height in pixels.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// let (width, height) = set_image_configuration();
/// println!("Image width: {}px, height: {}px", width, height);
/// ```
fn set_image_configuration() -> (usize, usize) {
    // Define the image aspect ratio and initial width.
    let aspect_ratio = 16.0 / 9.0;
    let image_width: usize = env::args()
        .nth(1)
        .expect("No image width argument provided")
        .parse()
        .expect("Invalid image width");
    // Calculate the necessary height to maintain the aspect ratio.
    let image_height: usize = (image_width as f32 / aspect_ratio) as usize;
    // Ensure the image height is at least 1 pixel tall.
    let image_height = if image_height < 1 { 1 } else { image_height };
    (image_width, image_height)
}

/// Determines the necessary width and height for a camera's viewport given the
/// image dimensions such that the viewport maintains the same aspect ratio as
/// the image.
///
/// # Arguments
///
/// * `dimensions` - A tuple `(width, height)` representing the width and height
///   of an image in pixels.
///
/// # Returns
///
/// A tuple `(viewport_width, viewport_height)` where `viewport_width` is the
/// width of the camera's viewport and `viewport_height` is a pre-established
/// height used to calculate the corresponding width for a correct aspect ratio.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// let image_dimensions = (800, 600);
/// let viewport_dimensions = set_camera_configuration(image_dimensions);
/// println!("Viewport width: {}, Viewport height: {}", viewport_dimensions.0, viewport_dimensions.1);
/// ```
fn set_camera_configuration(dimensions: (usize, usize)) -> (f32, f32) {
    // Extract the image dimensions.
    let image_width = dimensions.0;
    let image_height = dimensions.1;
    // Establish a fixed viewport height.
    let viewport_height = 2.0;
    // Calculate the viewport width based on the image aspect ratio.
    let viewport_width = viewport_height * (image_width as f32 / image_height as f32) as f32;
    (viewport_width, viewport_height)
}

/// Calculates viewport vectors and pixel delta vectors for a given viewport and image dimensions.
///
/// The viewport vectors represent the horizontal and vertical dimensions of the viewport plane.
/// The pixel delta vectors are the vector increments needed to move from one pixel to the next
/// in the u and v directions on the viewport plane.
///
/// # Arguments
///
/// * `viewport_dimensions` - A tuple `(width, height)` representing the width and height
///   of the viewport in scene units.
/// * `image_dimensions` - A tuple `(width, height)` representing the width and height
///   of the output image in pixels.
///
/// # Returns
///
/// A tuple of tuples: ((viewport_u_vector, viewport_v_vector), (pixel_delta_u, pixel_delta_v))
/// where the first element contains the u and v vectors of the viewport, and the second element
/// contains the u and v pixel delta vectors.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// // Dimensions in floating-point for the viewport
/// let viewport_dimensions = (3.0, 2.0);
/// // Pixel dimensions of the image
/// let image_dimensions = (1920, 1080);
///
/// let vectors = set_vectors(viewport_dimensions, image_dimensions);
///
/// let viewport_vectors = vectors.0; // Contains viewport_u_vector and viewport_v_vector
/// let pixel_deltas = vectors.1;     // Contains pixel_delta_u and pixel_delta_v
/// ```
fn set_vectors(
    viewport_dimensions: (f32, f32),
    image_dimensions: (usize, usize),
) -> ((Vec3, Vec3), (Vec3, Vec3)) {
    // Compute the vectors defining the viewport's horizontal and vertical size.
    let viewport_u_vector = Vec3::new(viewport_dimensions.0 as f64, 0.0, 0.0);
    let viewport_v_vector = Vec3::new(0.0, -viewport_dimensions.1 as f64, 0.0);

    // Determine the size change between each pixel for the u and v vectors.
    let pixel_delta_u = viewport_u_vector / image_dimensions.0 as f64;
    let pixel_delta_v = viewport_v_vector / image_dimensions.1 as f64;
    (
        (viewport_u_vector, viewport_v_vector),
        (pixel_delta_u, pixel_delta_v),
    )
}

/// Determines the color seen in the direction of the given ray.
///
/// If the ray intersects with a predefined sphere in the scene, the function returns
/// a red color. If there is no intersection, it calculates the color based on a linear
/// interpolation between white and light blue, creating a gradient sky effect. The
/// blending factor is determined by the vertical angle of the ray's direction vector.
///
/// # Arguments
///
/// * `ray` - A reference to a `Ray` object.
///
/// # Returns
///
/// A `Color` object representing the color seen in the direction of the ray.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 0.0));
/// let color = get_ray_color(&ray);
/// ```
fn get_ray_color(ray: &Ray, world: &Traceables) -> Color {
    let mut record: HitRecord = HitRecord::new();
    // If a ray intersects with the sphere in the scene, return a red color.
    if world.hit(ray, 0.0, INFINITY, &mut record) {
        return 0.5 * (record.normal() + Color::new(1.0, 1.0, 1.0));
    } else {
        // Normalize the ray's direction vector.
        let unit_direction = ray.direction().normalize();
        // Calculate blending factor for color interpolation.
        let blend_factor = 0.5 * (unit_direction.y() + 1.0);
        // Linearly interpolate between white and light blue colors based on the blend factor.
        return (1.0 - blend_factor) * Color::new(1.0, 1.0, 1.0)
            + blend_factor * Color::new(0.5, 0.7, 1.0);
    }
}

/// Renders an image in PPM format by tracing rays from a given point through
/// each pixel defined by the pixel origin and pixel deltas, writes the color
/// data to a PPM file, and flushes the file after every pixel update to ensure
/// data is not lost.
///
/// This function will create a PPM file named "image.ppm" and write the pixel
/// data as a series of lines, each representing a pixel color. It starts by
/// writing the header necessary for PPM files, followed by the pixel color
/// data. The pixel color for each ray shot through the scene is determined
/// using a ray-tracing algorithm.
///
/// The terminal will display the number of remaining scanlines during rendering
/// to provide feedback on progress.
///
/// # Arguments
///
/// * `pixel_deltas` - A tuple of `Vec3` representing the step increments for each pixel along
///   the horizontal and vertical axes in the image plane.
/// * `pixel_origin` - A `Vec3` representing the starting point for the tracing of rays through
///   the viewport's pixels.
/// * `camera_center` - A `Vec3` representing the camera's location in world space.
/// * `image_dimensions` - A tuple `(usize, usize)` representing the width and height of the
///   output image in pixels.
///
/// # Panics
///
/// The function panics if it is unable to create, write to, or flush the PPM file.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// // Assuming pixel_deltas, pixel_origin, camera_center, and image_dimensions have been defined
/// render(pixel_deltas, pixel_origin, camera_center, image_dimensions);
/// ```
fn render(
    pixel_deltas: (Vec3, Vec3),
    pixel_origin: Vec3,
    camera_center: Vec3,
    image_dimensions: (usize, usize),
    world: Traceables,
) {
    let mut file = match File::create("image.ppm") {
        Ok(file) => file,
        Err(e) => panic!("could not create file: {}", e),
    };

    let (image_width, image_height) = image_dimensions;

    // Compose header data for PPM format.
    let ppm_buffer = format!("P3\n{} {}\n255\n", image_width, image_height);

    // Write the PPM header to the file.
    match file.write_all(ppm_buffer.as_bytes()) {
        Ok(_t) => _t,
        Err(e) => panic!("failed to write to file: {}", e),
    };

    // Ensure all the header data is written before pixel data.
    match file.flush() {
        Ok(_f) => _f,
        Err(e) => panic!("failed to flush file: {}", e),
    }

    // Generate the image by iterating over each pixel and calculating its color.
    for j in 0..image_height {
        println!("\rscanlines remaining: {} ", image_height - j);
        for i in 0..image_width {
            let pixel_center =
                pixel_origin + (i as f64 * pixel_deltas.0) + (j as f64 * pixel_deltas.1);
            // Trace ray for each pixel
            let ray_direction = pixel_center - camera_center;
            let ray: Ray = Ray::new(camera_center, ray_direction);
            let pixel_color = get_ray_color(&ray, &world);

            // Convert the pixel color to a string in PPM format.
            let rgb_buffer = write_color(pixel_color);

            // Write the color data for the current pixel to the PPM file.
            match file.write_all(rgb_buffer.as_bytes()) {
                Ok(_t) => _t,
                Err(e) => panic!("failed to write to file: {}", e),
            };

            // Flush the file buffer after writing each pixel to ensure data is not lost.
            match file.flush() {
                Ok(_f) => _f,
                Err(e) => panic!("failed to flush file buffer: {}", e),
            }
        }
    }
    println!("image complete");
}
