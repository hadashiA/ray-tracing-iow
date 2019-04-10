use std::f32;
use ray_tracing_iow::*;

fn color<T: Hittable>(ray: &Ray, world: &T) -> Vec3 {
    let hit = world.hit(ray, 0.0, f32::MAX);

    match hit {
        Some(hit) => {
            Vec3::new(hit.normal.x() + 1.0,hit.normal.y() + 1.0,hit.normal.z() + 1.0) * 0.5
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
        let v = y as f32 / h as f32;

        for x in 0..w {
            let u = x as f32 / w as f32;
            let ray = Ray {
                origin,
                direction: bottom_left + u * horizontal + v * vertical
            };

            let col = color(&ray, &world);
            let r = (col.r() * 255.99) as u32;
            let g = (col.g() * 255.99) as u32;
            let b = (col.b() * 255.99) as u32;

            println!("{} {} {}", r, g, b);
        }
    }
}
