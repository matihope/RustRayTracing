use crate::my_math::{Point3, Ray, Vec3};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool
}

impl HitRecord {
    pub fn set_face_normal(ray: &Ray, outward_normal: &Vec3) {
        
    }
}

pub trait Hittable {
    fn hit(&self, ray: Ray, ray_tmin: f64, ray_tmax: f64, hit_record: &mut HitRecord) -> bool;
}
