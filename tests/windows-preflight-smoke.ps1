$ErrorActionPreference = 'Stop'
$root = Join-Path $env:RUNNER_TEMP 'aurora-preflight-fixture'
$target = Join-Path $root 'native\target'
New-Item -ItemType Directory -Force -Path $target | Out-Null
Set-Content -LiteralPath (Join-Path $target 'cache.tmp') -Value 'temporary'
Set-Content -LiteralPath (Join-Path $root 'keep.txt') -Value 'preserve'
$checker = Join-Path $PSScriptRoot '..\scripts\windows\aurora-native-preflight.ps1'
& $checker -LauncherRoot $root -MinFreeGiB 0 -Json | Out-Null
if ($LASTEXITCODE -ne 0) { throw 'Read-only preflight should pass' }
if (-not (Test-Path -LiteralPath (Join-Path $target 'cache.tmp'))) { throw 'Unexpected deletion in read-only mode' }
& $checker -LauncherRoot $root -MinFreeGiB 1024 -Json | Out-Null
if ($LASTEXITCODE -ne 2) { throw 'Low-space check should return exit code 2' }
try {
    & $checker -LauncherRoot $root -MinFreeGiB 0 -CleanNativeTarget 2>$null | Out-Null
    throw 'Unsafe cleanup unexpectedly succeeded'
} catch {
    if ($_.Exception.Message -like '*unexpectedly succeeded*') { throw }
}
if (-not (Test-Path -LiteralPath (Join-Path $target 'cache.tmp'))) { throw 'Unauthorized cleanup removed cache' }
& $checker -LauncherRoot $root -MinFreeGiB 0 -CleanNativeTarget -Force | Out-Null
if ($LASTEXITCODE -ne 0) { throw 'Authorized cache cleanup failed' }
if (Test-Path -LiteralPath $target) { throw 'Cache cleanup did not remove target' }
if (-not (Test-Path -LiteralPath (Join-Path $root 'keep.txt'))) { throw 'Cleanup removed unrelated data' }
Write-Output 'PASS: Windows diagnostics and safe cleanup'
