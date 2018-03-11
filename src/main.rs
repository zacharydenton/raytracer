extern crate rand;
use rand::Rng;

extern crate raytracer;
use raytracer::*;

fn color(r: &Ray, world: &Hitable) -> Vec3 {
    match world.hit(r, 0.0, std::f64::MAX) {
        Some(hit) => {
            return (hit.normal + 1.0) * 0.5;
        }
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
    let resolution = (800, 400);
    let origin = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
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
    let camera = Camera {
        origin,
        lower_left,
        horizontal,
        vertical,
    };
    let num_samples = 100;

    let center = Vec3 {
        x: 0.0,
        y: 0.0,
        z: -1.0,
    };
    let radius = 0.5;
    let sphere = Sphere { center, radius };
    let ground = Sphere {
        center: Vec3 {
            x: 0.0,
            y: -100.5,
            z: -1.0,
        },
        radius: 100.0,
    };

    let world = World {
        objects: vec![&sphere, &ground],
    };

    println!("P3");
    println!("{} {}", resolution.0, resolution.1);
    println!("255");
    let mut rng = rand::thread_rng();
    for j in (0..resolution.1).rev() {
        for i in 0..resolution.0 {
            let mut c = Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
            for _ in 0..num_samples {
                let u = ((i as f64) + rng.gen::<f64>()) / (resolution.0 as f64);
                let v = ((j as f64) + rng.gen::<f64>()) / (resolution.1 as f64);
                let r = camera.ray(u, v);
                c = c + color(&r, &world);
            }
            c = c / num_samples as f64;
            c = c * 255.0;
            println!("{} {} {}", c.x.round(), c.y.round(), c.z.round());
        }
    }
}
