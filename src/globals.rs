pub const ASPECT_RATIO: f64 = 16.0 / 9.0;

pub const IMAGE_WIDTH: u64 = 256;

// Calculate the image height
const IMAGE_HEIGHT_CALC: u64 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u64;
pub const IMAGE_HEIGHT: u64 = if IMAGE_HEIGHT_CALC > 0 {
    IMAGE_HEIGHT_CALC
} else {
    1
};

pub const FOCAL_LENGTH: f64 = 1.0;
pub const VIEWPORT_HEIGHT: f64 = 2.0;
pub const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * (IMAGE_WIDTH as f64) / (IMAGE_HEIGHT as f64);
