use crate::vector::Vec3;

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
pub fn write_color(pixel_color: Color) -> String {
    format!(
        "{} {} {}\n",
        (255.999 * pixel_color.x()) as i32,
        (255.999 * pixel_color.y()) as i32,
        (255.999 * pixel_color.z()) as i32
    )
}
