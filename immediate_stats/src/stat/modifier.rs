use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

/// Modifier values that can be applied to a [`super::Stat`].
#[derive(PartialEq, Debug, Copy, Clone)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct Modifier {
    /// Added to `base` of a [`super::Stat`] during calculation.
    pub bonus: i32,
    /// Multiplies the `base` of a [`super::Stat`] during calculation.
    pub multiplier: f32,
}

impl Modifier {
    pub fn new(bonus: i32, multiplier: f32) -> Self {
        Self { bonus, multiplier }
    }

    pub fn from_bonus(bonus: i32) -> Self {
        Self {
            bonus,
            ..Self::default()
        }
    }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let mut modifier = Modifier::default();
        modifier += 5;
        assert_eq!(
            modifier,
            Modifier {
                bonus: 5,
                multiplier: 1.0,
            }
        );
    }

    #[test]
    fn subtract() {
        let mut modifier = Modifier::default();
        modifier -= 5;
        assert_eq!(
            modifier,
            Modifier {
                bonus: -5,
                multiplier: 1.0,
            }
        );
    }

    #[test]
    fn multiply() {
        let mut modifier = Modifier::default();
        modifier *= 2.0;
        assert_eq!(
            modifier,
            Modifier {
                bonus: 0,
                multiplier: 2.0,
            }
        );
    }

    #[test]
    fn divide() {
        let mut modifier = Modifier::default();
        modifier /= 2.0;
        assert_eq!(
            modifier,
            Modifier {
                bonus: 0,
                multiplier: 0.5,
            }
        );
    }
}
