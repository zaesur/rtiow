use glm::Vec3;
use indicatif::ProgressIterator;
use rand::Rng;

use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::utils::random_unit_vector;

pub struct Camera {
    image_width: u32,
    image_height: u32,
    center: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel00: Vec3,
    samples_per_pixel: u32,
    max_depth: u32,
}

impl Camera {
    pub fn new(
        aspect_ratio: f32,
        image_width: u32,
        focal_length: f32,
        samples_per_pixel: u32,
        max_depth: u32,
    ) -> Self {
        let image_height = (image_width as f32 / aspect_ratio) as u32;

        let viewport_height = 2.0;
        let viewport_width: f32 = viewport_height * image_width as f32 / image_height as f32;
        let center = Vec3::new(0.0, 0.0, 0.0);

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / image_width as f32;
        let pixel_delta_v = viewport_v / image_height as f32;

        let viewport_upper_left =
            center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00 = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Camera {
            image_width,
            image_height,
            center,
            pixel_delta_u,
            pixel_delta_v,
            pixel00,
            samples_per_pixel,
            max_depth,
        }
    }

    pub fn render<T: Hittable>(&self, world: &T) {
        // Print metadata
        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        // Print data
        for j in (0..self.image_height).progress() {
            for i in 0..self.image_width {
                let pixel_color = (0..self.samples_per_pixel)
                    .fold(Vec3::new(0.0, 0.0, 0.0), |color, _| {
                        color + self.ray_color(&self.get_ray(i, j), self.max_depth, world)
                    });

                Camera::write_color(pixel_color / self.samples_per_pixel as f32);
            }
        }
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = Camera::sample_square();
        let pixel_sample = self.pixel00
            + ((i as f32 + offset.x) * self.pixel_delta_u)
            + ((j as f32 + offset.y) as f32 * self.pixel_delta_v);
        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square() -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3::new(rng.gen::<f32>() - 0.5, rng.gen::<f32>() - 0.5, 0.0)
    }

    fn ray_color<T: Hittable>(&self, ray: &Ray, depth: u32, world: &T) -> Vec3 {
        if depth <= 0 {
            Vec3::new(0.0, 0.0, 0.0)
        } else if let Some(hr) = world.hit(&ray, &Interval::new(0.001, f32::INFINITY)) {
            let direction = hr.normal + random_unit_vector();
            0.5 * self.ray_color(&Ray::new(hr.p, direction), depth - 1, world)
        } else {
            let unit_direction = ray.direction;
            let a = 0.5 * (unit_direction.y + 1.0);

            (1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0)
        }
    }

    fn write_color(color: Vec3) {
        let intensity = Interval::new(0.0, 0.999);
        let ir = (256.0 * intensity.clamp(Camera::linear_to_gamma(color.x))) as i32;
        let ig = (256.0 * intensity.clamp(Camera::linear_to_gamma(color.y))) as i32;
        let ib = (256.0 * intensity.clamp(Camera::linear_to_gamma(color.z))) as i32;

        println!("{ir} {ig} {ib}");
    }

    pub fn linear_to_gamma(linear_component: f32) -> f32 {
        if linear_component > 0.0 {
            f32::sqrt(linear_component)
        } else {
            0.0
        }
    }
}
