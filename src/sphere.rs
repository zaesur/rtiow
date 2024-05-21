use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use glm::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord> {
        let oc = self.center - ray.origin;
        let a = ray.direction.dot(&ray.direction);
        let h = ray.direction.dot(&oc);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrt = discriminant.sqrt();
        let root = (h - sqrt) / a;

        if !interval.surrounds(root) && !interval.surrounds((h + sqrt) / a) {
            return None;
        }

        let t = root;
        let p = ray.at(t);
        let normal = (p - self.center) * (1.0 / self.radius);

        Some(HitRecord::new(t, p, normal, ray))
    }
}
