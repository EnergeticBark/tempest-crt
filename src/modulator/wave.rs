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
        (std::f32::consts::TAU * phase).sin()
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
        if phase < 0.5 {
            return 1.0;
        }
        -1.0
    }
}
