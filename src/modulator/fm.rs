use super::{DiscreteTime, Phase, Signal};
use crate::modulator::IntSignal;
use std::sync::Arc;

pub trait FmCarrier: Send + Sync {
    fn sample_with_deviation(&self, t: &DiscreteTime, deviation: Phase) -> f32;
}

#[derive(Clone)]
pub struct FrequencyModulator {
    pub carrier: Arc<dyn FmCarrier>,
    pub information: Arc<dyn IntSignal>,
}

impl Signal for FrequencyModulator {
    fn sample(&self, t: &DiscreteTime) -> f32 {
        let max_deviation = 37500;

        let deviation = self.information.sample(t) * max_deviation;

        self.carrier.sample_with_deviation(t, deviation)
    }
}
