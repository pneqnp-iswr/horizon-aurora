# Aurora Visuals: Windows native-build environment check.
# Read-only by default. Does not modify Horizon, Minecraft saves or toolchains.
[CmdletBinding()]
param(
    [string]$LauncherRoot = (Join-Path $env:APPDATA 'horizon-launcher'),
    [ValidateRange(0, 1024)][double]$MinFreeGiB = 8,
    [switch]$Json,
    [switch]$CleanNativeTarget,
    [switch]$Force
)

$ErrorActionPreference = 'Stop'
$root = [System.IO.Path]::GetFullPath($LauncherRoot).TrimEnd('\', '/')
$target = Join-Path $root 'native\target'
$buildLog = Join-Path $root 'native\build.log'

# Never perform a destructive operation on a drive root or an unexpected path.
if ($CleanNativeTarget) {
    if (-not $Force) {
        throw 'Cleaning requires both -CleanNativeTarget and -Force. Run only after closing Horizon and all native builds.'
    }
    if ([System.IO.Path]::GetPathRoot($root).TrimEnd('\') -eq $root) {
        throw 'Refusing to operate on a drive root.'
    }
    if (-not (Test-Path -LiteralPath $root -PathType Container)) {
        throw "Launcher directory not found: $root"
    }
    if (Test-Path -LiteralPath $target) {
        $item = Get-Item -LiteralPath $target -Force
        if (($item.Attributes -band [IO.FileAttributes]::ReparsePoint) -ne 0) {
            throw 'Refusing to remove a linked or redirected native target directory.'
        }
        Remove-Item -LiteralPath $target -Recurse -Force
    }
}

$driveRoot = [System.IO.Path]::GetPathRoot($root)
$drive = [System.IO.DriveInfo]::new($driveRoot)
if (-not $drive.IsReady) { throw "Drive is not ready: $driveRoot" }
$freeGiB = [math]::Round($drive.AvailableFreeSpace / 1GB, 2)
$logs = @()
if (Test-Path -LiteralPath $buildLog -PathType Leaf) {
    $logs = @(Select-String -LiteralPath $buildLog -Pattern @(
        'os error 112',
        'Недостаточно места на диске',
        'STATUS_STACK_BUFFER_OVERRUN',
        'STATUS_STACK_OVERFLOW',
        'E_MODULE'
    ) | Select-Object -Last 12 | ForEach-Object { $_.Line.Trim() })
}
$result = [ordered]@{
    launcher_root = $root
    drive = $driveRoot
    free_gib = $freeGiB
    recommended_min_gib = $MinFreeGiB
    ready = ($freeGiB -ge $MinFreeGiB)
    native_target_exists = (Test-Path -LiteralPath $target -PathType Container)
    recent_failure_signatures = $logs
}
if ($Json) {
    $result | ConvertTo-Json -Depth 3
} else {
    Write-Host "Horizon native-build preflight | $driveRoot | $freeGiB GiB free"
    if (-not $result.ready) {
        Write-Warning "Insufficient free space for a reliable first compile. Free at least $MinFreeGiB GiB before retrying."
    } else {
        Write-Host 'Free-space check passed (not a guarantee against other compile errors).'
    }
    if ($logs.Count) {
        Write-Host 'Relevant signatures in native/build.log:'
        $logs | ForEach-Object { Write-Host "  $_" }
    }
    Write-Host 'To limit peak CPU/RAM usage, launch Horizon from a terminal after setting $env:CARGO_BUILD_JOBS = 2.'
    Write-Host 'Do not delete native/versions, game saves, or Cargo registries to fix this error.'
}
if (-not $result.ready) { exit 2 }
exit 0
