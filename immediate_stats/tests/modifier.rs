//! Tests the various methods of `Modifier`.

use immediate_stats::*;

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

#[test]
fn scaled() {
    let modifier = Modifier {
        bonus: 10,
        multiplier: 3.0,
    };
    assert_eq!(
        modifier.scaled(0.5),
        Modifier {
            bonus: 5,
            multiplier: 2.0,
        }
    );
}
