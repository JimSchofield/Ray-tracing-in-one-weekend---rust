use std::f64::{INFINITY, NEG_INFINITY};

pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Interval { min, max }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        let &Interval { min, max } = self;
        min <= x && x <= max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        let &Interval { min, max } = self;
        min < x && x < max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        match x {
            i if i < self.min => self.min,
            i if i > self.max => self.max,
            _ => x,
        }
    }
}

pub const EMPTY: Interval = Interval {
    min: INFINITY,
    max: NEG_INFINITY,
};
pub const UNIVERSE: Interval = Interval {
    min: NEG_INFINITY,
    max: INFINITY,
};

#[cfg(test)]
mod interval_tests {
    use std::f64::consts::PI;

    use super::*;

    #[test]
    fn clamp() {
        let interval = Interval::new(5.0, 10.0);


        assert_eq!(interval.clamp(2.0), 5.0);
        assert_eq!(interval.clamp(100.0), 10.0);
        assert_eq!(interval.clamp(6.5), 6.5);

        let x = 5.0 + PI;
        assert_eq!(interval.clamp(x), x);
    }
}
