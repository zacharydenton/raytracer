fn main() {
    let resolution = (200, 100);
    println!("P3");
    println!("{} {}", resolution.0, resolution.1);
    println!("255");
    for j in (0..resolution.1).rev() {
        for i in 0..resolution.0 {
            let r = i as f32 / resolution.0 as f32;
            let g = j as f32 / resolution.1 as f32;
            let b = 0.2;
            let ir = (255.99 * r) as u32;
            let ig = (255.99 * g) as u32;
            let ib = (255.99 * b) as u32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
