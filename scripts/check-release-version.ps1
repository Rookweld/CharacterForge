$ErrorActionPreference = 'Stop'
$expected = '1.0.0'

$workspace = Get-Content (Join-Path $PSScriptRoot '..\Cargo.toml') -Raw
if ($workspace -notmatch '(?ms)^\[workspace\.package\].*?^version\s*=\s*"([^"]+)"') {
    throw 'Could not read workspace.package.version.'
}
$cargoVersion = $Matches[1]

$rootPackage = Get-Content (Join-Path $PSScriptRoot '..\package.json') -Raw | ConvertFrom-Json
$desktopPackage = Get-Content (Join-Path $PSScriptRoot '..\apps\desktop\package.json') -Raw | ConvertFrom-Json
$tauriConfig = Get-Content (Join-Path $PSScriptRoot '..\apps\desktop\src-tauri\tauri.conf.json') -Raw | ConvertFrom-Json

$rows = @(
    [pscustomobject]@{ Surface = 'Cargo workspace'; Version = $cargoVersion },
    [pscustomobject]@{ Surface = 'Root package'; Version = $rootPackage.version },
    [pscustomobject]@{ Surface = 'Desktop package'; Version = $desktopPackage.version },
    [pscustomobject]@{ Surface = 'Tauri config'; Version = $tauriConfig.version }
)
$rows | Format-Table -AutoSize
$mismatches = @($rows | Where-Object Version -ne $expected)
if ($mismatches.Count -gt 0) {
    throw "Release versions must all equal $expected."
}

$metadataJson = cargo metadata --no-deps --format-version 1
if ($LASTEXITCODE -ne 0) { throw 'cargo metadata failed.' }
$metadata = $metadataJson | ConvertFrom-Json
$workspaceNames = @('character-forge-desktop', 'forge-app-core', 'forge-contracts', 'forge-export', 'forge-storage')
foreach ($name in $workspaceNames) {
    $package = $metadata.packages | Where-Object name -eq $name | Select-Object -First 1
    if ($null -eq $package -or $package.version -ne $expected) {
        throw "Cargo metadata does not report $name at $expected."
    }
}

$lock = Get-Content (Join-Path $PSScriptRoot '..\Cargo.lock') -Raw
foreach ($name in $workspaceNames) {
    $escapedName = [regex]::Escape($name)
    $match = [regex]::Match($lock, "(?ms)^\[\[package\]\]\s*\nname = `"$escapedName`"\s*\nversion = `"([^`"]+)`"")
    if (-not $match.Success -or $match.Groups[1].Value -ne $expected) {
        throw "Cargo.lock does not contain $name at $expected. Run Cargo metadata/build to refresh it."
    }
}

Write-Output "Release version consistency: PASS ($expected)."
