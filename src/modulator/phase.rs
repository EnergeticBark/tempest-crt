/* This struct keeps track of position along a wave.
I'm storing the numerator and denominator as two integers rather than a float.
This is so modulators can have simplified expressions and much smaller lookup tables (if enabled), while maintaining complete accuracy. */
pub struct Phase {
    pub numerator: u32,
    pub denominator: u32,
}

impl Phase {
    pub fn as_float(&self) -> f64 {
        self.numerator as f64 / self.denominator as f64
    }
}