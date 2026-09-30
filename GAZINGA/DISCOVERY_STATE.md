# THE FORGE — DISCOVERY STATE

## Current Phase

THE_FORGE_B4_ITEM_CORE — closed at exit gate

## Status

**B4 PASS — standalone Item Core automated exit gates satisfied**

B2 Hometown, B2.R1 reviewed-bundle provenance, and B3 Faction Core are closed as PASS.

Accepted implementation commits:

- `fe10c6c` — `feat: add Forge Hometown flow`
- `82440ff` — `feat: add Faction Forge core`

The operator has authorized continued non-destructive development through bounded missions.

## Current Repository State

- branch: `master`
- HEAD: `983398e5c407156ab18f394b1c1ab22efcdbda2a` (B4 starting/current HEAD)
- database schema: 2
- forgeproject archive format: v1
- application version: 1.0.0
- Character / Quest compatibility: green
- standalone Location Forge: implemented
- first-class Relationships: implemented
- Forge Hometown: implemented and verified
- standalone deterministic Faction Forge: implemented and verified
- standalone deterministic Item Forge: implemented and verified; B4 closed

Unrelated files to preserve and never absorb:

- `The Ashen Claim.json`
- `The Ashen Claim.md`
- `mission09-final-verify.log`
- `mission09-rust-test.log`

## Verified B1 Result

Location supports:

- typed Location payload
- 10 Location profiles
- 12 deterministic fields
- all 11 core presets
- ForgeDNA / field streams
- locks / rerolls / reroll-all
- manual edits / origins
- undo / redo
- Candidate save
- favorite
- Canon promotion / revision
- history
- JSON / Markdown export
- close / reopen
- Vault / forgeproject v1 roundtrip
- desktop Workbench Location mode and library

Human B1 smoke:
- requested workflows functioned
- Location locks functionally held
- visual locked feedback remained unreliable
- operator explicitly approved moving on

## Known Issue

See:

`GAZINGA/KNOWN_ISSUES.md`

UI-001:
- Location lock visual state can fail to show Locked / Unlock correctly
- functional lock behavior remains enforced
- non-blocking P3 debt
- must be revisited before a release-quality gate

Do not silently mark UI-001 resolved from automated tests alone.

## Verified B2 Result

B2 introduced the first complete Forge Further proof slice:

    saved Character
      → Forge Hometown
      → versioned Forge Action
      → bounded Context Recipe
      → frozen Context Snapshot
      → deterministic Location Candidate
      → proposed born_in Relationship
      → atomic Save Bundle
      → close/reopen/provenance

Verified:

- action `character.forge-hometown` v1
- recipe `character-hometown-v1`
- bounded Character semantic context
- source body hash + domain-separated BLAKE3 context hash
- inherited preset and Creative Axes
- deterministic explicit action seed
- Location profile constrained to Settlement
- source Character remains unchanged
- Candidate/Generated `born_in` relationship
- one atomic schema-2 transaction persists Location + Relationship + typed `forge_runs` provenance
- stale source prevents all writes
- injected failures roll back all writes
- duplicate bundle save is rejected
- close/reopen preserves exact bundle/provenance
- forgeproject export/import preserves bundle data
- desktop review/save/discard/Connections flow implemented
- unsupported manual/imported Character provenance is disabled clearly
- 24 deterministic Hometown examples across all 11 presets

B2.R1 additionally proves reviewed Candidate parity:

- lock + reroll-all + Save Bundle succeeds
- single-field reroll + Save Bundle succeeds
- persisted `recipe.location_dna` equals exact final reviewed Candidate DNA
- source Context Snapshot/hash remains frozen
- stable primary Entity ID and Relationship target are preserved
- close/reopen retains final reviewed state/provenance

Independent review reran focused B2.R1 tests successfully.

## B3 Result

Faction is now a first-class typed schema-v2 entity with:

- 13 fields
- 13 profiles
- independent Faction generation kind/version tuple
- all 11 core presets
- deterministic ForgeDNA generation
- locks / rerolls / reroll-all
- manual edits / origins
- undo / redo
- Candidate save
- favorite
- Canon promotion / revision
- history
- JSON / Markdown export
- close / reopen
- forgeproject v1 roundtrip
- fourth Workbench mode
- Faction library/detail

