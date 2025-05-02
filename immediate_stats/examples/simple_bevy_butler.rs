//! A very simple example using Bevy Butler. Requires the `bevy_butler` feature flag.
//! There are two other versions of this example, one using a simple main loop and the other using Bevy.

use bevy::prelude::*;
use bevy_butler::*;
use immediate_stats::*;

fn main() {
    App::new()
        .add_plugins((MinimalPlugins, ImmediateStatsPlugin, SpeedPlugin))
        .run();
}

#[butler_plugin]
struct SpeedPlugin;

// Implements `reset_modifiers` by passing the call onto `Stat`.
// This will also add the `ResetComponentPlugin` to `SpeedPlugin`.
#[derive(StatContainer, Component)]
#[add_component(plugin = SpeedPlugin)]
struct Speed(Stat);

#[add_system(plugin = SpeedPlugin, schedule = Startup)]
fn init_speed(mut commands: Commands) {
    commands.spawn(Speed(Stat::new(10))); // Set base speed to 10.
}

#[add_system(plugin = SpeedPlugin, schedule = Update, in_set = StatSystems::Modify)]
fn apply_modifiers(mut speeds: Query<&mut Speed>) {
    for mut speed in &mut speeds {
        speed.0 *= 2.0; // Applies a multiplier to the final result.
        speed.0 += 5; // Adds a bonus to the final result.
        // The order does not matter, bonuses are always applied before multipliers.
    }
}

#[add_system(plugin = SpeedPlugin, schedule = Update, in_set = StatSystems::Read)]
fn read_speed(speeds: Query<&Speed>) {
    for speed in &speeds {
        println!("The current speed is {}.", speed.0.total());
        assert_eq!(speed.0.total(), 30); // (10 + 5) * 2 = 30
    }
}
