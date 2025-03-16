use color::write_color;
use ray::Ray;
use vec3::{Vec3, dot, unit};

mod color;
mod ray;
mod vec3;

fn hit_sphere(center: Vec3, radius: f64, r: &Ray) -> f64 {
    let oc = center - r.origin;
    let a = r.direction.length_squared();
    let h = dot(r.direction, oc);
    let c = oc.length_squared() - radius * radius;
    let discriminant = h * h - a * c;

    if discriminant < 0.0 {
        return -1.0;
    }

    (h - discriminant.sqrt()) / a
}

fn ray_color(r: Ray) -> Vec3 {
    let t = hit_sphere(Vec3(0., 0., -1.), 0.5, &r);

    if t > 0.0 {
        let n = unit(r.at(t) - Vec3(0., 0., -1.));
        // println!("{:#?}", n);
        return 0.5 * Vec3(n.0 + 1.0, n.1 + 1.0, n.2 + 1.0);
        // return Vec3(1., 1., 1.);
    }

    let unit_direction = r.direction.unit();
    let a = 0.5 * (unit_direction.1 + 1.0);

    (1.0 - a) * Vec3(1., 1., 1.) + a * Vec3(0.5, 0.7, 1.0)
}

fn main() {
    // Image
    let aspect_ratio = 16. / 9.;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i64;

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

            let pixel_color = ray_color(r);
            write_color(pixel_color);
        }
    }
    eprint!("\rDone! ");
}
