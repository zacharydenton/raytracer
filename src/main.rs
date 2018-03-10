mod vec3;
use vec3::Vec3;
mod ray;
use ray::Ray;

fn hit_sphere(center: Vec3, radius: f64, r: &Ray) -> bool {
    let oc = r.origin - center;
    let a = r.direction.dot(r.direction);
    let b = 2.0 * oc.dot(r.direction);
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
}

fn color(r: &Ray) -> Vec3 {
    let sphere_position = Vec3 {
        x: 0.0,
        y: 0.0,
        z: -1.0,
    };
    if hit_sphere(sphere_position, 0.5, r) {
        return Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
    }
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

    for j in (0..resolution.1).rev() {
        for i in 0..resolution.0 {
            let u = i as f64 / resolution.0 as f64;
            let v = j as f64 / resolution.1 as f64;
            let r = Ray {
                origin: origin,
                direction: lower_left + horizontal * u + vertical * v,
            };
            let color = color(&r) * 255.0;
            println!(
                "{} {} {}",
                color.x.round(),
                color.y.round(),
                color.z.round()
            );
        }
    }
}
