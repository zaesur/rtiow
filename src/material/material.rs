use glm::Vec3;

use crate::geometry::hittable::HitRecord;
use crate::camera::ray::Ray;

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)>;
}
