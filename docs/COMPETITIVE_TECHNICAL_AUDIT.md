# Competitive technical audit — 2026-11-01

Scope: documented, publicly visible capabilities of premium and free Minecraft visual mods. We have NOT purchased, reverse-engineered, decompiled or inspected any paid source or assets. Vendor performance claims are not independent measurements. We will independently implement original designs using the Horizon Rust/WGPU/WGSL stack.

## Competitors and concrete lessons

| Project | Observed/publicly advertised | Engineering lesson for Aurora |
| --- | --- | --- |
| [Purxiy](https://purxiy.com/ru/) | Eight animated skies, composited HUD, hit effects, custom renderer and UI, capes; vendor benchmark timings | Unified effects/materials/theme engine and visual hierarchy. Treat claimed GPU times only as hypotheses. |
| [TopkaVisual V2.9](https://www.topkaproduct.com/mods/topkavisual-v2-9) | Particles, jump circles, trails, hit color and animations, with a version-1.16.5 target | Source event provenance, parametric ring and streak emitters, mode presets. Does not imply native Horizon compatibility. |
| [Furiah's Visuals](https://furiah.net/features?tab=world) | 14 feature sections and many styles, sky transitions, original VFX, HUD island, GUI search, sound themes and sharable presets | One reusable VFX graph underlying dozens of looks; elegant UX and accessibility. Their public pricing page currently claims 221 styles. |
| [RainVisuals](https://rainvisuals.pro/) | Dark searchable settings, armor durability, FOV, HUD and animation customizations | Efficient modular settings and per-feature labels, bounded defaults, accessible keyboard navigation. |
| [Horizon](https://github.com/horizon-rust/horizon/blob/main/docs/shaders.md) | Shader API 0.1: WGSL sky/material/post, compile-time options, official checker; native API 58 WGPU frame/depth hooks | Build graphics for Horizon itself, not a Java/Fabric shim; validate against pinned commits and version contracts. |

## Deeper technical findings

1. Visual complexity is not identical to GPU cost. A complex-looking aurora rendered in one procedural sky stage can be cheaper than many full-screen post passes; measure the actual GPU times and reference GPU before comparing.
2. Native Horizon has explicit world-frame data in WorldDrawn (WGPU encoder, camera, scene color/depth, entity spans). This suggests depth-tested soft particles, 3D weapon trails, world-space decals and localized post effects are architecturally feasible with API 58, but there is no proven Aurora implementation of those features yet. Source: https://github.com/horizon-rust/horizon/blob/main/crates/horizon-client-render/src/gfx/native_render.rs
3. True kill or hit effect triggers need confirmed local/game events. The existing Aurora mouse-flare is an input animation only, and must never be labeled hit confirmation. Do not infer combat events from network traffic that the normal client does not expose.
4. A desirable interaction principle is strict separation: small in-game quick panel, advanced browser-inspired Studio UI, single versioned preset schema. The existing Studio v0.3 is a functional standalone prototype, now in GitHub, not a native in-game GUI.
5. For audio, aim for soft original sounds with a master level, per-event cooldown, quiet theme, mute/streamer mode, and no sudden high amplitudes. Verify a client-side Horizon audio hook before promising working sound effects; do not misuse authoritative server sound APIs for cosmetic-only sounds.
6. Marketing phrases such as undetectable, zero FPS impact, or all-server safe cannot be accepted as test results. Aurora should not include combat automation, anti-cheat evasion, X-ray or revealing occluded players.

## Reusable effect graph specification

Proposed data pipeline (not yet implemented):

EventInput -> SpawnPattern -> KinematicModel -> RendererPrimitive -> ColorCurve -> AudioCue -> Lifetime/Budget.

- EventInput: local movement / landing / verified hit or kill / explicit preview key.
- SpawnPattern: ribbon, arc, cone, ring, spiral, line, burst, volumetric cells.
- KinematicModel: deterministic seeded wind, drag, gravity, eased trajectories.
- RendererPrimitive: WGSL sky, shader post pass, instanced ribbon, soft sprite, decal, HUD quad.
- ColorCurve: normalized linear color palette, HDR clamp, envelope, hue phase and gradual fade.
- Lifetime/Budget: pooled objects, maximum visible count, distance culling, overdraw guard, GPU time gate.

Original differentiator: a fully inspectable effect graph with source provenance and measured budgets, not a copied marketplace of opaque visual modules. It should support shareable original presets and editor undo/redo without relying on third-party Java bytecode.

## Quality tiers (targets, not measurements)

| Tier | Priority | Candidate overhead target (reference GPU to be specified) |
| --- | --- | --- |
| PvP Clarity | legibility + latency, no camera wobble | <= 0.5 ms GPU extra |
| Ambient | atmospheric sky + modest particles | <= 1.5 ms GPU extra |
| Cinematic | maximum effect quality, explicitly opt-in | <= 3 ms GPU extra |

Test FPS and frametime p50/p95/p99 with and without Aurora on identical scenes/resolution/settings, including camera turns, dense entities and battle events. Compare shader pack alone vs native HUD alone vs combined. Measure allocations and GPU counters; don't promise FPS increases until benchmarks support them.

## Priority roadmap

P0: fix installer/build diagnostics on Windows, preserve cache safely, ensure reproducible CI.
P1: WGSL sky and post refinement, preview import/export, theme contrast and frame cost UX.
P2: native VFX with GPU pooled geometry and actual occlusion; verified event bridge.
P3: audio events with licensing, safe mix levels, and screenshot/regression automation.
P4: public shader/effect SDK, shareable presets, plugin source trust and a polished in-game Studio.

## Source links checked

https://purxiy.com/ru/
https://www.topkaproduct.com/mods/topkavisual-v2-9
https://furiah.net/pricing
https://furiah.net/features?tab=world
https://rainvisuals.pro/
https://github.com/horizon-rust/horizon/blob/main/docs/shaders.md
https://github.com/horizon-rust/horizon/blob/main/docs/native-mods.md
https://github.com/horizon-rust/horizon/blob/main/crates/horizon-client-render/src/gfx/native_render.rs
