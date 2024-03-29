use crate::my_math::{prelude::Interval, vec3::Vec3};

pub type Color = Vec3;

const INTENSITY: Interval = Interval::new(0., 0.999);

impl Color {
    pub fn write_color(&self, num_samples: u64) {
        let num_samples = num_samples as f64;
        let clr = *self / num_samples;

        // Most image viewers assume an image is in gamma space, not in linear space,
        // so we have to convert it to gamma space (gamma 2 to be more specific).
        let clr = clr.to_gamma();

        println!(
            "{} {} {}",
            (256. * INTENSITY.clamp(clr.x)) as u64,
            (256. * INTENSITY.clamp(clr.y)) as u64,
            (256. * INTENSITY.clamp(clr.z)) as u64
        );
    }
    pub fn to_gamma(&self) -> Self {
        Color {
            x: self.x.sqrt(),
            y: self.y.sqrt(),
            z: self.z.sqrt(),
        }
    }
}
