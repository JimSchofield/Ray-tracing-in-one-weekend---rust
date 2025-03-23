use crate::{
    hittable::{HitRecord, Hittable, set_face_normal},
    interval::Interval,
    material::Material,
    vec3::{Vec3, dot},
};

pub struct Sphere {
    center: Vec3,
    radius: f64,
    mat: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, mat: Box<dyn Material>) -> Self {
        Sphere {
            center,
            radius,
            mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &crate::ray::Ray, ray_t: Interval) -> (bool, Option<HitRecord>) {
        let oc = self.center - r.origin;
        let a = r.direction.length_squared();
        let h = dot(r.direction, oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return (false, None);
        }

        let mut root = (h - discriminant.sqrt()) / a;

        if !ray_t.surrounds(root) {
            root = (h + discriminant.sqrt()) / a;
            if !ray_t.surrounds(root) {
                return (false, None);
            }
        }

        let t = root;
        let p = r.at(t);
        let outward_normal = (p - self.center) / self.radius;
        let (normal, front_face) = set_face_normal(r, outward_normal);

        (
            true,
            Some(HitRecord {
                t,
                p,
                normal,
                front_face,
                mat: &self.mat,
            }),
        )
    }
}
