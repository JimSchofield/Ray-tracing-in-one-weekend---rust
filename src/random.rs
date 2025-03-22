use rand::Rng;

use crate::vec3::Vec3;

trait Random<T> {
    fn rnd() -> T;
    fn rnd_rng(min: f64, max: f64) -> T;
}

impl Random<f64> for f64 {
    fn rnd() -> f64 {
        let mut rng = rand::rng();

        rng.random::<f64>()
    }

    fn rnd_rng(min: f64, max: f64) -> f64 {
        min + (max - min) * f64::rnd()
    }
}

impl Random<Vec3> for Vec3 {
    fn rnd() -> Vec3 {
        Vec3(f64::rnd(), f64::rnd(), f64::rnd())
    }
    fn rnd_rng(min: f64, max: f64) -> Vec3 {
        Vec3(
            f64::rnd_rng(min, max),
            f64::rnd_rng(min, max),
            f64::rnd_rng(min, max),
        )
    }
}

#[cfg(test)]
mod vec_tests {
    use super::*;

    // Obviously not exhaustive, but it's nice to test to see that the impl works
    #[test]
    fn test_random_f64() {
        let a = f64::rnd();
        let b = f64::rnd();

        assert!(a < 1.0);
        assert!(a >= 0.0);
        assert!(b < 1.0);
        assert!(b >= 0.0);

        assert_ne!(a, b);
    }

    #[test]
    fn test_random_rng_f64() {
        let a = f64::rnd_rng(1.0, 2.0);
        let b = f64::rnd_rng(2.0, 3.0);

        assert!(a < 2.0);
        assert!(a >= 1.0);
        assert!(b < 3.0);
        assert!(b >= 2.0);

        assert_ne!(a, b);
    }

    #[test]
    fn test_random_vec3() {
        let v = Vec3::rnd();
        let w = Vec3::rnd();

        assert_ne!(v.0, w.0);
        assert_ne!(v.1, w.1);
        assert_ne!(v.2, w.2);
    }

    #[test]
    fn test_random_rng_vec3() {
        let v = Vec3::rnd_rng(5.0, 10.0);

        assert!(v.0 < 10.0);
        assert!(v.1 < 10.0);
        assert!(v.2 < 10.0);
        assert!(v.0 >= 5.0);
        assert!(v.1 >= 5.0);
        assert!(v.2 >= 5.0);
    }
}
