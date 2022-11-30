use super::{Modulator, Phase};

#[derive(Clone)]
pub struct AmplitudeModulator {
    pub carrier: u32,
    pub information: u32,
}

impl Modulator for AmplitudeModulator {
    fn sample(&self, phase: Phase) -> f64 {
        let information_amplitude = (2.0 * std::f64::consts::PI * phase.as_float() * self.information as f64).sin();
        let information_amplitude = (information_amplitude + 1.0) / 2.0;

        let carrier_amplitude = (2.0 * std::f64::consts::PI * phase.as_float() * self.carrier as f64).sin();

        information_amplitude * carrier_amplitude
    }
}
