use crate::geometry::geometry::Geometry;
use crate::math::interval::Interval;
use crate::math::utils::random_vector_in_unit_disk;
use crate::ray::ray::Ray;

use glm::Vec3;
use indicatif::ProgressIterator;
use rand::random;
use rayon::prelude::*;

pub struct Camera {
    pub image_width: u32,
    pub image_height: u32,
    pub center: Vec3,
    pub pixel_delta_u: Vec3,
    pub pixel_delta_v: Vec3,
    pub pixel00_loc: Vec3,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub defocus_angle: f32,
    pub defocus_disk_u: Vec3,
    pub defocus_disk_v: Vec3,
}

impl Camera {
    pub fn render<T: Geometry + Sync>(&self, world: &T) {
        // Print metadata
        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        // Print data
        for j in (0..self.image_height).progress() {
            for i in 0..self.image_width {
                let pixel_color: Vec3 = (0..self.samples_per_pixel)
                    .into_par_iter()
                    .map(|_| Camera::ray_color(&self.get_ray(i, j), self.max_depth, world))
                    .sum();
                Camera::write_color(pixel_color / self.samples_per_pixel as f32);
            }
        }
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = Camera::sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f32 + offset.x) * self.pixel_delta_u)
            + ((j as f32 + offset.y) as f32 * self.pixel_delta_v);
        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample() 
        };
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square() -> Vec3 {
        Vec3::new(random::<f32>() - 0.5, random::<f32>() - 0.5, 0.0)
    }
    
    fn defocus_disk_sample(&self) -> Vec3 {
        let p = random_vector_in_unit_disk();
        self.center + p.x * self.defocus_disk_u + p.y * self.defocus_disk_v
    }

    fn ray_color<T: Geometry>(ray: &Ray, depth: u32, world: &T) -> Vec3 {
        if depth <= 0 {
            Vec3::new(0.0, 0.0, 0.0)
        } else if let Some(hit_record) = world.hit(&ray, &Interval::new(0.001, f32::INFINITY)) {
            if let Some((scattered_ray, attenuation)) =
                hit_record.material.scatter(ray, &hit_record)
            {
                attenuation.component_mul(&Camera::ray_color(&scattered_ray, depth - 1, world))
            } else {
                Vec3::new(0.0, 0.0, 0.0)
            }
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
