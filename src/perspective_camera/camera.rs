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
use rayon::prelude::*;
use std::io::{self};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
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
    /// Variation angle of rays going through each pixel
    pub defocus_angle: f64,
    /// Distance from camera origin point to plane of ideal focus
    pub focus_distance: f64,
    /// Where camera is looking from
    pub camera_origin: Point3,
    /// What camera is looking at
    pub camera_target: Point3,
    /// Vector pointing up
    pub up_vector: Vec3,
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
    /// Defocus disc horizontal radius
    defocus_disc_u: Vec3,
    /// Defocus disc vertical radius
    defocus_disc_v: Vec3,
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
            defocus_angle: 0.0,
            focus_distance: 0.0,
            camera_origin: Point3::from(0.0, 0.0, -1.0),
            camera_target: Point3::from(0.0, 0.0, 0.0),
            up_vector: Vec3::from(0.0, 1.0, 0.0),
            pixel_origin: Point3::new(),
            pixel_delta_u: Vec3::new(),
            pixel_delta_v: Vec3::new(),
            defocus_disc_u: Vec3::new(),
            defocus_disc_v: Vec3::new(),
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

        let ray_origin = match self.defocus_angle <= 0.0 {
            true => self.center,
            _ => self.sample_defocus_disc(),
        };
        let ray_direction = pixel_sample - ray_origin;

        let ray_time: f64 = random_number();
        return Ray::from(ray_origin, ray_direction, ray_time);
    }

    fn sample_defocus_disc(&self) -> Point3 {
        // return random point in camera defocus disk
        let point: Point3 = Vec3::random_unit_disk_point();
        return self.center + point.x() * self.defocus_disc_u + point.y() * self.defocus_disc_v;
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

        // Calculate viewport dimension
        let theta: f64 = degrees_to_radians(self.vfov);
        let height_component = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * height_component * self.focus_distance as f64;
        let viewport_width = viewport_height * self.image_width as f64 / self.image_height as f64;

        // Calculate basis vectors for camera coordinates
        self.w = (self.camera_origin - self.camera_target).normalize();
        self.u = Vec3::cross(&self.up_vector, &self.w).normalize();
        self.v = Vec3::cross(&self.w, &self.u);

        // Calculate horizontal & vertical viewport edge vectors
        let viewport_u_vector = viewport_width * self.u;
        let viewport_v_vector = viewport_height * -self.v;

        // Calculate horizontal and vertical pixel delta vectors
        self.pixel_delta_u = viewport_u_vector / self.image_width as f64;
        self.pixel_delta_v = viewport_v_vector / self.image_height as f64;

        // Calculate location of origin pixel [top left (0, 0)]
        let viewport_origin = self.center
            - (self.focus_distance as f64 * self.w)
            - viewport_u_vector / 2.0
            - viewport_v_vector / 2.0;
        self.pixel_origin = viewport_origin + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
        // Calculate camera defocus disk basis vectors
        let defocus_radius = self.focus_distance as f64
            * f64::tan(degrees_to_radians(self.defocus_angle as f64 / 2.0));
        self.defocus_disc_u = self.u * defocus_radius;
        self.defocus_disc_v = self.v * defocus_radius;
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

    pub fn render(&mut self, world: Arc<Traceables>) {
        self.initialize();

        let mut file = File::create("images/image.ppm").expect("Could not create file `image.ppm`");

        writeln!(file, "P3\n{} {}\n255", self.image_width, self.image_height)
            .expect("Failed to write PPM header to file");

        let total_scanlines = self.image_height;
        let scanlines_done = Arc::new(AtomicUsize::new(0));

        // Process each scanline in parallel
        let pixels: Vec<_> = (0..self.image_height)
            .into_par_iter()
            .map(|j| {
                let mut scanline_data = Vec::with_capacity(self.image_width);

                for i in 0..self.image_width {
                    let mut pixel_color = Color::new();
                    for _sample in 0..self.pixel_samples {
                        let ray: Ray = self.get_ray(i, j);
                        pixel_color += Self::get_ray_color(&ray, self.max_depth, &world);
                    }
                    // Convert the pixel color to a string in PPM format.
                    scanline_data.push(write_color(pixel_color, self.pixel_samples));
                }

                // Update progress
                let completed_scanlines = scanlines_done.fetch_add(1, Ordering::SeqCst) + 1;
                let scanline_percentage =
                    (completed_scanlines as f64 / total_scanlines as f64) * 100.0;

                // Create the progress bar.
                let scanline_progress_bar =
                    "∻".repeat((scanline_percentage / 100.0 * 50.0) as usize);

                // Print the progress bar.
                eprintln!(
                    "\rｓｃａｎｌｉｎｅｓ　ｔｒａｃｅｄ: {}/{} 『{}』 {:.1}%",
                    completed_scanlines,
                    total_scanlines,
                    scanline_progress_bar,
                    scanline_percentage,
                );

                scanline_data
            })
            .collect();

        // Write all the pixel data to the file at once
        for scanline in pixels {
            for pixel in scanline {
                file.write_all(pixel.as_bytes())
                    .expect("Failed to write to file");
            }
        }

        eprintln!("\nDone.");
    }
}
