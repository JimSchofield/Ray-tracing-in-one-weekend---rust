use crate::{
    hittable::Hittable,
    vec3::{Vec3, dot},
};

#[derive(Debug)]
pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        ray_tmin: f64,
        ray_tmax: f64,
        rec: &mut crate::hittable::HitRecord,
    ) -> bool {
        let oc = self.center - r.origin;
        let a = r.direction.length_squared();
        let h = dot(r.direction, oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return false;
        }

        let mut root = (h - discriminant.sqrt()) / a;

        if root <= ray_tmin || ray_tmax <= root {
            root = (h + discriminant.sqrt()) / a;
            if root <= ray_tmin || ray_tmax <= root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);

        true
    }
}
