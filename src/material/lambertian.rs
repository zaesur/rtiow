use glm::Vec3;

use super::material::Material;
use crate::geometry::hit_record::HitRecord;
use crate::math::utils::random_unit_vector;
use crate::ray::ray::Ray;

const EPSILON: f32 = 1e-8;

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)> {
        let scatter_direction = hit_record.normal + random_unit_vector();
        let result = (
            Ray::new(
                hit_record.p,
                if glm::length2(&scatter_direction) < EPSILON.powi(2) {
                    hit_record.normal
                } else {
                    scatter_direction
                },
            ),
            self.albedo,
        );
        Some(result)
    }
}
