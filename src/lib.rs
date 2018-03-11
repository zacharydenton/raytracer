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
mod material;
pub use material::Material;

#[derive(Debug, PartialEq)]
pub struct Object {
    pub geometry: Geometry,
    pub position: Vec3,
    pub scale: Vec3,
    pub material: Material,
}

#[derive(Debug)]
pub struct Scene<'a> {
    pub objects: &'a [Object],
}

struct Intersection<'a> {
    object: &'a Object,
    distance: f64,
    point: Vec3,
    normal: Vec3,
}

impl<'a> Scene<'a> {
    pub fn color<R: Rng>(&self, rng: &mut R, ray: &Ray, depth: u64) -> Vec3 {
        if depth >= 50 {
            return Vec3(0.0, 0.0, 0.0);
        }
        match self.intersect(ray, 0.001, std::f64::MAX) {
            Some(Intersection {
                object,
                distance: _,
                point,
                normal,
            }) => {
                if let Some((reflection, attenuation)) =
                    object.material.scatter(rng, ray, &point, &normal)
                {
                    attenuation * self.color(rng, &reflection, depth + 1)
                } else {
                    Vec3(0.0, 0.0, 0.0)
                }
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
                let transformed_origin = (ray.origin - object.position) / object.scale;
                let transformed_direction = ray.direction / object.scale;
                let transformed_ray = Ray {
                    origin: transformed_origin,
                    direction: transformed_direction,
                };
                if let Some(distance) = object.geometry.intersection(&transformed_ray, tmin, tmax) {
                    let transformed_point = transformed_ray.point(distance);
                    let transformed_normal = object.geometry.normal(&transformed_point);
                    let point = transformed_point * object.scale + object.position;
                    let mut normal = transformed_normal * object.scale;
                    normal.normalize();
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
