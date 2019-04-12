use super::{Hittable, Hit};
use crate::Ray;

pub struct HittableList {
    hittables: Vec<Box<Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList { hittables: vec![] }
    }

    pub fn from_vec(hittables: Vec<Box<Hittable>>) -> HittableList {
        HittableList { hittables }
    }

    pub fn add<T: Hittable + 'static>(&mut self, hittable: T) {
        self.hittables.push(Box::new(hittable));
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
