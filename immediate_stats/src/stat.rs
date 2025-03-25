use crate::StatContainer;
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

/// A stat that [resets] to a base value every iteration.
///
/// Temporary bonuses can be applied using [`+=`](add), [`-=`](sub), [`*=`](mul), and [`/=`](div).
/// During [calculation](Stat::total),
/// multiplication and division are always applied **after** addition and subtraction.
/// These bonuses are reset when [`reset_modifiers`](reset) is called.
///
/// [reset]:StatContainer::reset_modifiers
/// [add]:Stat::add_assign
/// [sub]:Stat::sub_assign
/// [mul]:Stat::mul_assign
/// [div]:Stat::div_assign
#[derive(PartialEq, Debug, Copy, Clone)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct Stat {
    /// The persistent value of the stat.
    pub base: i32,
    /// Added to `base` during calculation and gets reset to zero every iteration.
    /// This is added *before* `multiplier` is applied.
    ///
    /// Can be modified using [`Stat::add_assign`] and [`Stat::sub_assign`]
    pub bonus: i32,
    /// Multiplies the `base` during calculation and gets reset to one every iteration.
    /// This is applied *after* `bonus` is added.
    ///
    /// Can be modified using [`Stat::mul_assign`] and [`Stat::div_assign`]
    pub multiplier: f32,
}

impl Stat {
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

    pub fn with_bonus(mut self, bonus: i32) -> Self {
        self.bonus = bonus;
        self
    }

    pub fn with_multiplier(mut self, multiplier: f32) -> Self {
        self.multiplier = multiplier;
        self
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
    fn add_assign(&mut self, rhs: i32) {
        self.bonus += rhs;
    }
}

impl SubAssign<i32> for Stat {
    fn sub_assign(&mut self, rhs: i32) {
        self.bonus -= rhs;
    }
}

impl MulAssign<f32> for Stat {
    fn mul_assign(&mut self, rhs: f32) {
        self.multiplier *= rhs;
    }
}

impl DivAssign<f32> for Stat {
    fn div_assign(&mut self, rhs: f32) {
        self.multiplier /= rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reset() {
        for i in 0..10 {
            let mut stat = Stat {
                base: i,
                bonus: 4,
                multiplier: 1.5,
            };
            stat.reset_modifiers();

            assert_eq!(stat, Stat::new(i));
        }
    }

    #[test]
    fn add() {
        let mut stat = Stat::new(10);
        stat += 5;
        assert_eq!(
            stat,
            Stat {
                base: 10,
                bonus: 5,
                multiplier: 1.0,
            }
        );
    }

    #[test]
    fn subtract() {
        let mut stat = Stat::new(10);
        stat -= 5;
        assert_eq!(
            stat,
            Stat {
                base: 10,
                bonus: -5,
                multiplier: 1.0,
            }
        );
    }

    #[test]
    fn multiply() {
        let mut stat = Stat::new(10);
        stat *= 2.0;
        assert_eq!(
            stat,
            Stat {
                base: 10,
                bonus: 0,
                multiplier: 2.0,
            }
        );
    }

    #[test]
    fn divide() {
        let mut stat = Stat::new(10);
        stat /= 2.0;
        assert_eq!(
            stat,
            Stat {
                base: 10,
                bonus: 0,
                multiplier: 0.5,
            }
        );
    }

    #[test]
    fn default_total() {
        for i in 0..10 {
            assert_eq!(Stat::new(i).total(), i);
        }
    }

    #[test]
    fn total() {
        assert_eq!(
            Stat {
                base: 10,
                bonus: 4,
                multiplier: 1.5,
            }
            .total(),
            21
        )
    }

    #[test]
    fn total_no_bonus() {
        assert_eq!(Stat::new(20).with_multiplier(0.5).total(), 10);
    }

    #[test]
    fn total_no_multiplier() {
        assert_eq!(Stat::new(2).with_bonus(1).total(), 3);
    }
}
