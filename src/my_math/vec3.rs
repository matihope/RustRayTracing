use core::ops;

use super::prelude::random_double_range;

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    const EPSILON: f64 = 1e-8;

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x: x, y: y, z: z }
    }
    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn length_squared(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn dot(self, rhs: &Vec3) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
    pub fn cross(self, rhs: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
    pub fn normalized(self) -> Vec3 {
        self / self.length()
    }

    fn random() -> Self {
        Self::random_range(0., 1.)
    }

    pub fn random_range(min: f64, max: f64) -> Self {
        Vec3 {
            x: random_double_range(min, max),
            y: random_double_range(min, max),
            z: random_double_range(min, max),
        }
    }
    fn random_in_unit_sphere() -> Self {
        loop {
            let v = Vec3::random_range(-1., 1.);
            if v.length_squared() <= 1. {
                break v;
            }
        }
    }
    pub fn random_unit_vector() -> Self {
        Vec3::random_in_unit_sphere().normalized()
    }
    pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
        let v = Vec3::random_unit_vector();
        if v.dot(normal) < 0.0 {
            -v
        } else {
            v
        }
    }
    pub fn near_zero(&self) -> bool {
        return self.x.abs() < Self::EPSILON
            && self.y.abs() < Self::EPSILON
            && self.z.abs() < Self::EPSILON;
    }
    pub fn reflect(&self, normal: &Vec3) -> Vec3 {
        *self - *normal * self.dot(normal) * 2.
    }
    pub fn refract(&self, normal: &Vec3, refraction_fraction: f64) -> Vec3 {
        let cosine = -self.dot(normal).min(1.0);
        let r_prime_perpendicular = (*self + *normal * cosine) * refraction_fraction;
        let r_prime_parallel = -*normal * (1. - r_prime_perpendicular.length_squared()).abs().sqrt();
        r_prime_parallel + r_prime_perpendicular
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Self {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<T: Copy> ops::Mul<T> for Vec3
where
    f64: ops::Mul<T, Output = f64>,
{
    type Output = Vec3;
    fn mul(self, rhs: T) -> Vec3 {
        Vec3 {
            x: self.x * rhs.clone(),
            y: self.y * rhs.clone(),
            z: self.z * rhs,
        }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}
impl<T: Copy> ops::Div<T> for Vec3
where
    f64: ops::Div<T, Output = f64>,
{
    type Output = Vec3;
    fn div(self, rhs: T) -> Vec3 {
        Vec3 {
            x: self.x / rhs.clone(),
            y: self.y / rhs.clone(),
            z: self.z / rhs,
        }
    }
}
