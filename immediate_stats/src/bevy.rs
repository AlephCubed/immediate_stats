use crate::StatContainer;
use bevy_app::{App, Plugin, PreUpdate};
use bevy_ecs::prelude::Component;
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
    use crate::stat::Stat;
    use bevy_ecs::prelude::World;
    use immediate_stats_macros::StatContainer;

    #[derive(Component, StatContainer, PartialEq, Debug)]
    struct Health(Stat);

    #[test]
    fn plugin() {
        let mut world = World::new();
        let system = world.register_system(reset_modifiers::<Health>);

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
}
