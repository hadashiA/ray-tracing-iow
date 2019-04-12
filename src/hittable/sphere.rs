use crate::{Vec3, Ray, Hit, Hittable};
use crate::material::Material;
use core::borrow::Borrow;

pub struct Sphere<M: Material> {
    pub center: Vec3,
    pub radius: f32,
    material: M,
}

impl<M: Material> Sphere<M> {
    pub fn new(center: Vec3, radius: f32, material: M) -> Sphere<M> {
        Sphere { center, radius, material }
    }
}

impl<M: Material> Hittable for Sphere<M> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let diff = ray.origin - self.center;
        let a = Vec3::dot(&ray.direction, &ray.direction);
        let b = 2.0 * Vec3::dot(&diff, &ray.direction);
        let c = Vec3::dot(&diff, &diff) - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return None
        }

        let discriminant_sqrt = discriminant.sqrt();
        let a2 = 2.0 * a;

        // 小さい方の解
        let t = (-b - discriminant_sqrt) / a2;
        if t_min <= t && t <= t_max {
            let p = ray.point_at(t);
            let normal = (p - self.center).normalized();
            return Some(Hit { t, p, normal, material: &self.material })
        }

        // 大きい方の解
        let t = -b + discriminant_sqrt / a2;
        if t_min <= t && t <= t_max {
            let p = ray.point_at(t);
            let normal = (p - self.center).normalized();
            return Some(Hit { t, p, normal, material: &self.material })
        }

        None
    }
}
