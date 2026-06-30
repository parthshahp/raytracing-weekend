use crate::math::{interval::Interval, vec3::Vec3};

pub type Color = Vec3;

#[must_use]
pub fn write_color(pixel_color: Vec3) -> String {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);

    let intensity = Interval::new(0.000, 0.999);
    #[allow(clippy::cast_possible_truncation)]
    let rbyte = (256.0 * intensity.clamp(r)) as i32;
    #[allow(clippy::cast_possible_truncation)]
    let gbyte = (256.0 * intensity.clamp(g)) as i32;
    #[allow(clippy::cast_possible_truncation)]
    let bbyte = (256.0 * intensity.clamp(b)) as i32;

    format!("{rbyte} {gbyte} {bbyte}\n")
}

pub fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        return linear_component.sqrt();
    }

    0.0
}
