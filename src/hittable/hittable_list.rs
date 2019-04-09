use super::{Hittable};

pub struct HittableList {
    hittables: Vec<Box<Hittable>>,
}

impl HittableList {
    pub fn new(hittables: Vec<Box<Hittable>>) -> HittableList {
        HittableList { hittables }
    }
}

impl Hittable for HittableList {
    
}