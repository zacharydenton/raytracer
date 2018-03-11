use ray::Ray;
use vec3::Vec3;

#[derive(Debug, PartialEq)]
pub enum Geometry {
    Sphere,
}

impl Geometry {
    pub fn intersection(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<f64> {
        match self {
            &Geometry::Sphere => {
                let a = ray.direction.dot(&ray.direction);
                let b = ray.origin.dot(&ray.direction);
                let c = ray.origin.dot(&ray.origin) - 1.0;
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

    pub fn normal(&self, point: &Vec3) -> Vec3 {
        match self {
            &Geometry::Sphere => point.unit()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sphere_intersection() {
        let sphere = Geometry::Sphere;
        let origin = Vec3 {
            x: 0.0,
            y: 0.0,
            z: -2.5,
        };
        let direction = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        };
        let ray = Ray { origin, direction };
        if let Some(intersection) = sphere.intersection(&ray, 0.0, 1e6) {
            assert_eq!(intersection, 1.5);
            let intersection_point = ray.point(intersection);
            assert_eq!(
                intersection_point,
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: -1.0,
                }
            );
            let normal = sphere.normal(&intersection_point);
            assert_eq!(
                normal,
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: -1.0,
                }
            );
        } else {
            assert!(false, "intersection not found");
        }
        let ray2 = Ray {
            origin,
            direction: -direction,
        };
        assert_eq!(sphere.intersection(&ray2, 0.0, 1e6), None);
        let ray3 = Ray {
            origin,
            direction: Vec3 {
                x: 0.23,
                y: 0.05,
                z: 1.0,
            }.unit(),
        };
        assert_ne!(sphere.intersection(&ray3, 0.0, 1e6), None);
    }
}
