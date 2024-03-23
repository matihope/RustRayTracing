use std::sync::{Arc, Mutex};

use indicatif::ProgressBar;

use crate::{
    color::Color, hittables::prelude::*, material::material::ScatterResult, my_math::{prelude::*, ray},
};

use rayon::prelude::*;
use std::sync::RwLock;

struct CameraOrthonormalBasis {
    unit_vector_right: Vec3, // u
    camera_up: Vec3,         // v
    opposite_view: Vec3,     // w
}

impl CameraOrthonormalBasis {
    fn new(look_from: &Vec3, look_at: &Vec3, up_direction: &Vec3) -> Self {
        let opposite_view = (*look_from - *look_at).normalized();
        let unit_vector_right = up_direction.cross(&opposite_view).normalized();
        let camera_up = opposite_view.cross(&unit_vector_right).normalized();
        Self {
            unit_vector_right,
            camera_up,
            opposite_view,
        }
    }
}

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u64,
    pub samples_per_pixel: u64,
    pub max_ray_bounces: u64,
    image_height: u64,
    pub look_from: Point3,
    pub look_at: Point3,
    pub up_direction: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pub vfov: f64,
    orthonormals: CameraOrthonormalBasis,
    pub defocus_angle: f64,
    pub focus_dist: f64,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Default for Camera {
    fn default() -> Self {
        let vec_null = Vec3::new(0., 0., 0.);
        let vec_up = Vec3::new(0., 1., 0.);
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_ray_bounces: 10,
            image_height: 0,
            look_from: Point3::new(0., 0., -1.),
            look_at: vec_null,
            up_direction: vec_up,
            pixel00_loc: vec_null,
            pixel_delta_u: vec_null,
            pixel_delta_v: vec_null,
            vfov: 90.,
            orthonormals: CameraOrthonormalBasis::new(&Vec3::new(0., 0., 1.), &vec_null, &vec_up),
            defocus_angle: 0.,
            focus_dist: 10.,
            defocus_disk_u: vec_null,
            defocus_disk_v: vec_null,
        }
    }
}

impl Camera {
    pub fn render(&mut self, world: &HittableList) {
        self.initialize();
        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        let mem = Arc::new(RwLock::new(vec![
            vec![
                Color::new(0., 0., 0.);
                self.image_width as usize
            ];
            self.image_height as usize
        ]));

        let bar = Arc::new(Mutex::new(ProgressBar::new(
            self.image_width * self.image_height,
        )));

        let world = Arc::new(RwLock::new(world));
        (0..self.image_height)
            .flat_map(|y| (0..self.image_width).map(move |x| (x, y)))
            .collect::<Vec<(u64, u64)>>()
            .par_iter()
            // .iter()
            .for_each(|(x, y)| {
                let (x, y) = (*x, *y);
                let mut pixel_color = Color::new(0., 0., 0.);

                // Access the world
                let world = Arc::clone(&world);
                let world = world.read().unwrap();

                // Sample the color
                for _ in 0..self.samples_per_pixel {
                    pixel_color += self.ray_color(self.get_ray(x, y), *world, self.max_ray_bounces);
                }

                // Save the color to the memory
                let mem = Arc::clone(&mem);
                mem.write().unwrap()[y as usize][x as usize] = pixel_color;

                // Progress the bar
                let bar = Arc::clone(&bar);
                bar.lock().unwrap().inc(1);
            });

        mem.read().unwrap().iter().for_each(|row| {
            row.iter()
                .for_each(|c| c.write_color(self.samples_per_pixel))
        });
    }

    fn initialize(&mut self) {
        self.orthonormals =
            CameraOrthonormalBasis::new(&self.look_from, &self.look_at, &self.up_direction);
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as u64;
        if self.image_height < 1 {
            self.image_height = 1;
        }

        let theta = deg2rad(self.vfov);
        let h = (theta / 2.).tan();

        let viewport_height: f64 = 2.0 * h * self.focus_dist;
        let viewport_width: f64 =
            viewport_height * (self.image_width as f64) / (self.image_height as f64);

        let viewport_u = self.orthonormals.unit_vector_right * viewport_width;
        let viewport_v = -self.orthonormals.camera_up * viewport_height;

        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        let viewport_upper_left = self.look_from
            - (self.orthonormals.opposite_view * self.focus_dist)
            - viewport_u / 2.0
            - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5;

        let defocus_radius = self.focus_dist * deg2rad(self.defocus_angle / 2.).tan();

        self.defocus_disk_u = self.orthonormals.unit_vector_right * defocus_radius;
        self.defocus_disk_v = self.orthonormals.camera_up * defocus_radius;

        eprintln!(
            "Camera image: {} x {} with viewport: {} x {}...",
            self.image_width, self.image_height, viewport_width, viewport_height
        );
    }

    fn ray_color(&self, ray: Ray, world: &impl Hittable, bounces_left: u64) -> Color {
        let hit = if bounces_left == 0 {
            HitResult::Miss
        } else {
            world.hit(&ray, &Interval::new(0.01, INFINITY))
        };

        match hit {
            HitResult::Hit(hit_record) => match hit_record.material.scatter(&ray, &hit_record) {
                ScatterResult::Scatter { ray, attenuation } => {
                    let color: Color = attenuation;
                    let ray: Vec3 = self.ray_color(ray, world, bounces_left - 1);
                    ray * color
                }
                ScatterResult::Consume => Color::new(0., 0., 0.),
            },
            HitResult::Miss => {
                let unit_direction = ray.direction.normalized();
                let a = 0.5 * (unit_direction.y + 1.0);
                Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
            }
        }
    }

    fn get_ray(&self, i: u64, j: u64) -> Ray {
        let pixel_center =
            self.pixel00_loc + (self.pixel_delta_u * i as f64) + (self.pixel_delta_v * j as f64);
        let ray_origin = if self.defocus_angle <= 0. {
            self.look_from
        } else {
            self.defocus_disk_sample()
        };
        let pixel_sample = pixel_center + self.pixel_sample_square();
        let ray_direction = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction)
    }

    fn pixel_sample_square(&self) -> Vec3 {
        let px = -0.5 + random_double();
        let py = -0.5 + random_double();
        self.pixel_delta_u * px + self.pixel_delta_v * py
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = Vec3::random_in_unit_disk();
        self.look_from + (self.defocus_disk_u * p.x) + (self.defocus_disk_v * p.y)
    }
}
