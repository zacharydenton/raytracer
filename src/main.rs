extern crate rand;
use rand::Rng;

extern crate raytracer;
use raytracer::*;

fn main() {
    let resolution = (800, 400);
    let num_samples = 100;
    let camera = Camera {
        origin: Vec3(0.0, 0.0, 0.0),
        lower_left: Vec3(-2.0, -1.0, -1.0),
        horizontal: Vec3(4.0, 0.0, 0.0),
        vertical: Vec3(0.0, 2.0, 0.0),
    };

    let sphere = Object {
        position: Vec3(0.0, 0.0, -1.0),
        scale: Vec3(0.66, 0.5, 0.5),
        geometry: Geometry::Sphere,
    };
    let ground = Object {
        position: Vec3(0.0, -100.5, -1.0),
        scale: Vec3(100.0, 100.0, 100.0),
        geometry: Geometry::Sphere,
    };

    let objects = [sphere, ground];
    let scene = Scene {
        objects: &objects,
    };

    println!("P3");
    println!("{} {}", resolution.0, resolution.1);
    println!("255");
    let mut rng = rand::thread_rng();
    for j in (0..resolution.1).rev() {
        for i in 0..resolution.0 {
            let mut color = Vec3(0.0, 0.0, 0.0);
            for _ in 0..num_samples {
                let u = ((i as f64) + rng.gen::<f64>()) / (resolution.0 as f64);
                let v = ((j as f64) + rng.gen::<f64>()) / (resolution.1 as f64);
                let ray = camera.ray(u, v);
                color = color + scene.color(&mut rng, &ray);
            }
            color = color / num_samples as f64;
            color = Vec3 {
                x: color.x.sqrt(),
                y: color.y.sqrt(),
                z: color.z.sqrt(),
            };
            color = color * 255.0;
            println!("{} {} {}", color.x.round(), color.y.round(), color.z.round());
        }
    }
}
