use crate::vector::*;

/// Represents a ray in 3D space.
///
/// A ray is defined by an origin point `A` and a direction vector `b`. The position of any point `P` along the ray can be computed using the formula `P(t) = A + t*b`, where `t` is a real number.
///
/// Positive values of `t` correspond to points in front of the origin `A`, while negative values correspond to points behind the origin.
pub struct Ray {
    origin: Point3,  // origin coordinates
    direction: Vec3, // direction vector
}

impl Ray {
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
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        return Ray { origin, direction };
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

    /// Returns a new `Ray` with the direction vector normalized.
    ///
    /// # Returns
    ///
    /// A new `Ray` instance with the direction vector normalized.
    pub fn unit_vector(&self) -> Ray {
        let direction = self.direction.normalize();
        Ray {
            origin: self.origin,
            direction,
        }
    }
}
