extern crate nalgebra_glm as glm;

mod camera;
mod geometry;
mod material;
mod math;
mod ray;

use camera::camera::Camera;
use geometry::{sphere::Sphere, world::World};
use glm::Vec3;
use material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal};
use math::{interval::Interval, utils::random_vector};
use rand::{rngs::ThreadRng, Rng};

fn main() {
    const IMAGE_WIDTH: u32 = 600;
    const IMAGE_HEIGHT: u32 = 275;
    const SAMPLES_PER_PIXEL: u32 = 50;
    const MAX_DEPTH: u32 = 10;
    const FOV: f32 = 20.0;

    let position = Vec3::new(-2.0, 2.0, 1.0);
    let mut camera = Camera::new(IMAGE_WIDTH, IMAGE_HEIGHT, FOV, position);
    let world = metal_and_glass_scene();

    camera.lookat(Vec3::new(0.0, 0.0, -1.0));
    camera.render(&world, MAX_DEPTH, SAMPLES_PER_PIXEL);
}

#[allow(dead_code)]
fn metal_and_glass_scene() -> World {
    let material_ground = Lambertian::new(Vec3::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Vec3::new(0.1, 0.2, 0.5));
    let material_left = Dielectric::new(1.5);
    let material_bubble = Dielectric::new(1.0 / 1.5);
    let material_right = Metal::new(Vec3::new(0.8, 0.6, 0.2), 1.0);

    let mut world = World::new(vec![]);

    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, material_ground)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.2), 0.5, material_center)));
    world.add(Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, material_left)));
    world.add(Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.4, material_bubble)));
    world.add(Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, material_right)));

    world
}

#[allow(dead_code)]
fn random_scene() -> World {
    let ground_material = Lambertian::new(Vec3::new(0.5, 0.5, 0.5));
    let mut world = World::new(vec![Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ))]);

    for a in -11..=11 {
        for b in -11..=11 {
            let mut rng = ThreadRng::default();
            let choose_mat: f32 = rng.gen();
            let center = Vec3::new(
                a as f32 + rng.gen_range(0.0..0.9),
                0.2,
                b as f32 + rng.gen_range(0.0..0.9),
            );

            if glm::length(&(center - Vec3::new(4.0, 0.2, 0.0))) > 0.9 {
                if choose_mat < 0.8 {
                    let albedo =
                        random_vector(&mut rng, None).component_mul(&random_vector(&mut rng, None));
                    let sphere = Sphere::new(center, 0.2, Lambertian::new(albedo));
                    world.add(Box::new(sphere))
                } else if choose_mat < 0.95 {
                    let albedo = random_vector(&mut rng, Some(Interval::new(0.5, 1.0)));
                    let fuzz = rng.gen_range(0.0..0.5);
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

    world
}
