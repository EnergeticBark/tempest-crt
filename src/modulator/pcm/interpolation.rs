use super::{Pcm, PcmFormat, Signal};

pub enum Interpolation {
    Nearest,
    Linear,
}

pub struct Nearest<T>(pub(super) T);

impl<T> Signal for Nearest<Pcm<T>>
where
    T: PcmFormat,
{
    fn sample(&self, total_index: u32) -> f32 {
        let sample_index = (total_index as f32 / self.0.pixels_per_sample).floor() as usize;

        self.0.samples[sample_index].amplitude()
    }
}

pub struct Linear<T>(pub(super) T);

impl<T> Signal for Linear<Pcm<T>>
where
    T: PcmFormat,
{
    fn sample(&self, total_index: u32) -> f32 {
        let floating_sample_index = total_index as f32 / self.0.pixels_per_sample;
        let sample_index = floating_sample_index.floor() as usize;

        let t = floating_sample_index.fract();
        let sample = self.0.samples[sample_index].amplitude();
        let next_sample = self.0.samples[sample_index + 1].amplitude();

        (1.0 - t) * sample + t * next_sample
    }
}
