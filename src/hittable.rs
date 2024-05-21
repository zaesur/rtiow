use crate::interval::Interval;
use crate::material::material::Material;
use crate::ray::Ray;
use glm::Vec3;

pub struct HitRecord<'a> {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub material: &'a dyn Material,
}

impl<'a> HitRecord<'a> {
    pub fn new(t: f32, p: Vec3, normal: Vec3, material: &'a dyn Material) -> Self {
        HitRecord { t, p, normal, material }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord>;
}

pub struct HittableList {
    items: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new(items: Vec<Box<dyn Hittable>>) -> Self {
        HittableList { items }
    }
}

impl Hittable for HittableList {
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
