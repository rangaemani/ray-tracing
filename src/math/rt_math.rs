use crate::ray;
use crate::vector;
use rand::prelude::*;
// Constants
pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = std::f64::consts::PI;

// Utility Functions
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
// Returns random f64 between 0 and 1
pub fn random_number() -> f64 {
    rand::thread_rng().gen::<f64>()
}

// Returns random f64 within a specified range
pub fn random_number_in_range(min: f64, max: f64) -> f64 {
    rand::thread_rng().gen_range(min..=max)
}
