use glm::Mat3;
use glm::Vec3;
use indicatif::ProgressIterator;
use itertools::Itertools;

use crate::geometry::geometry::Geometry;
use crate::math::interval::Interval;
use crate::ray::ray::Ray;

pub struct Camera {
    image_width: u32,
    image_height: u32,
    raster_to_world: Mat3,
}

impl Camera {
    pub fn new(image_width: u32, image_height: u32) -> Self {
        let aspect_ratio = image_width as f32 / image_height as f32;
        let scale_y = 2.0 / image_height as f32;
        let scale_x = 2.0 * aspect_ratio / image_width as f32;

        #[rustfmt::skip]
        let raster_to_world = Mat3::new(
            scale_x,  0.0,     -aspect_ratio, // Map (0..N) to (-1*AR.. 1*AR)
            0.0,     -scale_y,  1.0,          // Map (0..M) to (1..-1)
            0.0,      0.0,      1.0,          // Don't touch Z axis
        );

        Camera {
            image_width,
            image_height,
            raster_to_world,
        }
    }

    pub fn render<T: Geometry>(&self, world: &T) {
        // Print metadata
        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        let pixels = (0..self.image_height)
            .progress()
            .cartesian_product(0..self.image_width)
            .map(|(y, x)| {
                let ray = self.get_ray(x, y);
                let color = self.ray_color(&ray, world);
                color
            });

        for pixel_color in pixels {
            Camera::write_color(pixel_color);
        }
    }

    fn ray_color<T: Geometry>(&self, ray: &Ray, world: &T) -> Vec3 {
        if let Some(_) = world.hit(ray, &Interval::new(0.0, f32::MAX)) {
            Vec3::new(1.0, 0.0, 0.0)
        } else {
            let unit_direction = ray.direction.normalize();
            let a = 0.5 * unit_direction.y + 1.0;
            (1.0 - a) * Vec3::repeat(1.0) + a * Vec3::new(0.5, 0.7, 1.0)
        }
    }

    fn get_ray(&self, x: u32, y: u32) -> Ray {
        let p_screen = Vec3::new(x as f32 + 0.5, y as f32 + 0.5, 1.0);
        let p_world = self.raster_to_world * p_screen;

        let origin = Vec3::repeat(0.0);
        let direction = Vec3::new(p_world.x, p_world.y, -1.0) - origin;
        Ray::new(origin, direction)
    }

    fn write_color(color: Vec3) {
        let corrected = color.map(|c| (c.sqrt().clamp(0.0, 0.999) * 256.0) as u32);
        println!("{} {} {}", corrected.x, corrected.y, corrected.z);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn square_camera_test_00() {
        let camera = Camera::new(10, 10);
        let ray = camera.get_ray(0, 0);

        let expected = Vec3::new(-0.9, 0.9, -1.0);
        let given = ray.direction;

        assert!(
            glm::equal_eps(&expected, &given, glm::epsilon())
                .iter()
                .all(|x| *x),
            "expected {}, given {}",
            expected,
            given
        )
    }

    #[test]
    fn square_camera_test99() {
        let camera = Camera::new(10, 10);
        let ray = camera.get_ray(9, 9);

        let expected = Vec3::new(0.9, -0.9, -1.0);
        let given = ray.direction;

        assert!(
            glm::equal_eps(&expected, &given, glm::epsilon())
                .iter()
                .all(|x| *x),
            "expected {}, given {}",
            expected,
            given
        )
    }

    #[test]
    fn rectangular_camera_test00() {
        let camera = Camera::new(20, 10);
        let ray = camera.get_ray(0, 0);

        let expected = Vec3::new(-1.9, 0.9, -1.0);
        let given = ray.direction;

        assert!(
            glm::equal_eps(&expected, &given, glm::epsilon())
                .iter()
                .all(|x| *x),
            "expected {}, given {}",
            expected,
            given
        )
    }

    #[test]
    fn rectangular_camera_test99() {
        let camera = Camera::new(20, 10);
        let ray = camera.get_ray(19, 9);

        let expected = Vec3::new(1.9, -0.9, -1.0);
        let given = ray.direction;

        assert!(
            glm::equal_eps(&expected, &given, glm::epsilon())
                .iter()
                .all(|x| *x),
            "expected {}, given {}",
            expected,
            given
        )
    }
}
