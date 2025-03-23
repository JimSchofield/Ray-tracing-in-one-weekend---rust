use camera::{Camera, CameraConfig};
use hittable_list::HittableList;
use sphere::Sphere;
use vec3::Vec3;

mod camera;
mod color;
mod global_stuff;
mod hittable;
mod hittable_list;
mod interval;
mod random;
mod ray;
mod sphere;
mod vec3;

fn main() {
    // World!
    let mut world: HittableList = Default::default();

    world.add(Box::new(Sphere::new(Vec3(0., 0., -1.), 0.5)));
    world.add(Box::new(Sphere::new(Vec3(0., -100.5, -1.), 100.)));

    let mut cam = Camera::new(CameraConfig {
        aspect_ratio: 16. / 9.,
        image_width: 400.0,
        samples_per_pixel: 100,
        max_depth: 50,
    });

    cam.render(&world);
}
