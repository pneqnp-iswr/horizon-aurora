# Capability matrix

Legend: WORKING, VERIFIED FEASIBLE, REQUIRES ENGINE PATCH, BLOCKED, PLANNED.

| Area | Capability | Status | Evidence / Notes |
|---|---|---:|---|
| Core | Typed config + profile serialization | WORKING | `mixins/aurora/runtime.rs` serde config + load/save APIs |
| Core | Feature toggles (theme/reduced motion) | WORKING | Key + settings click handling in runtime/helpers |
| Core | Diagnostics/perf counters | WORKING | Local CPS/combo counters in HUD snapshot |
| Core | EN/RU theme naming | WORKING | `ThemePreset::name_en/name_ru` |
| Graphics | HUD color accents + panel rendering | VERIFIED FEASIBLE | `hud` hook + `native_fill/native_text` helpers on pinned hook API |
| Graphics | Local animated feedback (flash decay) | WORKING | `flash` interpolation with reduced-motion clamp |
| Graphics | Bloom/blur full postprocess | REQUIRES ENGINE PATCH | Not exposed in pinned hook catalog |
| Graphics | Renderer access (`world_drawn`, `render_needs`) | PLANNED | Available on newer API 58 revisions |
| Combat | Combo/CPS local telemetry | WORKING | `key.mouse.left` capture + CPS window |
| Combat | Native hit-confirmation from damage hooks | BLOCKED | Not in pinned hook catalog |
| Interface | Mods page registration + settings buttons | VERIFIED FEASIBLE | `mods`, `settings`, `settings_click` named hooks |
| Interface | HUD editor with drag interaction | PLANNED | Requires panel/screen interaction flow and layout persistence |
| Motion | Frame-independent animation core | WORKING | `tick(dt)` + clamped decay |
| Motion | Reduced-motion behavior | WORKING | `reduced_motion` clamp logic |
