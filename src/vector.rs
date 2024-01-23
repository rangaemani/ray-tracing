use std::fmt;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

pub type Point3 = Vec3;

/// `Vec3` represents a three-dimensional vector with x, y, and z as its coordinates.
///
/// Fields:
/// * `x`: x-coordinate (f64)
/// * `y`: y-coordinate (f64)
/// * `z`: z-coordinate (f64)
#[derive(Copy, Clone)]
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
