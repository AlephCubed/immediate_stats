//! Contains a modifier that can be applied to [`Stat`](crate::Stat).

use super::StatNum;
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

/// Modifier values that can be [applied](super::Stat::apply) to a [`Stat`](super::Stat).
#[derive(PartialEq, Debug, Copy, Clone)]
#[cfg_attr(
    feature = "bevy",
    derive(bevy_reflect::Reflect),
    reflect(PartialEq, Debug, Clone)
)]
pub struct Modifier<B: StatNum, M: StatNum> {
    /// Added to `base` of a [`Stat`](super::Stat) during calculation.
    ///
    /// Can be modified using [`+=`](Modifier::add_assign) and [`-=`](`Modifier::sub_assign`).
    pub bonus: B,
    /// Multiplies the `base` of a [`Stat`](super::Stat) during calculation.
    ///
    /// Can be modified using [`*=`](`Modifier::mul_assign`) and [`/=`](`Modifier::div_assign`).
    pub multiplier: M,
}

pub type iModifier32 = Modifier<i32, f32>;
pub type iModifier64 = Modifier<i64, f64>;

pub type fModifier32 = Modifier<f32, f32>;
pub type fModifier64 = Modifier<f64, f64>;

pub type iModifier = iModifier32;
pub type fModifier = fModifier32;

impl<B: StatNum, M: StatNum> Modifier<B, M> {
    /// Creates a new modifier from a bonus and a multiplier.
    pub fn new(bonus: B, multiplier: M) -> Self {
        Self { bonus, multiplier }
    }

    /// Creates a new modifier from a bonus.
    pub fn from_bonus(bonus: B) -> Self {
        Self {
            bonus,
            ..Self::default()
        }
    }

    /// Creates a new modifier from a multiplier.
    pub fn from_multiplier(multiplier: M) -> Self {
        Self {
            multiplier,
            ..Self::default()
        }
    }
}

impl<B: StatNum, M: StatNum> Default for Modifier<B, M> {
    fn default() -> Self {
        Self {
            bonus: B::zero(),
            multiplier: M::one(),
        }
    }
}

impl<B: StatNum, M: StatNum> AddAssign<B> for Modifier<B, M> {
    /// Adds to the modifier's bonus.
    fn add_assign(&mut self, rhs: B) {
        self.bonus += rhs;
    }
}

impl<B: StatNum, M: StatNum> SubAssign<B> for Modifier<B, M> {
    /// Subtracts from the modifier's bonus.
    fn sub_assign(&mut self, rhs: B) {
        self.bonus -= rhs;
    }
}

impl<B: StatNum, M: StatNum> MulAssign<M> for Modifier<B, M> {
    /// Multiplies the modifier's multiplier.
    fn mul_assign(&mut self, rhs: M) {
        self.multiplier *= rhs;
    }
}

impl<B: StatNum, M: StatNum> DivAssign<M> for Modifier<B, M> {
    /// Divides the modifier's multiplier.
    fn div_assign(&mut self, rhs: M) {
        self.multiplier /= rhs;
    }
}
