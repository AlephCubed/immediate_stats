use super::*;
use crate::StatContainer;
use crate::stat::Stat;
use bevy_butler::*;
use bevy_ecs::prelude::*;
use bevy_reflect::prelude::*;
use immediate_stats_macros::StatContainer;

#[butler_plugin]
struct MyPlugin;

#[derive(Reflect, Resource, StatContainer, Default)]
#[resource(plugin = MyPlugin)]
struct Health(Stat);
