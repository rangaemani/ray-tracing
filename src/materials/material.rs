use crate::math;
use crate::vectors;
use crate::{
    color::Color,
    math::rt_math::{self, random_number},
    ray::Ray,
    traceable::HitRecord,
    vector::{dot, Vec3},
};
use std::ops::Neg;

/// Trait for materials in a ray tracer.
///
/// This trait provides the method `scatter`, which is used to simulate
/// the scattering of light as it interacts with the material of a surface.
pub trait Material: Send + Sync {
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
