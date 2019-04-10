use crate::{Vec3, Ray};

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    width: f32,
    height: f32,
}

impl Camera {
    pub fn new(origin: Vec3, lower_left_corner: Vec3, width: f32, height: f32) -> Camera {
        Camera { origin, lower_left_corner, width, height }
    }

    pub fn create_ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            origin: self.origin,
            direction: Vec3::new(
                self.lower_left_corner.x() + u * self.width,
                self.lower_left_corner.y() + v * self.height,
                self.lower_left_corner.z()
            ) - self.origin
        }
    }
}
