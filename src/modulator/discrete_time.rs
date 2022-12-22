// This struct keeps track of discrete time represented as a fraction.
// A value of 1 would be 1 second, a value of 1/2 would be half a second, etc.
pub struct DiscreteTime {
    pub numerator: u32,
    pub denominator: u32,
}

impl DiscreteTime {
    pub fn to_phase(&self, frequency: u32) -> f32 {
        let phase_per_step = frequency as f32 / self.denominator as f32;

        // We need to multiply phase_per_step by very large pixel indices, and f32 just isn't
        // precise enough. So we're going to represent the decimal part of the phase as a u32,
        // where 0.0 = 0 and 1.0 = u32::MAX.
        let base_2_decimal_part = (phase_per_step * u32::MAX as f32).round() as u32;

        // Wrapping multiplication maintains full 32-bit precision.
        // It also naturally reduces our phase modulo 1.0. :)
        base_2_decimal_part.wrapping_mul(self.numerator) as f32 / u32::MAX as f32
    }
}
