//! Tests the various methods of `Stat`.

use immediate_stats::*;

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

#[test]
fn total_with_modifier() {
    assert_eq!(Stat::new(5).with_modifier(Modifier::new(1, 0.5)).total(), 3);
}

#[test]
fn apply() {
    let mut stat = Stat::new(10);

    stat.apply(Modifier::new(2, 2.0));
    stat.apply(Modifier::new(3, 4.0));

    assert_eq!(
        stat,
        Stat {
            base: 10,
            bonus: 5,
            multiplier: 8.0,
        }
    )
}

#[test]
fn apply_scaled() {
    let mut stat = Stat::new(10);

    // Should result in +1, x1.5
    stat.apply_scaled(Modifier::new(2, 2.0), 0.5);

    // Should result in +2, x3.0
    stat.apply_scaled(Modifier::new(4, 5.0), 0.5);

    assert_eq!(
        stat,
        Stat {
            base: 10,
            bonus: 3,
            multiplier: 4.5,
        }
    )
}
