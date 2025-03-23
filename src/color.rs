use crate::{interval::Interval, vec3::Vec3};

pub fn write_color(pixel: Vec3) {
    let r = pixel.0;
    let g = pixel.1;
    let b = pixel.2;

    let intensity = Interval::new(0.0, 0.999);
    let ir = (intensity.clamp(r) * 256.) as i16;
    let ig = (intensity.clamp(g) * 256.) as i16;
    let ib = (intensity.clamp(b) * 256.) as i16;

    println!("{} {} {}", ir, ig, ib);
}
