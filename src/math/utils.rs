use crate::math::interval::Interval;

use glm::Vec3;
use rand::Rng;
use std::iter;

pub fn random_vector<T: Rng>(rng: &mut T, interval: Option<Interval>) -> Vec3 {
    match interval {
        None => Vec3::new(rng.gen(), rng.gen(), rng.gen()),
        Some(interval) => Vec3::new(
            rng.gen_range(interval.min..interval.max),
            rng.gen_range(interval.min..interval.max),
            rng.gen_range(interval.min..interval.max),
        ),
    }
}

pub fn random_vector_in_unit_disk<T: Rng>(rng: &mut T) -> Vec3 {
    let interval = Interval::new(-1.0, 1.0);
    iter::repeat_with(|| {
        Vec3::new(
            rng.gen_range(interval.min..interval.max),
            rng.gen_range(interval.min..interval.max),
            rng.gen_range(interval.min..interval.max),
        )
    })
    .filter(|vector| glm::length2(vector) < 1.0)
    .next()
    .expect("No vector found!")
}

pub fn random_vector_in_unit_sphere<T: Rng>(rng: &mut T) -> Vec3 {
    iter::repeat_with(|| random_vector(rng, Some(Interval::new(-1.0, 1.0))))
        .filter(|vector| glm::length2(vector) < 1.0)
        .next()
        .expect("No unit vector found!")
}

pub fn random_unit_vector<T: Rng>(rng: &mut T) -> Vec3 {
    random_vector_in_unit_sphere(rng).normalize()
}

#[allow(dead_code)]
pub fn random_vector_on_hemisphere<T: Rng>(rng: &mut T, normal: &Vec3) -> Vec3 {
    let vector = random_unit_vector(rng);

    if vector.dot(normal) > 0.0 {
        vector
    } else {
        -vector
    }
}
