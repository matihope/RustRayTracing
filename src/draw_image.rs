use crate::{color::Color, globals::*, hittables::{prelude::*, sphere::Sphere}, my_math::prelude::*};
use indicatif::ProgressBar;

use std::rc::Rc;


fn ray_color(ray: Ray, world: &HittableList) -> Color {
    let mut hit_record = HitRecord::empty();
    if world.hit(&ray, 0., INFINITY, &mut hit_record) {
        (hit_record.normal + Color::new(1., 1., 1.)) * 0.5
    } else {
        let unit_direction = ray.direction.normalized();
        let a = 0.5 * (unit_direction.y + 1.0);
        Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
    }
}

pub fn draw_image() {
    let camera_center = Point3::new(0., 0., 0.);

    let viewport_u = Vec3::new(VIEWPORT_WIDTH, 0., 0.);
    let viewport_v = Vec3::new(0., -VIEWPORT_HEIGHT, 0.);

    let pixel_delta_u = viewport_u / IMAGE_WIDTH as f64;
    let pixel_delta_v = viewport_v / IMAGE_HEIGHT as f64;

    let viewport_upper_left =
        camera_center - Vec3::new(0., 0., FOCAL_LENGTH) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    eprintln!(
        "Drawing image: {} x {} with viewport: {} x {}...",
        IMAGE_WIDTH, IMAGE_HEIGHT, VIEWPORT_WIDTH, VIEWPORT_HEIGHT
    );

    let bar = ProgressBar::new(IMAGE_WIDTH * IMAGE_HEIGHT);
    // World
    let mut world = HittableList::newEmpty();
    world.add(Rc::new(Sphere::new(Point3::new(0., 0., -1.), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0., -100.5, -1.), 100.)));

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
    for j in 0..IMAGE_HEIGHT {
        for i in 0..IMAGE_WIDTH {
            let pixel_center =
                pixel00_loc + (pixel_delta_u * i as f64) + (pixel_delta_v * j as f64);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(r, &world);
            pixel_color.write_color();

            bar.inc(1);
        }
    }
}
