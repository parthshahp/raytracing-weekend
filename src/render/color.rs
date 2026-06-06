use crate::math::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color(pixel_color: Vec3) -> String {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    #[allow(clippy::cast_possible_truncation)]
    let rbyte = (255.999 * r) as i32;
    #[allow(clippy::cast_possible_truncation)]
    let gbyte = (255.999 * g) as i32;
    #[allow(clippy::cast_possible_truncation)]
    let bbyte = (255.999 * b) as i32;

    format!("{rbyte} {gbyte} {bbyte}\n")
}
