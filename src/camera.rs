use glm::Vec3;

use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::ray::Ray;
use indicatif::ProgressIterator;

fn write_color(color: Vec3) {
    let ir = (255.999 * color.x) as i32;
    let ig = (255.999 * color.y) as i32;
    let ib = (255.999 * color.z) as i32;

    println!("{ir} {ig} {ib}");
}

pub struct Camera {
    image_width: u32,
    image_height: u32,
    center: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel00: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f32, image_width: u32, focal_length: f32) -> Self {
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
        }
    }

    pub fn render<T: Hittable>(&self, world: &T) {
        // Print metadata
        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        // Print data
        for j in (0..self.image_height).progress() {
            for i in 0..self.image_width {
                let pixel_center = self.pixel00
                    + (i as f32 * self.pixel_delta_u)
                    + (j as f32 * self.pixel_delta_v);
                let ray_direction = pixel_center - self.center;
                let r = Ray::new(self.center, ray_direction);

                let pixel_color = self.ray_color(&r, world);

                write_color(pixel_color);
            }
        }
    }

    fn ray_color<T: Hittable>(&self, ray: &Ray, world: &T) -> Vec3 {
        if let Some(hr) = world.hit(&ray, &Interval::new(0.0, f32::INFINITY)) {
            0.5 * (hr.normal + Vec3::new(1.0, 1.0, 1.0))
        } else {
            let unit_direction = ray.direction;
            let a = 0.5 * (unit_direction.y + 1.0);

            (1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0)
        }
    }
}
