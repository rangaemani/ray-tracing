use std::env;
use std::fs::File;
use std::io::Write;

use rand::random;

use crate::color::{self, *};
use crate::math::interval::Interval;
use crate::math::rt_math::{degrees_to_radians, random_number};
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
    /// How many anti-aliasing samples
    pub pixel_samples: usize,
    /// Maximum number of ray bounces
    pub max_depth: usize,
    /// Vertical FOV
    pub vfov: f64,
    /// The height of the image in pixels.
    image_height: usize,
    /// The camera's position in space.
    center: Point3,
    /// Where camera is looking from
    pub camera_origin: Point3,
    /// What camera is looking at
    pub camera_target: Point3,
    /// Vector pointing up
    pub up_vector: Vec3,
    /// The position of the pixel at coordinates (0,0) in space.
    pixel_origin: Point3,
    /// Displacement vector to the next pixel to the right.
    pixel_delta_u: Vec3,
    /// Displacement vector to the next pixel down.
    pixel_delta_v: Vec3,
    /// Frame basis vectors
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

impl Camera {
    /// Creates a new camera instance with default properties.
    pub fn new() -> Self {
        Camera {
            aspect_ratio: 1.0,
            image_width: 100,
            image_height: 0,
            image_dimensions: (0, 0),
            center: Point3::new(),
            camera_origin: Point3::from(0.0, 0.0, -1.0),
            camera_target: Point3::from(0.0, 0.0, 0.0),
            up_vector: Vec3::from(0.0, 1.0, 0.0),
            pixel_origin: Point3::new(),
            pixel_delta_u: Vec3::new(),
            pixel_delta_v: Vec3::new(),
            pixel_samples: 10,
            max_depth: 10,
            vfov: 90.0,
            u: Vec3::new(),
            v: Vec3::new(),
            w: Vec3::new(),
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
    fn get_ray_color(ray: &Ray, depth: usize, world: &Traceables) -> Color {
        let mut record: HitRecord = HitRecord::new();
        if depth <= 0 {
            return Color::new();
        }
        // If a ray intersects with the sphere in the scene, return a color.
        if world.hit(ray, Interval::new(0.1, f64::INFINITY), &mut record) {
            let mut scattered_ray: Ray = Ray::new();
            let mut attenuation: Color = Color::new();
            if record
                .material()
                .scatter(ray, &record, &mut attenuation, &mut scattered_ray)
            {
                return attenuation * Self::get_ray_color(&scattered_ray, depth - 1, world);
            }
            return Color::new();
        } else {
            // Normalize the ray's direction vector.
            let unit_direction: Vec3 = ray.direction().normalize();
            // Calculate blending factor for color interpolation.
            let blend_factor = 0.5 * (unit_direction.y() + 1.0);
            // Linearly interpolate between white and light blue colors based on the blend factor.
            return (1.0 - blend_factor) * Color::from(1.0, 1.0, 1.0)
                + blend_factor * Color::from(0.5, 0.7, 1.0);
        }
    }

    fn get_ray(&self, i: usize, j: usize) -> Ray {
        // gets randomly sampled ray for pixel at (i, j)
        let pixel_center =
            self.pixel_origin + (i as f64 * self.pixel_delta_u) + (j as f64 * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;
        return Ray::from(ray_origin, ray_direction);
    }

    fn pixel_sample_square(&self) -> Vec3 {
        // returns random point in square surrounding origin pixel
        let px = -0.5 + random_number();
        let py = -0.5 + random_number();
        return (px * self.pixel_delta_u) + (py * self.pixel_delta_v);
    }

    /// Initializes the camera properties based on the provided command-line arguments.
    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as usize;
        self.image_height = self.image_height.max(1);

        self.center = self.camera_origin;

        self.image_dimensions = (self.image_width, self.image_height);
        self.image_dimensions = (self.image_width, self.image_height);
        self.center = Point3::new();
        // Calculate viewport dimension
        let focal_length = (self.camera_origin - self.camera_target).length();
        let theta: f64 = degrees_to_radians(self.vfov);
        let height_component = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * height_component * focal_length;
        let viewport_width = viewport_height * self.aspect_ratio;

        // Calculate basis vectors for camera coordinates
        self.w = (self.camera_origin - self.camera_target).normalize();
        self.v = Vec3::cross(&self.up_vector, &self.w);
        self.u = Vec3::cross(&self.w, &self.u);

        // Calculate horizontal & vertical viewport edge vectors
        let viewport_u_vector = viewport_width * self.u;
        let viewport_v_vector = viewport_height * -self.v;

        // Calculate horizontal and vertical pixel delta vectors
        self.pixel_delta_u = viewport_u_vector / self.image_width as f64;
        self.pixel_delta_v = viewport_v_vector / self.image_height as f64;

        // Calculate location of origin pixel [top left (0, 0)]
        let viewport_origin = self.center
            - (focal_length * self.w)
            - viewport_u_vector / 2.0
            - viewport_v_vector / 2.0;
        self.pixel_origin = viewport_origin + 0.5 * self.pixel_delta_u + 0.5 * self.pixel_delta_v;
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

        let total_scanlines = self.image_height;

        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let mut pixel_color = Color::new();
                for _sample in 0..self.pixel_samples {
                    let ray: Ray = self.get_ray(i, j);
                    pixel_color += Self::get_ray_color(&ray, self.max_depth, &world);
                }
                // Convert the pixel color to a string in PPM format.
                let rgb_buffer = write_color(pixel_color, self.pixel_samples);

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

                // Calculate the percentage of completion for the current scanline.
                let completed_scanlines = j as f64;
                let scanline_percentage = (completed_scanlines / total_scanlines as f64) * 100.0;

                // Calculate the percentage of completion for the current pixel within the scanline.
                let completed_pixels = i as f64;
                let pixel_percentage = (completed_pixels / self.image_width as f64) * 100.0;

                // Create the progress bars.
                let scanline_progress_bar =
                    "∻".repeat((scanline_percentage / 100.0 * 50.0) as usize);
                let pixel_progress_bar = "≖".repeat((pixel_percentage / 100.0 * 50.0) as usize);

                // Print the progress bars.
                print!(
                    "\rlines traced: {}/{} 『{}』 | scanline progress: {:.1}% «{}» ",
                    completed_scanlines,
                    total_scanlines,
                    scanline_progress_bar,
                    pixel_percentage,
                    pixel_progress_bar,
                );
            }
        }

        eprintln!("\nDone.");
    }
}
