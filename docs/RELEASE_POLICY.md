# Release policy

Separate the following statuses rigorously:

- **Prototype:** isolated HTML/CSS/Canvas demo or code that has only offline structural checks.
- **Integration candidate:** repository source, CI and full native/WGSL integration checks under evaluation.
- **Verified beta:** pinned Horizon native build and actual game tests pass, with test evidence.
- **Release:** performance, startup/crash and regression tests pass on documented hardware.

Never call a packaged `.hnmod` a precompiled binary: it is a native source bundle applied at Horizon build time. Native code runs with the player's OS permissions; only trusted, reviewed source should be installed.

## Workflow

Build in a feature branch, open a PR, run checks, document what passed and did not pass, then merge. A green CI badge is not a substitute for manual gameplay evidence. Archive genuine logs and add performance numbers only when measured.
