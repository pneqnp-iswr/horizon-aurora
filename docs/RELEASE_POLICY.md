# Release policy

Separate the following statuses rigorously:

- **Prototype:** isolated HTML/CSS/Canvas demo or code that has only offline structural checks.
- **Integration candidate:** repository source, CI and full native/WGSL integration checks under evaluation.
- **Verified beta:** pinned Horizon native build and actual game tests pass, with test evidence.
- **Release:** performance, startup/crash and regression tests pass on documented hardware.

Never call a packaged `.hnmod` a precompiled binary: it is a native source bundle applied at Horizon build time. Native code runs with the player's OS permissions; only trusted, reviewed source should be installed.

## v0.7.2 release gate

The upcoming v0.7.2 candidate uses a **new release tag** (`v0.7.2-rc.1`) and never overwrites old v0.7.1 assets. PR CI builds and verifies the source-format `.hnmod` and shader pack, but **does not publish a release** on merge or PR completion.

After the official pinned Horizon integration build passes, a maintainer must supply actual Windows singleplayer and multiplayer acceptance evidence in a GitHub issue or PR, using `docs/RIBBON_WINDOWS_ACCEPTANCE.md`. Only then dispatch `Verify Aurora native mod` on `main` with the required `gameplay_evidence_url` input. That workflow repeats the full verification and publishes the prerelease only when it is still targeting the current `main` commit. FPS or GPU claims require their own reproducible benchmarks, not just a green workflow.

## Workflow

Build in a feature branch, open a PR, run checks, document what passed and did not pass, then merge. A green CI badge is not a substitute for manual gameplay evidence. Archive genuine logs and add performance numbers only when measured.
