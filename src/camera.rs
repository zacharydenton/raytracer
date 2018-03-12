extern crate cgmath;
use cgmath::Vector3;
use cgmath::prelude::*;

extern crate rand;
use rand::Rng;

use std::f64::consts::PI;
use ray::Ray;

#[derive(Debug, PartialEq)]
pub struct Camera {
    origin: Vector3<f64>,
    lower_left: Vector3<f64>,
    horizontal: Vector3<f64>,
    vertical: Vector3<f64>,
    u: Vector3<f64>,
    v: Vector3<f64>,
    w: Vector3<f64>,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        lookfrom: Vector3<f64>,
        lookat: Vector3<f64>,
        vup: Vector3<f64>,
        fov: f64,
        aspect: f64,
        aperture: f64,
        focus_distance: f64,
    ) -> Self {
        let lens_radius = aperture / 2.0;
        let theta = fov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let w = (lookfrom - lookat).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);
        Camera {
            origin: lookfrom,
            lower_left: lookfrom - u * focus_distance * half_width
                - v * focus_distance * half_height - focus_distance * w,
            horizontal: u * 2.0 * half_width * focus_distance,
            vertical: v * 2.0 * half_height * focus_distance,
            u,
            v,
            w,
            lens_radius,
        }
    }

    pub fn ray<R: Rng>(&self, rng: &mut R, u: f64, v: f64) -> Ray {
        let direction = self.lens_radius * random_point_in_unit_disk(rng);
        let offset = self.u * direction.x + self.v * direction.y;
        Ray {
            origin: self.origin + offset,
            direction: self.lower_left + self.horizontal * u + self.vertical * v - self.origin
                - offset,
        }
    }
}

fn random_point_in_unit_disk<R: Rng>(rng: &mut R) -> Vector3<f64> {
    let mut point: Vector3<f64>;
    loop {
        point = Vector3::new(rng.gen::<f64>() - 1.0, rng.gen::<f64>() - 1.0, 0.0) * 2.0;
        if point.magnitude2() < 1.0 {
            break;
        }
    }
    point
}

#[cfg(test)]
mod tests {
    use super::*;
}
