#![allow(dead_code)]
#![allow(unused_imports)]
use crate::color::{write_color, Color};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::traceable::{Traceable, Traceables};
use camera::Camera;
use interval::Interval;
use material::{Lambertian, Metal};
use rt_math::INFINITY;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::sync::Arc;
use traceable::HitRecord;
use vector::{Point3, Vec3};

mod camera;
mod color;
mod interval;
mod material;
mod ray;
mod rt_math;
mod sphere;
mod traceable;
mod vector;

fn main() {
    // Setup  World
    let mut world: Traceables = Traceables::new();

    let material_ground = Arc::new(Lambertian::new(Color::new(0.4, 1.0, 0.8)));
    let material_center = Arc::new(Lambertian::new(Color::new(0.2, 0.3, 0.3)));
    let material_left = Arc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let material_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    let mut camera = Camera::new();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 1200;
    camera.pixel_samples = 125;
    camera.max_depth = 50;
    camera.render(world);
}
