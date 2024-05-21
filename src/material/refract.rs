use glm::Vec3;

pub trait Refract {
    fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f32) -> Vec3 {
        let cos_theta = glm::dot(&(-uv), n).min(1.0);

        let r_out_perp = etai_over_etat * (uv + cos_theta * n);
        let r_out_parallel = -(1.0 - glm::length2(&r_out_perp)).abs().sqrt() * n;
        r_out_perp + r_out_parallel
    }
}
