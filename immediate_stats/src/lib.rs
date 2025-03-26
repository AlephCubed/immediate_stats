#[cfg(feature = "bevy")]
pub mod bevy;
pub mod stat;

/// Types that contain stats that need to be reset.
///
/// It is recommended to use the [derive macro](macro) instead of implementing manually.
///
/// [macro]: immediate_stats_macros::StatContainer
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
        other: bool,
    }

    #[test]
    fn reset() {
        for base in 0..10 {
            let mut movement = Movement {
                speed: Stat {
                    base,
                    bonus: 3,
                    multiplier: 1.5,
                },
                other: true,
            };

            movement.reset_modifiers();

            assert_eq!(
                movement,
                Movement {
                    speed: Stat::new(base),
                    other: true
                }
            );
        }
    }

    #[derive(StatContainer, PartialEq, Debug)]
    struct MaxHealth(Stat, bool);

    #[test]
    fn reset_tuple_struct() {
        for base in 0..10 {
            let mut max_health = MaxHealth(
                Stat {
                    base,
                    bonus: 3,
                    multiplier: 1.5,
                },
                true,
            );

            max_health.reset_modifiers();

            assert_eq!(max_health.0, Stat::new(base));
        }
    }

    #[derive(StatContainer, PartialEq, Debug)]
    struct Health {
        #[stat]
        max: MaxHealth,
        current: i32,
    }

    #[test]
    fn reset_with_attribute() {
        for base in 0..10 {
            let mut health = Health {
                max: MaxHealth(
                    Stat {
                        base,
                        bonus: 3,
                        multiplier: 1.5,
                    },
                    true,
                ),
                current: base,
            };

            health.reset_modifiers();

            assert_eq!(health.max.0, Stat::new(base));
        }
    }

    #[derive(StatContainer, PartialEq, Debug)]
    struct PartialReset {
        #[stat_ignore]
        ignored: Stat,
        reset: Stat,
    }

    #[test]
    fn reset_ignored() {
        for base in 0..10 {
            let stat = Stat {
                base,
                bonus: 3,
                multiplier: 1.5,
            };

            let mut partial = PartialReset {
                ignored: stat,
                reset: stat,
            };

            partial.reset_modifiers();

            assert_eq!(partial.ignored, stat);
            assert_eq!(partial.reset, Stat::new(base));
        }
    }

    #[derive(StatContainer, PartialEq, Debug)]
    enum EnumStat {
        Named {
            stat: Stat,
            other: u8,
        },
        Unnamed(Stat, u8),
        #[expect(dead_code)]
        Other,
    }

    #[test]
    fn reset_enum_named() {
        for base in 0..10 {
            let mut stat = EnumStat::Named {
                stat: Stat {
                    base,
                    bonus: 3,
                    multiplier: 1.5,
                },
                other: 0,
            };

            stat.reset_modifiers();

            assert_eq!(
                stat,
                EnumStat::Named {
                    stat: Stat::new(base),
                    other: 0,
                }
            );
        }
    }

    #[test]
    fn reset_enum_unnamed() {
        for base in 0..10 {
            let mut stat = EnumStat::Unnamed(
                Stat {
                    base,
                    bonus: 3,
                    multiplier: 1.5,
                },
                0,
            );

            stat.reset_modifiers();

            assert_eq!(stat, EnumStat::Unnamed(Stat::new(base), 0));
        }
    }
}
