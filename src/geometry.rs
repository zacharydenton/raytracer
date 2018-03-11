use ray::Ray;
use vec3::Vec3;

#[derive(Debug, PartialEq)]
pub enum Geometry {
    Sphere { center: Vec3, radius: f64 },
}

impl Geometry {
    pub fn intersection(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<f64> {
        match self {
            &Geometry::Sphere { center, radius } => {
                let oc = ray.origin - center;
                let a = ray.direction.len_squared();
                let b = oc.dot(&ray.direction);
                let c = oc.len_squared() - radius * radius;
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

    pub fn normal(&self, point: Vec3) -> Vec3 {
        match self {
            &Geometry::Sphere { center, radius }=> (point - center) / radius,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sphere_intersection() {
        let sphere = Geometry::Sphere { center: Vec3(0.0, 0.0, 0.0), radius: 1.0 };
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
            let normal = sphere.normal(intersection_point);
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
