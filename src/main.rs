extern crate nalgebra_glm as glm;

mod ray;
mod sphere;
mod hittable;

use glm::Vec3;
use ray::Ray;
use sphere::Sphere;
use hittable::Hittable;
use indicatif::ProgressBar;

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
        },
        Some(hr) => {
            return 0.5 * (hr.normal + Vec3::new(1.0, 1.0, 1.0))
        }
    }

}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width: u64 = 800;
    let image_height: u64 = (image_width as f32 / aspect_ratio) as u64;
    let pb = ProgressBar::new(image_height);

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width: f32 = viewport_height * image_width as f32 / image_height as f32;
    let camera_center = Vec3::new(0.0, 0.0, 0.0);

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / image_width as f32;
    let pixel_delta_v = viewport_v / image_height as f32;

    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Print metadata
    println!("P3\n{} {}\n255", image_width, image_height);

    // Print data
    for j in 0..image_height {
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (i as f32 * pixel_delta_u) + (j as f32 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&r, Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));
            write_color(pixel_color);
        }
        pb.inc(1);
    }

    pb.finish_with_message("Done.");
}
