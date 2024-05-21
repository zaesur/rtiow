use glm::Vec3;

use crate::math::interval::Interval;
use crate::material::material::Material;
use crate::camera::ray::Ray;

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

impl<T: Material> Geometry for Sphere<T> {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord> {
        let oc = self.center - ray.origin;
        let a = ray.direction.dot(&ray.direction);
        let h = ray.direction.dot(&oc);
        let c = oc.dot(&oc) - f32::powi(self.radius, 2);
        let discriminant = f32::powi(h, 2) - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrt = discriminant.sqrt();
        let root = (h - sqrt) / a;

        if !(interval.surrounds(root) || interval.surrounds((h + sqrt) / a)) {
            return None;
        }

        let t = root;
        let p = ray.at(t);
        let normal = (p - self.center) * (1.0 / self.radius);
        let front_face = ray.direction.dot(&normal) < 0.0;
        let corrected_normal = if front_face { normal } else { -normal };

        Some(HitRecord::new(t, p, corrected_normal, &self.material))
    }
}
