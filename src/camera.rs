use std::sync::{Arc, Mutex};

use indicatif::ProgressBar;

use crate::{color::Color, hittables::prelude::*, my_math::prelude::*};

use rayon::prelude::*;
use std::sync::RwLock;

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u64,
    pub samples_per_pixel: u64,
    image_height: u64,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Default for Camera {
    fn default() -> Self {
        let mut cam = Camera {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            image_height: 0,
            center: Point3::new(0., 0., 0.),
            pixel00_loc: Point3::new(0., 0., 0.),
            pixel_delta_u: Vec3::new(0., 0., 0.),
            pixel_delta_v: Vec3::new(0., 0., 0.),
        };
        cam.initialize();
        cam
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
            .for_each(|(x, y)| {
                let (x, y) = (*x, *y);
                let mut pixel_color = Color::new(0., 0., 0.);

                // Access the world
                let world = Arc::clone(&world);
                let world = world.read().unwrap();

                // Sample the color
                for _ in 0..self.samples_per_pixel {
                    pixel_color += self.ray_color(self.get_ray(x, y), *world);
                }

                // Save the color to the memory
                let mem = Arc::clone(&mem);
                mem.write().unwrap()[y as usize][x as usize] = pixel_color;

                // Progress the bar
                let bar = Arc::clone(&bar);
                bar.lock().unwrap().inc(1);
            });

        let mem = mem.read();
        mem.iter().for_each(|row| {
            row.iter().for_each(|col| {
                col.iter()
                    .for_each(|color| color.write_color(self.samples_per_pixel))
            })
        });

        // for j in 0..self.image_height {
        //     for i in 0..self.image_width {
        //         let mut pixel_color = Color::new(0., 0., 0.);
        //         for _ in 0..self.samples_per_pixel {
        //             pixel_color += self.ray_color(self.get_ray(i, j), world);
        //         }
        //         pixel_color.write_color(self.samples_per_pixel);

        //         bar.inc(1);
        //     }
        // }
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as u64;
        if self.image_height < 1 {
            self.image_height = 1;
        }

        self.center = Point3::new(0., 0., 0.);

        let focal_length: f64 = 1.0;
        let viewport_height: f64 = 2.0;
        let viewport_width: f64 =
            viewport_height * (self.image_width as f64) / (self.image_height as f64);

        let viewport_u = Vec3::new(viewport_width, 0., 0.);
        let viewport_v = Vec3::new(0., -viewport_height, 0.);

        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        let viewport_upper_left =
            self.center - Vec3::new(0., 0., focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5;

        eprintln!(
            "Camera image: {} x {} with viewport: {} x {}...",
            self.image_width, self.image_height, viewport_width, viewport_height
        );
    }

    fn ray_color(&self, ray: Ray, world: &dyn Hittable) -> Color {
        let mut hit_record = HitRecord::empty();
        match world.hit(&ray, &Interval::new(0., INFINITY)) {
            HitResult::Hit(hit_record) => (hit_record.normal + Color::new(1., 1., 1.)) * 0.5,
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
        let pixel_sample = pixel_center + self.pixel_sample_square();
        let ray_direction = pixel_sample - self.center;
        Ray::new(self.center, ray_direction)
    }

    fn pixel_sample_square(&self) -> Vec3 {
        let px = -0.5 + random_double();
        let py = -0.5 + random_double();
        self.pixel_delta_u * px + self.pixel_delta_v * py
    }
}
