extern crate nalgebra_glm as glm;

mod hittable;
mod ray;
mod sphere;

use glm::Vec3;
use hittable::{Hittable, HittableList};
use indicatif::ProgressBar;
use ray::Ray;
use sphere::Sphere;

const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 400;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;
const FOCAL_LENGTH: f32 = 1.0;

fn write_color(color: Vec3) {
    let ir = (255.999 * color.x) as i32;
    let ig = (255.999 * color.y) as i32;
    let ib = (255.999 * color.z) as i32;

    println!("{ir} {ig} {ib}");
}

fn lerp(a: f32, start: Vec3, end: Vec3) -> Vec3 {
    return (1.0 - a) * start + a * end;
}

fn ray_color<T: Hittable>(ray: &Ray, world: T) -> Vec3 {
    let result = world.hit(&ray, 0.0, 100.0);

    match result {
        None => {
            let unit_direction = ray.direction;
            let a = 0.5 * (unit_direction.y + 1.0);

            return lerp(a, Vec3::new(1.0, 1.0, 1.0), Vec3::new(0.5, 0.7, 1.0));
        }
        Some(hr) => return 0.5 * (hr.normal + Vec3::new(1.0, 1.0, 1.0)),
    }
}

fn main() {
    let pb = ProgressBar::new(IMAGE_HEIGHT.into());

    // Camera
    let viewport_height = 2.0;
    let viewport_width: f32 = viewport_height * IMAGE_WIDTH as f32 / IMAGE_HEIGHT as f32;
    let camera_center = Vec3::new(0.0, 0.0, 0.0);

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / IMAGE_WIDTH as f32;
    let pixel_delta_v = viewport_v / IMAGE_HEIGHT as f32;

    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, FOCAL_LENGTH) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Print metadata
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    // Print data
    for j in 0..IMAGE_HEIGHT {
        for i in 0..IMAGE_WIDTH {
            let pixel_center =
                pixel00_loc + (i as f32 * pixel_delta_u) + (j as f32 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            let world = HittableList::new(Vec::from([
                Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5),
                Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)
            ]));
            let pixel_color = ray_color(&r, world);

            write_color(pixel_color);
        }
        pb.inc(1);
    }

    pb.finish_with_message("Done.");
}
