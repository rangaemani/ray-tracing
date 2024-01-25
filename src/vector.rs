use std::fmt;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

use crate::rt_math::{random_number, random_number_in_range};

pub type Point3 = Vec3;

/// `Vec3` represents a three-dimensional vector with x, y, and z as its coordinates.
///
/// Fields:
/// * `x`: x-coordinate (f64)
/// * `y`: y-coordinate (f64)
/// * `z`: z-coordinate (f64)
#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    /// Constructs a new `Vec3`.
    ///
    /// Parameters:
    /// * `x`: x-coordinate (f64)
    /// * `y`: y-coordinate (f64)
    /// * `z`: z-coordinate (f64)
    ///
    /// Returns a `Vec3` instance.
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    /// Gets the x-coordinate.
    ///
    /// Returns the x-coordinate (f64).
    pub fn x(&self) -> f64 {
        self.x
    }

    /// Gets the y-coordinate.
    ///
    /// Returns the y-coordinate (f64).
    pub fn y(&self) -> f64 {
        self.y
    }

    /// Gets the z-coordinate.
    ///
    /// Returns the z-coordinate (f64).
    pub fn z(&self) -> f64 {
        self.z
    }

    /// Calculates the vector's length.
    ///
    /// Returns the length (f64).
    pub fn length(&self) -> f64 {
        self.magnitude().sqrt()
    }

    /// Calculates the squared length of the vector.
    ///
    /// Returns the squared length (f64).
    pub fn magnitude(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Calculates the dot product of two vectors.
    ///
    /// # Arguments
    ///
    /// * `other`: Another `Vec3` instance
    ///
    /// # Returns
    ///
    /// The dot product (f64).
    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Calculates the cross product of two vectors.
    ///
    /// # Arguments
    ///
    /// * `other`: Another `Vec3` instance
    ///
    /// # Returns
    ///
    /// The cross product (`Vec3`).
    pub fn cross(&self, other: &Self) -> Self {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    /// Normalizes the vector using fast inverse square r.
    ///
    /// # Returns
    ///
    /// A new `Vec3` instance with the same direction as the original vector but with a length of 1.
    pub fn normalize(&self) -> Self {
        let len = self.length();
        Vec3 {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }

    /// Generates a random vector within a unit sphere.
    ///
    /// Repeatedly generates a random vector within the range [-1.0, 1.0] for each
    /// coordinate until it finds one that lies inside the unit sphere (magnitude < 1).
    ///
    /// # Returns
    ///
    /// A `Vec3` that represents a random normalized vector within the unit sphere.
    pub fn random_unit_sphere_vector() -> Vec3 {
        loop {
            let vector: Vec3 = Vec3::random_in_range(-1.0, 1.0);
            if vector.magnitude() < 1.0 {
                return vector.normalize();
            }
        }
    }

    /// Generates a random vector that lies on the hemisphere surface defined by the given normal.
    ///
    /// This function creates a random vector that, when reflected across a diffuse (matte) surface,
    /// has an equal probability of bouncing in any direction above the surface tangent plane
    /// defined by `normal`. The function applies the rejection method of generating random vectors
    /// inside the unit sphere and normalizes them, ensuring that the resultant vector is on the
    /// hemisphere surface corresponding to the provided normal vector.
    ///
    /// # Parameters
    ///
    /// * `normal`: A reference to a `Vec3` representing the normal vector at the point of reflection on the surface.
    ///
    /// # Returns
    ///
    /// A `Vec3` that represents a random vector on the hemisphere defined by the `normal`. If the dot product
    /// of the generated vector and the `normal` is negative (indicating that the vector is on the opposite hemisphere),
    /// the vector is inverted to ensure it is in the correct hemisphere with respect to the normal vector.
    pub fn random_surface_hemisphere_vector(normal: &Vec3) -> Vec3 {
        let random_vector: Vec3 = Self::random_unit_sphere_vector();
        if dot(&random_vector, normal) > 0.0 {
            return random_vector;
        } else {
            return -random_vector;
        }
    }

    pub fn random() -> Vec3 {
        return Vec3::new(random_number(), random_number(), random_number());
    }

    pub fn random_in_range(min: f64, max: f64) -> Vec3 {
        return Vec3::new(
            random_number_in_range(min, max),
            random_number_in_range(min, max),
            random_number_in_range(min, max),
        );
    }
    // returns true if vec3 is mostly close to (0, 0, 0)
    pub fn approx_zero(&self) -> bool {
        let range = 1e-8;
        return self.x().abs() < range && self.y().abs() < range && self.z.abs() < range;
    }

    pub fn reflect(vector: &Vec3, normal: &Vec3) -> Vec3 {
        return vector.clone() - 2.0 * dot(vector, normal) * normal.clone();
    }
}
/// Calculates the dot product of two vectors. (Static Version)
///
/// # Arguments
///
/// * `v1: `Vec3` instance
/// * `v2`: Another `Vec3` instance
///
/// # Returns
///
/// The dot product (f64).
pub fn dot(v1: &Vec3, v2: &Vec3) -> f64 {
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
}

impl fmt::Display for Vec3 {
    /// Formats the `Vec3` as a string.
    ///
    /// Parameters:
    /// * `f`: Formatter
    ///
    /// Returns a `fmt::Result`.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl Add for Vec3 {
    type Output = Self;

    /// Adds two `Vec3` instances.
    ///
    /// Parameters:
    /// * `other`: Right-hand side `Vec3`
    ///
    /// Returns a new `Vec3`.
    fn add(self, other: Self) -> Self {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    /// Subtracts two `Vec3` instances.
    ///
    /// Parameters:
    /// * `other`: Right-hand side `Vec3`
    ///
    /// Returns a new `Vec3`.
    fn sub(self, other: Self) -> Self {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    /// Multiplies two `Vec3` instances component-wise.
    ///
    /// Parameters:
    /// * `other`: Right-hand side `Vec3`
    ///
    /// Returns a new `Vec3`.
    fn mul(self, other: Self) -> Vec3 {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    /// Scales a `Vec3` by a scalar.
    ///
    /// Parameters:
    /// * `v`: Vector to scale
    ///
    /// Returns a scaled `Vec3`.
    fn mul(self, v: Vec3) -> Vec3 {
        Vec3 {
            x: self * v.x,
            y: self * v.y,
            z: self * v.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    /// Scales `Vec3` by a scalar.
    ///
    /// Parameters:
    /// * `t`: Scalar to scale by
    ///
    /// Returns a scaled `Vec3`.
    fn mul(self, t: f64) -> Self {
        t * self
    }
}

impl Div for Vec3 {
    type Output = Vec3;

    /// Divides two `Vec3` instances component-wise.
    ///
    /// Parameters:
    /// * `other`: Right-hand side `Vec3`
    ///
    /// Returns a new `Vec3`.
    fn div(self, other: Self) -> Vec3 {
        Vec3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl Div<Vec3> for f64 {
    type Output = Vec3;

    /// Divides a scalar by a `Vec3`.
    ///
    /// Parameters:
    /// * `v`: Vector to divide by
    ///
    /// Returns a new `Vec3`.
    fn div(self, v: Vec3) -> Vec3 {
        Vec3 {
            x: self / v.x,
            y: self / v.y,
            z: self / v.z,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    /// Divides `Vec3` by a scalar.
    ///
    /// Parameters:
    /// * `t`: Scalar
    ///
    /// Returns a new `Vec3`.
    fn div(self, t: f64) -> Self {
        Vec3 {
            x: self.x / t,
            y: self.y / t,
            z: self.z / t,
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    /// Negates the `Vec3`.
    ///
    /// Returns a `Vec3` with negated components.
    fn neg(self) -> Self {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    /// Accesses an element by index.
    ///
    /// Parameters:
    /// * `i`: Index
    ///
    /// Returns a reference to the element.
    fn index(&self, i: usize) -> &Self::Output {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("index out of bounds"),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    /// Accesses a mutable element by index.
    ///
    /// Parameters:
    /// * `i`: Index
    ///
    /// Returns a mutable reference to the element.
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("index out of bounds"),
        }
    }
}

impl AddAssign for Vec3 {
    /// Adds another `Vec3` to this `Vec3`.
    ///
    /// Parameters:
    /// * `other`: `Vec3` to add
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl SubAssign for Vec3 {
    /// Adds another `Vec3` to this `Vec3`.
    ///
    /// Parameters:
    /// * `other`: `Vec3` to add
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl MulAssign<f64> for Vec3 {
    /// Multiplies `Vec3` by a scalar and assigns the result.
    ///
    /// Parameters:
    /// * `t`: Scalar
    fn mul_assign(&mut self, t: f64) {
        self.x *= t;
        self.y *= t;
        self.z *= t;
    }
}

impl DivAssign<f64> for Vec3 {
    /// Divides `Vec3` by a scalar and assigns the result.
    ///
    /// Parameters:
    /// * `t`: Scalar
    fn div_assign(&mut self, t: f64) {
        *self *= 1.0 / t;
    }
}
