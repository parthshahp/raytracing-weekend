pub mod math;
pub mod objects;
pub mod render;

use crate::math::ray::Ray;
use crate::math::vec3::{Vec3, dot, unit_vector};
use crate::render::color::Color;

pub fn ray_color(r: &Ray) -> Vec3 {
    let t = hit_sphere(&Vec3::from(0.0, 0.0, -1.0), 0.5, r);

    if t > 0.0 {
        let n = unit_vector(r.at(t) - Vec3::from(0.0, 0.0, -1.0));
        return 0.5 * Color::from(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
    }

    let unit_direction = unit_vector(r.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - a) * Color::from(1.0, 1.0, 1.0) + a * Color::from(0.5, 0.7, 1.0)
}

pub fn hit_sphere(center: &Vec3, radius: f64, r: &Ray) -> f64 {
    // TODO: Get rid of this to_owned conversion
    let oc = center.to_owned() - r.origin();
    let a = r.direction().length_squared();
    let h = dot(r.direction(), oc);
    let c = oc.length_squared() - radius * radius;
    let discriminant = h * h - a * c;

    if discriminant < 0.0 {
        return -1.0;
    }

    (h - discriminant.sqrt()) / a
}
