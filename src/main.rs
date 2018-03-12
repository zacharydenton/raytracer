extern crate rand;
use rand::Rng;

extern crate cgmath;
use cgmath::Vector3;
use cgmath::prelude::*;

extern crate raytracer;
use raytracer::*;

fn main() {
    let mut rng = rand::thread_rng();
    let resolution = (1200, 800);
    let num_samples = 100;
    let lookfrom = Vector3::new(13.0, 2.0, 3.0);
    let lookat = Vector3::new(0.0, 0.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let camera = Camera::new(
        lookfrom,
        lookat,
        Vector3::new(0.0, 1.0, 0.0),
        20.0,
        resolution.0 as f64 / resolution.1 as f64,
        aperture,
        dist_to_focus,
    );

    let mut objects = vec![];

    // Ground
    objects.push(Object {
        geometry: Geometry::Sphere {
            center: Vector3::new(0.0, -1000.0, 0.0),
            radius: 1000.0,
        },
        material: Material::Lambertian {
            albedo: Vector3::new(0.5, 0.5, 0.5),
        },
    });

    objects.push(Object {
        geometry: Geometry::Sphere {
            center: Vector3::new(0.0, 0.5, -1.0),
            radius: 0.5,
        },
        material: Material::Lambertian {
            albedo: Vector3::new(0.1, 0.2, 0.5),
        },
    });
    objects.push(Object {
        geometry: Geometry::Sphere {
            center: Vector3::new(1.0, 0.5, -1.0),
            radius: 0.5,
        },
        material: Material::Metal {
            albedo: Vector3::new(0.8, 0.6, 0.2),
            fuzziness: 0.1,
        },
    });
    objects.push(Object {
        geometry: Geometry::Sphere {
            center: Vector3::new(-1.0, 0.5, -1.0),
            radius: 0.5,
        },
        material: Material::Dielectric {
            refraction_index: 1.5,
        },
    });
    objects.push(Object {
        geometry: Geometry::Sphere {
            center: Vector3::new(-2.0, 0.5, -1.0),
            radius: 0.5,
        },
        material: Material::Lambertian {
            albedo: Vector3::new(0.8, 0.2, 0.1),
        },
    });
    objects.push(Object {
        geometry: Geometry::Sphere {
            center: Vector3::new(-3.0, 0.5, -1.0),
            radius: 0.5,
        },
        material: Material::Dielectric {
            refraction_index: 1.9,
        },
    });
    objects.push(Object {
        geometry: Geometry::Sphere {
            center: Vector3::new(-4.0, 0.5, -1.0),
            radius: 0.5,
        },
        material: Material::Metal {
            albedo: Vector3::new(0.1, 0.9, 0.1),
            fuzziness: 0.5,
        },
    });

    for i in -11..12 {
        for j in -11..12 {
            let choose_material = rng.gen::<f64>();
            let center = Vector3::new(
                (i as f64) + rng.gen::<f64>(),
                0.2,
                (j as f64) + 0.9 * rng.gen::<f64>(),
            );
            if (center - Vector3::new(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                if choose_material < 0.5 {
                    objects.push(Object {
                        geometry: Geometry::Sphere {
                            center,
                            radius: 0.2 + rng.gen::<f64>() * 0.1,
                        },
                        material: Material::Lambertian {
                            albedo: Vector3::new(
                                rng.gen::<f64>() * rng.gen::<f64>(),
                                rng.gen::<f64>() * rng.gen::<f64>(),
                                rng.gen::<f64>() * rng.gen::<f64>(),
                            ),
                        },
                    });
                } else if choose_material < 0.95 {
                    objects.push(Object {
                        geometry: Geometry::Sphere {
                            center,
                            radius: 0.2 + rng.gen::<f64>() * 0.2,
                        },
                        material: Material::Metal {
                            albedo: Vector3::new(
                                0.5 * (1.0 + rng.gen::<f64>()),
                                0.5 * (1.0 + rng.gen::<f64>()),
                                0.5 * (1.0 + rng.gen::<f64>()),
                            ),
                            fuzziness: 0.5 * (1.0 + rng.gen::<f64>()),
                        },
                    });
                } else {
                    objects.push(Object {
                        geometry: Geometry::Sphere {
                            center,
                            radius: 0.2 + rng.gen::<f64>() * 0.3,
                        },
                        material: Material::Dielectric {
                            refraction_index: 1.5,
                        },
                    });
                }
            }
        }
    }

    let scene = Scene { objects: &objects };

    println!("P3");
    println!("{} {}", resolution.0, resolution.1);
    println!("255");
    for j in (0..resolution.1).rev() {
        for i in 0..resolution.0 {
            let mut color = Vector3::new(0.0, 0.0, 0.0);
            for _ in 0..num_samples {
                let u = ((i as f64) + rng.gen::<f64>()) / (resolution.0 as f64);
                let v = ((j as f64) + rng.gen::<f64>()) / (resolution.1 as f64);
                let ray = camera.ray(&mut rng, u, v);
                color = color + scene.color(&mut rng, ray, 0);
            }
            color = color / num_samples as f64;
            color = Vector3::new(color.x.sqrt(), color.y.sqrt(), color.z.sqrt());
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
