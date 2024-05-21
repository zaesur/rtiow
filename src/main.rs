extern crate nalgebra_glm as glm;

mod ray;

use glm::Vec3;
use indicatif::ProgressBar;
use ray::Ray;

fn write_color(color: Vec3) {
    let ir = (255.999 * color.x) as i32;
    let ig = (255.999 * color.y) as i32;
    let ib = (255.999 * color.z) as i32;

    println!("{ir} {ig} {ib}");
}

fn lerp(a: f32, start: Vec3, end: Vec3) -> Vec3 {
    return (1.0 - a) * start + a * end;
}

fn hit_sphere(center: Vec3, radius: f32, ray: &Ray) -> f32 {
    let oc = center - &ray.origin;
    let a = &ray.direction.dot(&ray.direction);
    let h = &ray.direction.dot(&oc);
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = h * h - a * c;

    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (h - discriminant.sqrt()) / a;
    }
}

fn ray_color(ray: &Ray) -> Vec3 {
    let t = hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, &ray);
    if t > 0.0 {
        let n = (&ray.at(t) - Vec3::new(0.0, 0.0, -1.0)).normalize();
        return 0.5 * Vec3::new(n.x + 1.0, n.y + 1.0, n.z + 1.0)
    }

    let unit_direction = ray.direction;
    let a = 0.5 * (unit_direction.y + 1.0);

    return lerp(a, Vec3::new(1.0, 1.0, 1.0), Vec3::new(0.5, 0.7, 1.0));
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

            let pixel_color = ray_color(&r);
            write_color(pixel_color);
        }
        pb.inc(1);
    }

    pb.finish_with_message("Done.");
}
