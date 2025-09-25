//! A very simple example using Bevy Auto Plugin. Requires the `bevy_auto_plugin` feature flag.
//! There are two other versions of this example, one using a simple main loop and the other using Bevy.

use bevy::prelude::*;
use bevy_auto_plugin;
use bevy_auto_plugin::modes::global::prelude::{AutoPlugin, auto_component, auto_system};
use immediate_stats::*;

fn main() {
    App::new()
        .add_plugins((MinimalPlugins, ImmediateStatsPlugin, SpeedPlugin))
        .run();
}

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct SpeedPlugin;

// Implements `reset_modifiers` by passing the call onto `Stat`.
// This will also add the `ResetComponentPlugin` to `SpeedPlugin`.
#[derive(StatContainer, Component)]
#[auto_component(plugin = SpeedPlugin)]
struct Speed(Stat);

#[auto_system(plugin = SpeedPlugin, schedule = Startup)]
fn init_speed(mut commands: Commands) {
    commands.spawn(Speed(Stat::new(10))); // Set base speed to 10.
}

#[auto_system(plugin = SpeedPlugin, schedule = Update, config(in_set = StatSystems::Modify))]
fn apply_modifiers(mut speeds: Query<&mut Speed>) {
    for mut speed in &mut speeds {
        speed.0 *= 2.0; // Applies a multiplier to the final result.
        speed.0 += 5; // Adds a bonus to the final result.
        // The order does not matter, bonuses are always applied before multipliers.
    }
}

#[auto_system(plugin = SpeedPlugin, schedule = Update, config(in_set = StatSystems::Read))]
fn read_speed(speeds: Query<&Speed>) {
    for speed in &speeds {
        println!("The current speed is {}.", speed.0.total());
        assert_eq!(speed.0.total(), 30); // (10 + 5) * 2 = 30
    }
}
