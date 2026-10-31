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