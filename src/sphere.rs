use glm::Vec3;
use crate::ray::Ray;
use crate::hittable::{HitRecord, Hittable};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

#[allow(dead_code)]
impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = &self.center - &ray.origin;
        let a = &ray.direction.dot(&ray.direction);
        let h = &ray.direction.dot(&oc);
        let c = oc.dot(&oc) - &self.radius * &self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrt = discriminant.sqrt();
        let root = (h - sqrt) / a;

        if root <= t_min || t_max <= root {
            let new_root = (h + sqrt) / a;
            if new_root <= t_min || t_max <= root {
                return None
            }
        }
        
        let t = root;
        let p = ray.at(t);
        let normal = (p - &self.center) * (1.0 / &self.radius);

        Some(HitRecord::new(t, p, normal, &ray))
    }
}
