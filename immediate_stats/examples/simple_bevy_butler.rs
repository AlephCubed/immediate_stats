//! A very simple example using Bevy Butler. Requires the `bevy_butler` feature flag.
//! There are two other versions of this example, one using a simple main loop and the other using Bevy.

use bevy::prelude::*;
use bevy_butler::{add_system, butler_plugin};
use immediate_stats::{Stat, StatContainer};

#[butler_plugin]
struct SpeedPlugin;

#[derive(StatContainer, Component)]
#[add_component(plugin = SpeedPlugin)]
struct Speed(Stat);

fn main() {
    App::new().add_plugins((MinimalPlugins, SpeedPlugin)).run();
}

#[add_system(plugin = SpeedPlugin, schedule = Startup)]
fn init_speed(mut commands: Commands) {
    commands.spawn(Speed(Stat::new(10))); // Set base speed to 10.
}

#[add_system(plugin = SpeedPlugin, schedule = Update)]
fn apply_modifiers(mut speeds: Query<&mut Speed>) {
    for mut speed in &mut speeds {
        speed.0 *= 2.0; // Applies a multiplier to the final result.
        speed.0 += 5; // Adds a bonus to the final result.
        // The order does not matter. Bonuses are always applied before multipliers.
    }
}

// Modifiers must be applied before the speed can be read.
#[add_system(plugin = SpeedPlugin, schedule = Update, after = apply_modifiers)]
fn read_speed(speeds: Query<&Speed>) {
    for speed in &speeds {
        println!("The current speed is {}.", speed.0.total());
        assert_eq!(speed.0.total(), 30); // (10 + 5) * 2 = 30
    }
}
