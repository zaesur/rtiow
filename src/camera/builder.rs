use super::camera::Camera;

// Defaults
const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 400;
const FOCAL_LENGTH: f32 = 1.0;
const SAMPLE_SIZE: u32 = 100;
const MAX_BOUNCES: u32 = 25;

pub struct CameraBuilder {
    aspect_ratio: f32,
    image_width: u32,
    focal_length: f32,
    sample_size: u32, 
    max_bounces: u32,
}

#[allow(dead_code)]
impl CameraBuilder {
    pub fn new() -> Self {
        CameraBuilder {
            aspect_ratio: ASPECT_RATIO,
            image_width: IMAGE_WIDTH,
            focal_length: FOCAL_LENGTH,
            sample_size: SAMPLE_SIZE,
            max_bounces: MAX_BOUNCES,
        }
    }
    pub fn aspect_ratio(mut self, aspect_ratio: f32) -> Self {
        self.aspect_ratio = aspect_ratio;
        self
    }

    pub fn image_width(mut self, image_width: u32) -> Self {
        self.image_width = image_width;
        self
    }

    pub fn focal_length(mut self, focal_length: f32) -> Self {
        self.focal_length = focal_length;
        self
    }

    pub fn sample_size(mut self, sample_size: u32) -> Self {
        self.sample_size = sample_size;
        self
    }

    pub fn max_bounces(mut self, max_bounces: u32) -> Self {
        self.max_bounces = max_bounces;
        self
    }

    pub fn build(self) -> Camera {
        Camera::new(
            self.aspect_ratio,
            self.image_width,
            self.focal_length,
            self.sample_size,
            self.max_bounces
        )
    }
}
