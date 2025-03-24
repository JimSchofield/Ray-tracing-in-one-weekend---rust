use crate::{
    hittable::HitRecord,
    ray::Ray,
    vec3::{dot, random_unit_vector, reflect, refract, unit, Vec3},
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
        let refracted = refract(unit_direction, rec.normal, ri);

        let scattered = Ray::new(rec.p, refracted);

        (true, attenuation, scattered)
    }
}
