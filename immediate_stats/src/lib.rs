#![doc = include_str!("../README.md")]

#[cfg(feature = "bevy")]
pub mod bevy;
pub mod modifier;
pub mod stat;

/// Implements [`reset_modifiers`](StatContainer::reset_modifiers)
/// by passing on the call to any sub-stats.
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
/// By default, it will consider any value of type `Stat` to be a sub-stat.
/// You can use `#[stat]` to add a sub-stat and `#[stat_ignore]` to ignore one.
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
///     custom: Health,
///     #[stat_ignore]
///     ignored: Stat,
/// }
///
/// fn main () {
///     let mut partial = PartialReset {
///         custom: Health::default(),
///         ignored: Stat::new(1),
///     };
///
///     partial.custom.max += 10;
///     partial.ignored += 10;
///
///     partial.reset_modifiers();
///
///     assert_eq!(partial.custom, Health::default());
///     assert_eq!(partial.ignored, Stat::new(1).with_bonus(10));
/// }
/// ```
pub use immediate_stats_macros::StatContainer;
pub use modifier::*;
pub use stat::*;

#[cfg(feature = "bevy")]
pub use bevy::*;
#[cfg(feature = "bevy")] // Used by derive macro.
#[doc(hidden)]
pub use bevy_app::prelude::PreUpdate;

/// Types that contain stats that need to be reset.
///
/// It is recommended to use the [derive macro](macro@StatContainer)
/// instead of implementing manually.
#[cfg_attr(feature = "bevy", bevy_reflect::reflect_trait)]
pub trait StatContainer {
    /// Resets all stat bonuses to zero, and stat multipliers to one.
    fn reset_modifiers(&mut self);
}
