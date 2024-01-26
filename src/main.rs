#![allow(dead_code)]
#![allow(unused_imports)]
use crate::camera::camera::Camera;
use crate::camera::*;
use crate::drawable::*;
use crate::materials::*;
use crate::math::*;
use crate::traceable::{Traceable, Traceables};
use crate::vectors::*;
use drawable::{sphere::Sphere, traceable::HitRecord};
use materials::{dielectric::Dielectric, lambert::Lambertian, metal::Metal};
use std::{env, f64::consts::PI, fs::File, io::prelude::*, sync::Arc};

use vectors::{
    color::Color,
    ray::Ray,
    vector::{Point3, Vec3},
};
mod camera;
mod drawable;
mod materials;
mod math;
mod vectors;

fn main() {
    // Setup  World
    let mut world: Traceables = Traceables::new();

    let material_ground = Arc::new(Lambertian::from(Color::from(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::from(Color::from(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Dielectric::from(1.5));
    let material_right = Arc::new(Metal::from(Color::from(0.8, 0.6, 0.2), 0.0));

    world.add(Arc::new(Sphere::from(
        Point3::from(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Arc::new(Sphere::from(
        Point3::from(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.add(Arc::new(Sphere::from(
        Point3::from(-1.0, 0.0, -1.0),
        0.5,
        material_left.clone(),
    )));
    world.add(Arc::new(Sphere::from(
        Point3::from(-1.0, 0.0, -1.0),
        -0.4,
        material_left,
    )));
    world.add(Arc::new(Sphere::from(
        Point3::from(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    let mut camera = Camera::new();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 800;
    camera.pixel_samples = 125;
    camera.max_depth = 50;

    camera.vfov = 40.0;
    camera.camera_origin = Point3::from(-2.0, 2.0, 1.0);
    camera.camera_target = Point3::from(0.0, 0.0, -1.0);
    camera.up_vector = Vec3::from(0.0, 1.0, 0.0);
    camera.render(world);
}
