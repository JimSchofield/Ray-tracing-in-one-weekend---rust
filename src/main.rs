use camera::{Camera, CameraConfig};
use hittable_list::HittableList;
use material::Material::{Dialectric, Lambertian, Metal};
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

    let material_ground = Lambertian {
        albedo: Vec3(0.8, 0.8, 0.),
    };
    let material_center = Lambertian {
        albedo: Vec3(0.1, 0.2, 0.5),
    };
    let material_left = Dialectric {
        refraction_index: 1.5,
    };
    let material_bubble = Dialectric {
        refraction_index: 1. / 1.5,
    };
    let material_right = Metal {
        albedo: Vec3(0.8, 0.6, 0.2),
        fuzz: 1.0,
    };

    world.add(Sphere::new(
        Vec3(0., -100.5, -1.),
        100.,
        material_ground,
    ).into_box());
    world.add(Sphere::new(
        Vec3(0., 0., -1.2),
        0.5,
        material_center,
    ).into_box());
    world.add(Sphere::new(
        Vec3(-1., 0., -1.),
        0.5,
        material_left,
    ).into_box());
    world.add(Sphere::new(
        Vec3(-1., 0., -1.),
        0.4,
        material_bubble,
    ).into_box());
    world.add(Sphere::new(
        Vec3(1., 0., -1.),
        0.5,
        material_right,
    ).into_box());

    let mut cam = Camera::new(CameraConfig {
        vfov: 60.,
        look_from: Vec3(-2., 2., 1.),
        look_at: Vec3(0., 0., -1.),
        v_up: Vec3(0., 1., 0.),
        aspect_ratio: 16. / 9.,
        image_width: 1400.0,
        samples_per_pixel: 100,
        max_depth: 50,
        defocus_angle: 1.0,
        focus_dist: 3.4,
    });

    cam.render(&world);
}
