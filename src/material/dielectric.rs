use super::Material;
use crate::{Ray, Vec3};
use crate::hittable::Hit;
use crate::material::{Sample, refract};

pub struct Dielectric {
    refraction_index: f32 // 相対屈折率
}

impl Dielectric {
    pub fn new(refraction_index: f32) -> Dielectric {
        Dielectric { refraction_index }
    }
}

impl Material for Dielectric {
    fn sample(&self, ray: &Ray, hit: &Hit) -> Option<Sample> {
        let refracted =
            if Vec3::dot(&ray.direction, &hit.normal) > 0.0 {
                refract(&ray.direction, &(-hit.normal), self.refraction_index)
            } else {
                refract(&ray.direction, &hit.normal, 1.0 / self.refraction_index)
            };

        if let Some(refracted) = refracted {
            Some(Sample {
                attenuation: Vec3::ONE,
                scattered: Ray { origin: hit.p, direction: refracted }
            })
        } else {
            None
        }
    }
}