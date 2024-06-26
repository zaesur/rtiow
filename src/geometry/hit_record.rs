use glm::Vec3;

use crate::material::material::Material;

pub struct HitRecord<'a> {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub material: &'a dyn Material,
}

impl<'a> HitRecord<'a> {
    pub fn new(
        t: f32,
        p: Vec3,
        normal: Vec3,
        front_face: bool,
        material: &'a dyn Material,
    ) -> Self {
        HitRecord {
            t,
            p,
            normal,
            front_face,
            material,
        }
    }
}
