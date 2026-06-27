use std::rc::Rc;

use crate::{
    math::{interval::Interval, ray::Ray},
    objects::hittable::{HitRecord, Hittable},
};

pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
}

impl Default for HittableList {
    fn default() -> Self {
        Self::new()
    }
}

impl HittableList {
    #[must_use]
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }
    pub fn from(object: Rc<dyn Hittable>) -> Self {
        let mut list = Self::new();
        list.add(object);
        list
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut closest_so_far = ray_t.max;
        let mut ret = None;

        // What is the closest object hit in the list?
        for obj in &self.objects {
            if let Some(hit_record) = obj.hit(r, Interval::new(ray_t.min, closest_so_far)) {
                closest_so_far = hit_record.t;
                ret = Some(hit_record);
            }
        }

        ret
    }
}
