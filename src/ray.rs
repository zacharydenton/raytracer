extern crate cgmath;
use cgmath::Vector3;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Ray {
    pub origin: Vector3<f64>,
    pub direction: Vector3<f64>,
}

impl Ray {
    pub fn point(&self, t: f64) -> Vector3<f64> {
        self.origin + self.direction * t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_origin_and_direction() {
        let origin = Vector3::new(0.0, 0.0, 2.0);
        let direction = Vector3::new(0.0, 1.0, -1.0);

        let r = Ray {
            origin: origin,
            direction: direction,
        };

        assert_eq!(r.origin, origin);
        assert_eq!(r.direction, direction);
    }

    #[test]
    fn has_point_at_position() {
        let origin = Vector3::new(0.0, 0.0, 0.0);
        let direction = Vector3::new(1.0, 1.0, -1.0);

        let r = Ray {
            origin: origin,
            direction: direction,
        };

        assert_eq!(r.point(0.0), r.origin);
        assert_eq!(r.point(1.0), Vector3::new(1.0, 1.0, -1.0,));
        assert_eq!(r.point(-15.0), Vector3::new(-15.0, -15.0, 15.0,));
        assert_eq!(r.point(0.5), Vector3::new(0.5, 0.5, -0.5,));
    }
}
