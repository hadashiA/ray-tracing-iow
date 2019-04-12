extern crate rand;

use rand::Rng;

mod vec3;
mod ray;
mod camera;
mod hittable;
mod material;

pub use vec3::Vec3;
pub use ray::Ray;
pub use camera::Camera;
pub use hittable::*;
pub use material::*;

// 単位円のなかのランダムな座標を返す
// あてずっぽで座標をつくってチェックする棄却法
// TODO: 極座標で表現すればループする必要ない
pub fn random_in_unit_sphere() -> Vec3 {
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
