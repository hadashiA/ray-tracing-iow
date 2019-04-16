use std::f32;
use crate::{Vec3, Ray, random_in_unit_disc};

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lens_radius: f32,

    ex: Vec3,
    ey: Vec3,
    ez: Vec3,
}

impl Camera {
    pub fn new(origin: Vec3, lookat: Vec3, up: Vec3, vfov: f32, aspect: f32, aperture: f32, focus_dist: f32) -> Camera {
        let theta = vfov * f32::consts::PI / 180.0;
        let half_height = (theta * 0.5).tan();
        let half_width = half_height * aspect;

        let ez = (origin - lookat).normalized();
        let ex = Vec3::cross(&up, &ez).normalized();
        let ey = Vec3::cross(&ez, &ex).normalized();

        let lower_left_corner = origin - half_width * focus_dist * ex - half_height * focus_dist * ey - focus_dist * ez;
        let horizontal = 2.0 * half_width * focus_dist * ex;
        let vertical = 2.0 * half_height * focus_dist * ey;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            lens_radius: aperture * 0.5,
            ex, ey, ez,
        }
    }

    pub fn create_ray(&self, u: f32, v: f32) -> Ray {
        let rd = random_in_unit_disc();
        let offset = self.ex * rd.x() + self.ey * rd.y();

        Ray {
            origin: self.origin + offset,
            direction: self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset
        }
    }
}
