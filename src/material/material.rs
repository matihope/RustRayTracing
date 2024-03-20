use crate::{
    color::Color,
    hittables::hittable::HitRecord,
    my_math::prelude::{random_double, Ray, Vec3},
};

pub enum ScatterResult {
    Scatter { ray: Ray, attenuation: Color },
    Consume,
}

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> ScatterResult;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(color: Color) -> Self {
        Lambertian { albedo: color }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, hit_record: &HitRecord) -> ScatterResult {
        let mut scatter_direction = hit_record.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }

        ScatterResult::Scatter {
            ray: Ray::new(hit_record.intersection_point, scatter_direction),
            attenuation: self.albedo,
        }
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1. { fuzz } else { 1. },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> ScatterResult {
        let reflected = ray.direction.normalized().reflect(&hit_record.normal);
        let direction = reflected + Vec3::random_unit_vector() * self.fuzz;
        if direction.dot(&hit_record.normal) < 0. {
            ScatterResult::Consume
        } else {
            ScatterResult::Scatter {
                ray: Ray::new(hit_record.intersection_point, direction),
                attenuation: self.albedo,
            }
        }
    }
}

pub struct Dielectric {
    albedo: Color,
    refractive_index: f64,
}

impl Dielectric {
    pub fn new(albedo: Color, refractive_index: f64) -> Self {
        Self {
            albedo,
            refractive_index,
        }
    }
    fn reflectance(cosine: f64, refractive_index: f64) -> f64 {
        let r0 = ((1. - refractive_index) / (1. + refractive_index)).powi(2);
        r0 + (1. - r0) * (1. - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> ScatterResult {
        let normalized_direction = ray.direction.normalized();

        let cos_theta = -normalized_direction.dot(&hit_record.normal).min(1.);
        let sin_theta = (1. - cos_theta.powi(2)).sqrt();

        let refraction_fraction = if hit_record.front_face {
            1.0 / self.refractive_index
        } else {
            self.refractive_index
        };

        let should_reflect = refraction_fraction * sin_theta > 1.;
        let reflectance_schlick =
            Dielectric::reflectance(cos_theta, self.refractive_index) > random_double();

        let direction = if should_reflect || reflectance_schlick {
            // Here we are **REFLECTING**, not refracting.
            normalized_direction.reflect(&hit_record.normal)
        } else {
            normalized_direction.refract(&hit_record.normal, refraction_fraction)
        };

        ScatterResult::Scatter {
            ray: Ray::new(hit_record.intersection_point, direction),
            attenuation: self.albedo * 0.96,
        }
    }
}
