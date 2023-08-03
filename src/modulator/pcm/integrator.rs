use super::{Interpolation, /*Linear,*/ Nearest, PcmFormat};
use crate::modulator::phase::Phase;
use crate::modulator::{IntSignal, Pcm, PcmLoader};
use std::error::Error;
use std::num::Wrapping;

struct IntegratedSample<T: PcmFormat> {
    // The PCM sample itself.
    sample: T,
    // The cumulative phase shift of all previous phase shifts.
    // Phase is represented by a u32, with 0 being 0° and u32::MAX + 1 being a full 360° turn.
    cum_phase: Phase,
}

struct IntegratedPcm<T: PcmFormat> {
    samples: Vec<IntegratedSample<T>>,
    sample_rate: usize,
    pixels_per_sample: f32,
}

trait Integrable {
    type Interpolation;

    fn integrate(self) -> Self::Interpolation;
}

impl<T> Integrable for Nearest<Pcm<T>>
where
    T: PcmFormat,
{
    type Interpolation = Nearest<IntegratedPcm<T>>;

    fn integrate(self) -> Self::Interpolation {
        let samples: Vec<IntegratedSample<T>> = self
            .0
            .samples
            .iter()
            .scan(Phase(Wrapping(0)), |phase, &sample| {
                let integrated = IntegratedSample {
                    sample,
                    cum_phase: *phase,
                };
                let phase_per_sample = Phase::from(sample.amplitude() / self.0.sample_rate as f32);
                *phase += phase_per_sample;
                Some(integrated)
            })
            .collect();

        Nearest(IntegratedPcm {
            samples,
            sample_rate: self.0.sample_rate,
            pixels_per_sample: self.0.pixels_per_sample,
        })
    }
}

impl<T> IntSignal for Nearest<IntegratedPcm<T>>
where
    T: PcmFormat,
{
    fn sample(&self, total_index: u32) -> Phase {
        let index = (total_index as f32 / self.0.pixels_per_sample).floor() as usize;
        let int_sample = &self.0.samples[index];

        let current_sample_phase = Phase::from(
            int_sample.sample.amplitude() / self.0.sample_rate as f32
                * (total_index as f32 / self.0.pixels_per_sample).fract(),
        );

        int_sample.cum_phase + current_sample_phase
    }
}

pub struct PreintegratedLoader<T: PcmFormat> {
    internal_loader: PcmLoader<T>,
}

impl<T> PreintegratedLoader<T>
where
    T: PcmFormat + 'static,
{
    pub fn new(internal_loader: PcmLoader<T>) -> Self {
        Self { internal_loader }
    }

    pub fn next_frame(&mut self) -> Result<(), Box<dyn Error>> {
        self.internal_loader.next_frame()?;

        Ok(())
    }

    pub fn samples(&mut self) -> Box<dyn IntSignal> {
        let pcm = self.internal_loader.pcm();

        //match &self.internal_loader.interpolation {
        //  Interpolation::Nearest => {
        Box::new(Nearest(pcm).integrate())
        //}
        //Interpolation::Linear => Box::new(Linear(pcm)),
        //}
    }
}
