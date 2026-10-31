# Horizon Aurora roadmap

This is a staged **engineering** plan, not a fixed release promise.

## P0 — reproducible foundation

- Complete source import; pin the upstream Horizon and horizon-mixins revisions.
- Build and test Rust modules in CI; run official WGSL validation and native client integration compilation.
- Preserve historical prototype source snapshots, reproducible SHA-256 hashes and release notes.

## P1 — exceptional, efficient visuals

- Reusable VFX component graph: event provenance -> emitter -> simulation -> material -> composite.
- Native Shader API sky presets and bounded post-effects, including quality tiers.
- API 58 world-depth-aware trails and particles when confirmed compatible.
- Benchmarks for frame-time percentiles, overdraw, allocation and GPU cost; keep fair-play controls explicit.

## P2 — excellent UX

- Evaluate a native in-game settings surface, not merely an external browser mock.
- Versioned effect/preset schema with safe JSON import, undo/redo, portability and migration tests.
- Keyboard navigation, localization in English and Russian, accessibility and reduced-motion support.

## P3 — production discipline

- Cross-OS and cross-GPU compatibility tests, honest release gates, security review.
- Signed artifacts if distributable, user-facing documentation, regression tracking and reproducible benchmarks.

A meaningful engineering breakthrough would be a common, bounded VFX graph driving 2D HUD effects, depth-aware 3D ribbons, and post-processing without one renderer fork per effect. This has not yet been demonstrated.
