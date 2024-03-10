use rand::{distributions::{Distribution, Uniform}, Rng};

use super::constants::PI;

pub fn deg2rad(degrees: f64) -> f64 {
    degrees / 360.0 * 2.0 * PI
}

pub fn random_double() -> f64 {
    let rng = Uniform::new(0., 1.);
    rand::thread_rng().sample(rng)
}
