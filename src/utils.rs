use glm::Vec3;
use rand::Rng;

use crate::interval::Interval;

pub fn random_vector(interval: Option<Interval>) -> Vec3 {
    let mut rng = rand::thread_rng();
    match interval {
        None => Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>()),
        Some(interval) => Vec3::new(
            interval.min + (interval.max - interval.min) * rng.gen::<f32>(),
            interval.min + (interval.max - interval.min) * rng.gen::<f32>(),
            interval.min + (interval.max - interval.min) * rng.gen::<f32>(),
        ),
    }
}

pub fn random_vector_on_hemisphere(normal: &Vec3) -> Vec3 {
    let vector = random_vector(Some(Interval::new(-1.0, 1.0))).normalize();

    if vector.dot(normal) > 0.0 {
        vector
    } else {
        -vector
    }
}
