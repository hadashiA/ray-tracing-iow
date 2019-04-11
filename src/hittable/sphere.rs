extern crate rand;

use super::{Vec3, Ray, Hit, Hittable};
use self::rand::Rng;

#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    // 単位円のなかのランダムな座標を返す
    // あてずっぽで座標をつくってチェックする棄却法
    // TODO: 極座標で表現すればループする必要ない
    pub fn random_in_unit() -> Vec3 {
        let mut rng = rand::thread_rng();
        loop {
            let p = Vec3::new(
                rng.gen(),
                rng.gen(),
                rng.gen());

            if p.length_squared() <= 1.0 {
                return p
            }
        }
    }
}

impl Hittable for Sphere {
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
            return Some(Hit { t, p, normal })
        }

        // 大きい方の解
        let t = -b + discriminant_sqrt / a2;
        if t_min <= t && t <= t_max {
            let p = ray.point_at(t);
            let normal = (p - self.center).normalized();
            return Some(Hit { t, p, normal })
        }

        None
    }
}
