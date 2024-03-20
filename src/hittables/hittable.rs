use crate::{
    color::Color,
    material::material::{Lambertian, Material},
    my_math::prelude::*,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct HitRecord {
    pub intersection_point: Point3,
    pub normal: Vec3,
    pub material: Arc<dyn Material>,
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
            intersection_point: Point3::new(0., 0., 0.),
            normal: Vec3::new(0., 0., 0.),
            material: Arc::new(Lambertian::new(Color::new(1., 1., 1.))),
            t: 0.,
            front_face: false,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> HitResult;
}
