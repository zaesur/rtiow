use crate::camera::ray::Ray;
use crate::math::interval::Interval;

use super::geometry::Geometry;
use super::hit_record::HitRecord;

pub struct World {
    items: Vec<Box<dyn Geometry>>,
}

impl World {
    pub fn new(items: Vec<Box<dyn Geometry>>) -> Self {
        World { items }
    }

    pub fn add(&mut self, item: Box<dyn Geometry>) -> () {
        self.items.push(item);
    }
}

impl Geometry for World {
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
