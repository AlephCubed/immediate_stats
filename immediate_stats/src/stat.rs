use crate::StatContainer;
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Stat {
    pub base: i32,
    pub bonus: i32,
    pub multiplier: f32,
}

impl Stat {
    pub fn new(base: i32) -> Self {
        Self {
            base,
            ..Default::default()
        }
    }

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
