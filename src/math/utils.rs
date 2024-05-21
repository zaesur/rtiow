use crate::math::interval::Interval;
use glm::Vec3;
use rand::Rng;
use std::iter;

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

pub fn random_vector_in_unit_sphere() -> Option<Vec3> {
    iter::repeat_with(|| random_vector(Some(Interval::new(-1.0, 1.0))))
        .filter(|vector| glm::length2(vector) < 1.0)
        .next()
}

pub fn random_unit_vector() -> Vec3 {
    random_vector_in_unit_sphere()
        .and_then(|vector| Some(vector.normalize()))
        .expect("No unit vector found!")
}

#[allow(dead_code)]
pub fn random_vector_on_hemisphere(normal: &Vec3) -> Vec3 {
    let vector = random_unit_vector();

    if vector.dot(normal) > 0.0 {
        vector
    } else {
        -vector
    }
}

pub fn reflect(vector: &Vec3, normal: &Vec3) -> Vec3 {
    vector - 2.0 * vector.dot(normal) * normal
}
