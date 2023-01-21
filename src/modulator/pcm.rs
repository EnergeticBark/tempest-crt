use crate::modulator::Interpolation::Nearest;
use crate::DOT_CLOCK;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};
use std::marker::PhantomData;
use std::mem;
use std::path::Path;

use super::{DiscreteTime, Signal};

const BUFFER_PADDING: usize = 340;

pub trait PcmFormat: Send + Sync + Sized {
    const BYTES: usize;
    fn amplitude(&self) -> f32;
    fn from_bytes(bytes: &[u8]) -> Vec<Self>;
}

pub struct Unsigned8(u8);

impl PcmFormat for Unsigned8 {
    const BYTES: usize = mem::size_of::<u8>();
    fn amplitude(&self) -> f32 {
        // u8 PCM has a center of 128
        self.0 as f32 / 128.0 - 1.0
    }
    fn from_bytes(bytes: &[u8]) -> Vec<Self> {
        bytes.into_iter().map(|&byte| Self(byte)).collect()
    }
}

pub struct Signed16Le(i16);

impl PcmFormat for Signed16Le {
    const BYTES: usize = mem::size_of::<i16>();
    fn amplitude(&self) -> f32 {
        self.0 as f32 / 32768.0
    }
    fn from_bytes(bytes: &[u8]) -> Vec<Self> {
        bytes
            .chunks_exact(Self::BYTES)
            .map(|chunk| Self(i16::from_le_bytes(chunk.try_into().unwrap())))
            .collect()
    }
}

#[derive(Clone)]
pub struct Pcm<T: PcmFormat> {
    samples: Vec<T>,
    sample_rate: usize,
}

impl<T> Signal for Pcm<T>
    where
        T: PcmFormat,
{
    fn sample(&self, t: &DiscreteTime) -> f32 {
        let sample_index =
            (t.numerator as f32 / t.denominator as f32 * self.sample_rate as f32) as usize;

        self.samples[sample_index].amplitude()
    }
}

pub struct LerpPcm<T: PcmFormat>(Pcm<T>);

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

pub enum Interpolation {
    Nearest,
    Linear,
}

pub struct PcmLoader<T: PcmFormat> {
    buffer: BufReader<File>,
    sample_rate: usize,
    interpolation: Interpolation,
    phantom: PhantomData<T>,
}

impl<T> PcmLoader<T>
    where
        T: PcmFormat + 'static,
{
    pub fn open<P: AsRef<Path>>(path: P, sample_rate: usize) -> Result<Self, Box<dyn Error>> {
        let file = File::open(path)?;
        let mut buffer = BufReader::with_capacity(T::BYTES * (sample_rate + BUFFER_PADDING), file);
        buffer.fill_buf()?;

        Ok(PcmLoader {
            buffer,
            sample_rate,
            interpolation: Nearest,
            phantom: PhantomData,
        })
    }

    pub fn next_second(&mut self) -> Result<(), Box<dyn Error>> {
        self.buffer
            .seek(SeekFrom::Current((T::BYTES * self.sample_rate) as i64))?;
        self.buffer.fill_buf()?;

        Ok(())
    }

    pub fn samples(&mut self) -> Box<dyn Signal> {
        let bytes = self.buffer.buffer();
        let samples: Vec<T> = T::from_bytes(bytes);

        let pcm = Pcm {
            samples,
            sample_rate: self.sample_rate,
        };

        match &self.interpolation {
            Nearest => Box::new(pcm),
            Linear => Box::new(LerpPcm(pcm)),
        }
    }

    pub fn set_interp(&mut self, method: Interpolation) {
        self.interpolation = method;
    }
}
