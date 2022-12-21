use super::{DiscreteTime, Signal};
use std::sync::Arc;

#[derive(Clone)]
pub struct AmplitudeModulator {
    pub carrier: Arc<dyn Signal>,
    pub information: Arc<dyn Signal>,
}

impl Signal for AmplitudeModulator {
    fn sample(&self, t: &DiscreteTime) -> f64 {
        let information_amplitude = (self.information.sample(t) + 1.0) / 2.0;
        let carrier_amplitude = self.carrier.sample(t);

        information_amplitude * carrier_amplitude
    }
}
