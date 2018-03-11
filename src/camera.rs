use std::f64::consts::PI;
use vec3::Vec3;
use ray::Ray;

#[derive(Debug, PartialEq)]
pub struct Camera {
    origin: Vec3,
    lower_left: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(fov: f64, aspect: f64) -> Self {
        let theta = fov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        Camera {
            origin: Vec3(0.0, 0.0, 0.0),
            lower_left: Vec3(-half_width, -half_height, -1.0),
            horizontal: Vec3(2.0 * half_width, 0.0, 0.0),
            vertical: Vec3(0.0, 2.0 * half_height, 0.0),
        }
    }

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
        let camera = Camera {
            origin,
            lower_left,
            horizontal,
            vertical,
        };

        assert_eq!(
            camera.ray(0.0, 0.0),
            Ray {
                origin,
                direction: Vec3 {
                    x: -horizontal.x / 2.0,
                    y: -vertical.y / 2.0,
                    z: lower_left.z,
                },
            }
        );
        assert_eq!(
            camera.ray(1.0, 0.0),
            Ray {
                origin,
                direction: Vec3 {
                    x: horizontal.x / 2.0,
                    y: -vertical.y / 2.0,
                    z: lower_left.z,
                },
            }
        );
        assert_eq!(
            camera.ray(0.0, 1.0),
            Ray {
                origin,
                direction: Vec3 {
                    x: -horizontal.x / 2.0,
                    y: vertical.y / 2.0,
                    z: lower_left.z,
                },
            }
        );
        assert_eq!(
            camera.ray(0.5, 0.5),
            Ray {
                origin,
                direction: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: lower_left.z,
                },
            }
        );
    }
}
