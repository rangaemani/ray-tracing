use crate::materials;
use crate::Color;
use crate::HitRecord;
use crate::Ray;
use crate::Vec3;

use super::material::Material;
pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new() -> Self {
        Lambertian {
            albedo: Vec3::new(),
        }
    }

    pub fn from(color: Color) -> Self {
        Lambertian { albedo: color }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _ray_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Color,
        scattered_ray: &mut Ray,
    ) -> bool {
        let mut scatter_direction = record.normal() + Vec3::random_unit_sphere_vector();
        if scatter_direction.approx_zero() {
            scatter_direction = record.normal();
        }
        *scattered_ray = Ray::from(record.point(), scatter_direction);
        *attenuation = self.albedo;
        return true;
    }
}
