//! Tests the `add_component` attribute for automatic system registration.
#![cfg(feature = "bevy_auto_plugin")]

extern crate immediate_stats;
use crate::{Stat, StatContainer};
use bevy_app::App;
use bevy_auto_plugin::modes::global::prelude::{AutoPlugin, auto_component, auto_init_resource};
use bevy_ecs::prelude::*;
use immediate_stats::*;

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct MyPlugin;

#[derive(Resource, Component, StatContainer, Default, PartialEq, Debug)]
#[auto_component(plugin = MyPlugin)]
#[auto_init_resource(plugin = MyPlugin)]
struct Health(Stat);

#[test]
fn reset_component_auto() {
    let mut app = App::new();

    app.add_plugins(MyPlugin);

    let entity = app
        .world_mut()
        .spawn(Health(Stat {
            base: 100,
            bonus: 50,
            multiplier: 2.0,
        }))
        .id();

    app.update();

    assert_eq!(
        app.world().get::<Health>(entity),
        Some(Health(Stat::new(100))).as_ref()
    );
}

#[test]
fn reset_resource_auto() {
    let mut app = App::new();

    app.add_plugins(MyPlugin);

    app.insert_resource(Health(Stat {
        base: 100,
        bonus: 50,
        multiplier: 2.0,
    }));

    app.update();

    assert_eq!(
        app.world().get_resource::<Health>(),
        Some(Health(Stat::new(100))).as_ref()
    );
}
