#![cfg(feature = "bevy")]
//! Tests the reset systems and `PauseStatReset`.

use bevy_ecs::prelude::*;
use immediate_stats::*;

#[derive(Component, Resource, StatContainer, PartialEq, Debug, Clone)]
struct Health(Stat);

#[test]
fn reset_component() {
    let mut world = World::new();
    let system = world.register_system(reset_component_modifiers::<Health>);

    let entity = world
        .spawn(Health(Stat {
            base: 100,
            bonus: 50,
            multiplier: 2.0,
        }))
        .id();

    world.run_system(system).unwrap();

    assert_eq!(
        world.get::<Health>(entity),
        Some(Health(Stat::new(100))).as_ref()
    );
}

#[test]
fn pause_component() {
    let mut world = World::new();
    let system = world.register_system(reset_component_modifiers::<Health>);

    let health = Health(Stat {
        base: 100,
        bonus: 50,
        multiplier: 2.0,
    });

    let entity = world.spawn((health.clone(), PauseStatReset)).id();

    world.run_system(system).unwrap();

    assert_eq!(world.get::<Health>(entity), Some(health).as_ref());
}

#[test]
fn reset_resource() {
    let mut world = World::new();
    let system = world.register_system(reset_resource_modifiers::<Health>);

    world.insert_resource(Health(Stat {
        base: 100,
        bonus: 50,
        multiplier: 2.0,
    }));

    world.run_system(system).unwrap();

    assert_eq!(
        world.get_resource::<Health>(),
        Some(Health(Stat::new(100))).as_ref()
    );
}
