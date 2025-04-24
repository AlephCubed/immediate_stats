#![cfg(feature = "bevy_butler")]
//! Tests the `add_component` attribute for automatic system registration.

use crate::StatContainer;
use crate::stat::Stat;
use bevy_app::App;
use bevy_butler::*;
use bevy_ecs::prelude::*;
use immediate_stats::*;

#[butler_plugin]
struct MyPlugin;

#[derive(Resource, Component, StatContainer, Default, PartialEq, Debug)]
#[add_component(plugin = MyPlugin)]
#[add_resource(plugin = MyPlugin)]
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
