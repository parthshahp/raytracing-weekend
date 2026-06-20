pub mod math;
pub mod objects;
pub mod render;

use crate::math::ray::Ray;
use crate::math::vec3::{Vec3, unit_vector};
use crate::objects::hittable::Hittable;
use crate::objects::sphere::Sphere;
use crate::render::color::Color;

#[must_use]
pub fn ray_color(r: &Ray) -> Vec3 {
    let sphere = Sphere::new(Vec3::from(0.0, 0.0, -1.0), 0.5);

    if let Some(rec) = sphere.hit(r, 0.001, f64::INFINITY) {
        return 0.5
            * Color::from(
                rec.normal.x() + 1.0,
                rec.normal.y() + 1.0,
                rec.normal.z() + 1.0,
            );
    }

    let unit_direction = unit_vector(r.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);
    // This is that sky blue color
    (1.0 - a) * Color::from(1.0, 1.0, 1.0) + a * Color::from(0.5, 0.7, 1.0)
}
