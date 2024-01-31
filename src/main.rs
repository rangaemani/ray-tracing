#![allow(dead_code)]
#![allow(unused_imports)]
use crate::drawable::*;
use crate::materials::*;
use crate::math::*;
use crate::perspective_camera::camera::Camera;
use crate::perspective_camera::camera::*;
use crate::traceable::{Traceable, Traceables};
use crate::vectors::*;
use drawable::{sphere::Sphere, traceable::HitRecord};
use materials::{dielectric::Dielectric, lambert::Lambertian, material::Material, metal::Metal};
use math::rt_math::random_number_in_range;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{
    path::Path,
    process::Command,
    {env, f64::consts::PI, fs::File, io::prelude::*, sync::Arc, sync::Mutex},
};
use vectors::{
    color::Color,
    ray::Ray,
    vector::{Point3, Vec3},
};

mod drawable;
mod materials;
mod math;
mod perspective_camera;
mod vectors;

fn main() {
    if Path::exists(&Path::new("images/image.ppm")) {
        std::fs::remove_file("images/image.ppm").unwrap();
    }
    // Setup World
    // The world consists of a ground sphere and a number of randomly placed spheres with different materials.
    let world = Mutex::new(Traceables::new());
    let material_ground = Arc::new(Lambertian::from(Color::from_rgb(127, 127, 127)));
    world.lock().unwrap().add(Arc::new(Sphere::from(
        Point3::from(0.0, -1000.0, -1.0),
        1000.0,
        material_ground,
    )));

    // Create a new, empty world for the randomly placed spheres.

    // Generate a grid of spheres with random positions and materials.
    // Each sphere is given a random radius and position within the grid cell.
    // The radius and position are chosen such that the spheres do not overlap.
    // (0..=20).into_par_iter().for_each(|_| {
    (-11..11).into_par_iter().for_each(|a| {
        (-11..11).into_par_iter().for_each(|b| {
            let material_factor = rand::random::<f64>();
            let center = Point3::from(
                a as f64 + 0.9 * rand::random::<f64>(),
                0.2,
                b as f64 + 0.9 * rand::random::<f64>(),
            );

            if (center - Point3::from(4.0, 0.2, 0.0)).length() > 0.9 {
                let material: Arc<dyn Material>;

                // Choose a material for the sphere based on a random number.
                // The material can be diffuse, metal, or glass.
                if material_factor < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    material = Arc::new(Lambertian::from(albedo));
                    let target_center =
                        center + Vec3::from(0.0, random_number_in_range(0.0, 0.5), 0.0);
                    // Add the sphere to the world.
                    world.lock().unwrap().add(Arc::new(Sphere::new_in_motion(
                        center,
                        target_center,
                        0.2,
                        material,
                    )));
                } else if material_factor < 0.95 {
                    // metal
                    let albedo = Color::random_in_range(0.5, 1.0);
                    let fuzz = rand::random::<f64>() * 0.5;
                    material = Arc::new(Metal::from(albedo, fuzz));
                    // Add the sphere to the world.
                    world
                        .lock()
                        .unwrap()
                        .add(Arc::new(Sphere::from(center, 0.2, material)));
                } else {
                    // glass
                    material = Arc::new(Dielectric::from(1.5));
                    // Add the sphere to the world.
                    world
                        .lock()
                        .unwrap()
                        .add(Arc::new(Sphere::from(center, 0.2, material)));
                }
            }
        });
    });
    // });

    let material1 = Arc::new(Dielectric::from(1.5));
    world.lock().unwrap().add(Arc::new(Sphere::from(
        Point3::from(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian::from(Color::from(0.4, 0.2, 0.1)));
    world.lock().unwrap().add(Arc::new(Sphere::from(
        Point3::from(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::from(Color::from(0.7, 0.6, 0.5), 0.0));
    world.lock().unwrap().add(Arc::new(Sphere::from(
        Point3::from(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    let mut camera = Camera::new();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 1920;
    camera.pixel_samples = 125;
    camera.max_depth = 75;

    camera.vfov = 20.0;
    camera.camera_origin = Point3::from(13.0, 2.0, 3.0);
    camera.camera_target = Point3::from(0.0, 0.0, 0.0);
    camera.up_vector = Vec3::from(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.6;
    camera.focus_distance = 10.0;

    camera.render(Arc::from(world.into_inner().unwrap()));

    // REPLACE THIS WITH YOUR PREFERRED IMAGE VIEWING PROGRAM
    match Command::new("imageglass").arg("images/image.ppm").status() {
        Ok(status) => {
            if status.success() {
                println!("Image opened successfully!");
            } else {
                println!("Failed to open image!");
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
