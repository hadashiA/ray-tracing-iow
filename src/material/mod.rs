mod lambertian;
mod metal;
mod dielectric;

use crate::{Vec3, Ray};
use crate::hittable::Hit;

pub use lambertian::Lambertian;
pub use metal::Metal;
pub use dielectric::Dielectric;

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v + -(2.0 * (Vec3::dot(v, n) * *n))
}

// https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/refract.xhtml
// eta は相対屈折率
pub fn refract(v: &Vec3, n: &Vec3, eta: f32) -> Option<Vec3> {
    let v = v.normalized();
    let n = *n;
    let d = Vec3::dot(&n, &v);
    let k = 1.0 - eta * eta * (1.0 - d * d);
    if k > 0.0 {
        // Some(eta * v - (eta * d + k.sqrt()) * *n)
        Some(eta * (v - n * d) - n * k.sqrt())
    } else {
        None
    }
}

pub struct Sample {
    pub attenuation: Vec3,
    pub scattered: Ray,
}

pub trait Material {
    fn sample(&self, ray: &Ray, hit: &Hit) -> Option<Sample>;
}
