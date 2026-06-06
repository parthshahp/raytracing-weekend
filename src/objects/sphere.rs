use crate::{
    math::ray::Ray,
    math::vec3::Vec3,
    objects::hittable::{HitRecord, Hittable},
};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool {
        // vec3 oc = center - r.origin();
        // auto a = r.direction().length_squared();
        // auto h = dot(r.direction(), oc);
        // auto c = oc.length_squared() - radius*radius;
        //
        // auto discriminant = h*h - a*c;
        // if (discriminant < 0)
        //     return false;
        //
        // auto sqrtd = std::sqrt(discriminant);
        //
        // // Find the nearest root that lies in the acceptable range.
        // auto root = (h - sqrtd) / a;
        // if (root <= ray_tmin || ray_tmax <= root) {
        //     root = (h + sqrtd) / a;
        //     if (root <= ray_tmin || ray_tmax <= root)
        //         return false;
        // }
        //
        // rec.t = root;
        // rec.p = r.at(rec.t);
        // rec.normal = (rec.p - center) / radius;
        //
        // return true;
        true
    }
}
