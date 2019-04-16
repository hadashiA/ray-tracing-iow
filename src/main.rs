extern crate rand;

use std::f32;
use rand::Rng;
use ray_tracing_iow::*;

fn color<T: Hittable>(ray: &Ray, world: &T, count: u32) -> Vec3 {
    let hit = world.hit(ray, 0.001, f32::MAX);

    match hit {
        Some(hit) if count < 50 => {
            if let Some(x) = hit.material.sample(&ray, &hit) {
                 x.attenuation * color(&x.scattered, world, count + 1)
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

fn random_scene() -> impl Hittable {
    let mut world = HittableList::new();
    let mut rng = rand::thread_rng();

    // 地面
    world.add(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::new(Vec3::new(0.5, 0.5, 0.5))));

    let p1 = Vec3::new(4.0, 0.2, 0.0);

    for col in -11..11 {
        for row in -11..11 {
            let center = Vec3::new(
                col as f32 + 0.9 * rng.gen::<f32>(),
                0.2,
                row as f32 + 0.8 * rng.gen::<f32>());

            if (center - p1).length() > 0.9 {
                match rng.gen::<f32>() {
                    x if x < 0.8 => {
                        // diffuse
                        let material = Lambertian::new(
                            Vec3::new(
                                rng.gen::<f32>() * rng.gen::<f32>(),
                                rng.gen::<f32>() * rng.gen::<f32>(),
                                rng.gen::<f32>() * rng.gen::<f32>()));
                        world.add(Sphere::new(center, 0.2, material));
                    },
                    x if x < 0.95 => {
                        // metal
                        let material = Metal::new(
                            Vec3::new(
                                0.5 * (1.0 + rng.gen::<f32>()),
                                0.5 * (1.0 + rng.gen::<f32>()),
                                0.5 * (1.0 + rng.gen::<f32>())),
                            0.5 * rng.gen::<f32>());
                        world.add(Sphere::new(center, 0.2, material));
                    },
                    _ => {
                        // glass
                        let material = Dielectric::new(1.5);
                        world.add(Sphere::new(center, 0.2, material));
                    }
                }
            }
        }
    }

    world.add(Sphere::new(
       Vec3::new(0.0, 1.0, 0.0),
       1.0,
        Dielectric::new(1.5)
    ));

    world.add(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Lambertian::new(Vec3::new(0.4, 0.2, 0.1))
    ));

    world.add(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)));

    world
}

fn main() {
    let w = 400;
    let h = 200;
    let samplings = 100;

    let mut rng = rand::thread_rng();

    let look_from = Vec3::new(3.0, 3.0, 2.0);
    let look_at = Vec3::new(0.0, 0.0, -1.0);
    let dist_to_focus = (look_from - look_at).length();

    let camera = Camera::new(
        look_from,
    look_at,
    Vec3::new(0.0, 1.0, 0.0),
    20.0,
    w as f32 / h as f32,
    2.0,
    dist_to_focus);

    let world = random_scene();

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

            let r = (col.r().sqrt() * 255.99) as u32;
            let g = (col.g().sqrt() * 255.99) as u32;
            let b = (col.b().sqrt() * 255.99) as u32;

            println!("{} {} {}", r, g, b);
        }
    }
}
