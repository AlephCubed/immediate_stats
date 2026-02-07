# Immediate Stats

[![Version](https://img.shields.io/crates/v/immediate_stats)](https://crates.io/crates/immediate_stats)
[![Docs](https://img.shields.io/docsrs/immediate_stats)](https://docs.rs/immediate_stats)
![License](https://img.shields.io/crates/l/immediate_stats)

Game stats that reset every frame, inspired by immediate mode GUI.

This makes it easy to implement temporary buffs/debuffs, and effects that change over time.
Using a derive macro, stat resets are propagated to any stat fields, 
making it easy to compose stats into more complex objects.

```rust
#[derive(StatContainer)]
struct Speed(Stat);

fn main() {
    let mut speed = Speed(Stat::new(10)); // Set base speed to 10.

    loop {
        speed.0 *= 2.0; // Applies a multiplier to the final result.
        speed.0 += 5; // Adds a bonus to the final result.
        // The order does not matter, bonuses are always applied before multipliers.
        assert_eq!(speed.0.total(), 30); // (10 + 5) * 2 = 30
        
        speed.reset_modifiers(); // Reset bonus and multiplier, so speed is back to 10.
    }
}
```

## Bevy

There is build-in integration with the [Bevy Engine](https://bevyengine.org) via the `bevy` feature flag.
This adds plugins for resetting `StatContainer` components and resources.

```rust
#[derive(StatContainer, Component, Resource)]
struct Speed(Stat);

fn main() {
    App::new()
        .add_plugins((
            ImmediateStatsPlugin,
            ResetComponentPlugin::<Speed>::new(),
            ResetResourcePlugin::<Speed>::new(),
        ))
        .run();
}
```

### Bevy Auto Plugin

If you use [Bevy Auto Plugin](https://github.com/strikeforcezero/bevy_auto_plugin/), you can also use the `bevy_auto_plugin` feature flag.
This adds build hooks for that automatically add the reset plugin.

```rust
fn main() {
    App::new().add_plugins((ImmediateStatsPlugin, MyPlugin)).run();
}

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct MyPlugin;

#[derive(StatContainer, Component)]
// Use hook to add the `ResetComponentPlugin` to `MyPlugin` automatically.
#[auto_plugin_build_hook(plugin = MyPlugin, hook = ResetComponentHook)]
struct Speed(Stat);
```

### Version Compatibility
| Bevy   | Immediate Stats |
|--------|-----------------|
| `0.18` | `0.4`           |
| `0.17` | `0.3`           |
| `0.16` | `0.1` - `0.2`   |