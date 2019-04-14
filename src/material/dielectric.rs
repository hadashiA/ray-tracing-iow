extern crate rand;

use rand::{Rng};
use rand::rngs::ThreadRng;

use super::Material;
use crate::{Ray, Vec3};
use crate::hittable::Hit;
use crate::material::{Sample, refract, reflect, schlick};
use std::cell::RefCell;

pub struct Dielectric {
    refraction_index: f32, // 相対屈折率
    rng: RefCell<ThreadRng>,
}

impl Dielectric {
    pub fn new(refraction_index: f32) -> Dielectric {
        Dielectric {
            refraction_index,
            rng: RefCell::new(rand::thread_rng()),
        }
    }
}

impl Material for Dielectric {
    fn sample(&self, ray: &Ray, hit: &Hit) -> Option<Sample> {
        let sharply = Vec3::dot(&ray.direction, &hit.normal) > 0.0;

        let (outward_normal, refraction_index) =
            if sharply {
                (-hit.normal, self.refraction_index)
            } else {
                (hit.normal, 1.0 / self.refraction_index)
            };

        let refracted = refract(&ray.direction, &outward_normal, refraction_index);

        let direction =
            if let Some(refracted) = refracted {
                let cosine =
                    if sharply {
                        refraction_index * Vec3::dot(&ray.direction, &hit.normal) / ray.direction.length()
                    } else {
                        -Vec3::dot(&ray.direction, &hit.normal) / ray.direction.length()
                    };
                let reflect_probe = schlick(cosine, self.refraction_index);
                if self.rng.borrow_mut().gen::<f32>() < reflect_probe {
                    reflect(&ray.direction, &hit.normal)
                } else {
                    refracted
                }
            } else {
                reflect(&ray.direction, &hit.normal)
            };

        Some(Sample {
            attenuation: Vec3::ONE,
            scattered: Ray { origin: hit.p, direction }
        })
    }
}
