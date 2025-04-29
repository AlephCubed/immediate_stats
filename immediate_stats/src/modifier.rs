//! Contains a modifier that can be applied to [`Stat`](crate::Stat).

use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

/// Modifier values that can be [applied](super::Stat::apply) to a [`Stat`](super::Stat).
#[derive(PartialEq, Debug, Copy, Clone)]
#[cfg_attr(
    feature = "bevy",
    derive(bevy_reflect::Reflect),
    reflect(PartialEq, Debug, Clone)
)]
pub struct Modifier {
    /// Added to `base` of a [`super::Stat`] during calculation.
    pub bonus: i32,
    /// Multiplies the `base` of a [`super::Stat`] during calculation.
    pub multiplier: f32,
}

impl Modifier {
    /// Creates a new modifier from a bonus and a multiplier.
    pub fn new(bonus: i32, multiplier: f32) -> Self {
        Self { bonus, multiplier }
    }

    /// Creates a new modifier from a bonus.
    pub fn from_bonus(bonus: i32) -> Self {
        Self {
            bonus,
            ..Self::default()
        }
    }

    /// Creates a new modifier from a multiplier.
    pub fn from_multiplier(multiplier: f32) -> Self {
        Self {
            multiplier,
            ..Self::default()
        }
    }
}

impl Default for Modifier {
    fn default() -> Self {
        Self {
            bonus: 0,
            multiplier: 1.0,
        }
    }
}

impl AddAssign<i32> for Modifier {
    fn add_assign(&mut self, rhs: i32) {
        self.bonus += rhs;
    }
}

impl SubAssign<i32> for Modifier {
    fn sub_assign(&mut self, rhs: i32) {
        self.bonus -= rhs;
    }
}

impl MulAssign<f32> for Modifier {
    fn mul_assign(&mut self, rhs: f32) {
        self.multiplier *= rhs;
    }
}

impl DivAssign<f32> for Modifier {
    fn div_assign(&mut self, rhs: f32) {
        self.multiplier /= rhs;
    }
}
