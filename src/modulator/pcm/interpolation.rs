use super::{DiscreteTime, Pcm, PcmFormat, Signal};

pub enum Interpolation {
    Nearest,
    Linear,
}

pub struct LerpPcm<T: PcmFormat>(pub(super) Pcm<T>);

impl<T> Signal for LerpPcm<T>
where
    T: PcmFormat,
{
    fn sample(&self, t: &DiscreteTime) -> f32 {
        let floating_sample_index =
            t.numerator as f32 / t.denominator as f32 * self.0.sample_rate as f32;
        let sample_index = floating_sample_index as usize;

        let t = floating_sample_index.fract();
        let sample = self.0.samples[sample_index].amplitude();
        let next_sample = self.0.samples[sample_index + 1].amplitude();

        (1.0 - t) * sample + t * next_sample
    }
}
