$ErrorActionPreference = 'Stop'

$previousCargoTargetDir = $env:CARGO_TARGET_DIR
$env:CARGO_TARGET_DIR = 'target\release-verify'

try {
    & (Join-Path $PSScriptRoot 'check-release-version.ps1')
    if ($LASTEXITCODE -ne 0) { throw 'Release version consistency check failed.' }

    & (Join-Path $PSScriptRoot 'verify.ps1')
    if ($LASTEXITCODE -ne 0) { throw 'Repository verification failed.' }

    cargo metadata --no-deps --format-version 1 | Out-Null
    if ($LASTEXITCODE -ne 0) { throw 'Cargo metadata failed.' }

    git diff --check
    if ($LASTEXITCODE -ne 0) { throw 'Git diff whitespace check failed.' }

    Write-Output 'Release verification: PASS.'
}
finally {
    if ($null -eq $previousCargoTargetDir) {
        Remove-Item Env:\CARGO_TARGET_DIR -ErrorAction SilentlyContinue
    }
    else {
        $env:CARGO_TARGET_DIR = $previousCargoTargetDir
    }
}
