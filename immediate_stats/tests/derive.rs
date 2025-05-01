//! Tests the basic `StatContainer` derive with no extra features.

use immediate_stats::*;

#[derive(PartialEq, Debug)]
struct MyStat;

impl StatContainer for MyStat {
    fn reset_modifiers(&mut self) {}
}

#[derive(StatContainer, PartialEq, Debug)]
struct Movement {
    speed: iStat,
    custom: MyStat,
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
            custom: MyStat,
            other: true,
        };

        movement.reset_modifiers();

        assert_eq!(
            movement,
            Movement {
                speed: Stat::new(base),
                custom: MyStat,
                other: true
            }
        );
    }
}

#[derive(StatContainer, PartialEq, Debug)]
struct MaxHealth(iStat, bool);

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
    ignored: fStat,
    reset: fStat,
}

#[test]
fn reset_ignored() {
    for base in 0..10 {
        let stat = Stat {
            base: base as f32,
            bonus: 3.0,
            multiplier: 1.5,
        };

        let mut partial = PartialReset {
            ignored: stat,
            reset: stat,
        };

        partial.reset_modifiers();

        assert_eq!(partial.ignored, stat);
        assert_eq!(partial.reset, Stat::new(base as f32));
    }
}

#[derive(StatContainer, PartialEq, Debug)]
enum EnumStat {
    Named {
        stat: iStat,
        other: u8,
    },
    Unnamed(iStat, u8),
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
