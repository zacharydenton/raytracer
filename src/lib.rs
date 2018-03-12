extern crate cgmath;
use cgmath::Vector3;
use cgmath::prelude::*;
extern crate rand;
use rand::Rng;

mod ray;
pub use ray::Ray;
mod camera;
pub use camera::Camera;
mod geometry;
pub use geometry::Geometry;
mod material;
pub use material::Material;

#[derive(Debug, PartialEq)]
pub struct Object {
    pub geometry: Geometry,
    pub material: Material,
}

#[derive(Debug)]
pub struct Scene<'a> {
    pub objects: &'a [Object],
}

struct Intersection<'a> {
    object: &'a Object,
    distance: f64,
    point: Vector3<f64>,
    normal: Vector3<f64>,
}

impl<'a> Scene<'a> {
    pub fn color<R: Rng>(&self, rng: &mut R, ray: Ray, depth: u64) -> Vector3<f64> {
        if depth >= 50 {
            return Vector3::new(0.0, 0.0, 0.0);
        }
        match self.intersect(ray, 0.001, std::f64::MAX) {
            Some(Intersection {
                object,
                distance: _,
                point,
                normal,
            }) => {
                if let Some((reflection, attenuation)) =
                    object.material.scatter(rng, ray, point, normal)
                {
                    attenuation.mul_element_wise(self.color(rng, reflection, depth + 1))
                } else {
                    Vector3::new(0.0, 0.0, 0.0)
                }
            }
            None => {
                let direction = ray.direction.normalize();
                let t = 0.5 * (direction.y + 1.0);
                let white = Vector3::new(1.0, 1.0, 1.0);
                let color = Vector3::new(0.5, 0.7, 1.0);
                white.lerp(color, t)
            }
        }
    }

    fn intersect(&self, ray: Ray, tmin: f64, tmax: f64) -> Option<Intersection> {
        self.objects
            .iter()
            .filter_map(|object| {
                if let Some(distance) = object.geometry.intersection(ray, tmin, tmax) {
                    let point = ray.point(distance);
                    let normal = object.geometry.normal(point);
                    Some(Intersection {
                        object,
                        distance,
                        point,
                        normal,
                    })
                } else {
                    None
                }
            })
            .min_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap())
    }
}
