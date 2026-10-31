# Windows native-build recovery (Aurora Visuals)

Observed on 2026-11-01: the local Windows Horizon build log shows 'Недостаточно места на диске' (os error 112) while writing .rlib output under C:\Users\ASUS\AppData\Roaming\horizon-launcher\native\target\fast\deps. Other processes failed with 0xC0000409 (STATUS_STACK_BUFFER_OVERRUN) and 0xC00000FD (STATUS_STACK_OVERFLOW). The latter may have separate causes and should be evaluated on a clean disk-space-controlled build.

## Verified upstream facts

Official Horizon first Windows native build figures (October 2026): about 2.0 GB native/target, 1.7 GB bundled Rust/MinGW and 657 MB Cargo store, excluding scratch space and versions. The launcher intentionally caches the native target for incremental compiles. The environment variable CARGO_BUILD_JOBS is permitted through to builds. Sources: https://github.com/horizon-rust/horizon/blob/main/docs/native-mods.md#cache-and-disk and https://github.com/horizon-rust/horizon/blob/main/docs/native-mods.md

## Recovery actions

1. Exit Horizon and its launcher, wait for any currently running native compilation to finish/stop. Do not remove cache while rustc is writing files.
2. Free at least 8 GiB, preferably 10 GiB, on the volume containing %APPDATA%\horizon-launcher, including room for temporary files. This is a practical headroom recommendation, not an official hard requirement.
3. From a cloned Aurora repository use the read-only PowerShell script: powershell -NoProfile -File .\scripts\windows\aurora-native-preflight.ps1
4. If previous compilation cache is damaged and needs to be rebuilt, explicitly run: powershell -NoProfile -File .\scripts\windows\aurora-native-preflight.ps1 -CleanNativeTarget -Force. The script only targets the native/target intermediate compiler directory. The next native build will take longer because the cached target was removed. Back up anything customized there first.
5. For persistent process crashes after freeing disk space, restart Windows and retry with fewer parallel compiler jobs. Launch Horizon from the same PowerShell with $env:CARGO_BUILD_JOBS = '2' set before launching the executable; the setting does not transfer to an already running launcher.
6. Open %APPDATA%\horizon-launcher\native\build.log and inspect the FIRST error, not only the final Cargo summary. The PowerShell preflight prints relevant signatures.

## Important safeguards

- Do not remove the whole horizon-launcher directory; it contains versions, settings, other caches and game files.
- Do not remove native/target if builds are still in progress.
- Never turn off Windows Defender or disable OS security to make a native mod compile.
- Build cache cleaning does not increase free space permanently: recompilation recreates the cache.
- A Rust stack/process failure that persists after freeing disk space is not fixed by visually changing the mod. Collect its first real compiler error and consider upstream Horizon support.

## CI coverage

Windows GitHub runner smoke-checks preflight success, low-space exit, rejection of unintended deletion, and deletion limited to native/target in a synthetic fixture. Official native compilation and shader validation currently run on Linux. These do not constitute full Windows gameplay validation.
