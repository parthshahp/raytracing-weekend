use crate::math::vec3::Vec3;

pub struct Ray {
    orig: Vec3,
    dir: Vec3,
}

impl Ray {
    #[must_use]
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray {
            orig: origin,
            dir: direction,
        }
    }

    #[must_use]
    pub fn origin(&self) -> Vec3 {
        self.orig
    }

    #[must_use]
    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    #[must_use]
    pub fn at(&self, t: f64) -> Vec3 {
        self.orig + t * self.dir
    }
}
