extern crate nalgebra_glm as glm;

mod camera;
mod geometry;
mod material;
mod math;

use camera::builder::CameraBuilder;
use geometry::{sphere::Sphere, world::World};
use glm::Vec3;
use material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal};

fn main() {
    let ground = Lambertian::new(Vec3::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Vec3::new(0.1, 0.2, 0.5));
    let material_left = Dielectric::new(1.5);
    let material_bubble = Dielectric::new(1.0 / 1.5);
    let material_right = Metal::new(Vec3::new(0.8, 0.6, 0.2), 1.0);

    let world = World::new(vec![
        Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, ground)),
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.2), 0.5, material_center)),
        Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, material_left)),
        Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.4, material_bubble)),
        Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, material_right)),
    ]);

    let lookfrom = Vec3::new(-2.0, 2.0, 1.0);
    let lookat = Vec3::new(0.0, 0.0, -1.0);

    let camera = CameraBuilder::new()
        .aspect_ratio(16.0 / 9.0)
        .image_width(400)
        .samples_per_pixel(50)
        .max_depth(10)
        .vfov(20.0)
        .defocus_angle(10.0)
        .focus_dist(3.4)
        .build(lookfrom, lookat);

    camera.render(&world)
}
