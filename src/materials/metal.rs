use crate::materials;
use crate::Color;
use crate::HitRecord;
use crate::Ray;
use crate::Vec3;

use super::material::Material;
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new() -> Self {
        Metal {
            albedo: Vec3::new(),
            fuzz: 0.0,
        }
    }
    pub fn from(albedo: Color, fuzz: f64) -> Self {
        Metal {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Color,
        scattered_ray: &mut Ray,
    ) -> bool {
        let reflected_vector: Vec3 =
            Vec3::reflect(&ray_in.direction().normalize(), &record.normal());
        *scattered_ray = Ray::from(
            record.point(),
            reflected_vector + self.fuzz * Vec3::random_unit_sphere_vector(),
        );
        *attenuation = self.albedo;
        return true;
    }
}
