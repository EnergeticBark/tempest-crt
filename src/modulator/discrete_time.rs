// This struct keeps track of discrete time represented as a fraction.
// A value of 1 would be 1 second, a value of 1/2 would be half a second, etc.
pub struct DiscreteTime {
    pub numerator: u32,
    pub denominator: u32,
}

impl DiscreteTime {
    pub fn as_float(&self) -> f64 {
        self.numerator as f64 / self.denominator as f64
    }
}
