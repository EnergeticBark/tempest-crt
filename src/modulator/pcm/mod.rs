mod format;
mod interpolation;
mod loader;

pub use format::*;
pub use interpolation::*;
pub use loader::PcmLoader;

use super::{DiscreteTime, Signal};

#[derive(Clone)]
pub struct Pcm<T: PcmFormat> {
    samples: Vec<T>,
    sample_rate: usize,
}

impl<T> Signal for Pcm<T>
where
    T: PcmFormat,
{
    fn sample(&self, t: &DiscreteTime) -> f32 {
        let sample_index =
            (t.numerator as f32 / t.denominator as f32 * self.sample_rate as f32) as usize;

        self.samples[sample_index].amplitude()
    }
}
