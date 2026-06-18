use crate::{
    math::{
        ray::Ray,
        vec3::{Vec3, dot},
    },
    objects::hittable::{HitRecord, Hittable},
};

pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    #[must_use]
    pub fn new(center: Vec3, radius: f64) -> Self {
        Self {
            center,
            radius: radius.max(0.0),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool {
        let oc = self.center - r.origin();
        // A vector dotted with itself == squared length
        let a = r.direction().length_squared();
        let h = dot(r.direction(), oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        // No roots exist, no hits
        if discriminant < 0.0 {
            return false;
        }

        // Solve for t
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        // We must check both roots. If neither is in the range we have no hits
        let mut root = (h - sqrtd) / a;
        if root <= ray_tmin || ray_tmax <= root {
            root = (h + sqrtd) / a;
            if root <= ray_tmin || ray_tmax <= root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        rec.normal = (rec.p - self.center) / self.radius;

        true
    }
}
