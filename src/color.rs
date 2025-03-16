use crate::vec3::Vec3;

pub fn write_color(pixel: Vec3) {
    let r = pixel.0;
    let g = pixel.1;
    let b = pixel.2;

    let ir = (255.999 * r) as i16;
    let ig = (255.999 * g) as i16;
    let ib = (255.999 * b) as i16;

    println!("{} {} {}", ir, ig, ib);
}
