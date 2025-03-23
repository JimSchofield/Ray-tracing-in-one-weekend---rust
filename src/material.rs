use crate::{
    hittable::HitRecord,
    ray::Ray,
    vec3::{random_unit_vector, reflect, Vec3},
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
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (bool, Vec3, Ray) {
        let reflected = reflect(r_in.direction, rec.normal);
        let scattered = Ray::new(rec.p, reflected);
        let attenuation = self.albedo;

        (true, attenuation, scattered)
    }
}
