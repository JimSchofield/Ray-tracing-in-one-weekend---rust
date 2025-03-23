use crate::{interval::Interval, vec3::Vec3};

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0. {
        return linear_component.sqrt();
    }

    0.
}

pub fn write_color(pixel: Vec3) {
    let mut r = pixel.0;
    let mut g = pixel.1;
    let mut b = pixel.2;

    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);

    let intensity = Interval::new(0.0, 0.999);
    let ir = (intensity.clamp(r) * 256.) as i16;
    let ig = (intensity.clamp(g) * 256.) as i16;
    let ib = (intensity.clamp(b) * 256.) as i16;

    println!("{} {} {}", ir, ig, ib);
}
