use ray_tracing_iow::{Vec3, Ray};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn hit_test(&self, ray: &Ray) -> Option<f32> {
        let diff = ray.origin - self.center;
        let a = Vec3::dot(&ray.direction, &ray.direction);
        let b = 2.0 * Vec3::dot(&diff, &ray.direction);
        let c = Vec3::dot(&diff, &diff) - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            None
        } else {
            Some((-b - discriminant.sqrt()) / (2.0 * a))
        }
    }
}

fn color(ray: &Ray) -> Vec3 {
    let sphere = Sphere {
        center: Vec3::new(0.0, 0.0, -1.0),
        radius: 0.5
    };

    let t = sphere.hit_test(ray);

    match t {
        Some(t) => {
            let normal = (ray.point_at(t) - sphere.center).normalized();
            Vec3::new(normal.x() + 1.0,normal.y() + 1.0,normal.z() + 1.0) * 0.5
        },
        None => {
            let dir = ray.direction.normalized();
            let t = 0.5 * (dir.y() + 1.0);

            let white = Vec3::new(1.0, 1.0, 1.0);
            let blue = Vec3::new(0.5, 0.7, 1.0);
            Vec3::lerp(white, blue, t)
        }
    }
}

fn main() {
    let w = 200;
    let h = 100;

    let bottom_left = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    println!("P3\n{} {}\n255", w, h);

    for y in (0..h).rev() {
        let v = y as f32 / h as f32;

        for x in 0..w {
            let u = x as f32 / w as f32;
            let ray = Ray {
                origin,
                direction: bottom_left + u * horizontal + v * vertical
            };

            let col = color(&ray);
            let r = (col.r() * 255.99) as u32;
            let g = (col.g() * 255.99) as u32;
            let b = (col.b() * 255.99) as u32;

            println!("{} {} {}", r, g, b);
        }
    }
}
