use rand::{Rng, rngs::ThreadRng};

pub fn degrees_to_radians(degrees: f64) -> f64 {
    return degrees * std::f64::consts::PI / 180.0;
}

const INFINITY: f64 = std::f64::INFINITY;
