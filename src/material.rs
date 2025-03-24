use rand::random;

use crate::{
    hittable::HitRecord,
    ray::Ray,
    vec3::{Vec3, dot, random_unit_vector, reflect, refract, unit},
};

pub trait Material {
    /// Scatters vector- returns (true, attenuation, scattered ray)
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (bool, Vec3, Ray);
}

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> (bool, Vec3, Ray) {
        let mut scatter_direction = rec.normal + random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray::new(rec.p, scatter_direction);
        let attenuation = self.albedo;

        (true, attenuation, scattered)
    }
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (bool, Vec3, Ray) {
        let mut reflected = reflect(r_in.direction, rec.normal);
        reflected = unit(reflected) + (self.fuzz * random_unit_vector());
        let scattered = Ray::new(rec.p, reflected);
        let attenuation = self.albedo;

        let b = dot(scattered.direction, rec.normal) > 0.;

        (b, attenuation, scattered)
    }
}

pub struct Dialectric {
    pub refraction_index: f64,
}

impl Material for Dialectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (bool, Vec3, Ray) {
        let attenuation = Vec3(1., 1., 1.);
        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
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

fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
    let mut r0 = (1. - refraction_index) / (1. + refraction_index);
    r0 = r0 * r0;

    r0 + (1. - r0) * ((1. - cosine).powf(5.))
}
