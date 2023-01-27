use std::mem;

pub trait PcmFormat: Send + Sync + Sized + Copy + Clone {
    const BYTES: usize;
    fn amplitude(&self) -> f32;
    fn from_bytes(bytes: &[u8]) -> Vec<Self>;
}

#[derive(Copy, Clone)]
pub struct Unsigned8(u8);

impl PcmFormat for Unsigned8 {
    const BYTES: usize = mem::size_of::<u8>();
    fn amplitude(&self) -> f32 {
        // u8 PCM has a center of 128
        self.0 as f32 / 128.0 - 1.0
    }
    fn from_bytes(bytes: &[u8]) -> Vec<Self> {
        bytes.iter().map(|&byte| Self(byte)).collect()
    }
}

#[derive(Copy, Clone)]
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
