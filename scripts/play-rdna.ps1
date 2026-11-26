<#
.SYNOPSIS
    One-command launcher for VIctory-RDNA with tuned AMD RDNA3 defaults.

.DESCRIPTION
    Starts GTA VI (Xbox Series build) under VIctory-RDNA with the proven
    performance flags (including GPU-side 1 MiB+ buffer compares, zero-copy
    constants heap, patched descriptor tracking, and multi-threaded list
    translation).

    Auto-detects game_scarlett_master.exe in the current directory or parent
    directory when -Exe is omitted, and builds victory.exe automatically if
    not yet compiled.

.EXAMPLE
    .\scripts\play-rdna.ps1
.EXAMPLE
    .\scripts\play-rdna.ps1 -Exe D:\Games\GTA6\game_scarlett_master.exe -Log .\run.log
.EXAMPLE
    .\scripts\play-rdna.ps1 -Safe
.EXAMPLE
    .\scripts\play-rdna.ps1 -Fps 60 -RayTracing
#>
param(
    [string]$Exe = '',
    [ValidateSet('series-x', 'series-s')][string]$Console = 'series-x',
    [switch]$Safe,
    [switch]$RayTracing,
    [int]$Fps = 0,
    [string]$Log = '',
    [ValidateSet('release', 'diag')][string]$Build = 'release',
    [switch]$Rebuild,
    [hashtable]$More = @{}
)

$ErrorActionPreference = 'Stop'
$repoRoot = (Resolve-Path (Join-Path $PSScriptRoot '..')).Path

# 1. Locate game executable automatically if not passed explicitly.
if (-not $Exe) {
    $candidates = @(
        (Join-Path (Get-Location) 'game_scarlett_master.exe'),
        (Join-Path $repoRoot '..\game_scarlett_master.exe'),
        (Join-Path $repoRoot 'game_scarlett_master.exe')
    )
    $Exe = $candidates | Where-Object { Test-Path $_ } | Select-Object -First 1
    if (-not $Exe) {
        Write-Host "Could not find game_scarlett_master.exe automatically." -ForegroundColor Red
        Write-Host "Pass it explicitly:  .\scripts\play-rdna.ps1 -Exe <path\to\game_scarlett_master.exe>"
        exit 1
    }
}
if (-not (Test-Path $Exe)) {
    Write-Host "Executable not found: $Exe" -ForegroundColor Red
    exit 1
}
$Exe = (Resolve-Path $Exe).Path
$gameDir = Split-Path -Parent $Exe

# 2. Ensure victory.exe is built (build automatically on first run or -Rebuild).
$victory = Join-Path $repoRoot "target\$Build\victory.exe"
if ($Rebuild -or -not (Test-Path $victory)) {
    Write-Host "Building VIctory-RDNA ($Build profile)..." -ForegroundColor Cyan
    Push-Location $repoRoot
    try {
        if ($Build -eq 'diag') {
            cargo build --locked --profile diag -p victory --features diagnostics
        } else {
            cargo build --locked --release -p victory
        }
        if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
    } finally {
        Pop-Location
    }
}

# 3. Clean stale VICTORY_* vars from the shell (keep user path overrides).
$keepVars = @('VICTORY_CONFIG', 'VICTORY_GAME_DIR', 'VICTORY_CACHE_DIR', 'VICTORY_DATA_DIR')
foreach ($entry in @(Get-ChildItem Env:VICTORY_* -ErrorAction SilentlyContinue | Where-Object { $_.Name -notin $keepVars })) {
    Remove-Item "Env:$($entry.Name)"
}

# 4. Apply proven RDNA3 performance settings.
$settings = [ordered]@{
    VICTORY_TEXTURE_NO_WAIT     = '1'
    VICTORY_RELAY               = '2'
    VICTORY_WIDE_BACK           = '1'
    VICTORY_CARD_REHASH_RUNS    = '1'
    VICTORY_JOIN_COPIES         = '1'
    VICTORY_INDIRECT_WINDOW     = '1'
    VICTORY_RELAY_COPY          = '1'
    VICTORY_CARD_STEPS          = '2'
    VICTORY_INDIRECT_BATCH      = '2'
    VICTORY_MAP_THREADS         = '1'
    VICTORY_TITLE_PRIORITIES    = '0'
    VICTORY_CARD_AWAKE          = '1'
    VICTORY_SHOW_AFTER          = '1'
    VICTORY_AMBIENT             = '0.8'
    VICTORY_D3D12_DRED          = 'light'
    VICTORY_CARD_BUFFERS_MIN_MB = '1'
}

if ($Safe) {
    $settings['VICTORY_UNDER_WIDE'] = '0'
    $settings['VICTORY_LAND_ECHO']  = '0'
} else {
    $settings['VICTORY_THIN']             = '1'
    $settings['VICTORY_JOIN_PARTS']       = '8'
    $settings['VICTORY_UPLOAD_AHEAD']     = '1'
    $settings['VICTORY_PORT_CONSTANTS']   = '1'
    $settings['VICTORY_PORT_DESCRIPTORS'] = '1'
    $settings['VICTORY_SYNC_FINE']        = '1'
    $settings['VICTORY_LAND_SKIP']        = '1'
}

if ($RayTracing) {
    $settings['VICTORY_BVH_BUILDER'] = '1'
}

if ($Fps -gt 0) {
    $interval = [math]::Round(1000.0 / $Fps, 3)
    $settings['VICTORY_FRAME_INTERVAL_MS'] = $interval.ToString([System.Globalization.CultureInfo]::InvariantCulture)
}

foreach ($k in $More.Keys) {
    $settings[$k] = [string]$More[$k]
}

foreach ($k in $settings.Keys) {
    Set-Item "Env:$k" $settings[$k]
}

$mode = if ($Safe) { 'SAFE' } else { 'RDNA3 FAST' }
Write-Host "Launching VIctory-RDNA [$mode | $Console | $Build]" -ForegroundColor Green
Write-Host "  Binary : $victory"
Write-Host "  Game   : $Exe"
if ($Fps -gt 0) { Write-Host "  Target : $Fps FPS ($($settings['VICTORY_FRAME_INTERVAL_MS']) ms)" }
if ($Log)       { Write-Host "  Log    : $Log" }

if ($Log -and -not [System.IO.Path]::IsPathRooted($Log)) {
    $Log = Join-Path (Get-Location) $Log
}

# Run from the game directory so relative package/RPF paths resolve cleanly.
Push-Location $gameDir
try {
    if ($Log) {
        & $victory run --console $Console --execute $Exe 2>&1 | Tee-Object -FilePath $Log
    } else {
        & $victory run --console $Console --execute $Exe
    }
} finally {
    Pop-Location
}
