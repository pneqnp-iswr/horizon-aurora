# Aurora architecture (vertical slice)

## Scope implemented in this session

A minimal native-mod-compatible vertical slice centered on a PvP-readable HUD card:

- Theme system with five presets.
- Local CPS/combo telemetry using exposed input hook events.
- Animated flash intensity with reduced-motion fallback.
- Native mod registration + settings button integration.
- Persistent configuration (`HORIZON_NATIVE_DATA_DIR/aurora/config.json`).

## Components

1. `mixins/aurora/runtime.rs`
   - Pure Rust runtime core, serializable config, state machine, animation tick, tests.
2. `mixins/aurora/horizon-mixin.toml`
   - Hook wiring against named contracts (`hud`, `key`, `mods`, `settings`, `settings_click`).
3. Helpers (`hud.rs`, `key.rs`, `mods.rs`, `settings.rs`, `settings_click.rs`)
   - Thin Horizon-specific adapters to runtime core.
4. `compatibility.json`
   - Exact Horizon/Horizon-Mixins revision contract and source hashes.

## Reliability and safety choices

- No unsafe Rust.
- No network activity.
- No filesystem access beyond explicit config path under native data dir.
- Saturating counters and clamped time deltas to avoid runaway values.
- Settings persistence only on explicit user changes.
