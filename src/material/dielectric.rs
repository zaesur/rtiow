use glm::Vec3;
use rand::random;

use crate::geometry::hit_record::HitRecord;
use crate::ray::ray::Ray;

use super::material::Material;

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

        let direction = if ri * sin_theta > 1.0 || Dielectric::reflectance(cos_theta, ri) > random()
        {
            glm::reflect_vec(&unit_direction, &hit_record.normal)
        } else {
            glm::refract_vec(&unit_direction, &hit_record.normal, ri)
        };

        let scattered = Ray::new(hit_record.p, direction);
        Some((scattered, attenuation))
    }
}
