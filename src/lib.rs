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
    pub position: Vector3<f64>,
    pub scale: Vector3<f64>,
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
                let transformed_origin =
                    (ray.origin - object.position).div_element_wise(object.scale);
                let transformed_direction = ray.direction.div_element_wise(object.scale);
                let transformed_ray = Ray {
                    origin: transformed_origin,
                    direction: transformed_direction,
                };
                if let Some(distance) = object.geometry.intersection(transformed_ray, tmin, tmax) {
                    let transformed_point = transformed_ray.point(distance);
                    let transformed_normal = object.geometry.normal(transformed_point);
                    let point = transformed_point.mul_element_wise(object.scale) + object.position;
                    let mut normal =
                        (transformed_normal.mul_element_wise(object.scale)).normalize();
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
