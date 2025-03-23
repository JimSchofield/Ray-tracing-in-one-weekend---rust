use crate::{
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{Vec3, dot},
};

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval) -> (bool, Option<HitRecord>);
}

pub struct HitRecord<'a> {
    pub p: Vec3,
    pub normal: Vec3,
    pub mat: &'a Box<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

pub fn set_face_normal(r: &Ray, outward_normal: Vec3) -> (Vec3, bool) {
    let front_face = dot(r.direction, outward_normal) < 0.0;
    let normal = if front_face {
        outward_normal
    } else {
        -outward_normal
    };

    (normal, front_face)
}
