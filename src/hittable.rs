use crate::interval::Interval;
use crate::ray::Ray;
use glm::Vec3;

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
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord>;
}

pub struct HittableList<T: Hittable> {
    items: Vec<T>,
}

impl<T: Hittable> HittableList<T> {
    pub fn new(items: Vec<T>) -> Self {
        HittableList { items }
    }
}

impl<T: Hittable> Hittable for HittableList<T> {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord> {
        self.items.iter().fold(None, |closest_hit, hittable| {
            if let Some(ref closest) = closest_hit {
                hittable
                    .hit(ray, &Interval::new(interval.min, closest.t))
                    .or(closest_hit)
            } else {
                hittable.hit(ray, interval)
            }
        })
    }
}
