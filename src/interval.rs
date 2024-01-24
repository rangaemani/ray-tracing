use std::f64::{INFINITY, NEG_INFINITY};
use std::ops::Neg;

/// Represents a mathematical interval with a minimum and maximum value.
#[derive(Clone, Copy)]
pub struct Interval {
    min: f64,
    max: f64,
}

impl Interval {
    /// Constructs a new `Interval` with given min and max values.
    ///
    /// # Arguments
    ///
    /// * `min` - A 64-bit floating point number representing the minimum value of the interval.
    /// * `max` - A 64-bit floating point number representing the maximum value of the interval.
    ///
    /// # Examples
    ///
    /// ```
    /// let interval = Interval::new(0.0, 1.0);
    /// ```
    pub fn new(min: f64, max: f64) -> Self {
        Interval { min, max }
    }

    /// Creates a default `Interval` from negative infinity to positive infinity.
    ///
    /// # Examples
    ///
    /// ```
    /// let interval = Interval::default();
    /// ```
    pub fn default() -> Self {
        Interval {
            min: f64::NEG_INFINITY,
            max: f64::INFINITY,
        }
    }

    /// Checks if a value is contained within the interval.
    ///
    /// # Arguments
    ///
    /// * `x` - A 64-bit floating number to check for containment.
    ///
    /// # Examples
    ///
    /// ```
    /// let interval = Interval::new(0.0, 5.0);
    /// assert!(interval.contains(2.0));
    /// ```
    pub fn contains(self, x: f64) -> bool {
        return self.min <= x && x <= self.max;
    }

    /// Checks if a value is strictly inside the interval (excluding the boundaries).
    ///
    /// # Arguments
    ///
    /// * `x` - A 64-bit floating number to check for strict containment.
    ///
    /// # Examples
    ///
    /// ```
    /// let interval = Interval::new(0.0, 5.0);
    /// assert!(!interval.surrounds(0.0));
    /// ```
    pub fn surrounds(self, x: f64) -> bool {
        return self.min < x && x < self.max;
    }

    /// Returns the maximum value of the interval.
    ///
    /// # Examples
    ///
    /// ```
    /// let interval = Interval::new(0.0, 5.0);
    /// assert_eq!(interval.max(), 5.0);
    /// ```
    pub fn max(self) -> f64 {
        return self.max;
    }

    /// Returns the minimum value of the interval.
    ///
    /// # Examples
    ///
    /// ```
    /// let interval = Interval::new(0.0, 5.0);
    /// assert_eq!(interval.min(), 0.0);
    /// ```
    pub fn min(self) -> f64 {
        return self.min;
    }

    /// A constant representing an empty interval.
    pub const EMPTY: Interval = Interval {
        min: f64::INFINITY,
        max: f64::NEG_INFINITY,
    };

    /// A constant representing a universal interval covering all possible `f64` values.
    pub const UNIVERSE: Interval = Interval {
        min: f64::NEG_INFINITY,
        max: f64::INFINITY,
    };
}
