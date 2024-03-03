use super::hittable::{HitRecord, Hittable};
use crate::my_math::prelude::{Point3, Ray};

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
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool {
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

        let a = ray.direction.dot(&ray.direction);
        let half_b = ray.direction.dot(&oc);
        let c = oc.dot(&oc) - self.radius * self.radius;

        let delta: f64 = half_b * half_b - a * c;
        let mut root = (-half_b - delta.sqrt()) / a;
        if root <= ray_tmin || ray_tmax <= root {
            root = (-half_b + delta.sqrt()) / a;
            if root <= ray_tmin || ray_tmax <= root {
                return false;
            }
        }

        rec.t = root;
        rec.p = ray.at(root);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(ray, &outward_normal);

        true
    }
}
