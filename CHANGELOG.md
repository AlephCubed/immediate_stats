# Changelog

## 0.6.0 (2026-06-19)

- Updated to Bevy 0.19.
- Moved primary repository to Tangled.
- Moved changelog in-repo.

## 0.5.0 (2026-03-30)

The `bevy_auto_plugin` feature has been overhauled.

Deriving `StatContainer` will no longer add the reset plugin automatically.
Insead, build hooks are provided that can be added using `auto_plugin_build_hook`.

Before:

```rust
#[derive(StatContainer, Component)]
#[auto_component(plugin = MyPlugin)]
struct Speed(Stat);
```

After:

```rust
#[derive(StatContainer, Component)]
#[auto_plugin_build_hook(plugin = MyPlugin, hook = ResetComponentHook)]
struct Speed(Stat);
```

### Why?

The previous method had edge cases (such as not working with `auto_bind_plugin`)
and hid registration entierly. It was a hack that doesn't match any existing
system and could easily lead to invisible problems.

While the new method is more verbose,
it is also more explicit and uses tools directly provided by `bevy_auto_plugin`.

## 0.4.0 (2026-01-14)

- Updated to Bevy 0.18.

## 0.3.2 (2025-12-20)

- Added `Modifier::scaled` method.
- Implemented `Display` for `Stat` and `Modifier`.

## 0.3.1 (2025-12-18)

- Added `Stat::apply_scaled` method.

## 0.3.0 (2025-10-3)

- Updated to Bevy 0.17.
- Removed the depreciated `bevy_butler` feature flag.

## 0.2.0 (2025-09-28)

- No major changes compared to the beta.

## 0.2.0-beta.1 (2025-09-25)

- Added `bevy_auto_plugin` feature.
  - This will replace `bevy_butler`,
    which is [now depreciated](https://github.com/TGRCdev/bevy-butler/pull/35).
  - The `bevy_butler` feature flag is now depreciated,
    and will likely be removed in the next release.

## 0.1.3 (2025-09-03)

- Replace term "immediate mode rendering" with "immediate mode GUI".
- Added shields to README.

## 0.1.2 (2025-05-02)

- Fixed docs.rs not using any feature flags.

## 0.1.1 (2025-05-02)

- Initial release (for real this time).
