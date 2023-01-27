mod am;
mod discrete_time;
mod fm;
mod pcm;
mod phase;
mod wave;

pub use am::AmplitudeModulator;
pub use discrete_time::DiscreteTime;
pub use fm::FrequencyModulator;
pub use pcm::*;
pub use wave::*;

use phase::Phase;

pub trait Signal: Send + Sync {
    fn sample(&self, t: &DiscreteTime) -> f32;
}

pub trait IntSignal: Send + Sync {
    fn sample(&self, t: &DiscreteTime) -> Phase;
}
