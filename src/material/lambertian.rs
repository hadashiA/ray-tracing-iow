use crate::{Vec3, random_in_unit_sphere};
use crate::Ray;
use crate::hittable::{Hit, Sphere};
use crate::material::Sample;
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
    fn sample(&self, ray: &Ray, hit: &Hit) -> Option<Sample> {
        let dest = hit.p + hit.normal + random_in_unit_sphere();
        let scattered = Ray {
            origin: hit.p,
            direction: dest - hit.p
        };

        Some(Sample {
            attenuation: self.albedo,
            scattered,
        })
    }
}
