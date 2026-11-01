# Ribbon renderer — PR #5 Windows gameplay acceptance test

This is an **unreleased integration candidate**. The pinned native-mod API is Horizon API 58; build with the official Horizon launcher, not with a generic Rust-only substitute. The .hnmod contains **source**, not a precompiled DLL.

## Windows build

```powershell
git clone --branch fix/ribbon-continuous-geometry-fade https://github.com/pneqnp-iswr/horizon-aurora.git C:\Aurora\horizon-aurora-ribbon-test
$env:AURORA_RIBBON_DIAG = "1"
.\horizon-launcher.exe native-build "C:\Aurora\horizon-aurora-ribbon-test\mixins\aurora_visuals" --trust-native-code
```

Run the newly built **Native mods** instance. The `native-build` command takes a trusted local native source directory. Remove `AURORA_RIBBON_DIAG` or set it to `0` for normal play. Note the Horizon build log and installed game version.

## Acceptance matrix

Test each line in **singleplayer and multiplayer** after enabling `/aurora ribbon on`.

| Scenario | Commands/steps | Pass evidence |
| --- | --- | --- |
| Stationary demo | `/aurora ribbon-demo on`, stand still 30 seconds | Ribbon remains visible and rotates continuously without restarting |
| Movement trail | `/aurora ribbon-demo off`, walk/sprint/turn | Shared joins show no triangular splits, tail fades smoothly |
| Pause/resume | Demo ON, enter/leave pause and settings menus three times | No unexpected reset (game may intentionally pause simulation) |
| Camera | Toggle first/third person; vary camera distance | No sudden twisting, width collapse, or flashing |
| Teleport | Teleport more than 8 blocks | Old path clears once; new path begins without long cross-world spike |
| Dimension | Overworld → Nether → Overworld | No ribbon connecting dimensions |
| Resize | Windowed ↔ maximized, resize rapidly | Correct depth and target alignment, no crashes |
| Toggles | `/aurora ribbon off`, then on; demo toggled | Explicit disable tears down; re-enable creates clean state |
| Styles | `/aurora ribbon-style silk|prismatic|spectral` (one per command) | Distinct colors/geometry, no unbounded transparency accumulation |
| Stress | Five-minute session while moving and fighting mobs | No steadily growing memory or recurring GPU reallocation |

Use same Horizon revision, render distance, shader pack, resolution, GPU and scene for baseline vs Aurora comparisons.

## Diagnostic evidence

Capture the game's native log with `AURORA_RIBBON_DIAG=1`. Only transitions and GPU recreation should emit messages; a stable frame should not log per tick. Log entries include the `role=client`, `reason`, `epoch`, `sequence` and point count (when applicable).

Record exact build SHA, Windows/Horizon version, game mode, settings, repro steps, expected vs observed behavior, 20+ second video for the looping demo, and before/after screenshots of a sharp turn. Include logs on failure and obscure personal paths/server addresses before sharing.

**Not covered by CI:** actual Windows runtime stability, screenshots, GPU timestamps, FPS/p95/p99 or VRAM measurements. Passing source checks and Linux native integration alone does not establish these.
