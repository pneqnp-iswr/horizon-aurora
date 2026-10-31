# Roadmap

## Phase 1 (done in this slice)

- [x] Establish pinned native-mod compatibility contract.
- [x] Add core runtime config/state model with tests.
- [x] Implement first integrated feature: animated HUD + theme switching + local PvP telemetry.

## Phase 2 (next milestone)

- [ ] Add staged build validation flow (`tools/horizon.py` style) for true in-engine checks.
- [ ] Add native panel/search-driven settings UI and key rebinding surfaces.
- [ ] Persist and expose profile presets (Competitive/Cinematic/Minimal) via settings.
- [ ] Add localization table structure for all user-facing strings.

## Phase 3

- [ ] Move to a newer Horizon revision with API 58 renderer hooks.
- [ ] Add renderer-backed visual effects guarded by quality tiers.
- [ ] Add performance instrumentation with frame budget measurements.

## Phase 4

- [ ] CI for staged checks, package bundle reproducibly, publish install/validation guide.
- [ ] Capture real in-game screenshots/video and benchmark evidence.
