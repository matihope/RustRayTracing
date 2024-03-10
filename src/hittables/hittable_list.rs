use crate::my_math::prelude::*;

use super::hittable::{HitRecord, HitResult, Hittable};
use std::rc::Rc;

pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn new_empty() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }
    pub fn new(hittable: Rc<dyn Hittable>) -> Self {
        let mut list = HittableList::new_empty();
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
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> HitResult {
        let mut temp_rec = HitRecord::empty();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;
        let mut hit_result = HitResult::Miss;

        for obj in self.objects.iter() {
            match obj.hit(ray, &Interval::new(ray_t.min, closest_so_far)) {
                HitResult::Hit(x) => {
                    closest_so_far = temp_rec.t;
                    hit_result = HitResult::Hit(x);
                }
                HitResult::Miss => (),
            }
        }
        hit_result
    }
}
