use vec3::Vec3;
use ray::Ray;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Camera {
    pub origin: Vec3,
    pub lower_left: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left + self.horizontal * u + self.vertical * v - self.origin,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_rays() {
        let origin = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let lower_left = Vec3 {
            x: -2.0,
            y: -1.0,
            z: -1.0,
        };
        let horizontal = Vec3 {
            x: 4.0,
            y: 0.0,
            z: 0.0,
        };
        let vertical = Vec3 {
            x: 0.0,
            y: 2.0,
            z: 0.0,
        };
        let camera = Camera { origin, lower_left, horizontal, vertical };

        assert_eq!(camera.ray(0.0, 0.0), Ray {
            origin,
            direction: Vec3 {
                x: -horizontal.x / 2.0,
                y: -vertical.y / 2.0,
                z: lower_left.z,
            }
        });
        assert_eq!(camera.ray(1.0, 0.0), Ray {
            origin,
            direction: Vec3 {
                x: horizontal.x / 2.0,
                y: -vertical.y / 2.0,
                z: lower_left.z,
            }
        });
        assert_eq!(camera.ray(0.0, 1.0), Ray {
            origin,
            direction: Vec3 {
                x: -horizontal.x / 2.0,
                y: vertical.y / 2.0,
                z: lower_left.z,
            }
        });
        assert_eq!(camera.ray(0.5, 0.5), Ray {
            origin,
            direction: Vec3 {
                x: 0.0,
                y: 0.0,
                z: lower_left.z,
            }
        });
    }
}
