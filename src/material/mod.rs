mod lambertian;
mod metal;

use crate::{Vec3, Ray};
use crate::hittable::Hit;

pub use self::lambertian::Lambertian;
pub use self::metal::Metal;

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v + -(2.0 * (Vec3::dot(v, n) * *n))
}

pub struct Reflection {
    pub attenuation: Vec3,
    pub scattered: Ray,
}

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Reflection>;
}
