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

### Bevy Butler

If you use [Bevy Butler](https://github.com/TGRCdev/bevy-butler/), you can also use the `bevy_butler` feature flag.
This automatically registers the required system(s) using the `add_component` attribute
or the existing `insert_resource` macro.

```rust
fn main() {
    App::new().add_plugins((ImmediateStatsPlugin, MyPlugin)).run();
}

#[butler_plugin]
struct MyPlugin;

// `StatContainer` derive adds the `add_component` attribute 
// and hooks into the existing `insert_resource` macro.
#[derive(StatContainer, Component, Resource)]
#[add_component(plugin = MyPlugin)] // Adds `reset_component_modifiers` system.
#[insert_resource(plugin = MyPlugin)] // Adds `reset_resource_modifiers` system.
struct Speed(Stat);
```

### Version Compatibility
| Bevy   | Immediate Stats |
|--------|-----------------|
| `0.16` | `0.1`           |