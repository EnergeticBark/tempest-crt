mod am;
mod discrete_time;
mod fm;
mod wave;

pub use am::*;
pub use discrete_time::*;
pub use fm::*;
pub use wave::*;

pub trait Signal: Send + Sync {
    fn sample(&self, t: &DiscreteTime) -> f64;
}
