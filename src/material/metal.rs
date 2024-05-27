use crate::geometry::hit_record::HitRecord;
use crate::math::utils::random_unit_vector;
use crate::ray::ray::Ray;

use super::material::Material;
use super::reflect::Reflect;
use glm::Vec3;
use rand::rngs::ThreadRng;

pub struct Metal {
    albedo: Vec3,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Self {
        Metal { albedo, fuzz }
    }
}

impl Reflect for Metal {}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)> {
        let mut rng = ThreadRng::default();
        let reflection = Metal::reflect(&ray.direction, &hit_record.normal);
        let reflected = reflection.normalize() + (self.fuzz * random_unit_vector(&mut rng));
        let scattered = Ray::new(hit_record.p, reflected);

        if glm::dot(&scattered.direction, &hit_record.normal) > 0.0 {
            let result = (scattered, self.albedo);
            Some(result)
        } else {
            None
        }
    }
}
