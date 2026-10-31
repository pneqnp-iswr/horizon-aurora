# Open-source Java visuals → Horizon Rust research (2026-11-01)

## Verified repository

- [PulseVisual](https://github.com/creeleystromberg-sketch/PulseVisual) — **MIT**, Java Fabric 1.21.11. Code examined: `render/TrailRenderer.java`, `effect/HitEffectManager.java`, `animation/VisualSwingClock.java`.
- [VisualClient 1.21.4](https://github.com/Lopata12958/VisualClient-1.21.4) — public MIT-licensed integrated HUD/visual concept; verify provenance of each imported part before copying.

## What we learned

1. Trails use bounded position history, distance-based addition, age fading, and geometry quads. Horizon's existing `LightTrail` already bounds local points but lights produce **illumination only**, not visible ribbons.
2. HitEffectManager uses parametric circles and burst distributions driven by **real attack events**. Aurora does not have a verified local confirmed-hit hook, so its new rings are transparently driven by **mouse input**, not hits.
3. Swing animations use a finite event clock and easing curve; an independent Rust animator could be applied when Horizon exposes an appropriate render-stage hook.
4. To port a Java visual, map math/data/algorithms to Rust state + WGPU, not Java-specific OpenGL BufferBuilder or Fabric events. Directly executing a JAR inside Horizon is not supported.

## Implementation shipped in 0.5.0 candidate

The original Rust `ParticlePath` implementation uses Horizon's client-only `native_flame_particle` API for actual depth-tested particles. It is independent code, not a translation of PulseVisual source and does not reuse proprietary artwork. No external code is vendored, so attribution remains research documentation rather than copied MIT source distribution.

## Deferred work

A genuinely custom rainbow/ribbon particle renderer requires `native_world_drawn` and/or a new world particle draw interface (API 58) with a WGPU pipeline, depth test, material buffer and proper cross-platform shader handling. Prototype separately with pixel/GPU budgets, then integrate. Avoid claiming this 0.5 step implements custom-colored 3D trails or hit particles.
