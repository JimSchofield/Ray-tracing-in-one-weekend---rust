use rand::random;

use crate::{
    hittable::HitRecord,
    ray::Ray,
    vec3::{Vec3, dot, random_unit_vector, reflect, refract, unit},
};

#[derive(Clone)]
pub enum Material {
    Lambertian { albedo: Vec3 },
    Metal { albedo: Vec3, fuzz: f64 },
    Dialectric { refraction_index: f64 },
}

// pub trait Material {
//     /// Scatters vector- returns (true, attenuation, scattered ray)
//     fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (bool, Vec3, Ray);
// }

impl Material {
    pub fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (bool, Vec3, Ray) {
        match self {
            Material::Lambertian { albedo } => {
                let mut scatter_direction = rec.normal + random_unit_vector();

                if scatter_direction.near_zero() {
                    scatter_direction = rec.normal;
                }

                let scattered = Ray::new(rec.p, scatter_direction);
                let attenuation = albedo;

                (true, *attenuation, scattered)
            }
            Material::Metal { albedo, fuzz } => {
                let mut reflected = reflect(r_in.direction, rec.normal);
                reflected = unit(reflected) + (*fuzz * random_unit_vector());
                let scattered = Ray::new(rec.p, reflected);
                let attenuation = albedo;

                let b = dot(scattered.direction, rec.normal) > 0.;

                (b, *attenuation, scattered)
            }
            Material::Dialectric { refraction_index } => {
                let attenuation = Vec3(1., 1., 1.);
                let ri = if rec.front_face {
                    1.0 / *refraction_index
                } else {
                    *refraction_index
                };

                let unit_direction = unit(r_in.direction);
                let cos_theta = dot(-unit_direction, rec.normal).min(1.);
                let sin_theta = (1. - cos_theta * cos_theta).sqrt();

                let cannot_refract = ri * sin_theta > 1.;
                let direction = if cannot_refract || reflectance(cos_theta, ri) > random::<f64>() {
                    reflect(unit_direction, rec.normal)
                } else {
                    refract(unit_direction, rec.normal, ri)
                };

                let scattered = Ray::new(rec.p, direction);

                (true, attenuation, scattered)
            }
        }
    }
}

fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
    let mut r0 = (1. - refraction_index) / (1. + refraction_index);
    r0 = r0 * r0;

    r0 + (1. - r0) * ((1. - cosine).powf(5.))
}
