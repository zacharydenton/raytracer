use vec3::Vec3;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn point(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_origin_and_direction() {
        let origin = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 2.0,
        };
        let direction = Vec3 {
            x: 0.0,
            y: 1.0,
            z: -1.0,
        };

        let r = Ray {
            origin: origin,
            direction: direction,
        };

        assert_eq!(r.origin, origin);
        assert_eq!(r.direction, direction);
    }

    #[test]
    fn has_point_at_position() {
        let origin = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let direction = Vec3 {
            x: 1.0,
            y: 1.0,
            z: -1.0,
        };

        let r = Ray {
            origin: origin,
            direction: direction,
        };

        assert_eq!(r.point(0.0), r.origin);
        assert_eq!(
            r.point(1.0),
            Vec3 {
                x: 1.0,
                y: 1.0,
                z: -1.0,
            }
        );
        assert_eq!(
            r.point(-15.0),
            Vec3 {
                x: -15.0,
                y: -15.0,
                z: 15.0,
            }
        );
        assert_eq!(
            r.point(0.5),
            Vec3 {
                x: 0.5,
                y: 0.5,
                z: -0.5,
            }
        );
    }
}
