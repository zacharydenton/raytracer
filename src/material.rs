extern crate rand;
use rand::Rng;

use ray::Ray;
use vec3::Vec3;

#[derive(Debug, PartialEq)]
pub enum Material {
    Lambertian { albedo: Vec3 },
    Metal { albedo: Vec3, fuzziness: f64 },
}

impl Material {
    pub fn scatter<R: Rng>(
        &self,
        rng: &mut R,
        ray: &Ray,
        point: &Vec3,
        normal: &Vec3,
    ) -> Option<(Ray, Vec3)> {
        match self {
            &Material::Lambertian { albedo } => {
                let target = *point + *normal + random_point_in_unit_sphere(rng);
                let reflection = Ray {
                    origin: *point,
                    direction: target - *point,
                };
                Some((reflection, albedo))
            }
            &Material::Metal { albedo, fuzziness } => {
                let reflection = Ray {
                    origin: *point,
                    direction: reflect(&ray.direction.unit(), normal) + random_point_in_unit_sphere(rng) * fuzziness,
                };
                if reflection.direction.dot(normal) > 0.0 {
                    Some((reflection, albedo))
                } else {
                    None
                }
            }
        }
    }
}

fn reflect(vector: &Vec3, normal: &Vec3) -> Vec3 {
    *vector - *normal * (2.0 * vector.dot(&normal))
}

fn random_point_in_unit_sphere<R: Rng>(rng: &mut R) -> Vec3 {
    let mut point: Vec3;
    loop {
        point = Vec3 {
            x: 2.0 * rng.gen::<f64>() - 1.0,
            y: 2.0 * rng.gen::<f64>() - 1.0,
            z: 2.0 * rng.gen::<f64>() - 1.0,
        };
        if point.len_squared() < 1.0 {
            break;
        }
    }
    point
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_random_points_in_unit_sphere() {
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let p = random_point_in_unit_sphere(&mut rng);
            assert!(p.len_squared() < 1.0);
        }
    }
}
