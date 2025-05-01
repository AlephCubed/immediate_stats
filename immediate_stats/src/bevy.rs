#![cfg(feature = "bevy")]
//! Contains systems and components for resetting [`StatContainer`]s in the Bevy game engine.

use crate::StatContainer;
use crate::modifier::Modifier;
use crate::stat::Stat;
use bevy_app::{App, Plugin, PreUpdate, Update};
use bevy_ecs::component::Mutable;
use bevy_ecs::prelude::{Component, Query, ResMut, Resource, Without};
use bevy_ecs::prelude::{IntoScheduleConfigs, ReflectComponent, SystemSet};
use bevy_reflect::Reflect;
use bevy_reflect::prelude::ReflectDefault;
use std::marker::PhantomData;

/// Configures [system ordering](StatSystems) and registers types with the Bevy type registry.
///
/// - [`StatSystems::Reset`] runs in `PreUpdate`.
/// - [`StatSystems::Modify`] runs before [`StatSystems::Read`] in `Update`.
pub struct ImmediateStatsPlugin;

impl Plugin for ImmediateStatsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<PauseStatReset>()
            .register_type::<Stat>()
            .register_type::<Modifier>()
            .configure_sets(Update, StatSystems::Modify.before(StatSystems::Read))
            .configure_sets(PreUpdate, StatSystems::Reset);
    }
}

/// A [`SystemSet`] for ordering Immediate Stats operations.
/// Recommend configuration can be added via the [`ImmediateStatsPlugin`].
#[derive(SystemSet, Debug, Clone, Eq, PartialEq, Hash)]
pub enum StatSystems {
    /// Systems that reset [`StatContainers`](StatContainer).
    Reset,
    /// Systems that apply modifiers to stats.
    Modify,
    /// Systems that read the final value of stats.
    Read,
}

/// Prevents any [`StatContainers`](StatContainer) on an entity from resetting.
#[derive(Component, Reflect, Eq, PartialEq, Debug, Default, Clone)]
#[component(storage = "SparseSet")]
#[reflect(Component, PartialEq, Debug, Default, Clone)]
pub struct PauseStatReset;

/// Calls [`reset_modifiers`](StatContainer::reset_modifiers) on all `T` components.
///
/// Reset occurs in the [`Reset`](StatSystems::Reset) system set during [`PreUpdate`].
pub struct ResetComponentPlugin<T: Component<Mutability = Mutable> + StatContainer> {
    _phantom: PhantomData<T>,
}

impl<T: Component<Mutability = Mutable> + StatContainer> Plugin for ResetComponentPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            reset_component_modifiers::<T>.in_set(StatSystems::Reset),
        );
    }
}

/// Calls [`reset_modifiers`](StatContainer::reset_modifiers) on all `T` components.
///
/// Use the [`ResetResourcePlugin`] for recommended configuration.
pub fn reset_component_modifiers<T: Component<Mutability = Mutable> + StatContainer>(
    mut query: Query<&mut T, Without<PauseStatReset>>,
) {
    for mut stat in &mut query {
        stat.reset_modifiers();
    }
}

impl<T: Component<Mutability = Mutable> + StatContainer> ResetComponentPlugin<T> {
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T: Component<Mutability = Mutable> + StatContainer> Default for ResetComponentPlugin<T> {
    fn default() -> Self {
        Self {
            _phantom: PhantomData::default(),
        }
    }
}

/// Calls [`reset_modifiers`](StatContainer::reset_modifiers) on the `T` resource, if it exists.
///
/// Reset occurs in the [`Reset`](StatSystems::Reset) system set during [`PreUpdate`].
pub struct ResetResourcePlugin<T: Resource + StatContainer> {
    _phantom: PhantomData<T>,
}

impl<T: Resource + StatContainer> Plugin for ResetResourcePlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            reset_resource_modifiers::<T>.in_set(StatSystems::Reset),
        );
    }
}

/// Calls [`reset_modifiers`](StatContainer::reset_modifiers) on the `T` resource, if it exists.
///
/// Use the [`ResetResourcePlugin`] for recommended configuration.
pub fn reset_resource_modifiers<T: Resource + StatContainer>(res: Option<ResMut<T>>) {
    if let Some(mut res) = res {
        res.reset_modifiers();
    }
}

impl<T: Resource + StatContainer> ResetResourcePlugin<T> {
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T: Resource + StatContainer> Default for ResetResourcePlugin<T> {
    fn default() -> Self {
        Self {
            _phantom: PhantomData::default(),
        }
    }
}
