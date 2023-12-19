use crate::vector::Vec3;

pub type Color = Vec3;

pub fn write_color(pixel_color: Color) -> String {
    format!(
        "{} {} {}\n",
        (255.999 * pixel_color.x()) as i32,
        (255.999 * pixel_color.y()) as i32,
        (255.999 * pixel_color.z()) as i32
    )
}