Standalone Faction generation creates no Relationships.

B3 automated evidence:

- 152 Rust tests PASS
- 1 existing fixture-writer test ignored
- 44 frontend tests PASS
- 1,000-sequence deterministic state gauntlet PASS
- `scripts/verify.ps1` PASS
- frontend format/lint/typecheck/build PASS
- Rust format and strict Clippy PASS
- Cargo metadata PASS
- optimized Tauri no-bundle build PASS
- `git diff --check` PASS
- 256 × 11 preset sweep: valid, deterministic, no exact full-content clones
- 1,024-seed Chaos sweep: valid with broad profile/source distribution
- 52-sample content-author review: 40 usable as-is, 12 minor edit, 0 weak

Independent review inspected the typed Faction contract, storage/export/Tauri/UI integration, Name/Profile action/control mapping, reran all Faction app-core tests including preset/Chaos sweeps, reran the Faction storage lifecycle test, and found no new P0/P1/P2 blocker.

## B4 Result

B4 Item Core is **PASS — automated exit gates satisfied**. Item has 13 required fields, 11 stable profiles, independent generator/content versions 1/1, all 11 presets, deterministic DNA/field streams, lock/reroll/edit/origins, shared undo/redo, Candidate/favorite/Canon/revision/history persistence, Item Workbench/library/detail, and JSON/Markdown/project/Vault support. Standalone Item creates no Relationships. Name/Profile actual-control and invalid typed-edit regressions pass. Final adapter review also repaired the omitted Item/Faction historical-working-copy and Canon-revision eligibility cases, with a complete-snapshot parity regression.

Final verification: **166 Rust tests PASS, 1 existing fixture writer ignored; 48 frontend tests PASS**; format/lint/typecheck/frontend build, Rust format, strict Clippy, Cargo metadata, `scripts/verify.ps1`, explicit `cargo test --workspace --no-fail-fast`, 1,000-sequence state gauntlet, optimized Tauri no-bundle build, and `git diff --check` PASS. The exact executable is `C:\dev\CharacterForge\target\mission-b4-verify\release\character-forge-desktop.exe`. The npm launcher remains unavailable (missing npm-cli.js); configured pnpm checks pass. An earlier Relationship lifecycle test showed the known 1 ms timestamp fluctuation; isolated and subsequent full reruns passed without changing Relationship code/tests.

Quality: 2,816 preset-sweep Candidates and 1,024 Chaos seeds validate/replay without exact clones; all 11 profiles; 635 Chaos source patterns; no source group/preset above 11.52%. The 52-sample generated/content-author review rates 22 usable as-is, 30 minor edit, 0 weak; it is not independent human acceptance. Existing P3 UI-001 remains open; no P0/P1/P2 defect remains known in B4. Database schema 2, archive v1, app 1.0.0, and existing entity deterministic versions are unchanged. Mission 08's accepted transitive-advisory verification limitation remains documented.

Branch `master`, starting/final HEAD `983398e5c407156ab18f394b1c1ab22efcdbda2a`; B4 changes remain unstaged/uncommitted. The four unrelated operator/export/log files remain untracked and byte-identical by SHA-256. No commit/tag/push/publication occurred. **STOP at B4. No Creature Forge or Item Forge Further action has begun.** See `docs/missions/THE_FORGE_B4_ITEM_CORE_REPORT.md` and `docs/quality/THE_FORGE_B4_ITEM_SAMPLES.md`.

## B4 Exit Gate

**PASS. STOP.** No later mission is authorized by this closeout. Creature Forge and Item Forge Further remain deferred.

## Still Open

- Manual Link Existing Candidate vs Canon default
- bundled Entity + Relationship Canon promotion UX
- richer multi-source Forge
- generalized relationship cardinality UX
- Visual Canon
- World/Lore epistemic truth

None are required for B4.

## Deferred After B4

- Creature Forge
- Item Forge Further
- Faction Forge Further
- full six-entity Forge Further matrix
- multi-source Forge
- Visual generation
- Game Schema Packs
- engine adapters
- World / Ability Forge
- Graph UI
- Timeline
- collaboration / cloud
- arbitrary plugin code execution

## Resume

Connect to Nullfeather Bridge and read this file first.

## Last Updated

2026-09-30
