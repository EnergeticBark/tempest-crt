use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};
use std::marker::PhantomData;
use std::path::Path;

use super::{Interpolation, LerpPcm, Pcm, PcmFormat, Signal};

const BUFFER_PADDING: usize = 340;

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
            interpolation: Interpolation::Nearest,
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