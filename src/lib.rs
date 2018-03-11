extern crate rand;
use rand::Rng;

mod vec3;
pub use vec3::Vec3;
mod ray;
pub use ray::Ray;
mod camera;
pub use camera::Camera;
mod geometry;
pub use geometry::Geometry;

#[derive(Debug, PartialEq)]
pub struct Object {
    pub geometry: Geometry,
}

#[derive(Debug)]
pub struct Scene<'a> {
    pub objects: &'a [Object],
}

struct Intersection<'a> {
    object: &'a Object,
    distance: f64,
}

impl<'a> Scene<'a> {
    pub fn color<R: Rng>(&self, rng: &mut R, ray: &Ray) -> Vec3 {
        match self.intersect(ray, 0.001, std::f64::MAX) {
            Some(Intersection {
                object,
                distance,
            }) => {
                let point = ray.point(distance);
                let normal = object.geometry.normal(point);
                let reflection_target = point + normal + random_point_in_unit_sphere(rng);
                let reflection = Ray {
                    origin: point,
                    direction: reflection_target - point,
                };
                self.color(rng, &reflection) * 0.5
            }
            None => {
                let direction = ray.direction.unit();
                let t = 0.5 * (direction.y + 1.0);
                let white = Vec3(1.0, 1.0, 1.0);
                let color = Vec3(0.5, 0.7, 1.0);
                white.lerp(&color, t)
            }
        }
    }

    fn intersect(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<Intersection> {
        self.objects
            .iter()
            .filter_map(|object| {
                if let Some(distance) = object.geometry.intersection(ray, tmin, tmax) {
                    Some(Intersection {
                        object,
                        distance: distance,
                    })
                } else {
                    None
                }
            })
            .min_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap())
    }
}

fn random_point_in_unit_sphere<R: Rng>(rng: &mut R) -> Vec3 {
    let mut point: Vec3;
    loop {
        point = Vec3 {
            x: 2.0 * rng.gen::<f64>() - 1.0,
            y: 2.0 * rng.gen::<f64>() - 1.0,
            z: 2.0 * rng.gen::<f64>() - 1.0,
        };
        if point.len_squared() < 1.0 {
            break;
        }
    }
    point
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_random_points_in_unit_sphere() {
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let p = random_point_in_unit_sphere(&mut rng);
            assert!(p.len_squared() < 1.0);
        }
    }
}
