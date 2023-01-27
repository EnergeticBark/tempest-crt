use super::Phase;

// This struct keeps track of discrete time represented as a fraction.
// A value of 1 would be 1 second, a value of 1/2 would be half a second, etc.
pub struct DiscreteTime {
    pub numerator: u32,
    pub denominator: u32,
}

impl DiscreteTime {
    pub(super) fn to_phase(&self, frequency: u32) -> Phase {
        let phase_per_step = Phase::from(frequency as f32 / self.denominator as f32);
        phase_per_step * self.numerator
    }
}
