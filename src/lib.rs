pub mod math;
pub mod objects;
pub mod render;

use crate::math::ray::Ray;
use crate::math::vec3::{Vec3, unit_vector};
use crate::objects::hittable::Hittable;
use crate::render::color::Color;

#[must_use]
pub fn ray_color(r: &Ray, world: &impl Hittable) -> Vec3 {
    // If anything in the world is hit, return the "correct" color
    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        return 0.5 * (rec.normal + Color::from(1.0, 1.0, 1.0));
    }

    let unit_direction = unit_vector(r.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);
    // This is that sky blue color
    (1.0 - a) * Color::from(1.0, 1.0, 1.0) + a * Color::from(0.5, 0.7, 1.0)
}
