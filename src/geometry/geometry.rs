use crate::ray::ray::Ray;
use crate::math::interval::Interval;
use super::hit_record::HitRecord;

pub trait Geometry: Sync {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord>;
}
