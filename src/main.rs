use ray_tracing_iow::{Vec3, Ray};

fn color(ray: &Ray) -> Vec3 {
    let dir = ray.direction.normalized();
    let t = 0.5 * (dir.y() + 1.0);

    let white = Vec3::new(1.0, 1.0, 1.0);
    let blue = Vec3::new(0.5, 0.7, 1.0);
    Vec3::lerp(white, blue, t)
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
