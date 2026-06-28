use crate::math::{interval::Interval, vec3::Vec3};

pub type Color = Vec3;

#[must_use]
pub fn write_color(pixel_color: Vec3) -> String {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    let intensity = Interval::new(0.000, 0.999);
    #[allow(clippy::cast_possible_truncation)]
    let rbyte = (256.0 * intensity.clamp(r)) as i32;
    #[allow(clippy::cast_possible_truncation)]
    let gbyte = (256.0 * intensity.clamp(g)) as i32;
    #[allow(clippy::cast_possible_truncation)]
    let bbyte = (256.0 * intensity.clamp(b)) as i32;

    format!("{rbyte} {gbyte} {bbyte}\n")
}
