extern crate rand;

use std::f32;
use rand::Rng;
use ray_tracing_iow::*;

fn color<T: Hittable>(ray: &Ray, world: &T) -> Vec3 {
    let hit = world.hit(ray, 0.001, f32::MAX);

    match hit {
        Some(hit) => {
            let reflection_target = hit.p + hit.normal + Sphere::random_in_unit();
            let reflection_ray = Ray {
                origin: hit.p,
                direction: reflection_target - hit.p,
            };
            color(&reflection_ray, world) * 0.5
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
        Box::new(Sphere {
            center: Vec3::new(0.0, 0.0, -1.0),
            radius: 0.5,
        }),
        Box::new(Sphere {
            center: Vec3::new(0.0, -100.5, -1.0),
            radius: 100.0,
        })
    ]);

    println!("P3\n{} {}\n255", w, h);

    for y in (0..h).rev() {

        for x in 0..w {
            let u = x as f32 / w as f32;
            let v = y as f32 / h as f32;

            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..samplings {
                let u = u + rng.gen::<f32>() / w as f32;
                let v = v + rng.gen::<f32>() / h as f32;
                let ray = camera.create_ray(u, v);
                col += color(&ray, &world);
            }

            col = col / samplings as f32;

            // sqrt はガンマ補正
            let r = (col.r() * 255.99) as u32;
            let g = (col.g() * 255.99) as u32;
            let b = (col.b() * 255.99) as u32;

            println!("{} {} {}", r, g, b);
        }
    }
}
