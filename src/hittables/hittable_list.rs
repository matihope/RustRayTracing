use crate::my_math::prelude::Ray;

use super::hittable::{HitRecord, Hittable};
use std::rc::Rc;

pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn newEmpty() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }
    pub fn new(hittable: Rc<dyn Hittable>) -> Self {
        let mut list = HittableList {
            objects: Vec::new(),
        };
        list.add(hittable);
        list
    }
    pub fn add(&mut self, hittable: Rc<dyn Hittable>) {
        self.objects.push(hittable);
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64, _hit_record: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::empty();
        let mut hit_anything = false;
        let mut closest_so_far = ray_tmax;

        for obj in self.objects.iter() {
            if obj.hit(ray, ray_tmin, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                _hit_record.clone_from(&temp_rec);
            }
        }
        hit_anything
    }
}
