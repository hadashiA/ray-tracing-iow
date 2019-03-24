use ray_tracing_iow::Vec3;

fn main() {
    let w = 200;
    let h = 100;

    println!("P3\n{} {}\n255", w, h);

    for y in (0..h).rev() {
        for x in 0..w {
            let c = Vec3::new(
                x as f32 / w as f32,
                y as f32 / h as f32,
                0.2f32);

            let r = (255.99 * c.r()) as i32;
            let g = (255.99 * c.g()) as i32;
            let b = (255.99 * c.b()) as i32;

            println!("{} {} {}", r, g, b);
        }
    }
}
