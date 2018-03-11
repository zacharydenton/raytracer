extern crate rand;
use rand::Rng;

use ray::Ray;
use vec3::Vec3;

#[derive(Debug, PartialEq)]
pub enum Material {
    Lambertian { albedo: Vec3 },
    Metal { albedo: Vec3, fuzziness: f64 },
    Dielectric { refraction_index: f64 },
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
                    direction: reflect(&ray.direction.unit(), normal)
                        + random_point_in_unit_sphere(rng) * fuzziness,
                };
                if reflection.direction.dot(normal) > 0.0 {
                    Some((reflection, albedo))
                } else {
                    None
                }
            }
            &Material::Dielectric { refraction_index } => {
                let attenuation = Vec3(1.0, 1.0, 1.0);
                let reflection = (
                    Ray {
                        origin: *point,
                        direction: reflect(&ray.direction, normal),
                    },
                    attenuation,
                );

                let outward_normal: Vec3;
                let ni_over_nt: f64;
                let reflection_probability: f64;
                let cosine: f64;
                if ray.direction.dot(normal) > 0.0 {
                    outward_normal = -*normal;
                    ni_over_nt = refraction_index;
                    cosine = refraction_index * ray.direction.dot(normal) / ray.direction.len();
                } else {
                    outward_normal = *normal;
                    ni_over_nt = 1.0 / refraction_index;
                    cosine = -ray.direction.dot(normal) / ray.direction.len();
                }

                if let Some(refraction) = refract(&ray.direction, &outward_normal, ni_over_nt) {
                    reflection_probability = schlick(cosine, refraction_index);
                    if rng.gen::<f64>() < reflection_probability {
                        Some(reflection)
                    } else {
                        Some((
                            Ray {
                                origin: *point,
                                direction: refraction,
                            },
                            attenuation,
                        ))
                    }
                } else {
                    Some(reflection)
                }
            }
        }
    }
}

fn reflect(vector: &Vec3, normal: &Vec3) -> Vec3 {
    *vector - *normal * (2.0 * vector.dot(&normal))
}

fn refract(vector: &Vec3, normal: &Vec3, ni_over_nt: f64) -> Option<Vec3> {
    let uv = vector.unit();
    let dt = uv.dot(normal);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        Some((uv - *normal * dt) * ni_over_nt - *normal * discriminant.sqrt())
    } else {
        None
    }
}

fn schlick(cosine: f64, refraction_index: f64) -> f64 {
    let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    r0 *= r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
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
