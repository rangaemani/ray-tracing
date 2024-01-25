use crate::{interval::Interval, vector::Vec3};

pub type Color = Vec3;

/// The function `write_color` takes a `Color` object and returns a formatted string representing the
/// RGB values of the color.
///
/// Arguments:
///
/// * `pixel_color`: The `pixel_color` parameter is of type `Color`.
///
/// Returns:
///
/// The function `write_color` returns a formatted string that represents the RGB values of a pixel
/// color.
pub fn write_color(pixel_color: Color, pixel_samples: usize) -> String {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    let scale = 1.0 / pixel_samples as f64;
    r *= scale;
    g *= scale;
    b *= scale;

    let intensity: Interval = Interval::new(0.000, 0.999);
    format!(
        "{} {} {}\n",
        (256.0 * intensity.clamp(r)) as i32,
        (256.0 * intensity.clamp(g)) as i32,
        (256.0 * intensity.clamp(b)) as i32
    )
}
