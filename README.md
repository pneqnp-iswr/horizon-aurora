> **What is new in v0.6.0:** Actual client-only 3D flame particles using Horizon's native particle API, not just player glow. Blue soul-flame ring on each local mouse click and a bounded 2-particle footstep trail while moving. No game packets, no hit-detection claims. Enabled by default; use `/aurora particles off` or `/aurora fire flame` to switch colors. Visible in the world, separate from the Aurora Nebula shader pack. Full Linux native compile / in-game performance verification is tracked by GitHub Actions.

# Horizon Aurora — experimental native visuals for Horizon (v0.6.0)

The development home is **https://github.com/pneqnp-iswr/horizon-aurora**. Changes should be proposed through pull requests and recorded in the changelog.

A **community experimental Rust native mod** for Horizon, targeting Minecraft 26.3. The `.hnmod` is **readable source packed for the Horizon launcher**, not a precompiled binary or a Java/Fabric/Forge mod.

> **Status (1 November 2026):** v0.3.0 RC1 passed full official Horizon Linux native compile and WGSL validation. v0.4.0 adds a procedural starfield, committed Studio demo and Windows build diagnostics. Actual Windows gameplay and performance are still unverified.

## What's available in 0.4.0

- A distinct Aurora HUD with accent stripe, layered rows, dynamically measured font widths, compact mode and corner stacking.
- Seven themes: **violet / cyan / amber / rose / mint / electric / scarlet**.
- Three presets: **combat / minimal / streamer**, with streamer location display **off by default** (including safe migration of old settings).
- FPS and 1%-low, LMB/RMB CPS, remap-aware WASD indicators, estimated blocks/second, optional player coordinates.
- A bounded, rolling 32-sample FPS histogram (sampled every five game ticks; ~8 seconds at 20 TPS).
- Mouse input flash bars for primary/secondary physical mouse-button presses. **These indicate input, not damage or server-confirmed hits**. The animation degrades to an immediate on/off state in Reduced Motion mode.
- Optional client-only, theme-colored gently breathing ambient light around the local player, through Horizon's API 46 `native_visuals` hook. It does not change block light or server state. **Disabled by default**.
- Four configurable panel positions; 17 options; persisted settings; rebindable `O` toggle; `/aurora` commands.
- Bounded drawing complexity (7 rows + 32 bars), and world-transition-aware motion estimates.

This is **not** yet a shaderpack, 3D effects studio, KillAura, opponent ESP, aim assist, packet interceptor, confirmed-hit effect, or a replay recorder. Target information and server-dependent hit events are deferred until a safe verified client API is available.

## Interactive Aurora Studio (no game build required)

Open `studio/aurora-studio.html` in a modern browser. Tested with Chromium in both
1440px desktop and 390px mobile viewports. Includes procedural sky, simulated
particle VFX, searchable modules, quick presets, theme palette, intensity/glow/
density sliders, draggable HUD, JSON import/export and PNG scene capture.
**This is a functional but separate browser prototype**, not the in-game
Horizon GUI or proof of native mod compatibility. Its JSON is not consumed by
`.hnmod` automatically. `preview/aurora-preview.html` is the old 0.2 mock.

## Experimental Horizon shader pack — Aurora Nebula

A second artifact ships under `shaderpacks/aurora_nebula/`: a native Horizon
Shader API **0.1** WGSL pack with `sky` and two `post` stages. It should add
procedural aurora ribbons, night haze, one-pass highlight halo and cinematic
color grading. The pack structure and option names pass **offline checks**,
and the new shader is checked through the official Horizon validator in GitHub CI; real gameplay remains unverified.
The official Horizon reference already documents and exposes these stage
hooks, unlike Iris's shader-pack format. **As of the current API docs, the game
may still use only the WGSL default values for shader options**, so the six
values in the manifest should not be mistaken for a confirmed in-game slider UI.

To validate with Horizon's tool:

```powershell
horizon-shader check "C:\Aurora\aurora-horizon-v0.4.0\shaderpacks\aurora_nebula"
```

If it passes, place the `aurora_nebula` folder or its `.hshader` in
`shaderpacks/` beside the Horizon instance's `options.txt`. Select it under
**Options → Horizon Settings → Shader Packs**; the game will run its own
validation. Never claim a successful shader build without the output of that
checker. CI constructs release shaders with the official Horizon packager after validation. Read
`shaderpacks/aurora_nebula/README.md` for limitations.

## Install/test on Windows

