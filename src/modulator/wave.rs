use super::fm::FmCarrier;
use super::phase::Phase;
use super::Signal;
use crate::modulator::IntSignal;
use std::num::Wrapping;

#[derive(Copy, Clone)]
pub struct Sine {
    frequency: u32,
    phase_per_pixel: Phase,
    starting_angle: Phase,
}

impl Sine {
    pub fn from_freq(frequency: u32, dot_clock: u32) -> Self {
        Self {
            frequency,
            phase_per_pixel: Phase::from(frequency as f32 / dot_clock as f32),
            starting_angle: Phase(Wrapping(0)),
        }
    }

    pub fn next_frame(&mut self, frame_size: u32) {
        self.starting_angle = self.starting_angle + self.phase_per_pixel * frame_size;
    }
}

impl Signal for Sine {
    fn sample(&self, total_index: u32) -> f32 {
        let phase = self.starting_angle + self.phase_per_pixel * total_index;
        (std::f32::consts::TAU * phase.float()).sin()
    }
}

impl FmCarrier for Sine {
    fn sample_with_deviation(&self, total_index: u32, deviation: Phase) -> f32 {
        let phase = self.starting_angle + self.phase_per_pixel * total_index + deviation;
        (std::f32::consts::TAU * phase.float()).sin()
    }
}

impl IntSignal for Sine {
    fn sample(&self, total_index: u32) -> Phase {
        Phase::from(
            Signal::sample(self, total_index) / self.frequency as f32 / std::f32::consts::TAU,
        )
    }
}

#[derive(Copy, Clone)]
pub struct Square {
    phase_per_pixel: Phase,
    starting_angle: Phase,
}

impl Square {
    pub fn from_freq(frequency: u32, dot_clock: u32) -> Self {
        Self {
            phase_per_pixel: Phase::from(frequency as f32 / dot_clock as f32),
            starting_angle: Phase(Wrapping(0)),
        }
    }

    pub fn next_frame(&mut self, frame_size: u32) {
        self.starting_angle = self.starting_angle + self.phase_per_pixel * frame_size;
    }
}

impl Signal for Square {
    fn sample(&self, total_index: u32) -> f32 {
        let phase = self.starting_angle + self.phase_per_pixel * total_index;
        if phase.float() < 0.5 {
            return 1.0;
        }
        -1.0
    }
}

impl FmCarrier for Square {
    fn sample_with_deviation(&self, total_index: u32, deviation: Phase) -> f32 {
        let phase = self.starting_angle + self.phase_per_pixel * total_index + deviation;
        if phase.float() < 0.5 {
            return 1.0;
        }
        -1.0
    }
}
