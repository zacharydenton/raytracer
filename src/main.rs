extern crate raytracer;
use raytracer::*;

fn color(r: &Ray, world: &Hitable) -> Vec3 {
    match world.hit(r, 0.0, std::f64::MAX) {
        Some(hit) => {
            return (hit.normal + 1.0) * 0.5;
        },
        None => {
            let mut direction = r.direction;
            direction.normalize();
            let t = 0.5 * (direction.y + 1.0);
            let white = Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            };
            let color = Vec3 {
                x: 0.5,
                y: 0.7,
                z: 1.0,
            };
            white.lerp(color, t)
        }
    }
}

fn main() {
    let resolution = (200, 100);
    println!("P3");
    println!("{} {}", resolution.0, resolution.1);
    println!("255");

    let lower_left = Vec3 {
        x: -2.0,
        y: -1.0,
        z: -1.0,
    };
    let horizontal = Vec3 {
        x: 4.0,
        y: 0.0,
        z: 0.0,
    };
    let vertical = Vec3 {
        x: 0.0,
        y: 2.0,
        z: 0.0,
    };
    let origin = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    let center = Vec3 {
        x: 0.0,
        y: 0.0,
        z: -1.0,
    };
    let radius = 0.5;
    let sphere = Sphere { center, radius };

    for j in (0..resolution.1).rev() {
        for i in 0..resolution.0 {
            let u = i as f64 / resolution.0 as f64;
            let v = j as f64 / resolution.1 as f64;
            let r = Ray {
                origin: origin,
                direction: lower_left + horizontal * u + vertical * v,
            };
            let color = color(&r, &sphere) * 255.0;
            println!(
                "{} {} {}",
                color.x.round(),
                color.y.round(),
                color.z.round()
            );
        }
    }
}
