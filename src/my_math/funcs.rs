use rand::{distributions::Uniform, Rng};

use super::constants::PI;

pub fn deg2rad(degrees: f64) -> f64 {
    degrees / 360.0 * 2.0 * PI
}

pub fn random_double() -> f64 {
    random_double_range(0., 1.)
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
    let rng = Uniform::new(min, max);
    rand::thread_rng().sample(rng)
}
