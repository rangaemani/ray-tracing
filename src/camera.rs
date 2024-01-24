use std::env;
use std::fs::File;
use std::io::Write;

use crate::color::*;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::traceable::*;
use crate::vector::{Point3, Vec3};

/// A camera in the scene responsible for rendering the view.
pub struct Camera {
    /// The aspect ratio of the image (width over height).
    pub aspect_ratio: f64,
    /// The width and height of the image in pixels.
    pub image_dimensions: (usize, usize),
    /// The width of the image in pixels.
    pub image_width: usize,
    /// The height of the image in pixels.
    image_height: usize,
    /// The camera's position in space.
    center: Point3,
    /// The position of the pixel at coordinates (0,0) in space.
    pixel_origin: Point3,
    /// Displacement vector to the next pixel to the right.
    pixel_delta_u: Vec3,
    /// Displacement vector to the next pixel down.
    pixel_delta_v: Vec3,
}

impl Camera {
    /// Creates a new camera instance with default properties.
    pub fn new() -> Self {
        Camera {
            aspect_ratio: 0.0,
            image_width: 0,
            image_height: 0,
            image_dimensions: (0, 0),
            center: Point3::new(0.0, 0.0, 0.0),
            pixel_origin: Point3::new(0.0, 0.0, 0.0),
            pixel_delta_u: Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vec3::new(0.0, 0.0, 0.0),
        }
    }

    /// Determines the color seen in the direction of the given ray.
    ///
    /// If the ray intersects with an object in the world, it computes the color based on the material and lighting.
    /// Otherwise, it returns a gradient color representing the sky.
    ///
    /// # Arguments
    ///
    /// * `ray` - A reference to the ray being cast from the camera.
    /// * `world` - A reference to the world containing traceable objects.
    ///
    /// # Returns
    ///
    /// A `Color` object representing the color seen in the direction of the ray.
    ///
    /// # Examples
    ///
    /// ```
    /// let camera = Camera::new();
    /// let world = World::with_objects(vec![Box::new(Sphere::new())]);
    /// let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 0.0));
    /// let color = camera.get_ray_color(&ray, &world);
    /// ```
    fn get_ray_color(ray: &Ray, world: &Traceables) -> Color {
        let mut record: HitRecord = HitRecord::new();
        // If a ray intersects with the sphere in the scene, return a red color.
        if world.hit(ray, Interval::new(0.0, f64::INFINITY), &mut record) {
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

    /// Initializes the camera properties based on the provided command-line arguments.
    fn initialize(&mut self) {
        self.aspect_ratio = 16.0 / 9.0; // Update this to match the aspect ratio from the guide if needed
        self.image_width = 400; // Set this to the desired width
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as usize;
        self.image_height = self.image_height.max(1); // Ensure at least 1 pixel height
        self.image_dimensions = (self.image_width, self.image_height);
        self.center = Point3::new(0.0, 0.0, 0.0);

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * self.aspect_ratio;

        let viewport_u_vector = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v_vector = Vec3::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u_vector / self.image_width as f64;
        let pixel_delta_v = viewport_v_vector / self.image_height as f64;

        let viewport_origin = self.center
            - Vec3::new(0.0, 0.0, focal_length)
            - viewport_u_vector / 2.0
            - viewport_v_vector / 2.0;
        self.pixel_origin = viewport_origin + 0.5 * pixel_delta_u + 0.5 * pixel_delta_v;
    }

    /// Renders the scene described by `world` to a PPM file.
    ///
    /// Iterates over every pixel in the image and computes the color seen by the camera through ray tracing.
    ///
    /// # Arguments
    ///
    /// * `world` - A `Traceables` object containing the objects in the scene.
    ///
    /// After rendering, it outputs a file named `image.ppm` containing the result.
    pub fn render(&mut self, world: Traceables) {
        self.initialize();

        let mut file = File::create("image.ppm").expect("Could not create file `image.ppm`");

        writeln!(file, "P3\n{} {}\n255", self.image_width, self.image_height)
            .expect("Failed to write PPM header to file");

        for j in 0..self.image_height {
            println!("\rscanlines remaining: {} ", self.image_height - j);
            for i in 0..self.image_width {
                let pixel_center = self.pixel_origin
                    + (i as f64 * self.pixel_delta_u)
                    + (j as f64 * self.pixel_delta_v);
                // Trace ray for each pixel
                let ray_direction = pixel_center - self.center;
                let ray: Ray = Ray::new(self.center, ray_direction);
                let pixel_color = Self::get_ray_color(&ray, &world);

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

        eprintln!("\nDone.");
    }
}
