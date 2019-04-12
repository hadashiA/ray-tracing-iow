use crate::Vec3;
use crate::Ray;
use crate::hittable::{Hit, Sphere};
use crate::material::Reflection;
use super::{Material};

pub struct Lambertian {
    albedo: Vec3
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Reflection> {
        let dest = hit.p + hit.normal + Sphere::random_in_unit();
        let scattered = Ray {
            origin: hit.p,
            direction: dest - hit.p
        };

        Some(Reflection {
            attenuation: self.albedo,
            scattered,
        })
    }
}
