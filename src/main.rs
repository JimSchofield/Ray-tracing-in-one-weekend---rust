mod vec3;

fn main() {
    let image_width = 256;
    let image_height = 256;

    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in 0..image_height {
        eprint!("\rScanlines remaining: {} ", image_height - j);
        for i in 0..image_width {
            let r = i as f64 / (image_width as f64 - 1.0);
            let g = j as f64 / (image_height as f64 - 1.0);
            let b = 0.0;

            let ir = (255.999 * r) as i16;
            let ig = (255.999 * g) as i16;
            let ib = (255.999 * b) as i16;

            println!("{} {} {}", ir, ig, ib);
        }
    }
    eprint!("\rDone! ");
}
