extern crate rand;
use rand::Rng;

extern crate cgmath;
use cgmath::Vector3;

extern crate raytracer;
use raytracer::*;

fn main() {
    let resolution = (800, 400);
    let num_samples = 100;
    let camera = Camera::new(100.0, 2.0);

    let sphere = Object {
        position: Vector3::new(0.0, 0.0, -1.0),
        scale: Vector3::new(0.5, 0.5, 0.5),
        geometry: Geometry::Sphere,
        material: Material::Lambertian {
            albedo: Vector3::new(0.8, 0.3, 0.3),
        },
    };
    let ground = Object {
        position: Vector3::new(0.0, -100.5, -1.0),
        scale: Vector3::new(100.0, 100.0, 100.0),
        geometry: Geometry::Sphere,
        material: Material::Lambertian {
            albedo: Vector3::new(0.8, 0.8, 0.0),
        },
    };
    let marble1 = Object {
        position: Vector3::new(1.0, 0.0, -1.0),
        scale: Vector3::new(0.5, 0.5, 0.5),
        geometry: Geometry::Sphere,
        material: Material::Metal {
            albedo: Vector3::new(0.8, 0.6, 0.2),
            fuzziness: 0.3,
        },
    };
    let marble2 = Object {
        position: Vector3::new(-1.0, 0.0, -1.0),
        scale: Vector3::new(0.5, 0.5, 0.5),
        geometry: Geometry::Sphere,
        material: Material::Metal {
            albedo: Vector3::new(0.8, 0.8, 0.8),
            fuzziness: 0.1,
        },
    };
    let marble3 = Object {
        position: Vector3::new(0.5, -0.32, -0.9),
        scale: Vector3::new(0.15, 0.15, 0.15),
        geometry: Geometry::Sphere,
        material: Material::Dielectric {
            refraction_index: 2.4,
        },
    };

    let objects = [sphere, ground, marble1, marble2, marble3];
    let scene = Scene { objects: &objects };

    println!("P3");
    println!("{} {}", resolution.0, resolution.1);
    println!("255");
    let mut rng = rand::thread_rng();
    for j in (0..resolution.1).rev() {
        for i in 0..resolution.0 {
            let mut color = Vector3::new(0.0, 0.0, 0.0);
            for _ in 0..num_samples {
                let u = ((i as f64) + rng.gen::<f64>()) / (resolution.0 as f64);
                let v = ((j as f64) + rng.gen::<f64>()) / (resolution.1 as f64);
                let ray = camera.ray(u, v);
                color = color + scene.color(&mut rng, &ray, 0);
            }
            color = color / num_samples as f64;
            color = Vector3::new(
                color.x.sqrt(),
                color.y.sqrt(),
                color.z.sqrt(),
            );
            color = color * 255.0;
            println!(
                "{} {} {}",
                color.x.round(),
                color.y.round(),
                color.z.round()
            );
        }
    }
}
