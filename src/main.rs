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
use materials::{dielectric::Dielectric, lambert::Lambertian, metal::Metal};
use std::path::Path;
use std::process::Command;
use std::{env, f64::consts::PI, fs::File, io::prelude::*, sync::Arc};

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
    if Path::exists(&Path::new("image.ppm")) {
        std::fs::remove_file("image.ppm").unwrap();
    }
    // Setup  World
    let mut world: Traceables = Traceables::new();
    let material_ground = Arc::new(Lambertian::from(Color::from_rgb(97, 255, 134)));
    let material_center = Arc::new(Dielectric::from(4.5));
    // let material_left = Arc::new(Dielectric::from(1.5));
    // let material_right = Arc::new(Dielectric::from(3.5));
    // ground object
    world.add(Arc::new(Sphere::from(
        Point3::from(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    // glass sphere
    world.add(Arc::new(Sphere::from(
        Point3::from(0.0, 5.0, 0.0),
        -3.5,
        material_center,
    )));
    // world.add(Arc::new(Sphere::from(
    //     Point3::from(-1.0, 0.0, -1.0),
    //     0.5,
    //     material_left.clone(),
    // )));
    // world.add(Arc::new(Sphere::from(
    //     Point3::from(-1.0, 0.0, -1.0),
    //     -0.4,
    //     material_left,
    // )));
    // world.add(Arc::new(Sphere::from(
    //     Point3::from(1.0, 0.0, -2.0),
    //     -1.5,
    //     material_right,
    // )));

    let mut camera = Camera::new();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 2800;
    camera.pixel_samples = 125;
    camera.max_depth = 50;

    camera.vfov = 70.0;
    camera.camera_origin = Point3::from(-2.0, 2.0, 1.0);
    camera.camera_target = Point3::from(0.0, 0.0, -1.0);
    camera.up_vector = Vec3::from(0.0, 1.0, 0.0);

    camera.defocus_angle = 10.0;
    camera.focus_distance = 3.4;

    camera.render(Arc::from(world));

    match Command::new("imageglass").arg("image.ppm").status() {
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
