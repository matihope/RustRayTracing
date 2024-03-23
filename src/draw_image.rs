use crate::{
    camera::Camera,
    color::Color,
    hittables::{prelude::*, sphere::Sphere},
    material::material::{Dielectric, Lambertian, Material, Metal},
    my_math::prelude::*,
};
use std::rc::Rc;
use std::sync::Arc;

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

pub fn draw_image() {
    let mut cam = Camera::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.up_direction = Vec3::new(0., 1., 0.);
    cam.vfov = 25.;
    cam.look_from = Point3::new(-2., 0.7, 2.7);
    cam.look_at = Point3::new(0., 0., -1.);
    cam.defocus_angle = 1.0;
    cam.focus_dist = (cam.look_from - cam.look_at).length() - 0.25;

    // cam.vfov = 20.;
    // cam.look_from = Point3::new(-2., 2., 1.);
    // cam.look_at = Point3::new(0., 0., -1.);

    // cam.defocus_angle = 10.0;
    // cam.focus_dist = 3.4;

    cam.render(&make_world());
}
