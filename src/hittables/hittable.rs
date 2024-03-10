use crate::my_math::prelude::*;

#[derive(Copy, Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

pub struct HitRecordNoHit;

pub enum HitResult {
    Hit(HitRecord),
    Miss,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        // Sets the hit record normal vector
        // NOTE: the parameter `outward_normal` is assumed to have unit length

        self.front_face = ray.direction.dot(outward_normal) < 0.;
        self.normal.clone_from(&outward_normal);
        if !self.front_face {
            self.normal = -self.normal;
        }
    }
    pub fn empty() -> Self {
        HitRecord {
            p: Point3::new(0., 0., 0.),
            normal: Vec3::new(0., 0., 0.),
            t: 0.,
            front_face: false,
        }
    }
}

pub trait Hittable {
    fn hit<'a>(&self, ray: &Ray, ray_t: &Interval) -> HitResult;
}
