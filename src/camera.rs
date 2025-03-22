use crate::{
    color::write_color, hittable::{HitRecord, Hittable}, hittable_list::HittableList, interval::Interval, ray::Ray, vec3::Vec3
};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: f64,
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            aspect_ratio: 16.0 / 9.0,
            image_width: 400.,
        }
    }
}

impl Camera {
    pub fn render(&self, world: &HittableList) {
        // Image
        let &Camera { aspect_ratio, image_width } = self;
        let image_height = (image_width / aspect_ratio) as i64;

        // Camera
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width / image_height as f64);
        let camera_center = Vec3(0., 0., 0.);

        // horizontal and vertial vectors
        let viewport_u = Vec3(viewport_width, 0., 0.);
        let viewport_v = Vec3(0., -viewport_height, 0.);

        // Delta vectors
        let pixel_delta_u = viewport_u / image_width;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Upper left pixel
        let viewport_upper_left =
            camera_center - Vec3(0., 0., focal_length) - viewport_u / 2. - viewport_v / 2.;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        print!("P3\n{} {}\n255\n", image_width, image_height);

        for j in 0..image_height {
            eprint!("\rScanlines remaining: {} ", image_height - j);
            for i in 0..(image_width as i64) {
                let pixel_center = pixel00_loc + (i * pixel_delta_u) + (j * pixel_delta_v);
                let ray_direction = pixel_center - camera_center;
                let r = Ray::new(camera_center, ray_direction);

                let pixel_color = ray_color(&r, world);

                write_color(pixel_color);
            }
        }
        eprint!("\rDone! ");
    }
}

fn ray_color(r: &Ray, world: &dyn Hittable) -> Vec3 {
    let mut rec: HitRecord = Default::default();
    if world.hit(r, Interval::new(0., f64::INFINITY), &mut rec) {
        return 0.5 * (rec.normal + Vec3(1.0, 1.0, 1.0));
    }

    let unit_direction = r.direction.unit();
    let a = 0.5 * (unit_direction.1 + 1.0);

    (1.0 - a) * Vec3(1., 1., 1.) + a * Vec3(0.5, 0.7, 1.0)
}
