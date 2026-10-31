# AGENTS

This repository hosts **Horizon Aurora**, a native visual enhancement mod suite for Horizon.

## Development rules

1. Pin and document exact upstream compatibility revisions before changing hook integrations.
2. Prefer named hooks from `native-hooks.json`; avoid inventing unsupported engine APIs.
3. Keep runtime logic in testable Rust modules and keep helpers thin.
4. Never claim in-game functionality unless validated against a staged Horizon build.
5. Avoid combat automation, bypass behavior, or hidden-player disclosure features.
