extern crate cgmath;
use cgmath::Vector3;
use cgmath::prelude::*;
use ray::Ray;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Geometry {
    Sphere { center: Vector3<f64>, radius: f64 },
}

impl Geometry {
    pub fn intersection(&self, ray: Ray, tmin: f64, tmax: f64) -> Option<f64> {
        match self {
            &Geometry::Sphere { center, radius } => {
                let oc = ray.origin - center;
                let a = ray.direction.dot(ray.direction);
                let b = oc.dot(ray.direction);
                let c = oc.dot(oc) - radius * radius;
                let discriminant = b * b - a * c;
                if discriminant > 0.0 {
                    let dsqrt = discriminant.sqrt();
                    let mut distance = (-b - dsqrt) / a;
                    if distance > tmin && distance < tmax {
                        return Some(distance);
                    }
                    distance = (-b + dsqrt) / a;
                    if distance > tmin && distance < tmax {
                        return Some(distance);
                    }
                }
                None
            }
        }
    }

    pub fn normal(&self, point: Vector3<f64>) -> Vector3<f64> {
        match self {
            &Geometry::Sphere { center, radius } => (point - center) / radius,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sphere_intersection() {
        let sphere = Geometry::Sphere {
            center: Vector3::new(0.0, 0.0, 0.0),
            radius: 1.0,
        };
        let origin = Vector3::new(0.0, 0.0, -2.5);
        let direction = Vector3::new(0.0, 0.0, 1.0);
        let ray = Ray { origin, direction };
        if let Some(intersection) = sphere.intersection(ray, 0.0, 1e6) {
            assert_eq!(intersection, 1.5);
            let intersection_point = ray.point(intersection);
            assert_eq!(intersection_point, Vector3::new(0.0, 0.0, -1.0));
            let normal = sphere.normal(intersection_point);
            assert_eq!(normal, Vector3::new(0.0, 0.0, -1.0));
        } else {
            assert!(false, "intersection not found");
        }
        let ray2 = Ray {
            origin,
            direction: -direction,
        };
        assert_eq!(sphere.intersection(ray2, 0.0, 1e6), None);
        let ray3 = Ray {
            origin,
            direction: Vector3::new(0.23, 0.05, 1.0).normalize(),
        };
        assert_ne!(sphere.intersection(ray3, 0.0, 1e6), None);
    }
}
