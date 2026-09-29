# THE FORGE B1.R1 — Location Name Lock Indicator Report

**Automated remediation status: PASS.** The minimal operator recheck is pending, so Mission B1 remains **PARTIAL — R1 OPERATOR RECHECK REQUIRED**. B2 was not started.

## Reproduced issue and correction

The Name field renderer hardcoded `03-name`, which is Character's field key. Location uses the stable key `02-name`. The lock mutation already resolved the field generically by suffix, so Location locking worked but its `Locked` tag, pressed styling, and Unlock label were rendered from the wrong control.

The renderer now uses the shared `fieldControl(payload, field)` resolver, which resolves the suffix for Character, Quest, and Location. No field keys were renamed and no lock mutation behavior changed.

## Regression evidence

Added a frontend test backed by a typed LocationCandidate-shaped fixture with the canonical Location control keys and a locked `02-name`. The test verifies:

- `api.setLock('name', true)` is called;
- the Name field shows its `Locked` tag;
- the action is accessible as `Unlock Name`;
- its `aria-pressed` value is `true`.

The existing Character Name-lock regression also passes.

## Verification

- Regression reproduced before the fix: test failed because the Name field lacked the Locked tag while `02-name` was locked.
- Repository frontend tests: **38 passed**.
- Frontend formatting, lint, TypeScript typecheck, and production build: PASS.
- `scripts/verify.ps1`: PASS with `CARGO_TARGET_DIR=target/mission-b1-r1-verify`; it ran frontend gates, Rust workspace tests, strict Clippy, Rust formatting, and optimized Tauri no-bundle build.
- Rust workspace: **131 passed, 0 failed, 1 ignored** (existing one-time schema fixture writer); the existing 1,000-sequence state gauntlet passed.
- Optimized binary: `apps/desktop/src-tauri/target/mission-b1-r1-verify/release/character-forge-desktop.exe`.
- `npm test`: attempted as required; it exits before tests because the global npm launcher references missing `C:\Users\pontu\AppData\Roaming\npm\node_modules\npm\bin\npm-cli.js`. Repository-managed pnpm tests passed.
- Default-target `scripts/verify.ps1` attempt reached Tauri build but Windows denied replacement of `target/release/character-forge-desktop.exe` (`Access is denied`). The isolated target build passed and left that executable untouched.
- `git diff --check`: PASS.

## Operator evidence and remaining B1 gate

The operator reports that all other B1 smoke behavior passed and this lock indicator was the only observed issue. R1 is the narrow correction. B1 requires only this recheck on the rebuilt app:

1. Launch the rebuilt app and Forge/open a Location.
2. Lock Name.
3. Confirm Name displays `Locked` and the action reads `Unlock Name`.

Until that recheck passes, B1 is not PASS.

## Git state

- Branch: `master`
- HEAD: `e76cc225f0071cc77a57d9e47b6b773376d20639` (`docs: prepare Location Forge core mission`)
- Worktree remains uncommitted. Existing B1 changes and the four pre-existing Mission 09/operator files were preserved. No commit was created.

**Exit:** stop here. Do not begin B2.
