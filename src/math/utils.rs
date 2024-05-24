use crate::math::interval::Interval;

use glm::Vec3;
use rand::random;
use std::iter;

pub fn random_vector(interval: Option<Interval>) -> Vec3 {
    match interval {
        None => Vec3::new(random(), random(), random()),
        Some(interval) => Vec3::new(
            interval.min + (interval.max - interval.min) * random::<f32>(),
            interval.min + (interval.max - interval.min) * random::<f32>(),
            interval.min + (interval.max - interval.min) * random::<f32>(),
        ),
    }
}

pub fn random_vector_in_unit_disk() -> Vec3 {
    let interval = Interval::new(-1.0, 1.0);
    iter::repeat_with(|| {
        Vec3::new(
            interval.min + (interval.max - interval.min) * random::<f32>(),
            interval.min + (interval.max - interval.min) * random::<f32>(),
            interval.min + (interval.max - interval.min) * random::<f32>(),
        )
    })
    .filter(|vector| glm::length2(vector) < 1.0)
    .next()
    .expect("No vector found!")
}

pub fn random_vector_in_unit_sphere() -> Vec3 {
    iter::repeat_with(|| random_vector(Some(Interval::new(-1.0, 1.0))))
        .filter(|vector| glm::length2(vector) < 1.0)
        .next()
        .expect("No unit vector found!")
}

pub fn random_unit_vector() -> Vec3 {
    random_vector_in_unit_sphere().normalize()
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
