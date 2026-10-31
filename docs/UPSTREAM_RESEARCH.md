# Upstream research (evidence-backed)

## Audited sources

- `horizon-rust/horizon` at `ba6344a59cb9e755b730dcf02285fc0d33b97d3c` (Nightly 2026-10-30 pin used by native template).
- `horizon-rust/horizon-native-mod-template` at `cc8ce2d896782099abfa9f0abf26fa80c9e4bddb`.
- `horizon-rust/horizon-mixins` (tooling contracts and schema references).
- `horizon-mods/native` repository was not found (404); no assumptions made from it.

## Verified integration points

1. Hook catalog exists in Horizon root: `native-hooks.json` with named hooks like `hud`, `key`, `mods`, `settings`, `settings_click`.
2. Hook contracts are implemented in `crates/horizon-client/src/game/native_hooks.rs` and documented in `docs/native-mods.md`.
3. Native HUD drawing helpers (`native_fill`, `native_text`) and mod settings hooks are present on the pinned commit.
4. Template-compatible contracts:
   - mixin schema 6
   - Horizon Mixins revision `d9c83d7096b7ea5b83678da2efdfd6161f74997e`
5. Advanced renderer hooks (`native_world_drawn`, `native_render_needs`) are documented for API 58 in newer Horizon revisions, but are outside the pinned Nightly contract used in this slice.

## Concrete limitations confirmed

- No supported evidence was found for direct built-in bloom/blur postprocess APIs at this pin; advanced postprocessing must use renderer hooks on later API revisions or engine patches.
- Combat event hooks such as `hurt_enemy` are not listed in the pinned hook catalog, so this slice uses legitimate exposed input hooks (`key.mouse.left`) for local PvP telemetry.
- In-game behavior is not claimed as validated in this repository alone because the full staged Horizon build and runtime assets are not present here.
