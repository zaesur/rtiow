extern crate nalgebra_glm as glm;

mod camera;
mod hittable;
mod interval;
mod ray;
mod sphere;

use camera::Camera;
use glm::Vec3;
use hittable::HittableList;
use sphere::Sphere;

fn main() {
    let world = HittableList::new(vec![
        Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5),
        Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0),
    ]);

    let camera = Camera::new(16.0 / 9.0, 400, 1.0);

    camera.render(&world)
}
