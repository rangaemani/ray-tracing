use crate::ray::Ray;
use crate::traceable::{HitRecord, Traceable};
use crate::vector::{dot, Point3, Vec3};

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Sphere {
        return Sphere { center, radius };
    }
    pub fn radius(self) -> f64 {
        return self.radius;
    }
    pub fn center(self) -> Point3 {
        return self.center;
    }
}

impl Traceable for Sphere {
    fn hit(
        &self,
        ray: &Ray,
        ray_parameter_max: f64,
        ray_parameter_min: f64,
        record: &mut HitRecord,
    ) -> bool {
        let origin_vector: Vec3 = ray.origin() - self.center;
        let a = dot(&ray.direction(), &ray.direction());
        let half_b = dot(&origin_vector, &ray.direction());
        let c = (origin_vector.dot(&origin_vector)) - (self.radius * self.radius);

        let discriminant = (half_b * half_b) - (a * c);
        if discriminant < 0.0 {
            return false;
        } else {
            let discriminant_root = discriminant.sqrt();
            let root = (-half_b - discriminant_root) / a;
            if root <= ray_parameter_min || ray_parameter_max <= root {
                let root = (-half_b + discriminant_root) / a;
                if root <= ray_parameter_min || ray_parameter_max <= root {
                    return false;
                }
            }
            record.set_parameter(root);
            record.set_point(ray.at(record.parameter()));
            let outward_normal: Vec3 = (record.point() - self.center) / self.radius;
            record.set_normal_face(ray, &outward_normal);
            return true;
        }
    }
}
