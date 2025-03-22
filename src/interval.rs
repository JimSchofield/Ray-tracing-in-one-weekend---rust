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
}

pub const EMPTY: Interval = Interval {
    min: INFINITY,
    max: NEG_INFINITY,
};
pub const UNIVERSE: Interval = Interval {
    min: NEG_INFINITY,
    max: INFINITY,
};
