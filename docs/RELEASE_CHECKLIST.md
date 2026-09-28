# v1.0.0 FOUNDATION Release Checklist

This records the verified local Windows 11 x64 v1.0.0 release artifact. It does not claim public release. Mission 09 status is PASS.

## Freeze, scope, and versions

- [x] Mission 00–08 documented PASS; Mission 07 GAUNTLET and Mission 08 ARMOR reports reviewed.
- [x] FOUNDATION completeness recorded; no known unwaived security P0/P1/P2 release issue in the baseline evidence.
- [x] No post-v1 feature or implementation behavior added during release closeout.
- [x] Cargo workspace, Cargo.lock local workspace entries, root package, desktop package, and Tauri config stamped 1.0.0.
- [x] Compatibility versions preserved: SQLite schema 1; generator 1; recipe 1; Character content 1/2; Quest content 2/3; `.forgeproject` 1.
- [x] `scripts/check-release-version.ps1` run after the final build and passed.

## Automated verification

- [x] `cargo fmt --all -- --check` PASS.
- [x] `cargo test --workspace --locked`: 114 passed, 0 failed.
- [x] `cargo clippy --workspace --all-targets --locked -- -D warnings` PASS.
- [x] Cargo metadata PASS; `git diff --check` PASS.
- [x] `scripts/verify.ps1` PASS in isolated `target/mission09-final-verify`; optimized Tauri `build --no-bundle` succeeded.
- [x] Mission 08 baseline preserved: 36 frontend tests; formatting, lint, TypeScript typecheck, frontend production build, `scripts/verify.ps1`, repository pnpm tests, strict Clippy, metadata, and isolated optimized build PASS.
- [x] Frozen-lockfile pnpm restoration succeeded; the offline store lacked `@types/react@19.3.0`, then network-enabled restore reused all 117 packages without changing versions or lockfile.
- [x] Mission 09 frontend formatting, lint, TypeScript typecheck, all 36 tests, and frontend production build PASS.

## Package, data, and privacy

- [x] Portable ZIP staged in `artifacts/releases/v1.0.0/`; contains the optimized executable and README.
- [x] WebView2 Runtime requirement stated; runtime itself is not included.
- [x] Human release decision selects the Windows x64 portable ZIP as the primary v1.0.0 artifact; NSIS/MSI installers are not included and do not block this release.
- [x] Release staging inventory reviewed; package contains only the app executable and portable README.
- [x] SHA-256 checksums generated and independently recomputed; manifest and build provenance recorded.
- [x] Release notes, changelog, README, user guide, and implementation journal updated.
- [x] Unsigned build; no certificate or publisher identity asserted.
- [x] No project license selected; Cargo metadata says `UNLICENSED`; public redistribution needs an owner licensing decision.
- [x] Mission 08 dependency limitation retained: no complete automated transitive advisory scan; direct/runtime dependency review and ZIP advisory applicability review recorded; no graph-wide clean claim.
- [x] Optional sample project deferred; no safe existing fixture was identified and no fixture-specific app code was added.

## Native acceptance — required before Mission 09 PASS

- [x] Extracted the exact staged portable ZIP outside the repository and launched it without repository or dev-server files.
- [x] Recorded the exact artifact/hash and verified ProductVersion 1.0.0; Create/Open Project passed.
- [x] Character: deterministic seed workflow; Name and Secret locks survived Reroll Unlocked; manual edit; Undo/Redo; Favorite; Save; Canon promotion; History/Canon inspection.
- [x] Quest: Objective lock; Twist reroll; Quest Save; JSON and Markdown export.
- [x] Full close/relaunch passed; project, locks, favorite, history, and Canon persisted.
- [x] `.forgeproject` export/import passed.
- [x] Backup/Restore passed, followed by a second close/relaunch.
- [x] Offline use, no login/API key, resize/layout, keyboard focus, and recovery after an expected recoverable error passed.
- [x] Operator result PASS for the exact staged ZIP; no operator-visible P0/P1/P2 release defect found.

## Distribution and Git

- [x] Exact local artifacts, byte sizes, hashes, provenance, and unsigned status recorded in `artifacts/releases/v1.0.0/RELEASE_MANIFEST.md`.
- [x] Mission 09 report records branch, HEAD, worktree, tag, and publication state.
- [x] No commit, tag, tag push, website upload, GitHub release, Store submission, or other publication performed.
- [x] No subsequent mission started.

**Current overall status: PASS — v1.0.0 FOUNDATION release gates verified.** Automated gates and final native operator acceptance of the exact staged ZIP passed. Installers are not part of this release per the human decision. See [Mission 09 report](missions/MISSION_09_REPORT.md) and the [release manifest](../artifacts/releases/v1.0.0/RELEASE_MANIFEST.md).
