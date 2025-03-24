use crate::StatContainer;
use bevy_app::{App, Plugin, PreUpdate};
use bevy_ecs::prelude::*;
use bevy_ecs::query::Without;
use bevy_ecs::system::Query;
use std::marker::PhantomData;

/// [Resets](StatContainer::reset_modifiers) all stat modifiers for component `T` in [`PreUpdate`].
#[derive(Default)]
pub struct ImmediateStatsPlugin<T: Component + StatContainer> {
    phantom_data: PhantomData<T>,
}

/// Prevents all [`StatContainers`](StatContainer) from resetting.
#[derive(Component, Default)]
#[component(storage = "SparseSet")]
pub struct PauseStatReset;

impl<T: Component + StatContainer> Plugin for ImmediateStatsPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, reset_modifiers::<T>);
    }
}

fn reset_modifiers<T: Component + StatContainer>(
    mut query: Query<&mut T, Without<PauseStatReset>>,
) {
    for mut stat in &mut query {
        stat.reset_modifiers();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use immediate_stats_macros::StatContainer;

    #[derive(Component, StatContainer, PartialEq, Debug)]
    struct Damage {
        #[base(health)]
        damage_base: i32,
        #[bonus(health)]
        damage_bonus: i32,
        #[multiplier(health)]
        damage_multiplier: f32,
    }

    #[test]
    fn plugin() {
        let mut world = World::new();
        let system = world.register_system(reset_modifiers::<Damage>);

        let entity = world
            .spawn(Damage {
                damage_base: 100,
                damage_bonus: 50,
                damage_multiplier: 2.0,
            })
            .id();

        world.run_system(system).unwrap();

        assert_eq!(
            world.get::<Damage>(entity),
            Some(Damage {
                damage_base: 100,
                damage_bonus: 0,
                damage_multiplier: 1.0,
            })
            .as_ref()
        );
    }
}
