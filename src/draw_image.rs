use rand::{distributions::Uniform, Rng};
use rand_distr::{Distribution, Normal};

use crate::{
    camera::Camera,
    color::Color,
    hittables::{prelude::*, sphere::Sphere},
    material::material::{Dielectric, Lambertian, Material, Metal},
    my_math::prelude::*,
};
use std::rc::Rc;
use std::sync::Arc;
use rand::rngs::ThreadRng;

fn make_world() -> HittableList {
    let mut world = HittableList::new_empty();

    let material_ground = Arc::new(Lambertian::new(Color::new(0.2, 0.8, 0.0)));
    let material_center = Arc::new(Metal::new(Color::new(0.7, 0.3, 0.3), 0.));
    let material_left = Arc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.1));
    let material_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.1));
    let material_glass = Arc::new(Dielectric::new(Color::new(1.0, 1.0, 1.0), 1.5));

    // Ground
    world.add(Rc::new(Sphere::new(
        Point3::new(0., -100.5, -1.),
        100.,
        material_ground,
    )));
    // Center
    world.add(Rc::new(Sphere::new(
        Point3::new(0., 0., -1.),
        0.5,
        material_center,
    )));
    // Left
    world.add(Rc::new(Sphere::new(
        Point3::new(-1., 0., -1.),
        0.5,
        material_left,
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(1., 0., -1.),
        0.5,
        material_glass.clone(),
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(1., 0., -1.),
        -0.3,
        material_glass,
    )));

    world
}

pub fn make_big_camera() -> Camera {
    let mut cam = Camera::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;

    cam.samples_per_pixel = 500;
    cam.vfov = 20.0;

    cam.look_from = Point3::new(10., 2., -10.);
    cam.look_at = Point3::new(-1., 0., 0.);

    cam.defocus_angle = 1.0;
    cam.focus_dist = cam.look_from.length();

    cam
}

fn gen_material(rng: &mut ThreadRng) -> Arc<dyn Material> {
    let dist = Uniform::new(0, 3);
    let zerone = Uniform::new(0., 1.);
    let mat_type = rng.sample(zerone);
    if mat_type < 0.15 {
        Arc::new(Metal::new(Color::new(random_double(), random_double(), random_double()), random_double()))
    } else if mat_type <= 0.8 {
        Arc::new(Lambertian::new(Color::new(random_double(), random_double(), random_double())))
    } else {
        Arc::new(Dielectric::new(Color::new(1., 1., 1.), 1.5))
    }
}

pub fn make_big_render() -> HittableList {
    let mut world = HittableList::new_empty();

    let material_ground = Arc::new(Lambertian::new(Color::new(0.3, 0.3, 0.3)));
    world.add(Rc::new(Sphere::new(
        Point3::new(0., -1000., 0.),
        1000.,
        material_ground,
    )));

    let mut rng = rand::thread_rng();
    let mut dist = Uniform::new(-2., 2.);

    for x in -11..=11 {
        for z in -11..=11 {
            let center = Vec3::new(x as f64 + 0.9 * random_double(), 0.2, z as f64 + 0.9 * random_double());
            let sphere = Rc::new(Sphere::new(center, 0.2, gen_material(&mut rng)));
            world.add(sphere);
        }
    }


    let material_center = Arc::new(Dielectric::new(Color::new(1.0, 1.0, 1.0), 1.5));
    let material_left = Arc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.1));
    let material_right = Arc::new(Metal::new(Color::new(0.6, 0.6, 0.6), 0.));

    // Center
    world.add(Rc::new(Sphere::new(
        Point3::new(0., 1., -1.),
        1.,
        material_center,
    )));
    // Left
    world.add(Rc::new(Sphere::new(
        Point3::new(-2., 1., -1.),
        1.,
        material_left,
    )));
    // Right
    world.add(Rc::new(Sphere::new(
        Point3::new(2., 1., -1.),
        1.,
        material_right.clone(),
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(2., 1., -1.),
        -0.8,
        material_right,
    )));

    world
}

pub fn draw_image() {
    // let mut cam = Camera::default();

    // cam.aspect_ratio = 16.0 / 9.0;
    // cam.image_width = 400;
    // cam.samples_per_pixel = 100;
    // cam.up_direction = Vec3::new(0., 1., 0.);
    // cam.vfov = 25.;
    // cam.look_from = Point3::new(-2., 0.7, 2.7);
    // cam.look_at = Point3::new(0., 0., -1.);
    // cam.defocus_angle = 1.0;
    // cam.focus_dist = (cam.look_from - cam.look_at).length() - 0.25;

    // cam.vfov = 20.;
    // cam.look_from = Point3::new(-2., 2., 1.);
    // cam.look_at = Point3::new(0., 0., -1.);

    // cam.defocus_angle = 10.0;
    // cam.focus_dist = 3.4;

    // cam.render(&make_world());
    make_big_camera().render(&make_big_render());
}
