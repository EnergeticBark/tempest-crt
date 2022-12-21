use super::{DiscreteTime, Signal};

pub struct Sine {
    frequency: u32,
}

impl Sine {
    pub fn from_freq(frequency: u32) -> Self {
        Self { frequency }
    }
}

impl Signal for Sine {
    fn sample(&self, t: &DiscreteTime) -> f64 {
        (std::f64::consts::TAU * self.frequency as f64 * t.as_float()).sin()
    }
}

pub struct Square {
    frequency: u32,
}

impl Square {
    pub fn from_freq(frequency: u32) -> Self {
        Self { frequency }
    }
}

impl Signal for Square {
    fn sample(&self, t: &DiscreteTime) -> f64 {
        if (t.as_float() * self.frequency as f64) % 1.0 < 0.5 {
            return 1.0;
        }
        -1.0
    }
}
