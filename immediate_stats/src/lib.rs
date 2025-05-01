//! Game stats that reset every frame, inspired by immediate mode rendering.
//!
//! This makes it easy to implement temporary buffs/debuffs, and effects that change over time.
//! Using a [derive macro](macro@StatContainer), stat resets are propagated to any stat fields,
//! making it easy to compose stats into more complex or specific objects.
//!
//! ```rust no_run
//! # use immediate_stats::*;
//! #[derive(StatContainer)]
//! struct Speed(Stat);
//!
//! fn main() {
//!     loop {
//!         let mut speed = Speed(Stat::new(10)); // Set base speed to 10.
//!
//!         speed.0 *= 2.0; // Applies a multiplier to the final result.
//!         speed.0 += 5; // Adds a bonus to the final result.
//!         // The order does not matter. Bonuses are always applied before multipliers.
//!         assert_eq!(speed.0.total(), 30); // (10 + 5) * 2 = 30
//!
//!         speed.reset_modifiers(); // Reset speed back to 10.
//!     }
//! }
//! ```
//!
//! ## Bevy
//!
//! There is build-in integration with the [Bevy Engine](https://bevyengine.org)
//! via the `bevy` feature flag.
//! This adds systems for resetting [`StatContainer`] components and resources.
//!
#![cfg_attr(not(feature = "bevy"), doc = "```rust ignore")]
#![cfg_attr(feature = "bevy", doc = "```rust")]
//! # use bevy_app::prelude::*;
//! # use bevy_ecs::prelude::*;
//! # use immediate_stats::*;
//! #[derive(StatContainer, Component, Resource, Default)]
//! struct Speed(Stat);
//!
//! fn main() {
//!     App::new()
//!         .add_systems(PreUpdate, (
//!             reset_component_modifiers::<Speed>,
//!             reset_resource_modifiers::<Speed>,
//!         ))
//!         .run();
//! }
//! ```
//!
//! ### Bevy Butler
//!
//! If you use [Bevy Butler](https://github.com/TGRCdev/bevy-butler/),
//! you can also use the `bevy_butler` feature flag.
//! This automatically registers the required system(s) using the `add_component` attribute
//! or the existing `add_resource` macro.
//!
#![cfg_attr(not(feature = "bevy_butler"), doc = "```rust ignore")]
#![cfg_attr(feature = "bevy_butler", doc = "```rust")]
//! # use bevy_app::prelude::*;
//! # use bevy_ecs::prelude::*;
//! # use immediate_stats::*;
//! # use bevy_butler::*;
//! #[butler_plugin]
//! struct MyPlugin;
//!
//! // `StatContainer` derive adds the `add_component` attribute
//! // and hooks into the existing `add_resource` macro.
//! #[derive(StatContainer, Component, Resource, Default)]
//! #[add_component(plugin = MyPlugin)] // Adds `reset_component_modifiers` system.
//! #[add_resource(plugin = MyPlugin)] // Adds `reset_resource_modifiers` system.
//! struct Speed(Stat);
//! ```
//!
//! ### Version Compatibility
//! | Bevy   | Immediate Stats |
//! |--------|-----------------|
//! | `0.16` | `0.1`           |

#[cfg(feature = "bevy")]
mod bevy;
mod modifier;
mod stat;

/// Implements [`reset_modifiers`](StatContainer::reset_modifiers)
/// by propagating the call down to any stat fields.
/// ```rust
/// # use immediate_stats::*;
/// #[derive(StatContainer, Default, Debug, PartialEq)]
/// struct Health {
///     max: Stat, // `Health::reset_modifiers` calls will be passed onto `max`.
///     current: i32,
/// }
///
/// fn main() {
///     let mut health = Health {
///         max: Stat::new(10),
///         current: 10
///     };
///
///     health.max += 5;
///     health.reset_modifiers();
///     assert_eq!(health.max, Stat::new(10));
/// }
/// ```
/// # Configuration
/// By default, the macro will consider any field whose type contains the word "Stat"
/// to be a sub-stat.
/// You can use `#[stat]` to add other sub-stats and `#[stat_ignore]` to ignore one.
/// ```rust
/// # use immediate_stats::*;
/// # #[derive(StatContainer, Default, Debug, PartialEq)]
/// # struct Health {
/// #     max: Stat,
/// #     current: i32,
/// # }
/// #[derive(StatContainer)]
/// struct PartialReset {
///     #[stat]
///     custom: Health, // Will get reset.
///     #[stat_ignore]
///     ignored: Stat, // Will not get reset.
/// }
///
/// fn main () {
///     let mut partial = PartialReset {
///         custom: Health::default(),
///         ignored: Stat::default(),
///     };
///
///     partial.custom.max += 10;
///     partial.ignored += 10;
///
///     partial.reset_modifiers();
///
///     assert_eq!(partial.custom, Health::default());
///     assert_eq!(partial.ignored, Stat::default().with_bonus(10));
/// }
/// ```
/// # Bevy Butler
/// If the `bevy_butler` feature flag is enabled, you may also use the `add_component` attribute
/// or the existing `add_resource` macro to register [`reset_component_modifiers`]
/// and/or [`reset_resource_modifiers`] automatically.
#[cfg_attr(not(feature = "bevy_butler"), doc = "```rust ignore")]
#[cfg_attr(feature = "bevy_butler", doc = "```rust")]
/// # use bevy_butler::*;
/// # use bevy_ecs::prelude::*;
/// # use immediate_stats::*;
/// #[butler_plugin]
/// struct MyPlugin;
///
/// // `StatContainer` derive adds the `add_component` attribute
/// // and hooks into the existing `add_resource` macro.
/// #[derive(StatContainer, Component, Resource, Default)]
/// #[add_component(plugin = MyPlugin)] // Adds `reset_component_modifiers` system.
/// #[add_resource(plugin = MyPlugin)] // Adds `reset_resource_modifiers` system.
/// struct Speed(Stat);
/// ```
pub use immediate_stats_macros::StatContainer;
pub use modifier::*;
pub use stat::*;

#[cfg(feature = "bevy")]
pub use bevy::*;

// Used by derive macro.
#[cfg(feature = "bevy")]
#[doc(hidden)]
pub mod __internal {
    pub use bevy_app::prelude::PreUpdate;
}

/// Types that contain stats that need to be reset.
///
/// Consider using the [derive macro](macro@StatContainer) before implementing manually.
#[cfg_attr(feature = "bevy", bevy_reflect::reflect_trait)]
pub trait StatContainer {
    /// Resets all stats to a base value. For most use-cases, this should be called every frame/iteration.
    fn reset_modifiers(&mut self);
}
