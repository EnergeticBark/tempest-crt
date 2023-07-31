use super::Signal;
use std::sync::Arc;

#[derive(Clone)]
pub struct AmplitudeModulator {
    pub carrier: Arc<dyn Signal>,
    pub information: Arc<dyn Signal>,
}

impl Signal for AmplitudeModulator {
    fn sample(&self, total_index: u32) -> f32 {
        let information_amplitude = (self.information.sample(total_index) + 1.0) / 2.0;
        let carrier_amplitude = self.carrier.sample(total_index);

        information_amplitude * carrier_amplitude
    }
}
