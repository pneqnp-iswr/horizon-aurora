# Renderer investigation — Horizon Aurora

Research snapshot: 2026-11-01. Target is the Rust Minecraft Horizon client, **not** a Java mod loader.

## Public competitive references

- [TopkaVisual](https://www.topkaproduct.com/mods/topkavisual-v2-9): public feature descriptions include trails, particles, jump circles, hit colors and view-model changes.
- [Purxiy](https://purxiy.com/ru/): promotional material describes dynamic skies, custom HUD and premium client UI.
- [Furiah Visuals](https://furiah.net/faq): references target visualizations and ambient effects.
- [Cascade](https://modrinth.com/project/cascademc): example of declarative particle emitters and sequenced effects.
- [PulseVisuals](https://github.com/Lopata12958/PulseVisuals): public code architecture illustrates separating event detection from presentation.

Claims on product pages are not independent benchmarks; we do not copy closed code, proprietary game resources or commercial assets.

## Verified upstream integration points

- [Horizon native mods](https://github.com/horizon-rust/horizon/blob/main/docs/native-mods.md): source-level Rust mixin bundles; native code has user permissions.
- [Native game hooks](https://github.com/horizon-rust/horizon/blob/main/crates/horizon-client/src/game/native_hooks.rs): API 46 adds bounded world-space colored-light and block-rim requests, without server light changes.
- [Native renderer hook](https://github.com/horizon-rust/horizon/blob/main/crates/horizon-client-render/src/gfx/native_render.rs): API 58 exposes world color, depth, camera-relative transforms and WGPU render context. Depth-tested particles and ribbons are technically plausible but not implemented or benchmarked yet.
- [Shader pack documentation](https://github.com/horizon-rust/horizon/blob/main/docs/shaders.md): experimental Shader API 0.1 implements sky, material and post stages, plus validation.

## Proposed new architecture

The VFX pipeline should be an explicit bounded graph: **event provenance → emitter → simulation → material → world/GUI placement → post-processing**. The event layer must distinguish local input, prediction, and actual confirmed game/network events. No click animation may masquerade as verified hit damage.

The WGPU backend should use preallocated pools, camera-relative transforms and explicit depth occlusion. Bloom/soft particles should have measured quality tiers, never claim free GPU work. The premium UI needs a native game screen or documented renderer extension; the self-contained browser prototype is separate. A shared versioned schema between the Studio and in-game Aurora is a future task, not an existing feature.

## Success criterion

A high-value engineering achievement would be a reusable effect graph powering HUD effects, trails, world-space ribbons and post-processing with predictable GPU/memory budgets. This is a research and product-development hypothesis, **not** a proven world-first or performance record.
