mod am;
mod phase;

pub use am::*;
pub use phase::*;

pub trait Modulator {
    fn sample(&self, phase: Phase) -> f64;
}
