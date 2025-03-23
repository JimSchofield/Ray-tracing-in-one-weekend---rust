use crate::{
    color::write_color,
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    interval::Interval,
    random::Random,
    ray::Ray,
    vec3::{Vec3, random_on_hemisphere},
};

pub struct CameraConfig {
    pub aspect_ratio: f64,
    pub image_width: f64,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
}

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: f64,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    image_height: i64,
    camera_center: Vec3,
    pixel_samples_scale: f64,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

// impl Default for Camera {
//     fn default() -> Self {
//         Camera {
//             aspect_ratio: 16.0 / 9.0,
//             image_width: 400.,
//             samples_per_pixel: 10,
//             config: None,
//         }
//     }
// }

impl Camera {
    pub fn new(cfg: CameraConfig) -> Camera {
        let CameraConfig {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth,
        } = cfg;
        let image_height = (image_width / aspect_ratio) as i64;

        let pixel_samples_scale = 1.0 / samples_per_pixel as f64;

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

        Camera {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth,
            image_height,
            pixel_samples_scale,
            camera_center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn render(&mut self, world: &HittableList) {
        let &mut Camera {
            image_width,
            samples_per_pixel,
            image_height,
            pixel_samples_scale,
            max_depth,
            ..
        } = self;

        print!("P3\n{} {}\n255\n", image_width, image_height);

        for j in 0..(image_height) {
            eprint!("\rScanlines remaining: {} ", image_height - j);
            for i in 0..(image_width as i64) {
                let mut pixel_color = Vec3(0., 0., 0.);
                for _sample in 0..samples_per_pixel {
                    let r = self.get_ray(i, j);

                    pixel_color += ray_color(&r, max_depth, world);
                }

                let samples_scale_vec = Vec3::splat(pixel_samples_scale);
                write_color(samples_scale_vec * pixel_color);
            }
        }
        eprint!("\rDone! ");
    }

    fn get_ray(&self, i: i64, j: i64) -> Ray {
        let &Camera {
            camera_center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            ..
        } = self;

        let offset = sample_square();
        let pixel_center = pixel00_loc
            + ((i as f64 + offset.0) * pixel_delta_u)
            + ((j as f64 + offset.1) * pixel_delta_v);
        let ray_direction = pixel_center - camera_center;

        Ray::new(camera_center, ray_direction)
    }
}

fn sample_square() -> Vec3 {
    let mut v = <Vec3>::rnd_rng(-0.5, 0.5);
    v.2 = 0.0;
    v
}

fn ray_color(r: &Ray, depth: i32, world: &dyn Hittable) -> Vec3 {
    if depth <= 0 {
        return Vec3::splat(0.);
    }

    let mut rec: HitRecord = Default::default();
    if world.hit(r, Interval::new(0.001, f64::INFINITY), &mut rec) {
        let direction = random_on_hemisphere(rec.normal);
        return 0.5 * ray_color(&Ray::new(rec.p, direction), depth - 1, world);
    }

    let unit_direction = r.direction.unit();
    let a = 0.5 * (unit_direction.1 + 1.0);

    (1.0 - a) * Vec3(1., 1., 1.) + a * Vec3(0.5, 0.7, 1.0)
}
