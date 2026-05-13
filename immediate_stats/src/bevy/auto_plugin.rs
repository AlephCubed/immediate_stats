#![cfg(feature = "bevy_auto_plugin")]
//! Contains a hook for resetting [`StatContainer`]s using Bevy Auto Plugin.

use crate::{ResetComponentPlugin, ResetResourcePlugin, StatContainer};
use bevy_app::App;
use bevy_auto_plugin::prelude::AutoPluginBuildHook;
use bevy_ecs::component::Mutable;
use bevy_ecs::prelude::{Component, Resource};

/// A Bevy Auto Plugin hook that adds the [`ResetComponentPlugin`] for the component.
pub struct ResetComponentHook;

impl<T> AutoPluginBuildHook<T> for ResetComponentHook
where
    T: Component<Mutability = Mutable> + StatContainer,
{
    fn on_build(&self, app: &mut App) {
        app.add_plugins(ResetComponentPlugin::<T>::new());
    }
}

/// A Bevy Auto Plugin hook that adds the [`ResetResourcePlugin`] for the resource.
pub struct ResetResourceHook;

impl<T> AutoPluginBuildHook<T> for ResetResourceHook
where
    T: Resource<Mutability = Mutable> + StatContainer,
{
    fn on_build(&self, app: &mut App) {
        app.add_plugins(ResetResourcePlugin::<T>::new());
    }
}
