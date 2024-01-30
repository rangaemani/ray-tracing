use super::material::Material;
use crate::materials;
use crate::rt_math::*;
use crate::vector::*;
use crate::Color;
use crate::HitRecord;
use crate::Ray;
use crate::Vec3;
use std::ops::Neg;

pub struct Dielectric {
    index_of_refraction: f64,
}

impl Dielectric {
    pub fn new() -> Self {
        Dielectric {
            index_of_refraction: 0.0,
        }
    }
    pub fn from(index_of_refraction: f64) -> Self {
        Dielectric {
            index_of_refraction,
        }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::from(1.0, 1.0, 1.0);
        let refraction_ratio: f64 = match record.ray_faces_outside() {
            true => 1.0 / self.index_of_refraction,
            _ => self.index_of_refraction,
        };
        let ray_in_unit_direction: Vec3 = ray_in.direction().normalize();
        let cos_theta: f64 = f64::min(dot(&ray_in_unit_direction.neg(), &record.normal()), 1.0);
        let sin_theta: f64 = (1.0 - cos_theta * cos_theta).sqrt();

        let not_refractable: bool = refraction_ratio * sin_theta > 1.0;
        let direction: Vec3;
        if not_refractable || reflectance(cos_theta, refraction_ratio) > random_number() {
            direction = Vec3::reflect(&ray_in_unit_direction, &record.normal());
        } else {
            direction = Vec3::refract(&ray_in_unit_direction, &record.normal(), refraction_ratio);
        }
        *scattered = Ray::from(record.point(), direction, ray_in.time());
        return true;
    }
}

/// Calculates the reflectance using Schlick's approximation.
///
/// This function approximates the probability of reflectance occurring at an interface
/// between two materials, given the cosine of the angle of incidence and the ratio of the
/// refractive indices of the materials using Schlick's approximation.
///
/// * `cos_theta`: Cosine of the angle of incidence.
/// * `refraction_ratio`: Ratio of the refractive indices (`n2 / n1`).
///
/// Returns the approximate reflectance probability.
///
/// This function is used in the context of simulating the reflection and refraction of
/// light when it encounters a dielectric material in a ray tracer.
///
/// Reference: [Schlick's approximation](https://en.wikipedia.org/wiki/Schlick's_approximation)
fn reflectance(cos_theta: f64, refraction_ratio: f64) -> f64 {
    let r0 = (1.0 - refraction_ratio) / (1.0 + refraction_ratio);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * f64::powf(1.0 - cos_theta, 5.0)
}
