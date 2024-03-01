use crate::{color::Color, globals::*, my_math::*, hittable::*};
use indicatif::ProgressBar;

fn hit_sphere(center: Point3, radius: f64, ray: &Ray) -> f64 {
}

fn ray_color(ray: Ray) -> Color {
    let t = hit_sphere(Vec3::new(0., 0., -1.), 0.5, &ray);
    if t > 0.0 {
        let normal = (ray.at(t) - Vec3::new(0., 0., -1.)).normalized();
        Color::new(normal.x + 1., normal.y + 1., normal.z + 1.) * 0.5
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
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
    for j in 0..IMAGE_HEIGHT {
        for i in 0..IMAGE_WIDTH {
            let pixel_center =
                pixel00_loc + (pixel_delta_u * i as f64) + (pixel_delta_v * j as f64);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(r);
            pixel_color.write_color();

            bar.inc(1);
        }
    }
}
