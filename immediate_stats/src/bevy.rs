use crate::StatContainer;
use crate::modifier::Modifier;
use crate::stat::Stat;
use bevy_app::{App, Plugin};
use bevy_ecs::component::Mutable;
use bevy_ecs::prelude::ReflectComponent;
use bevy_ecs::prelude::{Component, Query, ResMut, Resource, Without};
use bevy_reflect::Reflect;
use bevy_reflect::prelude::ReflectDefault;

pub struct ImmediateStatesPlugin;

impl Plugin for ImmediateStatesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<PauseStatReset>()
            .register_type::<Stat>()
            .register_type::<Modifier>();
    }
}

/// Prevents a [`StatContainers`](StatContainer) from resetting.
#[derive(Component, Reflect, Eq, PartialEq, Debug, Default, Clone)]
#[component(storage = "SparseSet")]
#[reflect(Component, PartialEq, Debug, Default, Clone)]
pub struct PauseStatReset;

pub fn reset_component_modifiers<T: Component<Mutability = Mutable> + StatContainer>(
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
