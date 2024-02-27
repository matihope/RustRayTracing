use crate::my_math::Vec3;

const FUN_MAX: f64 = 255.999;
pub type Color = Vec3;

impl Color {
    pub fn write_color(self) {
        println!(
            "{} {} {}",
            (FUN_MAX * self.x) as u64,
            (FUN_MAX * self.y) as u64,
            (FUN_MAX * self.z) as u64
        );
    }
}
