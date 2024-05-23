use glm::Vec3;

use super::camera::Camera;

// Defaults
const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 400;
const SAMPLE_SIZE: u32 = 100;
const MAX_BOUNCES: u32 = 25;
const VFOV: f32 = 90.0;
const VUP: Vec3 = Vec3::new(0.0, 1.0, 0.0);

pub struct CameraBuilder {
    aspect_ratio: f32,
    image_width: u32,
    samples_per_pixel: u32,
    max_depth: u32,
    vfov: f32,
    vup: Vec3,
}

#[allow(dead_code)]
impl CameraBuilder {
    pub fn new() -> Self {
        CameraBuilder {
            aspect_ratio: ASPECT_RATIO,
            image_width: IMAGE_WIDTH,
            samples_per_pixel: SAMPLE_SIZE,
            max_depth: MAX_BOUNCES,
            vfov: VFOV,
            vup: VUP,
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

    pub fn samples_per_pixel(mut self, samples_per_pixel: u32) -> Self {
        self.samples_per_pixel = samples_per_pixel;
        self
    }

    pub fn max_depth(mut self, max_depth: u32) -> Self {
        self.max_depth = max_depth;
        self
    }

    pub fn vfov(mut self, vfov: f32) -> Self {
        self.vfov = vfov;
        self
    }

    pub fn build(self, lookfrom: Vec3, lookat: Vec3) -> Camera {
        let image_height = ((self.image_width as f32 / self.aspect_ratio) as u32).max(1);
        let center = lookfrom;

        // Determine viewport dimensions;
        let focal_length: f32 = glm::length(&(lookfrom - lookat));
        let theta = self.vfov.to_radians();
        let h = f32::tan(theta / 2.0);
        let viewport_height = 2.0 * h * focal_length;
        let viewport_width = viewport_height * (self.image_width as f32 / image_height as f32);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        let w = glm::normalize(&(lookfrom - lookat));
        let u = glm::cross(&self.vup, &w);
        let v = glm::cross(&w, &u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / self.image_width as f32;
        let pixel_delta_v = viewport_v / image_height as f32;

        // Calcualte the location of the upper left pixel.
        let viewport_upper_left = center - focal_length * w - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Camera {
            image_width: self.image_width,
            image_height,
            center,
            pixel_delta_u,
            pixel_delta_v,
            pixel00_loc,
            samples_per_pixel: self.samples_per_pixel,
            max_depth: self.max_depth,
        }
    }
}
