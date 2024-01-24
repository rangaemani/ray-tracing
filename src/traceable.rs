use std::ops::Neg;
use std::sync::Arc;

use crate::interval::Interval;
use crate::ray::Ray;
use crate::vector::{dot, Point3, Vec3};

/// Stores the intersection data when a ray hits an object.
#[derive(Copy, Clone)]
pub struct HitRecord {
    point: Point3,
    normal: Vec3,
    parameter: f64,
    ray_faces_outside: bool,
}

impl std::fmt::Display for HitRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "HitRecord {{ point: {:?}, normal: {:?}, parameter: {}, ray_faces_outside: {} }}",
            self.point, self.normal, self.parameter, self.ray_faces_outside
        )
    }
}

impl HitRecord {
    /// Creates a new `HitRecord` with default values.
    pub fn new() -> Self {
        HitRecord {
            point: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            parameter: 0.0,
            ray_faces_outside: true,
        }
    }

    /// Returns the intersection point.
    pub fn point(&self) -> Point3 {
        self.point
    }

    /// Returns the normal at the intersection.
    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    /// Returns the parameter `t` at which the intersection occurs.
    pub fn parameter(&self) -> f64 {
        self.parameter
    }

    /// Indicates whether the ray is facing outside the object.
    pub fn ray_faces_outside(&self) -> bool {
        self.ray_faces_outside
    }

    /// Sets the intersection point.
    pub fn set_point(&mut self, point: Point3) {
        self.point = point;
    }

    /// Sets the normal at the intersection.
    pub fn set_normal(&mut self, normal: Vec3) {
        self.normal = normal;
    }

    /// Sets the parameter `t` at which the intersection occurs.
    pub fn set_parameter(&mut self, parameter: f64) {
        self.parameter = parameter;
    }

    /// Sets the flag indicating the direction the ray is facing.
    pub fn set_ray_faces_outside(&mut self, ray_faces_outside: bool) {
        self.ray_faces_outside = ray_faces_outside;
    }

    /// Sets the normal orientation based on the incident ray direction.
    pub fn set_normal_face(&mut self, ray: &Ray, outward_normal: &Vec3) {
        outward_normal.normalize();
        self.ray_faces_outside = dot(&ray.direction(), &outward_normal) < 0.0;
        self.normal = if self.ray_faces_outside {
            outward_normal.clone()
        } else {
            outward_normal.neg()
        };
    }
}

/// Defines behavior for traceable objects.
pub trait Traceable {
    /// Determines if a ray intersects the object and records intersection data.
    fn hit(&self, ray: &Ray, ray_parameter: Interval, record: &mut HitRecord) -> bool;
}

/// Holds a collection of ray traceable objects.
pub struct Traceables {
    traceable_objects: Vec<Arc<dyn Traceable>>,
}

impl Traceables {
    /// Creates a new collection of traceable objects.
    pub fn new() -> Self {
        Traceables {
            traceable_objects: Vec::new(),
        }
    }

    /// Empties the collection of traceable objects.
    pub fn clear(&mut self) {
        self.traceable_objects.clear();
    }

    /// Adds an object to the collection.
    pub fn add(&mut self, object: Arc<dyn Traceable>) {
        self.traceable_objects.push(object);
    }
}

impl Traceable for Traceables {
    /// Determines if any object in the collection intersects with the ray.
    fn hit(&self, ray: &Ray, ray_parameter: Interval, record: &mut HitRecord) -> bool {
        let mut temp_record = HitRecord::new();
        let mut closest_parameter = ray_parameter.max();
        let mut has_hit: bool = false;

        for object in &self.traceable_objects {
            if object.hit(
                ray,
                Interval::new(ray_parameter.min(), closest_parameter),
                &mut temp_record,
            ) {
                has_hit = true;
                closest_parameter = temp_record.parameter();
                *record = temp_record.clone();
            }
        }

        has_hit
    }
}
