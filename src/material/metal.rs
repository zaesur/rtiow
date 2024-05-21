use crate::geometry::hit_record::HitRecord;
use crate::camera::ray::Ray;
use crate::math::utils::reflect;

use super::material::Material;
use glm::Vec3;

pub struct Metal {
    albedo: Vec3,
}

impl Metal {
    pub fn new(albedo: Vec3) -> Self {
        Metal { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)> {
        let reflected = reflect(&ray.direction, &hit_record.normal);
        let scattered = Ray::new(hit_record.p, reflected);
        let result = (scattered, self.albedo);
        Some(result)
    }
}
