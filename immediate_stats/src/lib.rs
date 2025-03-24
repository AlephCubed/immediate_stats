#[cfg(feature = "bevy")]
pub mod bevy;
pub mod stat;

pub trait StatContainer {
    /// Resets all stat bonuses to zero, and stat multipliers to one.
    fn reset_modifiers(&mut self);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stat::Stat;
    use immediate_stats_macros::StatContainer;

    #[derive(StatContainer, PartialEq, Debug)]
    struct Movement {
        speed: Stat,
    }

    #[derive(StatContainer, PartialEq, Debug)]
    struct Health(Stat);

    #[test]
    fn reset() {
        for base in 0..10 {
            let mut movement = Movement {
                speed: Stat {
                    base,
                    bonus: 3,
                    multiplier: 1.5,
                },
            };

            movement.reset_modifiers();

            assert_eq!(movement.speed, Stat::new(base));
        }
    }

    #[test]
    fn reset_tuple() {
        for base in 0..10 {
            let mut health = Health(Stat {
                base,
                bonus: 3,
                multiplier: 1.5,
            });

            health.reset_modifiers();

            assert_eq!(health.0, Stat::new(base));
        }
    }
}
