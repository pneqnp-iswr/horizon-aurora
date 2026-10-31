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