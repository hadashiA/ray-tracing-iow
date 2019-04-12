extern crate rand;

use std::f32;
use rand::Rng;
use ray_tracing_iow::*;

fn color<T: Hittable>(ray: &Ray, world: &T, count: u32) -> Vec3 {
    let hit = world.hit(ray, 0.001, f32::MAX);

    match hit {
        Some(hit) if count < 50 => {
            if let Some(reflection) = hit.material.scatter(&ray, &hit) {
                 reflection.attenuation * color(&reflection.scattered, world, count + 1)
            } else {
                Vec3::ZERO
            }
        },
        Some(_) => {
            Vec3::ZERO
        },
        None => {
            // ヒットしなくなったら、空の色を描いておしまい
            let dir = ray.direction.normalized();
            let t = 0.5 * (dir.y() + 1.0);

            let white = Vec3::new(1.0, 1.0, 1.0);
            let blue = Vec3::new(0.5, 0.7, 1.0);
            Vec3::lerp(white, blue, t)
        }
    }
}

fn main() {
    let w = 400;
    let h = 200;
    let samplings = 100;

    let mut rng = rand::thread_rng();

    let camera = Camera::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(-2.0, -1.0, -1.0),
        4.0,
        2.0);

    let world = HittableList::new(vec![
        Box::new(
            Sphere::new(
                Vec3::new(0.0, 0.0, -1.0),
                0.5,
                Lambertian::new(Vec3::new(0.8, 0.3, 0.3)))),
        Box::new(
            Sphere::new(
                Vec3::new(0.0, -100.5, -1.0),
                100.0,
                Lambertian::new(Vec3::new(0.8, 0.8, 0.0)))),
        Box::new(
            Sphere::new(
                Vec3::new(1.0, 0.0, -1.0),
                0.5,
                Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.3))),
        Box::new(
            Sphere::new(
                Vec3::new(-1.0, 0.0, -1.0),
                0.5,
                Metal::new(Vec3::new(0.8, 0.8, 0.8), 0.1))),
    ]);

    println!("P3\n{} {}\n255", w, h);

    for y in (0..h).rev() {
        let v = y as f32 / h as f32;

        for x in 0..w {
            let u = x as f32 / w as f32;

            let mut col = Vec3::new(0.0, 0.0, 0.0);

            // ランダムに周辺をサンプリングしてAAする
            for _ in 0..samplings {
                let u = u + rng.gen::<f32>() / w as f32;
                let v = v + rng.gen::<f32>() / h as f32;
                let ray = camera.create_ray(u, v);
                col += color(&ray, &world, 0);
            }

            col = col / samplings as f32;

            let r = (col.r() * 255.99) as u32;
            let g = (col.g() * 255.99) as u32;
            let b = (col.b() * 255.99) as u32;

            println!("{} {} {}", r, g, b);
        }
    }
}
