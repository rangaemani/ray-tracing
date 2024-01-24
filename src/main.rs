#![allow(dead_code)]
#![allow(unused_imports)]
use crate::color::{write_color, Color};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::traceable::{Traceable, Traceables};
use camera::Camera;
use interval::Interval;
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
mod ray;
mod rt_math;
mod sphere;
mod traceable;
mod vector;

fn main() {
    // Setup  World
    let mut world: Traceables = Traceables::new();

    world.add(Arc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let mut camera = Camera::new();
    camera.aspect_ratio = 3.0 / 2.0;
    camera.image_width = 400;
    camera.render(world);
}
