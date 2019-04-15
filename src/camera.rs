use std::f32;
use crate::{Vec3, Ray};

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(origin: Vec3, lookat: Vec3, up: Vec3, vfov: f32, aspect: f32) -> Camera {
        let theta = vfov * f32::consts::PI / 180.0;
        let half_height = (theta * 0.5).tan();
        let half_width = half_height * aspect;

        let ez = (origin - lookat).normalized();
        let ex = Vec3::cross(&up, &ez).normalized();
        let ey = Vec3::cross(&ez, &ex).normalized();

        let lower_left_corner = origin - half_width * ex - half_height * ey - ez;
        let horizontal = 2.0 * half_width * ex;
        let vertical = 2.0 * half_height * ey;

        Camera { origin, lower_left_corner, horizontal, vertical }
    }

    pub fn create_ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin
        }
    }
}
