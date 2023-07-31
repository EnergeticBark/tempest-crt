use crate::{DOT_CLOCK, H_TOTAL, VERTICAL_SYNC, V_TOTAL};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::marker::PhantomData;
use std::path::Path;

use super::{Interpolation, Linear, Nearest, Pcm, PcmFormat, Signal};

pub struct PcmLoader<T: PcmFormat> {
    buffer: BufReader<File>,
    pub(super) sample_rate: usize,
    pub(super) interpolation: Interpolation,
    samples_per_frame: usize,
    pub(super) pixels_per_sample: f32,
    phantom: PhantomData<T>,
}

impl<T> PcmLoader<T>
where
    T: PcmFormat + 'static,
{
    pub fn open<P: AsRef<Path>>(path: P, sample_rate: usize) -> Result<Self, Box<dyn Error>> {
        let file = File::open(path)?;
        dbg!(DOT_CLOCK);
        dbg!(sample_rate);
        let samples_per_frame = (sample_rate as f64 / VERTICAL_SYNC).round() as usize;
        dbg!(samples_per_frame);
        let pixels_per_sample = (H_TOTAL * V_TOTAL) as f32 / samples_per_frame as f32;
        dbg!(pixels_per_sample);
        let mut buffer = BufReader::with_capacity(T::BYTES * samples_per_frame, file);
        buffer.fill_buf()?;

        Ok(PcmLoader {
            buffer,
            sample_rate,
            interpolation: Interpolation::Nearest,
            pixels_per_sample,
            samples_per_frame,
            phantom: PhantomData,
        })
    }

    pub fn next_frame(&mut self) -> Result<(), Box<dyn Error>> {
        self.buffer.consume(T::BYTES * self.samples_per_frame);
        loop {
            self.buffer.fill_buf()?;
            if self.buffer.buffer().len() == self.buffer.capacity() {
                break;
            }
        }

        Ok(())
    }

    pub(super) fn pcm(&self) -> Pcm<T> {
        let bytes = self.buffer.buffer();
        let samples: Vec<T> = T::from_bytes(bytes);

        Pcm {
            samples,
            sample_rate: self.sample_rate,
            pixels_per_sample: self.pixels_per_sample,
        }
    }

    pub fn samples(&self) -> Box<dyn Signal> {
        let pcm = self.pcm();

        match &self.interpolation {
            Interpolation::Nearest => Box::new(Nearest(pcm)),
            Interpolation::Linear => Box::new(Linear(pcm)),
        }
    }

    pub fn set_interp(&mut self, method: Interpolation) {
        self.interpolation = method;
    }
}
