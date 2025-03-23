use camera::Camera;
use hittable_list::HittableList;
use sphere::Sphere;
use vec3::Vec3;

mod camera;
mod color;
mod global_stuff;
mod hittable;
mod hittable_list;
mod interval;
mod ray;
mod random;
mod sphere;
mod vec3;

fn main() {
    // World!
    let mut world: HittableList = Default::default();

    world.add(Box::new(Sphere::new(Vec3(0., 0., -1.), 0.5)));
    world.add(Box::new(Sphere::new(Vec3(0., -100.5, -1.), 100.)));

    let mut cam = Camera  {
        image_width: 400.,
        aspect_ratio: 16. / 9.,
        samples_per_pixel: 100,
        ..Default::default()
    };

    cam.render(&world);
}
