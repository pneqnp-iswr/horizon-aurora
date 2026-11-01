# Unreleased — ribbon geometry and lifecycle integration candidate (PR #5)

- WGPU Ribbon: shared endpoint frames across adjacent triangles, camera-facing sides with fallback, centripetal spline, adaptive 4–7 subdivisions per movement sample, clamped local overshoot.
- Individual point-age fades with smooth entrance/exit and WGSL lateral alpha edges. Demo geometry retains a continuous time origin while client snapshots are temporarily stale.
- Optional `AURORA_RIBBON_DIAG=1` life-cycle logs (client/server role, player available, publish sequence, animation epoch, dimension and GPU reset reasons). Diagnostics are off by default and transition-only.
- Added native unit and offline Python regressions; see `docs/RIBBON_WINDOWS_ACCEPTANCE.md` for Windows singleplayer/multiplayer validation.
- Status: candidate only; Linux native integration CI and real gameplay/benchmark evidence are required before promotion. No FPS claim.

# 0.7.1-rc.1 — singleplayer ribbon stability hotfix

- Fixed integrated-server native_tick clearing the process-shared GPU ribbon snapshot in singleplayer. Non-window/server game ticks now return without mutating client-owned GPU state.
- Client-side mod disable, player absence and world transition teardown remain in place.
- Added a regression that enforces the client/server guard order and verifies that only the client can request aurora_gpu::inactive().
- Bumped mod and shader package versions to release separately from immutable 0.7.0.
- User-provided gameplay screenshots show ribbon geometry in multiplayer and intermittent reset in singleplayer. The source-level cause is consistent with those observations; the fix still needs on-device singleplayer retesting.
- No new graphics shaders or measured FPS claims.
# 0.7.0-rc.1 — experimental true WGPU Ribbon Renderer

- Added real geometry renderer to Horizon's API 58 `native_world_drawn` and `native_render_needs` hooks.
- Added three actual triangle-mesh styles: Silk Ribbon, Prismatic Flow and Spectral Helix.
- Player-position snapshots cross from the game crate into the renderer crate; no client packets.
- Reversed-Z scene depth sampled in fragment shader, with scaled frame/depth mapping.
- Camera-relative f64->f32 conversion, finite sample and vertex caps, per-device WGPU resource cache and adaptive vertex-buffer growth.
- Native settings and `/aurora ribbon`, `ribbon-style`, `ribbon-demo` commands. Default OFF for safety.
- Static regression tests for module legality, API58, render pipeline and commands.
- **No game-run or FPS proof yet.** WGPU code must pass the official full Horizon compile before prerelease is published.

# 0.6.0-rc.1 — PulseVisual-inspired original Rust cosmetics

- Three input bursts: ring, layered burst, rising spiral.
- Four movement patterns: off, spline-smoothed line, twin rails, dual helix.
- Optional head-height particle crown (not yet a 3D China Hat mesh).
- Smoothstep finite input ornament, blue/orange mixed native flame sprites.
- Adaptive 2–10 particles/tick budget; safe world/teleport reset, reduced-motion support.
- MIT PulseVisual research/feature parity matrix and explicit notices.
- Original source compiled only after official Horizon native integration CI; Windows runtime visuals and FPS remain unmeasured.

# 0.5.0-rc.1 — 2026-11-01

- Implemented actual world-space native flame/soul-flame particles for movement and input rings.
- Added procedural nighttime meteor streaks to the Aurora Nebula sky (enabled by default, intensity adjustable). Default ON; bounded per-tick; teleport/dimension resets; Reduced Motion supported; client-only.
- Added `/aurora particles on|off` and `/aurora fire soul|flame`, two settings buttons and pure Rust regression tests.
- Evaluated MIT-licensed PulseVisual Java visual routines for algorithms; original independent Horizon implementation uses the official native particle hook.
- No release as stable without CI and real Windows gameplay measurements.

# Changelog

## 0.4.0 RC1 — 2026-11-01

- Added optional procedural night-sky starfield (WGSL, six shader options).
- Committed the interactive Aurora Studio browser prototype.
- Added Windows disk/log preflight with explicit safe-cache cleanup and Windows CI smoke tests.
- Documented resource exhaustion and a source-backed premium visual-mod research plan.
- Linux official native compile and WGSL checks gate pre-release; Windows gameplay, audio and frame timing still require direct tests.

# Changelog

## 0.2.0 — experimental source candidate (2026-11-01)

- Added five theme palettes; Rose and Mint join Violet, Cyan and Amber.
- Added 32-sample ring-buffer FPS histogram (5-tick sampling).
- Added lightweight input feedback bars (never claims a confirmed PvP hit).
- Added opt-in client-only colored ambient light via API 46 `native_visuals`, with bounded breathing and reduced-motion fallback.
- Added compact, reduced-motion, ambient, graph and feedback settings.
- Added public `/aurora ...` controls for new switches and a native-command discovery hook (API 35).
- Fixed movement estimator on dimension changes; ignored large teleport jumps.
- Changed Streamer preset to hide coordinates by default; automatically migrate legacy v0.1 Streamer settings that exposed coordinates.
- Separated HUD geometry into testable pure shape composition.
- Added runtime test cases, metadata/packaging/CI scaffolding and a zero-dependency standalone HTML style preview.
- Updated pin to Horizon `97ded175` / horizon-mixins `a2af5c0d` and hook requirement 46.
- Not yet compiled with full Horizon or live-game tested.

## 0.1.0 — initial experimental source MVP

- Native HUD, three themes, three presets, basic metrics, settings and commands.