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
        assert_eq!(iStat::new(i).total(), i);
    }
}

#[test]
fn total() {
    assert_eq!(
        fStat {
            base: 10.0,
            bonus: 4.0,
            multiplier: 1.5,
        }
        .total(),
        21.0
    )
}

#[test]
fn total_no_bonus() {
    assert_eq!(iStat::new(20).with_multiplier(0.5).total(), 10);
}

#[test]
fn total_no_multiplier() {
    assert_eq!(fStat::new(2.0).with_bonus(1.0).total(), 3.0);
}

#[test]
fn total_with_modifier() {
    assert_eq!(
        iStat::new(5).with_modifier(Modifier::new(1, 0.5)).total(),
        3
    );
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
