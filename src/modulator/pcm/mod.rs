mod format;
mod integrator;
mod interpolation;
mod loader;

pub use format::*;
pub use integrator::PreintegratedLoader;
pub use interpolation::*;
pub use loader::PcmLoader;

use super::Signal;

#[derive(Clone)]
pub struct Pcm<T: PcmFormat> {
    samples: Vec<T>,
    sample_rate: usize,
    pixels_per_sample: f32,
}
