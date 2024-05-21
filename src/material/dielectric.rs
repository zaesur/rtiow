use glm::Vec3;
use rand::random;

use crate::camera::ray::Ray;
use crate::geometry::hit_record::HitRecord;

use super::material::Material;
use super::reflect::Reflect;
use super::refract::Refract;

pub struct Dielectric {
    refraction_index: f32,
}

impl Dielectric {
    pub fn new(refraction_index: f32) -> Self {
        Dielectric { refraction_index }
    }

    fn reflectance(cosine: f32, refraction_index: f32) -> f32 {
        let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        let r02 = r0.powi(2);
        r02 + (1.0 - r02) * (1.0 - cosine).powi(5)
    }
}

impl Reflect for Dielectric {}
impl Refract for Dielectric {}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)> {
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        let ri = if hit_record.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let unit_direction = ray.direction.normalize();
        let cos_theta = glm::dot(&-unit_direction, &hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let direction = if ri * sin_theta > 1.0 || Dielectric::reflectance(cos_theta, ri) > random() {
            Dielectric::reflect(&unit_direction, &hit_record.normal)
        } else {
            Dielectric::refract(&unit_direction, &hit_record.normal, ri)
        };

        let scattered = Ray::new(hit_record.p, direction);
        Some((scattered, attenuation))
    }
}
