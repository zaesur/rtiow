use glm::Vec3;

use crate::ray::ray::Ray;
use crate::material::material::Material;
use crate::math::interval::Interval;

use super::geometry::Geometry;
use super::hit_record::HitRecord;

pub struct Sphere<T: Material> {
    pub center: Vec3,
    pub radius: f32,
    material: T,
}

impl<T: Material> Sphere<T> {
    pub fn new(center: Vec3, radius: f32, material: T) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl<T: Material + Sync> Geometry for Sphere<T> {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord> {
        let oc = self.center - ray.origin;
        let a = glm::length2(&ray.direction);
        let h = glm::dot(&ray.direction, &oc);
        let c = glm::length2(&oc) - self.radius.powi(2);
        let discriminant = h.powi(2) - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrt = discriminant.sqrt();
        let mut root = (h - sqrt) / a;

        if !interval.surrounds(root) {
            root = (h + sqrt) / a;
            if !interval.surrounds(root) {
                return None
            }
        }

        let t = root;
        let p = ray.at(t);
        let outward_normal = (p - self.center) / self.radius;
        let front_face = glm::dot(&ray.direction, &outward_normal) < 0.0;
        let normal = if front_face { outward_normal } else { -outward_normal };

        Some(HitRecord::new(
            t,
            p,
            normal,
            front_face,
            &self.material,
        ))
    }
}
