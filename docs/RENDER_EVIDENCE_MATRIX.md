# Aurora graphics evidence ledger — 2026-11-01

| Feature | implemented | compiled | game-tested | visually-verified | benchmarked | prototype-only | not-implemented | blocked |
|---|---|---|---|---|---|---|---|---|
| 0.6 native particle emitters | yes | yes (Linux) | no | no | no | no | no | no |
| 0.7 WGPU Ribbon geometry | yes (source) | pending official CI | no | no | no | no | no | no |
| Silk/Prismatic/Spectral materials | yes (source) | pending | no | no | no | no | no | no |
| Reversed-depth fragment occlusion | yes (source) | pending | no | no | no | no | no | no |
| Player snapshot data bridge | yes (source) | pending | no | no | no | no | no | no |
| Native commands/settings | yes (source) | pending | no | no | no | no | no | no |
| Integrated native ClickGUI | no | no | no | no | no | no | yes | no |
| Browser Aurora Studio | yes | n/a | no | browser-only | no | yes | no | no |
| GPU performance adaptive governor | no (FPS heuristic only) | no | no | no | no | no | yes | no |
| Real China Hat + Jump Circle meshes | no | no | no | no | no | no | yes | no |
| Audio Engine | no | no | no | no | no | no | yes | no |

**Definitions:** Implemented means source is present, not that runtime output is verified. Compiled requires official full Horizon CI success. Gameplay requires an actual Horizon session. Visually verified requires captured game imagery. Benchmark requires reproducible paired measurements. A standalone HTML demo is prototype-only.

## Reproducible benchmark plan

Run on same Horizon commit/graphics backend/view distance/scene and fixed route, preferably 180 seconds with warmup, 1080p + 1440p. Cases: vanilla, HUD only, ribbon only, Nebula only, all effects. Scenes: simple landscape, dense mobs, PvP, dimension change, resize, camera turns. Capture per-frame CPU+GPU frame times (if tool available), calculate p50, p95, p99, 1% low, draw calls, vertex allocations, VRAM. Repeat 3 times and report median and variance. Proposed GPU overhead budgets: PvP <=0.5 ms, Balanced <=1.5 ms, Cinematic <=3 ms on a named test GPU; **no measured values yet**.
