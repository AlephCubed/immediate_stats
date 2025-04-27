//! A very simple example using Bevy. Requires the `bevy` feature flag.
//! There are two other versions of this example, one using a simple main loop and the other using Bevy Butler.

use bevy::prelude::*;
use immediate_stats::{Stat, StatContainer, reset_component_modifiers};

#[derive(StatContainer, Component)]
struct Speed(Stat);

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_systems(Startup, init_speed)
        .add_systems(PreUpdate, reset_component_modifiers::<Speed>)
        // Modifiers must be applied before the speed can be read.
        .add_systems(Update, (apply_modifiers, read_speed).chain())
        .run();
}

fn init_speed(mut commands: Commands) {
    commands.spawn(Speed(Stat::new(10))); // Set base speed to 10.
}

fn apply_modifiers(mut speeds: Query<&mut Speed>) {
    for mut speed in &mut speeds {
        speed.0 *= 2.0; // Applies a multiplier to the final result.
        speed.0 += 5; // Adds a bonus to the final result.
        // The order does not matter. Bonuses are always applied before multipliers.
    }
}

fn read_speed(speeds: Query<&Speed>) {
    for speed in &speeds {
        println!("The current speed is {}.", speed.0.total());
        assert_eq!(speed.0.total(), 30); // (10 + 5) * 2 = 30
    }
}
