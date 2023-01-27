use super::{Interpolation, Linear, Nearest, PcmFormat};
use crate::modulator::phase::Phase;
use crate::modulator::{DiscreteTime, IntSignal, Pcm, PcmLoader};
use std::error::Error;
use std::num::Wrapping;

const BUFFER_PADDING: usize = 340;

struct IntegratedSample<T: PcmFormat> {
    // The PCM sample itself.
    sample: T,
    // The exact total pixel index we expect this sample to start playing.
    start_time: u32,
    // The cumulative phase shift of all previous phase shifts.
    // Phase is represented by a u32, with 0 being 0° and u32::MAX being a full 360° turn.
    cum_phase: Phase,
}

struct IntegratedPcm<T: PcmFormat> {
    samples: Vec<IntegratedSample<T>>,
    sample_rate: usize,
}

trait Integrable {
    type Interpolation;

    fn integrate(self, pixel_rate: u32, sample_start_times: &[u32]) -> Self::Interpolation;
}

impl<T> Integrable for Nearest<Pcm<T>>
where
    T: PcmFormat,
{
    type Interpolation = Nearest<IntegratedPcm<T>>;

    fn integrate(self, pixel_rate: u32, sample_start_times: &[u32]) -> Self::Interpolation {
        let samples: Vec<IntegratedSample<T>> = self
            .0
            .samples
            .iter()
            .enumerate()
            .scan(Phase(Wrapping(0)), |phase, (index, &sample)| {
                let start_time = sample_start_times[index];

                let integrated = IntegratedSample {
                    sample,
                    start_time,
                    cum_phase: *phase,
                };

                let next_start_time = sample_start_times[index + 1];
                let pixels_in_sample = next_start_time - start_time;

                let phase_per_pixel = Phase::from(sample.amplitude() / pixel_rate as f32);
                let phase_per_sample = phase_per_pixel * pixels_in_sample;

                *phase = *phase * phase_per_sample;

                Some(integrated)
            })
            .collect();

        Nearest(IntegratedPcm {
            samples,
            sample_rate: self.0.sample_rate,
        })
    }
}

impl<T> IntSignal for Nearest<IntegratedPcm<T>>
where
    T: PcmFormat,
{
    fn sample(&self, t: &DiscreteTime) -> Phase {
        let index =
            ((t.numerator as f32 / t.denominator as f32) * self.0.sample_rate as f32) as usize;
        let int_sample = &self.0.samples[index];

        let phase_per_pixel = Phase::from(int_sample.sample.amplitude() / t.denominator as f32);

        int_sample.cum_phase + phase_per_pixel * (t.numerator - int_sample.start_time)
    }
}

pub struct PreintegratedLoader<T: PcmFormat> {
    internal_loader: PcmLoader<T>,
    pixel_rate: u32,
    /*
    Keeps track of the total pixel indices that mark the start of each sample. When the frequency
    modulator is running, each pixel needs to know how long the current sample has been playing for.
    The modulator could be running on multiple threads at once, so each calculation must not be
    dependent on the results of previous pixels. Basically, each pixel needs to be rendered knowing
    nothing other than it's own index and the information stored in IntegratedPcm.

    Imagine you're a pixel, in practice the process looks like this:
    I'm pixel 14,594 out of 64,995,840. 14,594 * 30,000 (sample rate) / 64,995,840 = 6, so I'm
    playing the 6th sample of PCM audio. IntegratedPcm says that sample 6 started at index 13,000.
    That means sample 6 has been playing for 1,594 pixels. So, my phase is equal to
    cum_phase + 1,594 * pcm_sample_phase_shift.
    */
    sample_start_times: Vec<u32>,
}

impl<T> PreintegratedLoader<T>
where
    T: PcmFormat + 'static,
{
    pub fn new(internal_loader: PcmLoader<T>, pixel_rate: u32) -> Self {
        // Predict total pixel index for each sample change.
        // These values won't change as long as the pixel_rate and sample_rate stay the same.
        // We only need to precompute them once and then we can reuse the same PcmIntegrator.
        let mut sample_start_times: Vec<u32> =
            Vec::with_capacity(internal_loader.sample_rate + BUFFER_PADDING);
        sample_start_times.push(0);

        let mut last_sample = 0;
        for total_pixel_index in 0.. {
            let sample_index = ((total_pixel_index as f32 / pixel_rate as f32)
                * internal_loader.sample_rate as f32) as usize;
            if sample_index > last_sample {
                sample_start_times.push(total_pixel_index);
                last_sample = sample_index;
            }

            // If that was the last sample, then break from the loop.
            if sample_index == internal_loader.sample_rate + BUFFER_PADDING {
                break;
            }
        }

        Self {
            internal_loader,
            pixel_rate,
            sample_start_times,
        }
    }

    pub fn next_second(&mut self) -> Result<(), Box<dyn Error>> {
        self.internal_loader.next_second()?;

        Ok(())
    }

    pub fn samples(&mut self) -> Box<dyn IntSignal> {
        let pcm = self.internal_loader.pcm();

        //match &self.internal_loader.interpolation {
        //  Interpolation::Nearest => {
        Box::new(Nearest(pcm).integrate(self.pixel_rate, &self.sample_start_times))
        //}
        //Interpolation::Linear => Box::new(Linear(pcm)),
        //}
    }
}