1. Get an up-to-date **official Horizon launcher** from [playhorizon.dev](https://playhorizon.dev/) and prepare a licensed Minecraft 26.3 installation once.
2. Extract the source ZIP to `C:\Aurora\aurora-horizon-v0.4.0` (the ZIP contains a root folder).
3. Open PowerShell in the directory containing `horizon-launcher.exe` and run:

   ```powershell
   .\horizon-launcher.exe native-build "C:\Aurora\aurora-horizon-v0.4.0\mixins\aurora_visuals" --trust-native-code
   ```

   Native code executes with your OS user permissions. **Review the full source first.** The native build uses a separate game instance. The first build may download toolchains and compile the game, requiring significant disk space. The launcher can also accept the `.hnmod` if local native-mod installation is supported in its UI.
4. Launch the generated **Native mods** instance. Open a singleplayer world. You should see the HUD in the upper-right corner; `O` toggles it.
5. Navigate to **Options → Mods → Aurora Visuals → Settings**. Test movement trail via `/aurora trail on` and ambient glow via `/aurora ambient on`; turn them back off to compare native FPS. Try click flares, and reduced motion.

## Commands

```text
/aurora help
/aurora toggle
/aurora preset combat
/aurora preset minimal
/aurora preset streamer
/aurora theme violet
/aurora theme cyan
/aurora theme amber
/aurora theme rose
/aurora theme mint
/aurora theme electric
/aurora theme scarlet
/aurora trail on
/aurora trail off
/aurora flare on
/aurora flare off
/aurora adaptive on
/aurora adaptive off
/aurora graph on
/aurora graph off
/aurora feedback on
/aurora feedback off
/aurora ambient on
/aurora ambient off
/aurora compact on
/aurora compact off
/aurora motion on
/aurora motion off
/aurora reset
```

`motion off` activates reduced-motion mode; `motion on` uses animated input bars.

## Developer verification

```bash
python3 tests/static_validate.py
python3 tests/package.py
python3 tests/shader_static.py
python3 tests/package_shader.py
python3 tests/browser_smoke.py  # optional Chromium + playwright
cargo test --manifest-path Cargo.toml
```

For an **actual game-level compile**, clone [horizon-rust/horizon-mods](https://github.com/horizon-rust/horizon-mods), copy this repository folder into its `native/aurora` directory (so `native/aurora/compatibility.json` exists), and run:

```bash
python3 native/tools/native.py check aurora
python3 native/tools/native.py build aurora
```

This checks the complete Horizon integration using the commit pinned in `compatibility.json`, not just the isolated Rust logic. For a future Nightly try `python3 native/tools/native.py check aurora --horizon latest` after you have confirmed its API is at least 46. CI runs on main with official Horizon native integration and WGSL validation.

## Field-test checklist

- [ ] Windows native-build completes without warnings/errors related to Aurora.
- [ ] In-world HUD shows and hides with `O`; no duplicate toggles in chat/menu.
- [ ] All seven themes and all three presets render and persist after restart.
- [ ] All four corners work; F3 suppression behaves as Horizon documents.
- [ ] Remapped movement keys display correct pressed state.
- [ ] FPS graph updates while in-world; minimal mode hides it.
- [ ] Mouse buttons cause INPUT FX pulses. Clicking air must **not** claim a successful hit.
- [ ] Coordinates stay hidden in streamer preset by default.
- [ ] Ambient Glow, trails and click flares tint the local world when enabled, and do not affect server gameplay.
- [ ] Teleport, dimension change, pause/resume and disconnect/reconnect do not show speed spikes.
- [ ] Coexists with official Horizon FPS/Keystrokes native mods.
- [ ] Compare FPS/frametime with and without Aurora for a 5-minute session.

## Architecture

`mixins/aurora_visuals/runtime.rs` is the stateful and pure testable core: settings, frame history, motion, input pulses, and HUD shape generation. Files `hud.rs`, `tick.rs`, `key.rs`, `visuals.rs`, `command.rs`, `commands.rs`, `list.rs`, `settings.rs`, `click.rs` are narrowly scoped Horizon API adapters. `horizon-mixin.toml` ties them to nine stable hook points (including **API 46 `native_visuals`**). `tests/package.py` uses the official deterministic archive layout. No network calls or third-party runtime dependencies in the mod.

## Constraints / risk

- Per-20-tick BPS is an approximation, not real-time server velocity. The local light is a render request, not a full shader engine.
- Native mods are source-level mixins and trigger **a build of the game**. They cannot hot-load like many Java mods.
- `native_visuals` depends on Horizon hook API **46+**. Pin: Horizon commit `97ded175738a56163e40698be72e968d9b956f73`, horizon-mixins commit `a2af5c0dd4117d51ecd70066ba7a00b4a2c10dab`; **tested in source inspection only**.
- Never distribute a mod claiming it was runtime-tested until the actual Horizon compile and smoke tests pass.
- MIT-licensed independent community project, not affiliated with Horizon, Microsoft or Mojang. No Minecraft proprietary assets included.


## v0.3 native VFX (actual in-game hooks)

- `trail` adds up to three **local world-space colored light** requests behind the player. This modifies terrain/entity illumination only, not the world state or server. Disabled by default.
- `flare` adds one brief camera-local light near the player when a mouse button is pressed, and rings on the in-panel input indicator. This is **input feedback**, not confirmed damage. Enabled by default.
- `ambient` keeps the optional player glow. All emitters share an 8-light hard cap and use bounded intensities.
- Seven accent themes; existing settings are read through fallback defaults, with a v2 schema marker retained for coordinates migration.
- Accessible `Reduced motion` disables scrolling world trails and HUD pulse rings.
- New `/aurora trail on|off`, `/aurora flare on|off`, `/aurora theme electric|scarlet`.
- `studio/aurora-studio.html` is a **separate browser prototype** (not an in-game settings screen). It demonstrates a future advanced UI and VFX choreography; exported JSON profiles do not yet auto-import into the Horizon mod.

The `.hnmod` is a **source bundle**, not a binary DLL or precompiled game. Local verification in this release can run Python structural and browser tests, but a complete Horizon cargo build is still required before claiming game compatibility.

- Adaptive light quality uses a conservative FPS-only hysteresis heuristic (not GPU timestamps), lowering optional trail-light count after sustained FPS below 55, restoring gradually above 110. It can be disabled with `/aurora adaptive off`.

## Repository and release integrity

- `main` is canonical for source and pre-releases. The GitHub default branch may need to be changed from `master` in repository settings.
- GitHub Actions must pass for pre-release; actual Windows gameplay testing is a separate gate.
- Historical v0.1.0/v0.2.0 source snapshots are retained under `history/` after repository import.
- Do **not** distribute the experimental `.hnmod` or `.hshader` as a game-tested release without matching integration checks.
- See `docs/PROJECT_STATUS.md` and `docs/RELEASE_POLICY.md` for the exact evidence state and promotion gates.
## Windows build disk diagnostics

The screenshots of local Windows build failure show os error 112 (disk full). The native build may consume multiple GiB; make at least 8–10 GiB free for headroom. To check the build drive and relevant logs without deleting anything:

```powershell
powershell -NoProfile -File .\scripts\windows\aurora-native-preflight.ps1
```

For cleanup and memory/stack-process recovery, see [Windows build recovery](docs/WINDOWS_BUILD_RECOVERY.md). Research notes: [Competitive technical audit](docs/COMPETITIVE_TECHNICAL_AUDIT.md).

## Visible local 3D particles (0.6.0)

Use `/aurora particles on` to enable. Each click produces 3–6 vanilla flame particles around your feet; walking emits no more than two every four ticks. `/aurora fire soul` (blue) and `/aurora fire flame` (orange) choose a Minecraft-native particle. Both survive without a shaderpack and do not rely on fake screen brightness. The effect is client-local; the actual gameplay status and FPS cost must be checked on Windows. Night-sky visuals are independent: select Aurora Nebula in Horizon's shader pack menu.

## PulseVisual-inspired FX patterns (0.6.0)

Ported original Rust implementations of effect patterns, limited geometry and animation math inspired by the **MIT** Java [PulseVisual](https://github.com/creeleystromberg-sketch/PulseVisual). These use Horizon's actual local native particles; Fabric Java and its JVM classes are not embedded.

- `/aurora burst ring` — angular click-input ring near player feet.
- `/aurora burst burst` — layered click-input burst with alternating native blue/orange flames.
- `/aurora burst spiral` — upward spiral input burst.
- `/aurora path off|line|twin|helix` — smoothed particle trails while moving.
- `/aurora crown on|off` — optional sparse particle halo near player head (not a solid 3D China Hat mesh).
- `/aurora particles on|off` — instantly disables/enables the entire particle system.
- Adaptive emit cap: at most 10 local particles per tick (lower in low-FPS or reduced-motion mode).

Mouse-input particles **do not indicate confirmed hits**. The high-end 3D Ribbon/China Hat, held item ViewModel, pearl ownership and attack event hooks still require a separate WGPU/API 58 migration. Read the full [PulseVisual port matrix](docs/PULSEVISUAL_PORT.md) and [third-party attribution](THIRD_PARTY_NOTICES.md).

## Isolated VFX test

In a singleplayer world (preferably third-person) run `/aurora particles on` and `/aurora demo` to queue a local particle burst without needing to click an entity. This bypasses mouse mapping for diagnostics only; it does not register or claim any combat hit. Disable ambient and the Aurora Nebula shader to tell real world particles apart from a brightness-only effect. `/aurora burst spiral` then `/aurora demo` switches the burst pattern. The demo command requires an actual in-world client tick to display particles.
