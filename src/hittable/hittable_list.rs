use super::{Hittable, Hit};
use crate::Ray;

pub struct HittableList {
    hittables: Vec<Box<Hittable>>,
}

impl HittableList {
    pub fn new(hittables: Vec<Box<Hittable>>) -> HittableList {
        HittableList { hittables }
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let mut closest_so_far = t_max;
        let mut result: Option<Hit> = None;

        for hittable in self.hittables.iter() {
            if let Some(hit) = hittable.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit.t;
                result = Some(hit);
            }
        }
        result
    }
}
