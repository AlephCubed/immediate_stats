//! Contains the basic stat object.

pub mod modifier;

use crate::StatContainer;
use modifier::Modifier;
use num::Num;
use std::fmt::Debug;
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

/// Trait bounds for [`Stat`] and [`Modifier`] numbers.
pub trait StatNum:
    Num + AddAssign + SubAssign + MulAssign + DivAssign + Debug + Clone + Copy
{
}

impl<T: Num + AddAssign + SubAssign + MulAssign + DivAssign + Debug + Clone + Copy> StatNum for T {}

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
pub struct Stat<B: StatNum, M: StatNum> {
    /// The persistent value of the stat.
    /// After being [reset](StatContainer::reset_modifiers), [`Stat::total`] will be equal to `base`.
    pub base: B,
    /// Added to `base` during calculation and gets reset to zero every iteration.
    /// This is added *before* `multiplier` is applied.
    ///
    /// Can be modified using [`+=`](Stat::add_assign) or [`-=`](Stat::sub_assign).
    pub bonus: B,
    /// Multiplies the `base` during calculation and gets reset to one every iteration.
    /// This is applied *after* `bonus` is added.
    ///
    /// Can be modified using [*=](`Stat::mul_assign`) or [/=](`Stat::div_assign`).
    pub multiplier: M,
}

impl<B: StatNum, M: StatNum> Stat<B, M> {
    /// Creates a new modifier from a base value.
    pub fn new(base: B) -> Self {
        Self {
            base,
            ..Default::default()
        }
    }

    /// A builder that overwrites the current bonus with a new value.
    pub fn with_bonus(mut self, bonus: B) -> Self {
        self.bonus = bonus;
        self
    }

    /// A builder that overwrites the multiplier bonus with a new value.
    pub fn with_multiplier(mut self, multiplier: M) -> Self {
        self.multiplier = multiplier;
        self
    }

    /// A builder that overwrites the current bonus and multiplier with a new value.
    pub fn with_modifier(mut self, modifier: Modifier<B, M>) -> Self {
        self.bonus = modifier.bonus;
        self.multiplier = modifier.multiplier;
        self
    }

    /// Applies the [`Modifier`] values to the bonus and multiplier.
    ///
    /// This adds the bonuses, and multiplies the multipliers.
    pub fn apply(&mut self, modifier: Modifier<B, M>) {
        self.bonus += modifier.bonus;
        self.multiplier *= modifier.multiplier;
    }
}

macro_rules! stat_impl {
    ($B:ident, $M:ident) => {
        impl Stat<$B, $M> {
            /// Calculates the total value of the stat.
            pub fn total(&self) -> $B {
                ((self.base + self.bonus) as $M * self.multiplier) as $B
            }
        }
    };
}

stat_impl!(i32, f32);
stat_impl!(i64, f64);

stat_impl!(f32, f32);
stat_impl!(f64, f64);

pub type iStat32 = Stat<i32, f32>;
pub type iStat64 = Stat<i64, f64>;

pub type fStat32 = Stat<f32, f32>;
pub type fStat64 = Stat<f64, f64>;

pub type iStat = iStat32;
pub type fStat = fStat32;

impl<B: StatNum, M: StatNum> StatContainer for Stat<B, M> {
    fn reset_modifiers(&mut self) {
        self.bonus = B::zero();
        self.multiplier = M::one();
    }
}

impl<B: StatNum, M: StatNum> Default for Stat<B, M> {
    fn default() -> Self {
        Self {
            base: B::zero(),
            bonus: B::zero(),
            multiplier: M::one(),
        }
    }
}

impl<B: StatNum, M: StatNum> AddAssign<B> for Stat<B, M> {
    /// Adds to the stat's bonus.
    fn add_assign(&mut self, rhs: B) {
        self.bonus += rhs;
    }
}

impl<B: StatNum, M: StatNum> SubAssign<B> for Stat<B, M> {
    /// Subtracts from the stat's bonus.
    fn sub_assign(&mut self, rhs: B) {
        self.bonus -= rhs;
    }
}

impl<B: StatNum, M: StatNum> MulAssign<M> for Stat<B, M> {
    /// Multiplies the stat's multiplier.
    fn mul_assign(&mut self, rhs: M) {
        self.multiplier *= rhs;
    }
}

impl<B: StatNum, M: StatNum> DivAssign<M> for Stat<B, M> {
    /// Divides the stat's multiplier.
    fn div_assign(&mut self, rhs: M) {
        self.multiplier /= rhs;
    }
}
