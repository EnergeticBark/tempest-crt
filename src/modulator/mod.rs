mod am;
mod discrete_time;
mod pcm;
mod wave;

pub use am::*;
pub use discrete_time::*;
pub use pcm::*;
pub use wave::*;

pub trait Signal: Send + Sync {
    fn sample(&self, t: &DiscreteTime) -> f32;
}
