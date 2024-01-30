use crate::material::Material;
use crate::materials::{self, *};
use crate::math::interval::Interval;
use crate::ray::Ray;
use crate::traceable::{HitRecord, Traceable};
use crate::vector::{dot, Point3, Vec3};
use materials::{dielectric::Dielectric, lambert::Lambertian, metal::Metal};
use std::sync::Arc;

/// A geometric representation of a sphere with a center point and a radius.
pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Arc<dyn Material>,
    in_motion: bool,
    motion_blur_lerp_factor: Point3,
}

impl Sphere {
    pub fn new() -> Self {
        Sphere {
            center: Vec3::new(),
            radius: 0.0,
            material: Arc::from(Lambertian::new()),
            in_motion: false,
            motion_blur_lerp_factor: Point3::new(),
        }
    }

    pub fn new_in_motion(
        center_start: Point3,
        center_end: Point3,
        radius: f64,
        material: Arc<dyn Material>,
    ) -> Sphere {
        return Sphere {
            center: center_start,
            radius,
            material,
            in_motion: true,
            motion_blur_lerp_factor: center_end - center_start,
        };
    }
    /// Creates a new `Sphere` with a given `center` and `radius`.
    ///
    /// # Arguments
    ///
    /// * `center` - A `Point3` representing the center of the sphere.
    /// * `radius` - A `f64` representing the radius of the sphere.
    ///
    /// # Returns
    ///
    /// Returns a `Sphere` instance.
    pub fn from(center: Point3, radius: f64, material: Arc<dyn Material>) -> Sphere {
        return Sphere {
            center,
            radius,
            material,
            in_motion: false,
            motion_blur_lerp_factor: Point3::new(),
        };
    }

    /// Returns the radius of the sphere.
    pub fn radius(self) -> f64 {
        return self.radius;
    }

    /// Returns the center point of the sphere.
    pub fn center(&self, time: f64) -> Point3 {
        if self.in_motion {
            return self.center + time * self.motion_blur_lerp_factor;
        } else {
            return self.center;
        }
    }
}

impl Traceable for Sphere {
    /// Determines if a ray intersects with the sphere and, if so, populates the hit record.
    /// The intersection of a ray `R(t) = P + t*D` with a sphere defined by `(X - C) · (X - C) = r^2`
    /// can be found by substituting `X` with `R(t)` leading to a quadratic equation of form `at^2 + 2bt + c = 0`.
    /// We compute this using: `a = D · D`, `b = (P - C) · D`, `c = (P - C) · (P - C) - r^2`
    /// where `D` is the ray direction, `P` is the ray origin, and `C` is the sphere center.
    ///
    /// # Arguments
    ///
    /// * `ray` - The ray being traced.
    /// * `ray_parameter` - The interval within which the ray must hit.
    /// * `record` - The hit record to be populated with hit information.
    ///
    /// # Returns
    ///
    /// Returns `true` if the ray hits the sphere within the interval, `false` otherwise.
    fn hit(&self, ray: &Ray, ray_parameter: Interval, record: &mut HitRecord) -> bool {
        let center: Point3 = match self.in_motion {
            true => self.center(ray.time()),
            _ => self.center,
        };
        let origin_vector: Vec3 = ray.origin() - center;
        let a = ray.direction().magnitude();
        let half_b = dot(&origin_vector, &ray.direction());
        let c = (origin_vector.dot(&origin_vector)) - (self.radius * self.radius);

        let discriminant = (half_b * half_b) - (a * c);
        if discriminant < 0.0 {
            return false;
        }
        let discriminant_root = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - discriminant_root) / a;
        if !ray_parameter.surrounds(root) {
            root = (-half_b + discriminant_root) / a;
            if !ray_parameter.surrounds(root) {
                return false;
            }
        }

        record.set_parameter(root);
        record.set_point(ray.at(record.parameter()));
        let outward_normal: Vec3 = (record.point() - self.center) / self.radius;
        record.set_normal_face(ray, &outward_normal);
        record.set_material(self.material.clone());
        return true;
    }
}
