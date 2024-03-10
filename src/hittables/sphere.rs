use super::hittable::{HitRecord, HitResult, Hittable};
use crate::my_math::prelude::*;

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> HitResult {
        // A Point3 is on a sphere if:
        // (P - C) ^ 2 == radius^2
        // In other words, we want to know if it ever hits the sphere:
        // (ray.origin + ray.direction * t - center) ^ 2 == radius ^ 2
        // (ray.dir * t + (ray.org - center)) ^ 2 == r^2
        // ray.dir^2 * t^2 + 2 * ray.dir * t * (ray.org - center) + (ray.org - center)^ 2 == r^2
        // ray.dir^2 * t^2 + 2 * ray.dir * t * (ray.org - center) + (ray.org - center) ^ 2 - r^2 == 0
        // It does hit the ray if delta >= 0.
        // Delta = (2 ray.dir (ray.org - center))^ 2 - 4 * (ray.dir^2) (ray.org - center) ^ 2

        let oc = ray.origin - self.center;

        let a = ray.direction.length_squared();
        let half_b = ray.direction.dot(&oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let delta: f64 = half_b * half_b - a * c;
        if delta < 0. {
            return HitResult::Miss;
        }
        let sqrt_delta = delta.sqrt();
        let mut root = (-half_b - sqrt_delta) / a;
        if !ray_t.surrounds(root) {
            root = (-half_b + sqrt_delta) / a;
            if !ray_t.surrounds(root) {
                return HitResult::Miss;
            }
        }

        let mut rec = HitRecord::empty();
        rec.t = root;
        rec.p = ray.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(ray, &outward_normal);

        HitResult::Hit(rec)
    }
}
