use super::fm::FmCarrier;
use super::phase::Phase;
use super::{DiscreteTime, Signal};

pub struct Sine {
    frequency: u32,
}

impl Sine {
    pub fn from_freq(frequency: u32) -> Self {
        Self { frequency }
    }
}

impl Signal for Sine {
    fn sample(&self, t: &DiscreteTime) -> f32 {
        let phase = t.to_phase(self.frequency);
        (std::f32::consts::TAU * phase.float()).sin()
    }
}

impl FmCarrier for Sine {
    fn sample_with_deviation(&self, t: &DiscreteTime, deviation: Phase) -> f32 {
        let phase = t.to_phase(self.frequency) + deviation;
        (std::f32::consts::TAU * phase.float()).sin()
    }
}

pub struct Square {
    frequency: u32,
}

impl Square {
    pub fn from_freq(frequency: u32) -> Self {
        Self { frequency }
    }
}

impl Signal for Square {
    fn sample(&self, t: &DiscreteTime) -> f32 {
        let phase = t.to_phase(self.frequency);
        if phase.float() < 0.5 {
            return 1.0;
        }
        -1.0
    }
}

impl FmCarrier for Square {
    fn sample_with_deviation(&self, t: &DiscreteTime, deviation: Phase) -> f32 {
        let phase = t.to_phase(self.frequency) + deviation;
        if phase.float() < 0.5 {
            return 1.0;
        }
        -1.0
    }
}
