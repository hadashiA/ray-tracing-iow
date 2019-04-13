use crate::material::{Material, reflect};
use crate::{Ray, Vec3, random_in_unit_sphere};
use crate::hittable::{Hittable, Hit, Sphere};
use crate::material::Sample;

pub struct Metal {
    albedo: Vec3,
    fuzziness: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzziness: f32) -> Metal {
        Metal { albedo, fuzziness }
    }
}

impl Material for Metal {
    fn sample(&self, ray: &Ray, hit: &Hit) -> Option<Sample> {
        let reflected = reflect(&ray.direction.normalized(), &hit.normal);
        let scattered = Ray {
            origin: hit.p,
            direction: reflected + self.fuzziness * random_in_unit_sphere(),
        };

        if Vec3::dot(&reflected, &hit.normal) > 0.0 {
            Some(Sample { attenuation: self.albedo, scattered })
        } else {
            None
        }
    }
}