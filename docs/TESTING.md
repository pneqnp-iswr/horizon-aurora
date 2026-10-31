# Testing and validation

## Executed in this repository

- `cargo test`
- `cargo fmt --all --check`
- `cargo clippy --all-targets -- -D warnings`

## What this validates

- Runtime config serialization and localization mappings.
- Theme cycling and reduced-motion behavior.
- CPS/combo animation state transitions.

## Not validated in this repository alone

- Real in-game rendering behavior (requires staged Horizon build + resources).
- Hook patch application via horizon-mixins CLI.
- GPU pipeline behavior and postprocessing paths.

These are intentionally documented as untested claims until staged engine validation is run.
