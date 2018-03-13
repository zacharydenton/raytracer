extern crate cgmath;
use cgmath::Vector3;
use cgmath::prelude::*;

extern crate rand;
use rand::Rng;

use ray::Ray;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Material {
    Lambertian {
        albedo: Vector3<f64>,
    },
    Metal {
        albedo: Vector3<f64>,
        fuzziness: f64,
    },
    Dielectric {
        refraction_index: f64,
    },
}

impl Material {
    pub fn scatter<R: Rng>(
        &self,
        rng: &mut R,
        ray: Ray,
        point: Vector3<f64>,
        normal: Vector3<f64>,
    ) -> Option<(Ray, Vector3<f64>)> {
        match self {
            &Material::Lambertian { albedo } => {
                let target = point + normal + random_point_in_unit_sphere(rng);
                let reflection = Ray {
                    origin: point,
                    direction: target - point,
                };
                Some((reflection, albedo))
            }
            &Material::Metal { albedo, fuzziness } => {
                let reflection = Ray {
                    origin: point,
                    direction: reflect(ray.direction.normalize(), normal)
                        + random_point_in_unit_sphere(rng) * fuzziness,
                };
                if reflection.direction.dot(normal) > 0.0 {
                    Some((reflection, albedo))
                } else {
                    None
                }
            }
            &Material::Dielectric { refraction_index } => {
                let attenuation = Vector3::new(1.0, 1.0, 1.0);
                let reflection = (
                    Ray {
                        origin: point,
                        direction: reflect(ray.direction, normal),
                    },
                    attenuation,
                );

                let outward_normal: Vector3<f64>;
                let ni_over_nt: f64;
                let reflection_probability: f64;
                let cosine: f64;
                if ray.direction.dot(normal) > 0.0 {
                    outward_normal = -normal;
                    ni_over_nt = refraction_index;
                    cosine =
                        refraction_index * ray.direction.dot(normal) / ray.direction.magnitude();
                } else {
                    outward_normal = normal;
                    ni_over_nt = 1.0 / refraction_index;
                    cosine = -ray.direction.dot(normal) / ray.direction.magnitude();
                }

                if let Some(refraction) = refract(ray.direction, outward_normal, ni_over_nt) {
                    reflection_probability = schlick(cosine, refraction_index);
                    if rng.gen::<f64>() < reflection_probability {
                        Some(reflection)
                    } else {
                        Some((
                            Ray {
                                origin: point,
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

fn reflect(vector: Vector3<f64>, normal: Vector3<f64>) -> Vector3<f64> {
    vector - normal * (2.0 * vector.dot(normal))
}

fn refract(vector: Vector3<f64>, normal: Vector3<f64>, ni_over_nt: f64) -> Option<Vector3<f64>> {
    let uv = vector.normalize();
    let dt = uv.dot(normal);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        Some((uv - normal * dt) * ni_over_nt - normal * discriminant.sqrt())
    } else {
        None
    }
}

fn schlick(cosine: f64, refraction_index: f64) -> f64 {
    let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    r0 *= r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

fn random_point_in_unit_sphere<R: Rng>(rng: &mut R) -> Vector3<f64> {
    let mut point: Vector3<f64>;
    loop {
        point = Vector3::new(
            2.0 * rng.gen::<f64>() - 1.0,
            2.0 * rng.gen::<f64>() - 1.0,
            2.0 * rng.gen::<f64>() - 1.0,
        );
        if point.magnitude2() < 1.0 {
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
            assert!(p.magnitude2() < 1.0);
        }
    }
}
