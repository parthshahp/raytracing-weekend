pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    const EMPTY: Interval = Interval {
        min: f64::INFINITY,
        max: -f64::INFINITY,
    };

    const UNIVERSE: Interval = Interval {
        min: -f64::INFINITY,
        max: f64::INFINITY,
    };

    #[must_use]
    pub fn new(min: f64, max: f64) -> Self {
        Interval { min, max }
    }

    #[must_use]
    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    #[must_use]
    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    #[must_use]
    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            return self.min;
        } else if x > self.max {
            return self.max;
        }
        x
    }
}

impl Default for Interval {
    fn default() -> Self {
        Interval::EMPTY
    }
}
