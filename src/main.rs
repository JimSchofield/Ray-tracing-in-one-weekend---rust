use camera::{Camera, CameraConfig};
use hittable_list::HittableList;
use material::Material::{Dialectric, Lambertian, Metal};
use rand::random_range;
use random::Random;
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

    let ground_material = Lambertian {
        albedo: Vec3(0.5, 0.5, 0.5),
    };
    world.add(Sphere::new(Vec3(0., -1000., 0.), 1000., ground_material).into_box());

    for a in -15..15 {
        for b in -15..15 {
            let choose_mat: f64 = rand::random();

            let center = Vec3(
                a as f64 + 0.9 * rand::random::<f64>(),
                0.2,
                b as f64 + 0.9 * rand::random::<f64>(),
            );

            if (center - Vec3(4.0, 0.2, 0.)).length() > 0.9 {
                match choose_mat {
                    i if i < 0.8 => {
                        let albedo = Vec3::rnd() * Vec3::rnd();
                        let mat = Lambertian { albedo };
                        world.add(Sphere::new(center, 0.2, mat).into_box());
                    }
                    i if i < 0.95 => {
                        let albedo = Vec3::rnd_rng(0.5, 1.);
                        let fuzz = f64::rnd_rng(0., 0.5);
                        let mat = Metal { albedo, fuzz };
                        world.add(Sphere::new(center, 0.2, mat).into_box());
                    }
                    _ => {
                        let mat = Dialectric {
                            refraction_index: 1.5,
                        };
                        world.add(Sphere::new(center, 0.2, mat).into_box());
                    }
                }
            }
        }
    }

    let material1 = Dialectric {
        refraction_index: 1.5,
    };
    world.add(Sphere::new(Vec3(0., 1., 0.), 1., material1).into_box());

    let material2 = Lambertian {
        albedo: Vec3(0.4, 0.2, 0.1),
    };
    world.add(Sphere::new(Vec3(-4., 1., 0.), 1., material2).into_box());

    let material3 = Metal {
        albedo: Vec3(0.7, 0.6, 0.5),
        fuzz: 0.0,
    };
    world.add(Sphere::new(Vec3(4., 1., 0.), 1., material3).into_box());

    let mut cam = Camera::new(CameraConfig {
        vfov: 20.,
        look_from: Vec3(13., 2., 3.),
        look_at: Vec3(0., 0., 0.),
        v_up: Vec3(0., 1., 0.),
        aspect_ratio: 16. / 9.,
        image_width: 1200.0,
        samples_per_pixel: 500,
        max_depth: 50,
        defocus_angle: 0.6,
        focus_dist: 10.0,
    });

    cam.render(&world);
}
