use rand::RngExt;
use std::{
    fmt,
    ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub},
};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec3 {
    pub e: [f64; 3],
}

impl Vec3 {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn from(e0: f64, e1: f64, e2: f64) -> Self {
        Vec3 { e: [e0, e1, e2] }
    }

    #[must_use]
    pub fn x(&self) -> f64 {
        self.e[0]
    }

    #[must_use]
    pub fn y(&self) -> f64 {
        self.e[1]
    }

    #[must_use]
    pub fn z(&self) -> f64 {
        self.e[2]
    }

    #[must_use]
    fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    #[must_use]
    pub fn length_squared(&self) -> f64 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    #[must_use]
    pub fn random() -> Vec3 {
        let mut rng = rand::rng();
        let x: f64 = rng.random();
        let y: f64 = rng.random();
        let z: f64 = rng.random();
        Vec3::from(x, y, z)
    }

    #[must_use]
    pub fn random_clamp(min: f64, max: f64) -> Vec3 {
        let mut rng = rand::rng();
        let x: f64 = rng.random_range(min..max);
        let y: f64 = rng.random_range(min..max);
        let z: f64 = rng.random_range(min..max);
        Vec3::from(x, y, z)
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3::from(-self.x(), -self.y(), -self.z())
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, v: Vec3) -> Vec3 {
        Vec3::from(self.e[0] + v.e[0], self.e[1] + v.e[1], self.e[2] + v.e[2])
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, v: Vec3) -> Vec3 {
        Vec3::from(self.e[0] - v.e[0], self.e[1] - v.e[1], self.e[2] - v.e[2])
    }
}

impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::from(self.e[0] * v.e[0], self.e[1] * v.e[1], self.e[2] * v.e[2])
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::from(self * v.e[0], self * v.e[1], self * v.e[2])
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, t: f64) -> Vec3 {
        t * self
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, t: f64) -> Vec3 {
        (1.0 / t) * self
    }
}

#[must_use]
pub fn dot(u: Vec3, v: Vec3) -> f64 {
    u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
}

#[must_use]
pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3::from(
        u.e[1] * v.e[2] - u.e[2] * v.e[1],
        u.e[2] * v.e[0] - u.e[0] * v.e[2],
        u.e[0] * v.e[1] - u.e[1] * v.e[0],
    )
}

#[must_use]
pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

fn random_unit_vector() -> Vec3 {
    loop {
        let p = Vec3::random_clamp(-1.0, 1.0);
        let lensq = p.length_squared();
        if 1e-160 < lensq && lensq <= 1.0 {
            return p / lensq.sqrt();
        }
    }
}

pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
    let on_unit_sphere = random_unit_vector();
    // TODO: Do we need to deref here?
    if dot(on_unit_sphere, *normal) > 0.0 {
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}
