mod am;
mod phase;

pub use phase::*;
pub use am::*;

pub trait Modulator {
    fn sample(&self, phase: Phase) -> f64;
}