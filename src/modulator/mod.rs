mod am;
mod fm;
mod pcm;
mod phase;
mod wave;

pub use am::AmplitudeModulator;
pub use fm::FrequencyModulator;
pub use pcm::*;
pub use wave::*;

use phase::Phase;

pub trait Signal: Send + Sync {
    fn sample(&self, total_index: u32) -> f32;
}

pub trait IntSignal: Send + Sync {
    fn sample(&self, total_index: u32) -> Phase;
}
