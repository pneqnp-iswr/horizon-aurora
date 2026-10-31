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
