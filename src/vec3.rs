use std::ops;

use crate::random::Random;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Vec3(pub f64, pub f64, pub f64);

pub fn dot(u: Vec3, v: Vec3) -> f64 {
    u.0 * v.0 + u.1 * v.1 + u.2 * v.2
}

pub fn unit(v: Vec3) -> Vec3 {
    v / v.length()
}

pub fn random_unit_vector() -> Vec3 {
    loop {
        let p = Vec3::rnd_rng(-1., 1.);
        let lensq = p.length_squared();
        if 1e-160 < lensq && lensq <= 1. {
            return p / lensq.sqrt();
        }
    }
}

pub fn random_on_hemisphere(normal: Vec3) -> Vec3 {
    let on_unit_sphere = random_unit_vector();
    if dot(on_unit_sphere, normal) > 0.0 {
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let mut p = Vec3::rnd_rng(-1., 1.);
        p.2 = 0.;
        if p.length_squared() < 1. {
            return p;
        }
    }
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2. * dot(v, n) * n
}

pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = dot(-uv, n).min(1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = (1.0 - r_out_perp.length_squared()).abs().sqrt() * -n;

    r_out_perp + r_out_parallel
}

pub fn cross(u: Vec3, w: Vec3) -> Vec3 {
    Vec3(
        u.1 * w.2 - u.2 * w.1,
        u.2 * w.0 - u.0 * w.2,
        u.0 * w.1 - u.1 * w.0,
    )
}

#[allow(dead_code)]
impl Vec3 {
    pub fn splat(x: f64) -> Vec3 {
        Vec3(x, x, x)
    }

    pub fn scale(self, k: f64) -> Vec3 {
        Vec3(self.0 * k, self.1 * k, self.2 * k)
    }

    pub fn dot(self, other: Vec3) -> f64 {
        dot(self, other)
    }

    pub fn cross(self, other: Vec3) -> Vec3 {
        cross(self, other)
    }

    pub fn length_squared(self) -> f64 {
        dot(self, self)
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn unit(self) -> Vec3 {
        unit(self)
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.0 < s && self.1 < s && self.2 < s
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Vec3 {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Vec3(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

// Multiply combinations

impl ops::Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Self::Output {
        v.scale(self)
    }
}
impl ops::Mul<Vec3> for i32 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Self::Output {
        v.scale(self as f64)
    }
}

impl ops::Mul<Vec3> for i64 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Self::Output {
        v.scale(self as f64)
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, k: f64) -> Self::Output {
        self.scale(k)
    }
}

// Divide

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, d: f64) -> Self::Output {
        Vec3(self.0 / d, self.1 / d, self.2 / d)
    }
}

// divide combinations

#[cfg(test)]
mod vec_tests {
    use super::*;

    #[test]
    fn vector_scale() {
        let v1 = Vec3(1., 2., 3.);
        let v2 = v1.scale(10.0);
        assert_eq!(v2, Vec3(10., 20., 30.));
    }

    #[test]
    fn vector_add() {
        let v1 = Vec3(1., 2., 3.);
        let v2 = Vec3(10., 20., 30.);
        assert_eq!(v1 + v2, Vec3(11., 22., 33.));
    }

    #[test]
    fn vector_sub() {
        let v1 = Vec3(11., 12., 13.);
        let v2 = Vec3(10., 10., 10.);
        assert_eq!(v1 - v2, Vec3(1., 2., 3.));
    }

    #[test]
    fn vector_add_assign() {
        let mut v1 = Vec3(11., 12., 13.);
        v1 += Vec3(0., 10., 20.);
        assert_eq!(v1, Vec3(11., 22., 33.));
    }

    #[test]
    fn vector_sub_assign() {
        let mut v1 = Vec3(11., 12., 13.);
        v1 -= Vec3(10., 10., 10.);
        assert_eq!(v1, Vec3(1., 2., 3.));
    }

    #[test]
    fn vector_negate() {
        let v1 = Vec3(11., 12., 13.);
        assert_eq!(-v1, Vec3(-11., -12., -13.));
    }

    #[test]
    fn scalar_multiply() {
        let v = Vec3(1., 2., 3.);
        assert_eq!(3. * v, Vec3(3., 6., 9.));
        assert_eq!(v * 3., Vec3(3., 6., 9.));
    }

    #[test]
    fn vector_dot() {
        let v = Vec3(1., 2., 3.);
        let u = Vec3(5., 6., 7.);
        assert_eq!(v.dot(u), 38.);
    }

    #[test]
    fn vector_length_squared() {
        let v = Vec3(1., 4., 8.);
        assert_eq!(v.length_squared(), 81.);
    }

    #[test]
    fn vector_length() {
        let v = Vec3(1., 4., 8.);
        assert_eq!(v.length(), 9.);
        assert_eq!(Vec3(3., 4., 0.).length(), 5.);
    }

    #[test]
    fn vector_divide() {
        let v = Vec3(2., 4., 8.);
        assert_eq!(v / 2., Vec3(1., 2., 4.));
    }

    #[test]
    fn vector_unit() {
        let v = unit(Vec3(2., 3., 6.));
        assert_eq!(v, Vec3(2. / 7., 3. / 7., 6. / 7.));
        assert_eq!(Vec3(2., 3., 6.).unit(), Vec3(2. / 7., 3. / 7., 6. / 7.));
    }

    #[test]
    fn vector_cross() {
        let u = Vec3(2., 3., 0.);
        let v = Vec3(-2., 5., 1.);
        assert_eq!(cross(u, v), Vec3(3., -2., 16.));
        let w = Vec3(1., 0., 0.);
        let x = Vec3(0., 1., 0.);
        assert_eq!(cross(w, x), Vec3(0., 0., 1.));
    }
}
