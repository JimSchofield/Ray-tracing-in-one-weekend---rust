use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
};

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

unsafe impl Sync for HittableList {}

impl HittableList {
    pub fn add(&mut self, obj: Box<dyn Hittable>) {
        self.objects.push(obj);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &crate::ray::Ray, ray_t: Interval) -> (bool, Option<HitRecord>) {
        let mut rec: Option<HitRecord> = None;
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for obj in &self.objects {
            let (did_hit, hit_record) = obj.hit(r, Interval::new(ray_t.min, closest_so_far));
            if did_hit {
                if let Some(r) = hit_record {
                    hit_anything = true;
                    closest_so_far = r.t;
                    rec = Some(r)
                } else {
                    panic!("Hit record missing with a hit?")
                }
            }
        }

        (hit_anything, rec)
    }
}
