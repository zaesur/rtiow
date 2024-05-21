use glm::Vec3;

pub trait Reflect {
    fn reflect(vector: &Vec3, normal: &Vec3) -> Vec3 {
        vector - 2.0 * glm::dot(vector, normal) * normal
    }
}
