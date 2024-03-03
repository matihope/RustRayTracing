use crate::my_math::prelude::{Point3, Ray, Vec3};

#[derive(Copy, Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        // Sets the hit record normal vector
        // NOTE: the parameter `outward_normal` is assumed to have unit length

        self.front_face = ray.direction.dot(outward_normal) < 0.;
        self.normal = if self.front_face {
            outward_normal.clone()
        } else {
            -outward_normal.clone()
        }
    }
    pub fn empty() -> Self {
        HitRecord {
            p: Point3::new(0., 0., 0.),
            normal: Vec3::new(0., 0., 0.,),
            t: 0.,
            front_face: false
        }
    }
}

pub trait Hittable {
    fn hit<'a>(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64, hit_record: &'a mut HitRecord) -> bool;
}
