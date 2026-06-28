use std::rc::Rc;

mod math;
mod objects;
mod render;

use crate::math::vec3::Vec3;
use crate::objects::hittable_list::HittableList;
use crate::objects::sphere::Sphere;
use crate::render::camera::Camera;

fn main() {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .init();

    // World
    let mut world = HittableList::new();
    // We are adding two spheres here: the multi-colored one and the plain green one
    world.add(Rc::new(Sphere::new(Vec3::from(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Vec3::from(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let mut cam = Camera::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.render(&world);

    tracing::info!("\rDone.\n");
}
