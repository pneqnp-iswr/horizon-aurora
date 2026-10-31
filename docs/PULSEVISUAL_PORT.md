# PulseVisual 2.2.1 → Aurora 0.6 port matrix

Public upstream: https://github.com/creeleystromberg-sketch/PulseVisual (MIT, © 2026 PulseVisual contributors). Examined Java files: `render/TrailRenderer.java`, `render/PearlTracer.java`, `render/ChinaHatRenderer.java`, `animation/VisualSwingClock.java`, `animation/SwingAnimator.java`, `effect/HitEffectManager.java`, `config/VisualConfig.java`. See [license notice](../THIRD_PARTY_NOTICES.md).

PulseVisual targets Fabric/Minecraft Java **1.21.11**. Horizon Aurora targets native Rust Horizon **26.3**. None of the Java Fabric/Blaze3D mixins or bytecode are shipped, because they cannot execute under Horizon.

| PulseVisual feature | Aurora 0.6 code | Real parity? |
|---|---|---|
| Hit effects: Burst, Ring, Upward | `burst ring|burst|spiral` via `native_flame_particle` | **Partial:** physical input animation, NOT a server-validated hit. Particle type limited to native flame/soul |
| Movement trails Line/Ribbon/Glow/Rainbow/Pulse | `path off|line|twin|helix` uses local path sampling, 3D flame sprites, quality caps | **Partial:** no WGPU ribbons/meshes, dynamic alpha gradients or custom sprites |
| Swing / clock | Finite cubic smoothstep envelope to animate input indicator | **Partial:** no held-hand transform, swing speed remains vanilla |
| China Hat | `crown on|off` emits sparse 3D particles near local head | **Partial:** not filled 3D mesh, no gradient cones |
| Pearl Tracer | Not implemented | Needs verified owner-identified local projectile data and a 3D renderer |
| ViewModel hand transformations | Not implemented | Needs first-person hand-render hooks |
| Fire overlay customization | Not implemented | Needs client overlay render surface; game fire/damage must remain unchanged |
| HUD and crosshair | Aurora HUD has FPS/CPS/WASD/graph/input visual | Partial: custom in-game crosshair renderer not verified |
| UI theme, menu Glass/Minimal | Settings + separate interactive HTML Studio | Partial: browser Studio is **not** in-game ClickGUI |
| Auto-updates | GitHub-native .hnmod release installation | Updater logic belongs to official launcher, no self-modifying JAR |

## Technical choices and meaningful differences

The current code is original Rust and uses standard Catmull-Rom cubic interpolation (cf. PulseVisual's `TrailRenderer`) and bounded finite smoothstep animation envelopes (cf. `VisualSwingClock`). Positions are reset on dimension changes and >8-block per-tick discontinuities; emissions are bounded to 10 tick-local particles, with automatic downgrade at low FPS. A click is not a confirmed attack. No packets, game state, anti-cheat behavior or targeting are modified.

Cubic, interpolation and effect-configuration ideas were studied in the MIT source; no third-party shaders, code files, textures, sounds or compiled JAR are vendored. Do not claim port parity in releases until rendered, tested on both Windows and Linux, and benchmarked.

## Next engineering milestone: full graphics compatibility

Upgrade the Horizon pin to verified native render API 58 and implement world-relative WGPU ribbon quads, depth-faded soft sprites, smooth GPU buffers, per-instance color and lifetime without excessive draw calls. A suitable renderer works **without** the Aurora Nebula brightening shader. Design input source and ownership explicitly for pearl trail; add a separate true in-game menu and original, pleasant sound presets.

Track under: https://github.com/pneqnp-iswr/horizon-aurora/issues/3.
