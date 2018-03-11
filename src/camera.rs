extern crate cgmath;
use cgmath::Vector3;
use std::f64::consts::PI;
use ray::Ray;

#[derive(Debug, PartialEq)]
pub struct Camera {
    origin: Vector3<f64>,
    lower_left: Vector3<f64>,
    horizontal: Vector3<f64>,
    vertical: Vector3<f64>,
}

impl Camera {
    pub fn new(fov: f64, aspect: f64) -> Self {
        let theta = fov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        Camera {
            origin: Vector3::new(0.0, 0.0, 0.0),
            lower_left: Vector3::new(-half_width, -half_height, -1.0),
            horizontal: Vector3::new(2.0 * half_width, 0.0, 0.0),
            vertical: Vector3::new(0.0, 2.0 * half_height, 0.0),
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
        let origin = Vector3::new(0.0, 0.0, 0.0);
        let lower_left = Vector3::new(-2.0, -1.0, -1.0);
        let horizontal = Vector3::new(4.0, 0.0, 0.0);
        let vertical = Vector3::new(0.0, 2.0, 0.0);
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
                direction: Vector3::new(-horizontal.x / 2.0, -vertical.y / 2.0, lower_left.z),
            }
        );
        assert_eq!(
            camera.ray(1.0, 0.0),
            Ray {
                origin,
                direction: Vector3::new(horizontal.x / 2.0, -vertical.y / 2.0, lower_left.z),
            }
        );
        assert_eq!(
            camera.ray(0.0, 1.0),
            Ray {
                origin,
                direction: Vector3::new(-horizontal.x / 2.0, vertical.y / 2.0, lower_left.z),
            }
        );
        assert_eq!(
            camera.ray(0.5, 0.5),
            Ray {
                origin,
                direction: Vector3::new(0.0, 0.0, lower_left.z),
            }
        );
    }
}
