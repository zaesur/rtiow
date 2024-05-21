use glm::Vec3;
use crate::ray::Ray;

pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f32,
}

impl HitRecord {
    pub fn new(t: f32, p: Vec3, normal: Vec3, ray: &Ray) -> Self {
        let front_face = ray.direction.dot(&normal) < 0.0;
        HitRecord {
            t,
            p,
            normal: if front_face { normal } else { -normal },
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct HittableList<T: Hittable> {
    items: Vec<T>,
}

#[allow(dead_code)]
impl<T: Hittable> HittableList<T> {
    pub fn new(items: Vec<T>) -> Self {
        HittableList { items }
    }
}

impl<T: Hittable> Hittable for HittableList<T> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.items.iter().fold(None, |closest_hit, hittable| {
            if let Some(ref closest) = closest_hit {
                hittable.hit(ray, t_min, closest.t).or(closest_hit)
            } else {
                hittable.hit(ray, t_min, t_max)
            }
        })
    }
}
