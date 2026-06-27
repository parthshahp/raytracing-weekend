use crate::math::interval::Interval;
use crate::math::ray::Ray;
use crate::math::vec3::{Vec3, dot};

pub struct HitRecord {
    /// The intersection point in 3D space.
    pub p: Vec3,
    /// The surface normal at the hit point
    pub normal: Vec3,
    /// The ray scalar, p(t) = origin + t * direction
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = dot(r.direction(), *outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord>;
}
