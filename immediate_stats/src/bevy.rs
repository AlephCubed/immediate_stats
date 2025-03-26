#[cfg(feature = "bevy_butler")]
mod butler;

use crate::StatContainer;
use bevy_ecs::prelude::{Component, ResMut};
use bevy_ecs::query::Without;
use bevy_ecs::system::{Query, Resource};

/// Prevents all [`StatContainers`](StatContainer) from resetting.
#[derive(Component, Default)]
#[component(storage = "SparseSet")]
pub struct PauseStatReset;

pub fn reset_component_modifiers<T: Component + StatContainer>(
    mut query: Query<&mut T, Without<PauseStatReset>>,
) {
    for mut stat in &mut query {
        stat.reset_modifiers();
    }
}

pub fn reset_resource_modifiers<T: Resource + StatContainer>(res: Option<ResMut<T>>) {
    if let Some(mut res) = res {
        res.reset_modifiers();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stat::Stat;
    use bevy_ecs::prelude::World;
    use immediate_stats_macros::StatContainer;

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
}
