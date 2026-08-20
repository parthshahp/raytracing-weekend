use std::rc::Rc;

mod math;
mod objects;
mod render;

use crate::math::vec3::Vec3;
use crate::objects::hittable_list::HittableList;
use crate::objects::material::{Lambertian, Metal};
use crate::objects::sphere::Sphere;
use crate::render::camera::Camera;
use crate::render::color::Color;

fn main() {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .init();

    // World
    let mut world = HittableList::new();

    let material_ground = Rc::new(Lambertian {
        albedo: Color::from(0.8, 0.8, 0.0),
    });
    let material_center = Rc::new(Lambertian {
        albedo: Color::from(0.1, 0.2, 0.5),
    });
    let material_left = Rc::new(Metal {
        albedo: Color::from(0.8, 0.8, 0.8),
    });
    let material_right = Rc::new(Metal {
        albedo: Color::from(0.8, 0.6, 0.2),
    });

    world.add(Rc::new(Sphere::new(
        Vec3::from(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Rc::new(Sphere::new(
        Vec3::from(0.0, 0.0, -1.2),
        0.5,
        material_center,
    )));
    world.add(Rc::new(Sphere::new(
        Vec3::from(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Rc::new(Sphere::new(
        Vec3::from(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    // Camera
    let mut cam = Camera::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.render(&world);

    tracing::info!("\rDone.\n");
}
