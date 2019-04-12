use crate::material::{Material, reflect};
use crate::{Ray, Vec3};
use crate::hittable::{Hittable, Hit, Sphere};
use crate::material::Reflection;

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
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Reflection> {
        let reflected = reflect(&ray.direction.normalized(), &hit.normal);
        let scattered = Ray {
            origin: hit.p,
            direction: reflected + self.fuzziness * Sphere::random_in_unit(),
        };

        if Vec3::dot(&reflected, &hit.normal) > 0.0 {
            Some(Reflection { attenuation: self.albedo, scattered })
        } else {
            None
        }
    }
}