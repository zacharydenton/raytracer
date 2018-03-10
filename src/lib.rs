mod vec3;
pub use vec3::Vec3;
mod ray;
pub use ray::Ray;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Hit {
    pub t: f64,
    pub point: Vec3,
    pub normal: Vec3,
}

pub trait Hitable {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<Hit>;
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<Hit> {
        let oc = r.origin - self.center;
        let a = r.direction.dot(r.direction);
        let b = oc.dot(r.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let dsqrt = discriminant.sqrt();
            let mut solution = (-b - dsqrt) / a;
            if solution > tmin && solution < tmax {
                return Some(Hit {
                    t: solution,
                    point: r.point(solution),
                    normal: (r.point(solution) - self.center) / self.radius,
                });
            }
            solution = (-b + dsqrt) / a;
            if solution > tmin && solution < tmax {
                return Some(Hit {
                    t: solution,
                    point: r.point(solution),
                    normal: (r.point(solution) - self.center) / self.radius,
                });
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spheres_can_be_hit() {
        let center = Vec3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        };
        let radius = 0.5;
        let sphere = Sphere { center, radius };
        let origin = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let direction = Vec3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        };
        let ray = Ray { origin, direction };
        assert_eq!(
            sphere.hit(&ray, 0.0, 5.0),
            Some(Hit {
                t: 0.5,
                point: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: -0.5,
                },
                normal: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                },
            })
        );
        let ray2 = Ray {
            origin,
            direction: -direction,
        };
        assert_eq!(sphere.hit(&ray2, 0.0, 5.0), None);
    }
}
