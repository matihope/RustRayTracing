use indicatif::ProgressBar;

const IMAGE_WIDTH: u64 = 256;
const IMAGE_HEIGHT: u64 = 256;

mod my_math;

mod color;
use crate::color::Color;

fn main() {
    let bar = ProgressBar::new(IMAGE_WIDTH * IMAGE_HEIGHT);
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
    for j in 0..IMAGE_HEIGHT {
        for i in 0..IMAGE_WIDTH {
            let color = Color::new(
                i as f64 / (IMAGE_WIDTH - 1) as f64,
                j as f64 / (IMAGE_HEIGHT - 1) as f64,
                0.0f64,
            );
            color.write_color();
            bar.inc(1);
        }
    }
}
