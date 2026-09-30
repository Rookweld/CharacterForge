# THE FORGE B2 — Hometown Mission Report

**Status: PASS — B2 and B2.R1 automated exit gates satisfied**

## Scope and baseline

Implemented from branch `master`, starting HEAD `48905176ef9fe86799c8a2cfbc3dbc6eef177b4c` (`docs: prepare Forge Hometown mission`). The schema remains version 2. This closeout covers only the bounded saved Character → Hometown flow. No later mission was started, and no commit was created.

The four unrelated operator/export/log files present at start remain unmodified and untracked: `The Ashen Claim.json`, `The Ashen Claim.md`, `mission09-final-verify.log`, and `mission09-rust-test.log`.

## Delivered

- Added the app-core versioned `character.forge-hometown` action and `character-hometown-v1` context recipe. Compilation accepts only saved generated Candidate or Canon Characters with valid Character ForgeDNA. It selects the eight specified semantic facts, inherits preset and Creative Axes, hashes the exact persisted source snapshot and bounded context with domain-separated BLAKE3, and performs no writes.
- Hometown generation reuses the Location Forge and its field streams, derives its seed from action identity/version, context hash, and explicit action seed, and supplies the supported `location.profile=settlement` constraint. Unconstrained standalone Location generation remains on its prior path.
- Added typed versioned recipe/context and output-mapping records to `forge_runs` JSON. One SQLite transaction verifies the source write version and exact compiled context, then writes the Location Candidate, proposed Candidate/Generated `born_in` relationship, and provenance event. Transaction rollback, stale-source rejection, duplicate-save rejection, close/reopen, and `.forgeproject` import/export are covered.
- Added the desktop review flow from saved Character detail, with a numeric action seed, source/context/hash summary, Location field editing controls, proposed relationship, discard, and atomic Save Bundle. Manual/imported Characters without supported generated provenance have a disabled action and explanation. Saved relationships are available through the Character Connections read model.
- Added 24 deterministic Character → Hometown examples across all 11 presets in [THE_FORGE_B2_HOMETOWN_SAMPLES.md](../quality/THE_FORGE_B2_HOMETOWN_SAMPLES.md).

## Verification

All required repository gates passed:

- Frontend format, lint, TypeScript typecheck, production build: PASS.
- Frontend tests: **42 passed**.
- Rust formatting and strict Clippy (`-D warnings`): PASS.
- Workspace Rust tests: **140 passed, 1 ignored** (the existing one-time frozen schema fixture writer). Suite counts: desktop 12, app-core 66, contracts 14, export 21, storage 27 passed.
- `scripts/verify.ps1`: PASS with `CARGO_TARGET_DIR=target/mission-b2-verify`.
- Cargo metadata: PASS.
- Optimized Tauri `build --no-bundle`: PASS; executable produced at `apps/desktop/src-tauri/target/mission-b2-verify/release/character-forge-desktop.exe`.
- `git diff --check`: PASS.

`npm test` was also attempted as required by the repository working agreement, but the machine's npm shim could not find `C:\Users\pontu\AppData\Roaming\npm\node_modules\npm\bin\npm-cli.js`. Repository tests ran successfully through the configured Corepack/pnpm workflow and through `scripts/verify.ps1`.

## Exit-gate review

| Criterion | Result |
|---|---|
| Forge Hometown for eligible saved Characters; unsupported provenance fails clearly | PASS |
| Source Character remains unchanged; compiler is bounded, deterministic, and read-only | PASS |
| Versioned Action and Context Recipe; frozen Context Snapshot stored in immutable `forge_runs` provenance | PASS |
| Inherited preset/axes, explicit deterministic seed, Settlement profile constraint | PASS |
| Normal Location Candidate lock/reroll/edit behavior and unconstrained standalone compatibility | PASS; existing Location regression suite and explicit constraint tests pass |
| Candidate/Generated `born_in` relationship with valid typed endpoints | PASS |
| Atomic bundle write; injected rollback points, stale-source and duplicate-save protections | PASS |
| Close/reopen and `.forgeproject` export/import preserve Location, relationship, and typed provenance | PASS |
| Character/Quest/Location regression suites and complete automated verification | PASS |
| Known P0/P1/P2 defects | None identified |

Existing P3 UI-001 (intermittent Location lock visual feedback) remains open as documented in `GAZINGA/KNOWN_ISSUES.md`; B2 did not expand into its remediation. B2 does not require an immediate human smoke gate under the mission's stated review policy. No manual operator acceptance is claimed.

## Final repository state

- Branch: `master`
- HEAD: `48905176ef9fe86799c8a2cfbc3dbc6eef177b4c`
- Worktree: B2 implementation, tests, and closeout documentation remain unstaged/uncommitted; the four pre-existing operator/export/log files remain untouched.
- Database schema: 2.
- P0/P1/P2: 0/0/0 known for this mission; existing P3 UI-001 remains open.

**B2 exit gate: PASS. Stop here.**


## Independent review amendment

Post-closeout review found one P2 blocker: lock/reroll/undo/redo could update session working Location DNA without updating cached bundle recipe DNA, so atomic Save Bundle rejected a valid reviewed candidate. B2.R1 resolves this below; the prior verification above is historical evidence from before the fix.

The B2.R1 remediation is complete. `finalize_hometown_review` now synchronizes recipe DNA from the exact reviewed primary while preserving frozen source context/hash and stable primary/relationship identity. Final verification passed: 143 Rust tests (1 existing ignored fixture writer), 42 frontend tests, `scripts/verify.ps1`, formatting, lint, typecheck, frontend production build, strict Clippy, and optimized isolated-target Tauri build. The initial full rerun encountered one 1 ms timestamp assertion fluctuation in the existing relationship lifecycle test; its isolated rerun and the subsequent full verification passed. `npm test` remains unavailable because the configured npm-cli.js is missing; Corepack/pnpm passed. See [THE_FORGE_B2_R1_REVIEWED_BUNDLE_PROVENANCE_REPORT.md](THE_FORGE_B2_R1_REVIEWED_BUNDLE_PROVENANCE_REPORT.md) for complete regression coverage and final state. B2 and B2.R1 are PASS; existing P3 UI-001 remains open and separate. No later entity mission has started.

**Final B2/B2.R1 status: PASS.** Branch `master`, HEAD `48905176ef9fe86799c8a2cfbc3dbc6eef177b4c`; work remains uncommitted and unstaged. The four unrelated operator/export/log files remain untouched.
