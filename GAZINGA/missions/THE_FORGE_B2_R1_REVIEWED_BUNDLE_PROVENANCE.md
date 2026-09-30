# EXECUTION CONTRACT

You are executing one bounded remediation on the CURRENT uncommitted B2 worktree.

Before coding:

1. Read GAZINGA/CANON_CORE.md.
2. Read GAZINGA/THE_FORGE_CANON_CORE.md.
3. Read GAZINGA/DISCOVERY_STATE.md.
4. Read GAZINGA/missions/THE_FORGE_B2_HOMETOWN.md.
5. Read docs/missions/THE_FORGE_B2_HOMETOWN_REPORT.md.
6. Read this remediation completely.
7. Inspect the actual current Tauri session/bundle mutation path and storage bundle validation before editing.

Do not reset/discard current B2 changes.
Do not begin any later mission.

# THE FORGE B2.R1 — REVIEWED HOMETOWN DNA / BUNDLE PROVENANCE PARITY

## Independent review finding

B2 automated closeout reported PASS, but independent post-Codex review found an uncovered workflow mismatch.

Current review flow:

1. forge_hometown creates:
   - session.hometown = original ForgeResultBundle
   - session.working = same LocationCandidate
2. Location lock/reroll/edit/undo/redo mutate session.working through the shared candidate commands.
3. save_forge_bundle replaces bundle.primary with the final session.working LocationCandidate.
4. However, bundle.recipe.location_dna remains the ORIGINAL pre-review DNA.
5. forge-storage rejects the save when:

       bundle.recipe.location_dna != bundle.primary.dna

This means a reviewed Hometown whose DNA changed — especially Lock/Unlock, single-field Reroll, Reroll All, or Undo/Redo across those operations — can fail Save Bundle even though B2 explicitly promises normal Location Candidate shaping before atomic save.

Manual text editing may not change DNA and therefore can mask this defect.

This is a B2 exit-gate blocker because:
- normal Location lock/reroll behavior is part of the B2 product contract;
- final persisted provenance must describe the exact reviewed Candidate that is saved;
- the frontend test currently saves the untouched generated Hometown and therefore does not cover this path.

Re-verify this source behavior before editing.

## Required invariant

At atomic Save Bundle time:

- primary Location snapshot = exact reviewed session.working candidate
- recipe/context source snapshot remains the ORIGINAL frozen source context
- action_id/action_version/context_recipe/context_hash/action_seed remain unchanged
- recipe.location_dna = exact FINAL reviewed primary.dna
- result mapping retains the same primary Entity ID and Relationship ID/type
- born_in target remains the reviewed Location identity
- source Character remains unchanged
- one atomic transaction persists final Location + relationship + immutable operation provenance

The persisted Forge Further recipe must therefore preserve:
- frozen source/action provenance
- final reviewed Location ForgeDNA sufficient to reproduce the final generated field state/nonces/locks

Do not rewrite the context hash after Location review; the source context did not change.

## Preferred implementation direction

Synchronize the bundle's review-dependent primary provenance from the exact final working LocationCandidate before calling forge-storage.

A small typed helper in app-core or Tauri is acceptable if it centralizes and validates:

- same primary Entity ID
- final LocationCandidate validity
- recipe.location_dna = primary.dna
- result.primary_output_id remains primary Entity ID
- relationship target remains primary Entity ID

Do NOT weaken forge-storage's equality check merely to make saves pass.

Storage must continue rejecting internally inconsistent bundles.

## Required regression tests

### A. Core/storage — lock then save

1. create saved eligible Character
2. forge Hometown
3. lock a Location field, causing final DNA lock state to differ from original bundle recipe DNA
4. synchronize/finalize reviewed bundle through the same application helper used by production
5. save atomically
6. assert:
   - save succeeds
   - persisted Location DNA equals final reviewed DNA
   - recipe_json location_dna equals final reviewed DNA
   - source context hash/source snapshot unchanged
   - born_in target ID unchanged
   - source Character unchanged

### B. Core/storage — reroll then save

1. forge Hometown
2. reroll one unlocked Location field
3. assert DNA nonce changes
4. finalize reviewed bundle
5. atomic save succeeds
6. persisted Location snapshot equals rerolled Candidate
7. persisted recipe location_dna equals rerolled DNA
8. close/reopen preserves exact final state/provenance

### C. Reroll all / lock firewall

- lock one Location field
- reroll all unlocked
- save bundle
- locked field value/nonce remains preserved
- final DNA in provenance equals final Candidate DNA

### D. Undo/Redo

Where current Tauri/session test architecture permits:
- Forge Hometown
- reroll field
- Undo => save must persist the undone/current DNA
- or Redo => save must persist the redone/current DNA
- no stale original recipe DNA survives accidentally

At minimum prove the production finalization helper uses the current working candidate, not cached initial candidate.

### E. Frontend/Tauri workflow regression

Extend the B2 UI regression so the operator path is represented:

- Forge Hometown
- invoke at least one Location lock or reroll action
- Save Bundle
- backend save is invoked after the reviewed candidate update
- no fallback to ordinary Save Candidate

A Rust/Tauri command-boundary test is strongly preferred for exact bundle synchronization because mocked frontend APIs cannot prove internal Rust provenance.

## Failure semantics

If the current working candidate:

- is not LocationCandidate
- has a different Entity ID from the bundle primary/result mapping/relationship target
- is invalid
- otherwise cannot be reconciled safely

then Save Bundle must fail clearly and persist nothing.

Never silently replace Entity identity.

## Scope

Expected files:

- apps/desktop/src-tauri/src/lib.rs
- possibly crates/app-core/src/forge_further.rs for a typed review-finalization helper
- crates/storage/src/tests.rs and/or Tauri tests
- apps/desktop/tests/App.test.tsx only where useful
- docs/missions/THE_FORGE_B2_HOMETOWN_REPORT.md
- GAZINGA/DISCOVERY_STATE.md
- dedicated R1 report

Avoid unrelated UI or storage refactors.

UI-001 remains separate P3 debt.

## Verification

Run:

- focused new reviewed-bundle tests
- cargo test -p forge-storage for relevant bundle tests
- cargo test --workspace --no-fail-fast
- frontend tests
- frontend format/lint/typecheck/build
- strict Clippy
- scripts/verify.ps1 with isolated target if required
- git diff --check

Do not update deterministic goldens to hide a regression.

## Documentation

Create:

docs/missions/THE_FORGE_B2_R1_REVIEWED_BUNDLE_PROVENANCE_REPORT.md

Update:

- docs/missions/THE_FORGE_B2_HOMETOWN_REPORT.md
- GAZINGA/DISCOVERY_STATE.md
- GAZINGA/PROJECT_STATE.md only if needed

Correct the B2 status from PASS to PARTIAL while remediation is in progress.

## Exit gate

B2.R1 PASS only if all are true:

- a Hometown can be locked/rerolled before Save Bundle and still save successfully
- persisted primary snapshot is exactly the reviewed LocationCandidate
- persisted recipe.location_dna is exactly the final reviewed DNA
- frozen source Context Snapshot/hash remains unchanged
- relationship target/result mapping preserve stable Location identity
- source Character remains unchanged
- stale-source/rollback/duplicate-save protections remain green
- close/reopen preserves final reviewed state and provenance
- full verification passes
- no P0/P1/P2 defect remains known

STOP.
Do not begin the next entity mission.
