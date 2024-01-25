use crate::{color::Color, ray::Ray, rt_math, traceable::HitRecord, vector::Vec3};

/// Trait for materials in a ray tracer.
///
/// This trait provides the method `scatter`, which is used to simulate
/// the scattering of light as it interacts with the material of a surface.
pub trait Material {
    /// Determines how an incoming ray (`ray_in`) is affected when it hits a surface.
    ///
    /// It sets the `attenuation` to describe how much the ray is dimmed and
    /// modifies the `scattered` ray according to the material properties.
    /// The method returns a boolean indicating whether the ray has been scattered.
    ///
    /// Arguments:
    /// * `ray_in`: A reference to the incoming ray.
    /// * `record`: A reference to the hit record describing the intersection details.
    /// * `attenuation`: A mutable reference to the ray color which is modified based on the material.
    /// * `scattered`: A mutable reference to the scattered ray which is modified by this method.
    ///
    /// Returns:
    /// * `bool`: `true` if the ray scatters; `false` otherwise.
    fn scatter(
        &self,
        ray_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(color: Color) -> Self {
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
        *scattered_ray = Ray::new(record.point(), scatter_direction);
        *attenuation = self.albedo;
        return true;
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
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
        *scattered_ray = Ray::new(
            record.point(),
            reflected_vector + self.fuzz * Vec3::random_unit_sphere_vector(),
        );
        *attenuation = self.albedo;
        return true;
    }
}
