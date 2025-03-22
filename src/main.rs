use color::write_color;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use ray::Ray;
use sphere::Sphere;
use vec3::Vec3;

mod color;
mod global_stuff;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;

fn ray_color(r: &Ray, world: &dyn Hittable) -> Vec3 {
    let mut rec: HitRecord = Default::default();
    if world.hit(r, 0., f64::INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Vec3(1.0, 1.0, 1.0));
    }

    let unit_direction = r.direction.unit();
    let a = 0.5 * (unit_direction.1 + 1.0);

    (1.0 - a) * Vec3(1., 1., 1.) + a * Vec3(0.5, 0.7, 1.0)
}

fn main() {
    // Image
    let aspect_ratio = 16. / 9.;
    let image_width = 1200;
    let image_height = (image_width as f64 / aspect_ratio) as i64;

    // World!
    let mut world: HittableList = Default::default();

    world.add(Box::new(Sphere::new(Vec3(0., 0., -1.), 0.5)));
    world.add(Box::new(Sphere::new(Vec3(0., -100.5, -1.), 100.)));

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Vec3(0., 0., 0.);

    // horizontal and vertial vectors
    let viewport_u = Vec3(viewport_width, 0., 0.);
    let viewport_v = Vec3(0., -viewport_height, 0.);

    // Delta vectors
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // Upper left pixel
    let viewport_upper_left =
        camera_center - Vec3(0., 0., focal_length) - viewport_u / 2. - viewport_v / 2.;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in 0..image_height {
        eprint!("\rScanlines remaining: {} ", image_height - j);
        for i in 0..image_width {
            let pixel_center = pixel00_loc + (i * pixel_delta_u) + (j * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&r, &world);
            write_color(pixel_color);
        }
    }
    eprint!("\rDone! ");
}
