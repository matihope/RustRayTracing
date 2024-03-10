use crate::{
    camera::Camera,
    hittables::{prelude::*, sphere::Sphere},
    my_math::prelude::*,
};

use std::rc::Rc;

pub fn draw_image() {
    let mut world = HittableList::new_empty();
    world.add(Rc::new(Sphere::new(Point3::new(0., 0., -1.), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0., -100.5, -1.), 100.)));

    let mut cam = Camera::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.render(&world);
}
