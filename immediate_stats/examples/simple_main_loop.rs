//! A very simple example using a main loop.
//! There are two other versions of this example, one using Bevy and the other using Bevy Butler.

use immediate_stats::*;

#[derive(StatContainer)]
struct Speed(iStat);

fn main() {
    let mut speed = Speed(Stat::new(10)); // Set base speed to 10.

    loop {
        // 1. Apply modifiers:
        speed.0 *= 2.0; // Applies a multiplier to the final result.
        speed.0 += 5; // Adds a bonus to the final result.
        // The order does not matter. Bonuses are always applied before multipliers.

        // 2. Read total:
        println!("The current speed is {}.", speed.0.total());
        assert_eq!(speed.0.total(), 30); // (10 + 5) * 2 = 30

        // 3. Reset modifiers:
        speed.reset_modifiers(); // Reset bonus and multiplier, so speed is back to 10.
    }
}
