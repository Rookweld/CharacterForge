$ErrorActionPreference = 'Stop'

pnpm run format:check
if ($LASTEXITCODE -ne 0) { throw 'Frontend formatting check failed.' }
pnpm run lint
if ($LASTEXITCODE -ne 0) { throw 'Frontend lint failed.' }
pnpm run typecheck
if ($LASTEXITCODE -ne 0) { throw 'Frontend typecheck failed.' }
pnpm test
if ($LASTEXITCODE -ne 0) { throw 'Frontend tests failed.' }
pnpm run build
if ($LASTEXITCODE -ne 0) { throw 'Frontend build failed.' }

cargo fmt --all -- --check
if ($LASTEXITCODE -ne 0) { throw 'Rust formatting check failed.' }
cargo clippy --workspace --all-targets -- -D warnings
if ($LASTEXITCODE -ne 0) { throw 'Rust clippy check failed.' }
cargo test --workspace
if ($LASTEXITCODE -ne 0) { throw 'Rust tests failed.' }

pnpm run tauri:build
if ($LASTEXITCODE -ne 0) { throw 'Tauri compile failed.' }
