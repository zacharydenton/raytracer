mod vec3;
use vec3::Vec3;

fn main() {
    let resolution = (200, 100);
    println!("P3");
    println!("{} {}", resolution.0, resolution.1);
    println!("255");
    for j in (0..resolution.1).rev() {
        for i in 0..resolution.0 {
            let point = Vec3 {
                x: i as f64 / resolution.0 as f64,
                y: j as f64 / resolution.1 as f64,
                z: 0.2,
            };
            let color = point * 255.0;
            println!(
                "{} {} {}",
                color.x.round(),
                color.y.round(),
                color.z.round()
            );
        }
    }
}
