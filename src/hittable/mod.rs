mod sphere;
mod hittable_list;

pub use sphere::Sphere;
pub use hittable_list::HittableList;

use super::{Vec3, Ray};

#[derive(Debug, Copy, Clone)]
pub struct Hit {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit>;
}
