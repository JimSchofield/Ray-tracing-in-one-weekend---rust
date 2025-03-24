use camera::{Camera, CameraConfig};
use hittable_list::HittableList;
use material::{Lambertian, Metal};
use sphere::Sphere;
use vec3::Vec3;

mod camera;
mod color;
mod global_stuff;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod random;
mod ray;
mod sphere;
mod vec3;

fn main() {
    // World!
    let mut world: HittableList = Default::default();

    let material_ground = Box::new(Lambertian {
        albedo: Vec3(0.8, 0.8, 0.),
    });
    let material_center = Box::new(Lambertian {
        albedo: Vec3(0.1, 0.2, 0.5),
    });
    let material_left = Box::new(Metal {
        albedo: Vec3(0.8, 0.8, 0.8),
        fuzz: 0.3,
    });
    let material_right = Box::new(Metal {
        albedo: Vec3(0.8, 0.6, 0.2),
        fuzz: 1.0,
    });

    world.add(Box::new(Sphere::new(
        Vec3(0., -100.5, -1.),
        100.,
        material_ground,
    )));
    world.add(Box::new(Sphere::new(
        Vec3(0., 0., -1.2),
        0.5,
        material_center,
    )));
    world.add(Box::new(Sphere::new(
        Vec3(-1., 0., -1.),
        0.5,
        material_left,
    )));
    world.add(Box::new(Sphere::new(
        Vec3(1., 0., -1.),
        0.5,
        material_right,
    )));

    let mut cam = Camera::new(CameraConfig {
        aspect_ratio: 16. / 9.,
        image_width: 400.0,
        samples_per_pixel: 50,
        max_depth: 50,
    });

    cam.render(&world);
}
