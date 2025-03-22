use crate::hittable::Hittable;

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn add(&mut self, obj: Box<dyn Hittable>) {
        self.objects.push(obj);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(
        &self,
        r: & crate::ray::Ray,
        ray_tmin: f64,
        ray_tmax: f64,
        rec: &mut crate::hittable::HitRecord,
    ) -> bool {
        let mut temp_rec = rec.clone();
        let mut hit_anything = false;
        let mut closest_so_far = ray_tmax;

        for obj in &self.objects {
            if obj.hit(r, ray_tmin, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }

        hit_anything
    }
}
