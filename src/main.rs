extern crate nalgebra_glm as glm;

mod camera;
mod hittable;
mod interval;
mod ray;
mod sphere;
mod utils;

use camera::Camera;
use glm::Vec3;
use hittable::HittableList;
use sphere::Sphere;

fn main() {
    let world = HittableList::new(vec![
        Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5),
        Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0),
    ]);

    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;
    const FOCAL_LENGTH: f32 = 1.0;
    const SAMPLES_PER_PIXEL: u32 = 100;
    const MAX_DEPTH: u32 = 50;

    let camera = Camera::new(
        ASPECT_RATIO,
        IMAGE_WIDTH,
        FOCAL_LENGTH,
        SAMPLES_PER_PIXEL,
        MAX_DEPTH,
    );

    camera.render(&world)
}
