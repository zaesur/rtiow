use glm::Mat3;
use glm::Mat4;
use glm::Vec3;
use glm::Vec4;
use indicatif::ProgressIterator;
use itertools::Itertools;

use crate::geometry::geometry::Geometry;
use crate::math::interval::Interval;
use crate::ray::ray::Ray;

pub struct Camera {
    image_width: u32,
    image_height: u32,
    raster_to_world: Mat4,
}

impl Camera {
    pub fn new(image_width: u32, image_height: u32, fov: f32) -> Self {
        let scale_y = 1.0 / image_height as f32;
        let scale_x = 1.0 / image_width as f32;
        let aspect_ratio = image_width as f32 / image_height as f32;
        let theta = fov.to_radians();
        let h = (theta / 2.0).tan();

        #[rustfmt::skip]
        // NDC: Normalized Device Coordinates.
        // X axis: (0..X) remapped to (0..1).
        // Y axis: (0..Y) remapped to (0..1).
        let raster_to_ndc = Mat3::new(
            scale_x, 0.0,     0.0,                      
            0.0,     scale_y, 0.0,
            0.0,     0.0,     1.0,
        );

        #[rustfmt::skip]
        // X axis: (0..1) remapped to (-1..1), where AR = aspect ratio.
        // Y axis: (0..1) remapped to (1..-1).
        let ndc_to_screen = Mat3::new(
            2.0,  0.0, -1.0,
            0.0, -2.0,  1.0,
            0.0,  0.0,  1.0,
        );

        #[rustfmt::skip]
        // AR: aspect ratio.
        // H: tan(fov / 2).
        // X axis: (-1..1) remapped to (-AR..AR).
        // Y axis: (1..-1) remapped to (H..-H).
        let screen_to_camera = Mat3::new(
            aspect_ratio * h, 0.0, 0.0,
            0.0,              h,   0.0,
            0.0,              0.0, 1.0,
        );

        let raster_to_camera = screen_to_camera * ndc_to_screen * raster_to_ndc;

        #[rustfmt::skip]
        let camera_to_world = Mat4::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        );

        let raster_to_world = camera_to_world * glm::mat3_to_mat4(&raster_to_camera);

        Camera {
            image_width,
            image_height,
            raster_to_world,
        }
    }

    pub fn render<T: Geometry>(&self, world: &T, max_depth: u32) {
        // Print metadata
        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        let pixels = (0..self.image_height)
            .progress()
            .cartesian_product(0..self.image_width)
            .map(|(y, x)| {
                let ray = self.get_ray(x, y);
                let color = Camera::ray_color(&ray, world, max_depth);
                color
            });

        for pixel_color in pixels {
            Camera::write_color(pixel_color);
        }
    }

    fn ray_color<T: Geometry>(ray: &Ray, world: &T, depth: u32) -> Vec3 {
        if depth <= 0 {
            Vec3::new(0.0, 0.0, 0.0)
        } else if let Some(hit_record) = world.hit(&ray, &Interval::new(0.001, f32::INFINITY)) {
            if let Some((scattered_ray, attenuation)) =
                hit_record.material.scatter(ray, &hit_record)
            {
                attenuation.component_mul(&Camera::ray_color(&scattered_ray, world, depth - 1))
            } else {
                Vec3::new(0.0, 0.0, 0.0)
            }
        } else {
            let unit_direction = ray.direction;
            let a = 0.5 * (unit_direction.y + 1.0);

            (1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0)
        }
    }

    fn get_ray(&self, x: u32, y: u32) -> Ray {
        let p_screen = Vec4::new(x as f32 + 0.5, y as f32 + 0.5, 1.0, 1.0);
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
        let camera = Camera::new(10, 10, 90.0);
        let ray = camera.get_ray(0, 0);

        let expected = Vec3::new(-0.9, 0.9, -1.0);
        let given = ray.direction;

        assert!(
            glm::equal_eps(&expected, &given, glm::epsilon())
                .iter()
                .all(|&x| x),
            "expected {:?}, given {:?}",
            expected,
            given
        )
    }

    #[test]
    fn square_camera_test99() {
        let camera = Camera::new(10, 10, 90.0);
        let ray = camera.get_ray(9, 9);

        let expected = Vec3::new(0.9, -0.9, -1.0);
        let given = ray.direction;

        assert!(
            glm::equal_eps(&expected, &given, glm::epsilon())
                .iter()
                .all(|&x| x),
            "expected {:?}, given {:?}",
            expected,
            given
        )
    }

    #[test]
    fn rectangular_camera_test00() {
        let camera = Camera::new(20, 10, 90.0);
        let ray = camera.get_ray(0, 0);

        let expected = Vec3::new(-1.9, 0.9, -1.0);
        let given = ray.direction;

        assert!(
            glm::equal_eps(&expected, &given, glm::epsilon())
                .iter()
                .all(|&x| x),
            "expected {:?}, given {:?}",
            expected,
            given
        )
    }

    #[test]
    fn rectangular_camera_test99() {
        let camera = Camera::new(20, 10, 90.0);
        let ray = camera.get_ray(19, 9);

        let expected = Vec3::new(1.9, -0.9, -1.0);
        let given = ray.direction;

        assert!(
            glm::equal_eps(&expected, &given, glm::epsilon())
                .iter()
                .all(|&x| x),
            "expected {:?}, given {:?}",
            expected,
            given
        )
    }
}
