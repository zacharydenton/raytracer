extern crate cgmath;
use cgmath::Vector3;
use cgmath::prelude::*;
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
    pub fn new(lookfrom: Vector3<f64>, lookat: Vector3<f64>, vup: Vector3<f64>, fov: f64, aspect: f64) -> Self {
        let theta = fov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let w = (lookfrom - lookat).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);
        Camera {
            origin: lookfrom,
            lower_left: lookfrom - u * half_width - v * half_height - w,
            horizontal: u * 2.0 * half_width,
            vertical: v * 2.0 * half_height,
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
