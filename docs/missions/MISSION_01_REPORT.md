# Mission 01 — DNA

## Mission

Mission 01 — DNA: canonical contracts and deterministic ForgeDNA field generation infrastructure.

## Status

PASS

## Scope delivered

- Added typed, versioned project/entity contracts; Character and Quest data contracts; generation provenance; canonical UUID identities; creative axes; field controls/nonces; constraints; recipes; ForgeDNA; and structured validation errors.
- Added strict canonical compact JSON serialization and decoding, with sorted map keys, normalized identifiers/constraints, fixed-width hexadecimal root seeds, and explicit supported versions.
- Added deterministic per-field BLAKE3 streams using explicit recipe inputs and declared dependencies.
- Added pure lock/unlock, single-field reroll, and reroll-all-unlocked operations. Dependency invalidation is explicit and transitive; generated values and nonce changes commit atomically.
- Added 29 Rust tests covering contracts, canonical roundtrip, ordering stability, Unicode, deterministic streams, field isolation, locks, dependency invalidation, overflow, and failure atomicity, including a bounded 64-seed property-style matrix and explicit cycle/firewall audit cases.
- Updated architecture and implementation notes and added ADR 0002 for canonical serialization and field-stream compatibility.
- A later pre-Mission-02 contract audit confirmed cycle rejection and the locked-dependency firewall, added explicit automated tests for both, and corrected stale native-readiness notes in ADR 0001.
- No UI, persistence, content packs, AI, or later-mission features were added.

## Files materially changed

- `Cargo.toml`, `Cargo.lock`, `crates/contracts/Cargo.toml`, `crates/contracts/src/`
- `crates/app-core/Cargo.toml`, `crates/app-core/src/`
- `docs/ARCHITECTURE.md`, `docs/IMPLEMENTATION_JOURNAL.md`
- `docs/adr/0002-forge-dna-and-field-streams.md`
- `docs/missions/MISSION_01_REPORT.md`, `GAZINGA/PROJECT_STATE.md`

## Commands executed

- `cargo fmt --all` — PASS.
- `cargo fmt --all -- --check` during the contract audit — first run reported formatting differences in the two new audit tests; `cargo fmt --all` corrected them and the rerun passed.
- `cargo clippy --workspace --all-targets -- -D warnings` during the contract audit — PASS.
- `cargo test --workspace` during the contract audit — PASS (29 tests; 0 failed).
- `$env:CARGO_TARGET_DIR = Join-Path $env:TEMP 'characterforge-mission01-target'; ./scripts/verify.ps1` during the contract audit — PASS, including the frontend suite/build, Rust checks/tests, and optimized Tauri build.
- `git diff --check` after the contract audit — PASS (exit code 0; line-ending conversion notices only).
- `cargo test -p forge-contracts -p forge-app-core` — first attempt could not reach crates.io; after network access was available, first compilation exposed an incorrect BLAKE3 XOF call; implementation was corrected and a later run passed all 27 tests.
- `cargo fmt --all -- --check` — PASS.
- `cargo clippy --workspace --all-targets -- -D warnings` — PASS.
- `cargo metadata --no-deps --format-version 1` — PASS (exit code 0).
- `cargo test --workspace` — PASS (27 tests; all other workspace and doc-test targets passed with zero tests).
- `./scripts/verify.ps1` — initial run passed frontend checks/build and Rust checks/tests, then the Tauri release build failed because Windows denied removing the existing `target/release/character-forge-desktop.exe`; a responsive process named `character-forge-desktop` was present. The process was left untouched.
- `$env:CARGO_TARGET_DIR = Join-Path $env:TEMP 'characterforge-mission01-target'; pnpm run tauri:build` — PASS; optimized desktop executable built in the temporary target directory.
- `$env:CARGO_TARGET_DIR = Join-Path $env:TEMP 'characterforge-mission01-target'; ./scripts/verify.ps1` — PASS. Frontend formatting, lint, typecheck, React test (1/1), frontend build, Rust formatting, clippy, workspace tests, and optimized Tauri build all completed successfully.
- `git status --short --branch` — PASS; branch `master`, working tree dirty with Mission 01 changes.
- `git rev-parse HEAD` — `994ac6c95c51651afd90fa2e9864257cefc4694c`.
- `git diff --check` — PASS (exit code 0; Git reported line-ending conversion notices only).
- `git diff --stat` — recorded tracked-file changes; new source/report files are untracked until added.

## Verification evidence

- Targeted crate suite: 18 app-core tests and 9 contracts tests passed.
- Original Mission 01 workspace suite: 27 tests passed, 0 failed; all workspace/doc-test targets completed successfully.
- Pre-Mission-02 audit workspace suite: 29 tests passed, 0 failed; 19 app-core and 10 contracts tests passed, with all other workspace/doc-test targets completing successfully.
- Cycle safety: explicit A→B→C→A dependency insertion returns `DependencyCycle` without changing the accepted graph. Directly supplied cyclic graph data also returns the same structured validation error. Validation uses iterative Kahn traversal, so cycle checking does not recurse.
- Locked dependency firewall: in A→B→C, rerolling A while B is locked advances A only; B and unlocked descendant C retain both values and nonces.
- The property-style test checks 64 root seeds across four fields and asserts that changing a field nonce does not change unrelated field fingerprints.
- Consolidated verification passed using a separate temporary Cargo target directory because an existing executable was held open. The original executable/process was not altered.
- GUI interaction is not part of Mission 01's exit gate; no new GUI behavior was introduced.

## Architecture decisions

- ForgeDNA canonical form is compact JSON with declaration-order struct fields and sorted `BTreeMap` keys. UUIDs use lowercase hyphenated text; root seeds use 16 lowercase hexadecimal digits; identifiers and constraint values are normalized.
- Initial schema, generator, content, and recipe versions are `1`; unsupported ForgeDNA versions return structured validation errors.
- Field streams use BLAKE3 derive-key context `character-forge/forge-dna/field-stream/v1`; their canonical inputs include the version tuple, root seed, preset, axes, constraints, target field/nonce, and sorted explicit dependency field/nonces.
- Single-field reroll advances the target and transitive unlocked dependents. A locked dependent and its descendants remain unchanged. Reroll-all advances every unlocked field. Planned nonce and value changes commit only after all requested generation calls succeed.
- See `docs/adr/0002-forge-dna-and-field-streams.md`.

## Known defects

None found within Mission 01 scope.

## Deferred items

Persistence, final Character/Quest content, UI workflows, AI/provider integrations, content packs, export workflows, and later missions remain deferred.

## Repository state

- Branch: `master`
- HEAD: `b137179181ed90e07d6937c7629542d735e56681` (`feat: establish Forge DNA deterministic core`)
- Status: dirty with pre-Mission-02 audit changes in the reroll/cycle tests, ADR 0001, this report, and `GAZINGA/PROJECT_STATE.md`; Mission 01 baseline is committed.

## Next mission readiness

YES — Mission 01 exit gates pass. Stop here; Mission 02 has not been started.
