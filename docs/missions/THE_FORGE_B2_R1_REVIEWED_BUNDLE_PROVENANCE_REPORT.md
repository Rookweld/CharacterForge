# THE FORGE B2.R1 — Reviewed Bundle Provenance Report

**Status: PASS — reviewed Location DNA is persisted with frozen source provenance**

## Finding and correction

Independent review found that Hometown lock/reroll/edit/undo/redo could change `session.working` while the cached `session.hometown.recipe.location_dna` remained at generation-time DNA. Save Bundle then supplied the current primary with stale recipe DNA, and storage correctly rejected the mismatch.

Added `finalize_hometown_review` in app-core and routed the Tauri Save Bundle command through it. It validates the reviewed Location candidate and requires its Entity ID to match the original primary, result mapping, and relationship target. It clones the original bundle and changes only the primary candidate and `recipe.location_dna`. The source context snapshot/hash, action id/version, context recipe, action seed, result mapping, and relationship identity remain frozen. Storage equality validation was not weakened.

## Regression coverage

- App-core verifies finalization uses the reviewed candidate, preserves source/action provenance, and rejects primary identity replacement.
- Storage locks Name, rerolls all unlocked fields, then saves atomically. It verifies the locked value and nonce, exact persisted candidate/recipe DNA, frozen context, stable relationship target, and unchanged source Character.
- Storage rerolls Purpose, verifies changed DNA, saves atomically, closes/reopens, and verifies the exact final Location, recipe DNA, context snapshot, and history result snapshot.
- Frontend regression forges Hometown, locks Name, then calls Save Bundle; it asserts ordinary Save Candidate is not used.
- Existing storage stale-source, rollback, duplicate-save, project export/import, and close/reopen tests remain green.

## Verification

- Focused reviewed Hometown storage tests: **2 passed**.
- App-core Forge Further tests: **5 passed**.
- Frontend tests: **42 passed**.
- Full workspace: **143 Rust tests passed, 1 existing one-time fixture writer ignored**. This includes the deterministic 1,000-sequence state gauntlet.
- `scripts/verify.ps1`: PASS with `CARGO_TARGET_DIR=target/mission-b2-verify`; includes frontend format/lint/typecheck/tests/build, Rust format, strict Clippy, full workspace tests, and optimized Tauri no-bundle build.
- Optimized executable: `apps/desktop/src-tauri/target/mission-b2-verify/release/character-forge-desktop.exe`.
- `git diff --check`: PASS.
- One initial full run hit an existing timestamp-sensitive relationship lifecycle assertion whose `updated_at_unix_ms` reads differed by 1 ms. The isolated test passed, and the complete rerun passed.
- `npm test` was attempted per repository agreement but the configured npm shim cannot find `C:\Users\pontu\AppData\Roaming\npm\node_modules\npm\bin\npm-cli.js`; the repository's Corepack/pnpm workflow passed.

## Mission state and boundary

B2 and B2.R1 are **PASS**. Existing P3 UI-001 (intermittent Location lock visual feedback) remains documented and separate. No P0/P1/P2 blocker remains known for B2. No later entity mission was started.

`GAZINGA/DISCOVERY_STATE.md` was rewritten as clean UTF-8 to repair prior mojibake; it now records B2.R1 PASS. `GAZINGA/PROJECT_STATE.md` and the B2 report record the final status and verification evidence.

- Branch: `master`
- HEAD: `48905176ef9fe86799c8a2cfbc3dbc6eef177b4c` (`docs: prepare Forge Hometown mission`)
- Worktree: B2 implementation and closeout remain uncommitted/unstaged. Four pre-existing operator/export/log files remain untouched.
- Schema: 2, unchanged.

**B2.R1 exit gate: PASS. Stop here; do not begin the next entity mission.**
