# Project status — 2026-11-01

The v0.3.0 code has been produced in the current Aurora development session. This branch is a **repository bootstrap** and is not yet a complete, buildable code import. Do not mistake a draft PR, an archived source bundle, or offline metadata checks for a tested native Horizon release.

## Baseline

- Candidate components: `Aurora Visuals 0.3.0` (Rust native mod), `Aurora Nebula 0.3.0` (WGSL Shader API 0.1 candidate), and a browser-only Aurora Studio.
- Target: Horizon on Minecraft 26.3; native hook API 46 or later.
- Tested here: Python static manifest/packing checks.
- Not established: Rust compilation, Horizon full-client compilation, Naga shader validation, game startup, PvP behavior, latency or GPU benchmarks.

## Integration gates

1. Import every source file from the v0.3.0 snapshot with hashes.
2. Run Rust unit tests, `horizon-shader check` and a full Horizon compile in GitHub Actions.
3. Test on a real Windows Horizon instance, including keyboard remapping, streamer privacy defaults and world transitions.
4. Compare frame times using repeatable A/B test sessions.
5. Only create a tagged release when the evidence matches the claim.

## Truthfulness of effects

Input pulses only prove physical clicks. They do **not** prove that the server accepted a hit, dealt damage or identified a target. Browser Studio is a separate prototype and does not imply the game has a premium ClickGUI.

Keep upstream code, proprietary assets, third-party shaders, and commercial mod contents out of this repository unless their licenses explicitly permit inclusion.


## 2026-11-01: Windows build recovery / v0.4.0 candidate

- User screenshot confirms disk exhaustion (os error 112), with additional compiler stack/process errors to recheck after freeing space.
- Added read-only native-build preflight; deletion of intermediate files requires explicit opt-in.
- Added procedural starfield, committed browser Studio, Windows CI smoke tests and researched VFX roadmap. Gameplay and GPU timings remain unverified.

## 2026-11-01 — Aurora 0.7.0 true WGPU vertical slice

The pinned Horizon commit `97ded175738a56163e40698be72e968d9b956f73` contains the render API 58 hooks despite Aurora 0.6 declaring only minimum API 46. New modules: `ribbon_gpu.rs` in the renderer crate, `gpu_needs.rs` and `gpu_world.rs` HEAD injection helpers. The gameplay/render bridge transports only position, dimension hash, style, enabled/demo bits and sequence. Render thread owns the trail history and WGPU pipeline/buffers.

In the fragment stage, native world depth uses reversed Z (0=sky); world and depth sizes may differ in dynamic resolution. Ribbon world positions are converted to camera-relative f32 after subtracting camera in f64. Renderer has hard upper bounds (72 trail points and 8192 vertices); disable clears render-thread GPU caches at the next applicable render callback. Frame-size changes use updated uniforms; format/device changes recreate the WGPU pipeline. A device loss while not in-world needs follow-up runtime checks.

**Evidence classification:** implemented=source present, compiled=only after successful CI, game-tested=no, visually-verified=no, benchmarked=no, native UI=not implemented, browser Studio=prototype-only. Do not publish a stable release from this alone.

### Local manual acceptance

1. On Windows free >=8-10 GiB on Horizon's build volume; launch native mod in trusted isolated instance.
2. Verify `/aurora ribbon on`, `/aurora ribbon-demo on`, `/aurora ribbon-style silk` in third-person in a daytime singleplayer world **without Aurora Nebula**.
3. Compare Prismatic and Spectral, then `/aurora ribbon-demo off` and walk: player trail should fade.
4. Move behind solid walls to verify occlusion, switch dimension, resize window, re-enable.
5. Capture actual frame via Horizon (not browser HTML), inspect official F3 GPU measurements on same scene with/without mod.
6. Repeat at 1080p/1440p; track p50/p95/p99 total and GPU frametime, 1% lows, and VRAM.
