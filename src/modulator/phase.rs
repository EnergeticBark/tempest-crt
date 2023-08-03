use std::num::Wrapping;
use std::ops::{Add, AddAssign, Mul};

// We need to multiply by very large pixel indices, and f32 loses too much precision.
// So we're going to represent the decimal part of the phase as a u32,
// where 0° = 0 and 360° = u32::MAX + 1.
#[derive(Copy, Clone)]
pub struct Phase(pub(super) Wrapping<u32>);

impl Phase {
    pub(super) fn float(&self) -> f32 {
        self.0 .0 as f32 / (u32::MAX as f32 + 1.0)
    }
}

impl From<f32> for Phase {
    fn from(value: f32) -> Self {
        if value.is_sign_negative() {
            // Flipping bits and adding is the same as subtracting.
            Self(Wrapping(
                !((-value * (u32::MAX as f32 + 1.0)).round() as u32),
            ))
        } else {
            Self(Wrapping((value * (u32::MAX as f32 + 1.0)).round() as u32))
        }
    }
}

impl Mul<Self> for Phase {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl Mul<u32> for Phase {
    type Output = Self;

    fn mul(self, rhs: u32) -> Self::Output {
        // Wrapping multiplication maintains full 32-bit precision.
        // It also naturally reduces our phase modulo 2pi. :)
        Self(self.0 * Wrapping(rhs))
    }
}

impl Add<Self> for Phase {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for Phase {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}
