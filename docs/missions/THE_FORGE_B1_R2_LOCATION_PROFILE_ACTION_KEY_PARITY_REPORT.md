# THE FORGE B1.R2 — Location Profile Action-Key Parity

**Automated remediation: PASS. Final operator recheck: REQUIRED.** Mission B1 remains **PARTIAL**. B2 was not started.

## Operator finding and cause

The operator reported that Location Profile actions produced `Unknown Location field: location_profile`. Independent review confirmed that the UI had conflated three identifiers:

- payload property: `location_profile`
- typed domain/action field: `profile`
- stable ForgeDNA lock-control key: `01-location-profile`

The operator also reported the Name lock indicator. Process inspection established that the recheck was running the stale default executable at `C:\dev\CharacterForge\target\release\character-forge-desktop.exe`. The R1 isolated-target build had not replaced it because Windows denied replacement while the old process was running. R1's verified renderer fix remains in source and its frontend regression passes; the Name display fix has not yet had an operator check on the current binary.

## Correction and regression coverage

The Location descriptor now uses action key `profile` while reading its value from `location_profile`. Shared action, lock, lock-state, and origin resolution use the domain field key, which maps to the existing `01-location-profile` control. No Rust field key was renamed and no Tauri compatibility alias was added.

Frontend regressions use the typed LocationCandidate-shaped payload and verify:

- `api.setLock('profile', true)` is called and the `01-location-profile` returned control renders `Locked`, `Unlock Location profile`, and `aria-pressed="true"`.
- Profile reroll calls `api.reroll('profile')`.
- Profile edit submits the supported enum value `ruins` as `api.editField('profile', 'ruins')`.
- The existing Location Name R1 and Character Name lock regressions remain green.

The core lock/reroll test now locks Profile alongside Name and Secret and confirms its value and nonce survive reroll-all-unlocked.

## Verification

- `scripts/verify.ps1`: PASS using `CARGO_TARGET_DIR=target/mission-b1-r2-verify`.
- Rust workspace: **131 passed, 0 failed, 1 ignored** (the existing one-time fixture-writer test). The 1,000-sequence state gauntlet passed.
- Frontend: **40 passed**.
- Frontend format, lint, TypeScript typecheck, and production build: PASS.
- `cargo fmt --all -- --check`: PASS.
- Strict Clippy (`cargo clippy --workspace --all-targets -- -D warnings`): PASS through the verification script.
- Cargo metadata (`cargo metadata --format-version 1 --no-deps`): PASS.
- Optimized Windows Tauri no-bundle build: PASS.
- `git diff --check`: PASS.
- Required `npm test` attempt stopped before tests because the global npm shim references missing `C:\Users\pontu\AppData\Roaming\npm\node_modules\npm\bin\npm-cli.js`. Repository-managed `corepack pnpm test` passed all 40 tests.

## Current executable for the operator check

The exact current verification executable is:

`C:\dev\CharacterForge\apps\desktop\src-tauri\target\mission-b1-r2-verify\release\character-forge-desktop.exe`

Last write: **2026-09-29 15:10:57.056 UTC**. Size: 13,317,120 bytes. The default `target/release` executable was not treated as current.

## Final minimal operator recheck

Launch only the exact executable above, then:

1. Forge/open a Location.
2. Lock Name; confirm `Locked` and `Unlock Name`.
3. Lock Location profile; confirm `Locked` and `Unlock Location profile`.
4. Unlock profile and reroll it; confirm no error appears.

The operator reported the remainder of the full B1 smoke passed. Do not repeat the full smoke unless this check exposes another issue.

## Git state and exit

- Branch: `master`
- HEAD: `e76cc225f0071cc77a57d9e47b6b773376d20639` (`docs: prepare Location Forge core mission`)
- Worktree: B1/R1 changes remain uncommitted; R2 source and documentation changes are also uncommitted. Four pre-existing Mission 09/operator files remain untouched. No commit was created.

**Exit:** stop here and await the final minimal operator recheck on the exact R2 executable. Do not begin B2.
