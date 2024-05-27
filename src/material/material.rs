use glm::Vec3;

use crate::geometry::hit_record::HitRecord;
use crate::ray::ray::Ray;

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)>;
}
