use crate::{
    color::write_color,
    global_stuff::degrees_to_radians,
    hittable::Hittable,
    hittable_list::HittableList,
    interval::Interval,
    random::Random,
    ray::Ray,
    vec3::{cross, random_in_unit_disk, unit, Vec3},
};

pub struct CameraConfig {
    pub vfov: f64,
    pub look_from: Vec3,
    pub look_at: Vec3,
    pub v_up: Vec3,
    pub aspect_ratio: f64,
    pub image_width: f64,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    pub defocus_angle: f64,
    pub focus_dist: f64,
}

#[allow(dead_code)]
pub struct Camera {
    pub vfov: f64,
    pub look_from: Vec3,
    pub look_at: Vec3,
    pub v_up: Vec3,
    pub aspect_ratio: f64,
    pub image_width: f64,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    pub defocus_angle: f64,
    pub focus_dist: f64,
    image_height: i64,
    camera_center: Vec3,
    pixel_samples_scale: f64,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn new(cfg: CameraConfig) -> Camera {
        let CameraConfig {
            vfov,
            look_from,
            look_at,
            v_up,
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth,
            defocus_angle,
            focus_dist,
        } = cfg;
        let image_height = (image_width / aspect_ratio) as i64;

        let pixel_samples_scale = 1.0 / samples_per_pixel as f64;

        let camera_center = look_from;

        // Camera
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.).tan();
        let viewport_height = 2. * h * focus_dist;
        let viewport_width = viewport_height * (image_width / image_height as f64);

        // u,v,w unit basis vectors for camera coordinate frame
        let w = unit(look_from - look_at);
        let u = unit(cross(v_up, w));
        let v = cross(w, u);

        // horizontal and vertial vectors
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        // Delta vectors
        let pixel_delta_u = viewport_u / image_width;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Upper left pixel
        let viewport_upper_left =
            camera_center - (focus_dist * w) - viewport_u / 2. - viewport_v / 2.;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        // Camera defocus disk basis vectors
        let defocus_radius = focus_dist * degrees_to_radians(defocus_angle / 2.).tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Camera {
            vfov,
            look_at,
            look_from,
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
            u,
            v,
            w,
            v_up,
            defocus_angle,
            focus_dist,
            defocus_disk_u,
            defocus_disk_v,
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
            defocus_angle,
            ..
        } = self;

        let offset = sample_square();
        let pixel_sample = pixel00_loc
            + ((i as f64 + offset.0) * pixel_delta_u)
            + ((j as f64 + offset.1) * pixel_delta_v);

        let ray_origin = if defocus_angle <= 0. {
            camera_center
        } else {
            self.defocus_disk_sample()
        };

        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn defocus_disk_sample(&self) -> Vec3 {
        let p = random_in_unit_disk();

        self.camera_center + (p.0 * self.defocus_disk_u) + (p.1 * self.defocus_disk_v)
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

    let (is_hit, hit_record) = world.hit(r, Interval::new(0.001, f64::INFINITY));
    if is_hit {
        if let Some(rec) = hit_record {
            let (_b, attenuation, scattered) = rec.mat.scatter(r, &rec);
            if _b {
                return attenuation * ray_color(&scattered, depth - 1, world);
            }

            return Vec3(0., 0., 0.);
        }
    }

    let unit_direction = r.direction.unit();
    let a = 0.5 * (unit_direction.1 + 1.0);

    (1.0 - a) * Vec3(1., 1., 1.) + a * Vec3(0.5, 0.7, 1.0)
}
