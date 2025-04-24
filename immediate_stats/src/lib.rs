#![doc = include_str!("../README.md")]

#[cfg(feature = "bevy")]
pub mod bevy;
pub mod modifier;
pub mod stat;

pub use immediate_stats_macros::StatContainer;
pub use modifier::*;
pub use stat::*;

#[cfg(feature = "bevy")]
pub use bevy::*;
#[cfg(feature = "bevy")]
pub use bevy_app::prelude::PreUpdate;

/// Types that contain stats that need to be reset.
///
/// It is recommended to use the [derive macro](macro) instead of implementing manually.
///
/// [macro]: immediate_stats_macros::StatContainer
#[cfg_attr(feature = "bevy", bevy_reflect::reflect_trait)]
pub trait StatContainer {
    /// Resets all stat bonuses to zero, and stat multipliers to one.
    fn reset_modifiers(&mut self);
}
