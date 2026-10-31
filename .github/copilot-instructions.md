# Copilot instructions for Horizon Aurora

- Keep changes small, explicit, and evidence-backed against pinned Horizon revisions.
- Prefer named hook contracts from `native-hooks.json` over hardcoded target signatures.
- Do not claim in-game behavior without staged Horizon build validation.
- Keep runtime logic testable outside Horizon by isolating engine-specific code in helper files.
- Avoid unsafe Rust and avoid allocations/blocking work in frequent render paths when possible.
