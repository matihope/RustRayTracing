use super::hittable::{HitRecord, HitResult, Hittable};
use crate::{material::material::Material, my_math::prelude::*};
use std::sync::Arc;

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Arc<dyn Material>) -> Self {
        Sphere {
            center,
            radius,
            material: Arc::clone(&material),
        }
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
        rec.intersection_point = ray.at(rec.t);
        let outward_normal = (rec.intersection_point - self.center) / self.radius;
        rec.set_face_normal(ray, &outward_normal);
        rec.material = Arc::clone(&self.material);

        HitResult::Hit(rec)
    }
}
