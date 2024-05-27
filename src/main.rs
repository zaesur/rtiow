extern crate nalgebra_glm as glm;

mod camera;
mod geometry;
mod material;
mod math;
mod ray;

use camera::builder::CameraBuilder;
use geometry::{sphere::Sphere, world::World};
use glm::Vec3;
use material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal};
use math::{interval::Interval, utils::random_vector};
use rand::random;

fn main() {
    let ground_material = Lambertian::new(Vec3::new(0.5, 0.5, 0.5));
    let mut world = World::new(vec![Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ))]);

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f32 = random();
            let center = Vec3::new(
                a as f32 + 0.9 * random::<f32>(),
                0.2,
                b as f32 + 0.9 * random::<f32>(),
            );

            if glm::length(&(center - Vec3::new(4.0, 0.2, 0.0))) > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = random_vector(None).component_mul(&random_vector(None));
                    let sphere = Sphere::new(center, 0.2, Lambertian::new(albedo));
                    world.add(Box::new(sphere))
                } else if choose_mat < 0.95 {
                    let albedo = random_vector(Some(Interval::new(0.5, 1.0)));
                    let fuzz = random::<f32>() * 0.5;
                    let sphere = Sphere::new(center, 0.2, Metal::new(albedo, fuzz));
                    world.add(Box::new(sphere));
                } else {
                    let sphere = Sphere::new(center, 0.2, Dielectric::new(1.5));
                    world.add(Box::new(sphere));
                }
            }
        }
    }

    let material1 = Dielectric::new(1.5);
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Lambertian::new(Vec3::new(0.4, 0.2, 0.1));
    world.add(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0);
    world.add(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);

    let camera = CameraBuilder::new()
        .aspect_ratio(16.0 / 9.0)
        .image_width(1200)
        .samples_per_pixel(500)
        .max_depth(50)
        .vfov(20.0)
        .defocus_angle(0.6)
        .focus_dist(10.0)
        .build(lookfrom, lookat);

    camera.render(&world)
}
