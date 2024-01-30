use std::fmt::Display;

use crate::vector::*;

/// Represents a ray in 3D space.
///
/// A ray is defined by an origin point `A` and a direction vector `b`. The position of any point `P` along the ray can be computed using the formula `P(t) = A + t*b`, where `t` is a real number.
///
/// Positive values of `t` correspond to points in front of the origin `A`, while negative values correspond to points behind the origin.
#[derive(Clone, Copy)]
pub struct Ray {
    origin: Point3,  // origin coordinates
    direction: Vec3, // direction vector
    time: f64,       // random sample time for interpolation
}

impl Display for Ray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Ray(origin: {}, direction: {})",
            self.origin, self.direction
        )
    }
}

impl Ray {
    pub fn new() -> Self {
        Ray {
            origin: Vec3::new(),
            direction: Vec3::new(),
            time: 0.0,
        }
    }

    /// Constructs a new `Ray` with the given origin and direction.
    ///
    /// # Arguments
    ///
    /// * `origin` - The origin point of the ray.
    /// * `direction` - The direction vector of the ray.
    ///
    /// # Returns
    ///
    /// A new `Ray` instance.
    pub fn from(origin: Point3, direction: Vec3, time: f64) -> Ray {
        return Ray {
            origin,
            direction,
            time,
        };
    }

    /// Returns the origin point of the ray.
    ///
    /// # Returns
    ///
    /// The origin point of the ray.
    pub fn origin(&self) -> Point3 {
        return self.origin;
    }

    /// Returns the direction vector of the ray.
    ///
    /// # Returns
    ///
    /// The direction vector of the ray.
    pub fn direction(&self) -> Vec3 {
        return self.direction;
    }

    /// Computes the position of a point along the ray.
    ///
    /// # Arguments
    ///
    /// * `t` - The parameter value.
    ///
    /// # Returns
    ///
    /// The position of the point along the ray.
    pub fn at(&self, t: f64) -> Point3 {
        return self.origin + t * self.direction;
    }

    pub fn time(&self) -> f64 {
        return self.time;
    }
}
