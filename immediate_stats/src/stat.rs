//! Contains the basic stat object.

use crate::StatContainer;
use crate::modifier::Modifier;
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

/// A stat that [resets][reset] to a base value every iteration.
///
/// Temporary bonuses can be applied using [`+=`][add], [`-=`][sub], [`*=`][mul], and [`/=`][div].
/// During [calculation](Stat::total),
/// multiplication and division are always applied **after** addition and subtraction.
/// These bonuses are reset when [`reset_modifiers`][reset] is called.
///
/// [reset]: StatContainer::reset_modifiers
/// [add]: Stat::add_assign
/// [sub]: Stat::sub_assign
/// [mul]: Stat::mul_assign
/// [div]: Stat::div_assign
#[derive(PartialEq, Debug, Copy, Clone)]
#[cfg_attr(
    feature = "bevy",
    derive(bevy_reflect::Reflect),
    reflect(PartialEq, Debug, Clone)
)]
pub struct Stat {
    /// The persistent value of the stat.
    /// After being [reset](StatContainer::reset_modifiers), [`Stat::total`] will be equal to `base`.
    pub base: i32,
    /// Added to `base` during calculation and gets reset to zero every iteration.
    /// This is added *before* `multiplier` is applied.
    ///
    /// Can be modified using [`+=`](Stat::add_assign) or [`-=`](Stat::sub_assign).
    pub bonus: i32,
    /// Multiplies the `base` during calculation and gets reset to one every iteration.
    /// This is applied *after* `bonus` is added.
    ///
    /// Can be modified using [*=](`Stat::mul_assign`) or [/=](`Stat::div_assign`).
    pub multiplier: f32,
}

impl Stat {
    /// Creates a new modifier from a base value.
    pub fn new(base: i32) -> Self {
        Self {
            base,
            ..Default::default()
        }
    }

    /// Calculates the total value of the stat.
    pub fn total(&self) -> i32 {
        ((self.base + self.bonus) as f32 * self.multiplier) as i32
    }

    /// A builder that overwrites the current bonus with a new value.
    pub fn with_bonus(mut self, bonus: i32) -> Self {
        self.bonus = bonus;
        self
    }

    /// A builder that overwrites the multiplier bonus with a new value.
    pub fn with_multiplier(mut self, multiplier: f32) -> Self {
        self.multiplier = multiplier;
        self
    }

    /// A builder that overwrites the current bonus and multiplier with a new value.
    pub fn with_modifier(mut self, modifier: Modifier) -> Self {
        self.bonus = modifier.bonus;
        self.multiplier = modifier.multiplier;
        self
    }

    /// Applies the [`Modifier`] values to the bonus and multiplier.
    ///
    /// This adds the bonuses, and multiplies the multipliers.
    pub fn apply(&mut self, modifier: Modifier) {
        self.bonus += modifier.bonus;
        self.multiplier *= modifier.multiplier;
    }
}

impl StatContainer for Stat {
    fn reset_modifiers(&mut self) {
        self.bonus = 0;
        self.multiplier = 1.0;
    }
}

impl Default for Stat {
    fn default() -> Self {
        Self {
            base: 0,
            bonus: 0,
            multiplier: 1.0,
        }
    }
}

impl AddAssign<i32> for Stat {
    /// Adds to the stat's bonus.
    fn add_assign(&mut self, rhs: i32) {
        self.bonus += rhs;
    }
}

impl SubAssign<i32> for Stat {
    /// Subtracts from the stat's bonus.
    fn sub_assign(&mut self, rhs: i32) {
        self.bonus -= rhs;
    }
}

impl MulAssign<f32> for Stat {
    /// Multiplies the stat's multiplier.
    fn mul_assign(&mut self, rhs: f32) {
        self.multiplier *= rhs;
    }
}

impl DivAssign<f32> for Stat {
    /// Divides the stat's multiplier.
    fn div_assign(&mut self, rhs: f32) {
        self.multiplier /= rhs;
    }
}
