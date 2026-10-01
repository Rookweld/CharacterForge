# THE FORGE — DISCOVERY STATE

## Current Phase

THE_FORGE_B10_COMFYUI_LOCAL_PROVIDER — prepared for execution

## Status

**B9 PASS — B10 LOCAL COMFYUI PROVIDER AUTHORIZED**

B2 Hometown, B2.R1 reviewed-bundle provenance, and B3 Faction Core are closed as PASS.

Accepted implementation commits:

- `fe10c6c` — `feat: add Forge Hometown flow`
- `82440ff` — `feat: add Faction Forge core`
- `8527d51` — `feat: add Item Forge core`
- `821dbeb` — `feat: add Creature Forge core`

The operator has authorized continued non-destructive development through bounded missions.

## Current Repository State

- branch: `master`
- HEAD: `9284c08` (`feat: add Visual Forge foundation`)
- worktree: B10 mission prep only; four unrelated files preserved
- database schema: 2
- forgeproject archive format: v1
- application version: 1.0.0
- Character / Quest compatibility: green
- standalone Location Forge: implemented
- first-class Relationships: implemented
- Forge Hometown: implemented and verified
- standalone deterministic Faction Forge: implemented and verified
- standalone deterministic Item Forge: implemented and verified; B4 closed
- standalone deterministic Creature Forge: implemented and verified; B5 closed
- universal Connections / independent Relationship lifecycle: implemented and verified; B6 closed
- Character → Forge Their Faction: implemented and verified; B7 closed
- Character → Forge Their Signature Item: implemented and verified; B8 accepted at `7b22eb9``r`n- provider-neutral Visual Forge foundation: implemented and verified; B9 accepted at `9284c08`

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

At B4 acceptance: branch `master`, implementation commit `8527d51` (`feat: add Item Forge core`). Current B5 starting HEAD is recorded above. The four unrelated operator/export/log files remain untracked and byte-identical by SHA-256. No commit/tag/push/publication occurred. **STOP at B4. No Creature Forge or Item Forge Further action has begun.** See `docs/missions/THE_FORGE_B4_ITEM_CORE_REPORT.md` and `docs/quality/THE_FORGE_B4_ITEM_SAMPLES.md`.

## Historical B4 Exit Gate

**PASS. STOP.** No later mission is authorized by this closeout. Creature Forge and Item Forge Further remain deferred.

## B5 Result

B5 Creature Core is **PASS — automated exit gates satisfied**. The sixth authoring mode has fourteen required fields, eleven stable profiles, independent generator/content versions 1/1, all eleven presets, deterministic DNA/field streams, locks/rerolls/manual origins, shared undo/redo, Candidate/favorite/Canon/revision/history persistence, Workbench/library/detail and JSON/Markdown/project/Vault support. Standalone Creature creates no Relationships. Both historical working-copy and Canon transition adapters include Creature; actual Name/Profile controls and invalid typed-edit guards are tested.

Final evidence: **182 Rust tests PASS, 1 existing fixture writer ignored; 52 frontend tests PASS**. Format/lint/typecheck/frontend build, Rust format, strict Clippy, full offline Cargo metadata, `scripts/verify.ps1`, explicit `cargo test --workspace --no-fail-fast`, 1,000-sequence gauntlet, optimized Tauri no-bundle build and `git diff --check` PASS. Final Rust suites use `RUST_TEST_THREADS=1`; prior parallel timestamp-test failures remain recorded and are not claimed fixed. Exact executable: `C:\dev\CharacterForge\target\mission-b5-verify\release\character-forge-desktop.exe`, ProductVersion/FileVersion 1.0.0. The npm launcher is unavailable; configured pnpm checks pass.

Quality: 2,816 preset Candidates and 1,024 Chaos seeds validate/replay with zero full-content clones; all eleven profiles, 640 source patterns, 1,013 mixed-source Candidates, maximum source share 11.82%. The 52-sample generated/content-author review rates 17 usable as-is, 35 minor edit, 0 weak; repeated local-encounter motifs and minor authoring needs are documented. This is not independent human acceptance. Existing P3 UI-001 remains open; no B5 P0/P1/P2 remains known. Schema 2, archive v1, app 1.0.0 and old deterministic contracts are unchanged. Mission 08's dependency-advisory verification limitation remains residual maintenance/release risk.

Branch `master`; accepted B5 implementation commit `821dbeb` (`feat: add Creature Forge core`). Four unrelated files remain untracked and preserved. See `docs/missions/THE_FORGE_B5_CREATURE_CORE_REPORT.md` and `docs/quality/THE_FORGE_B5_CREATURE_SAMPLES.md`.

## B5 Exit Gate

**PASS. STOP.** No Creature Forge Further, World Forge or later mission was begun.

## B6 Result

**PASS — automated exit gates satisfied.** Universal saved-entity Connections, metadata-driven source/inverse grammar for the four existing types, actual persisted write versions, explicit Candidate promotion, archive, and read-only history are implemented. No schema/vocabulary/entity-truth change or new Forge action is included. The prior promotion timestamp race is repaired by sharing one monotonic timestamp between the current row and its Canon revision; baseline assertions remain intact.

Final evidence: **187 Rust tests PASS, 1 existing fixture writer ignored; 79 frontend tests PASS**. Normal parallel Rust execution passed both explicit no-fail-fast and final sequential `scripts/verify.ps1` runs, including the 1,000-sequence gauntlet. Format/lint/typecheck/frontend build, Rust format/strict Clippy, full offline Cargo metadata, optimized Tauri no-bundle build and final `git diff --check` PASS. Exact executable: `C:\dev\CharacterForge\target\mission-b6-verify\release\character-forge-desktop.exe`; ProductVersion/FileVersion 1.0.0. The initial overlapping consolidated run hit a Windows test-executable linker lock; its log is preserved and the sequential complete rerun passed. The machine npm launcher remains unavailable; configured pnpm checks pass.

Real reviewed Hometown Candidate → Canon → Archived is visible from both endpoints with immutable revisions, unchanged entity truth and frozen provenance. No B6 P0/P1/P2 remains known. No independent native smoke is claimed. Existing P3 UI-001 and Mission 08's accepted incomplete transitive advisory verification limitation remain documented.

Branch `master`; accepted B6 implementation commit `ce6caf0` (`feat: add relationship lifecycle UI`). Four unrelated files remain untracked and preserved. See `docs/missions/THE_FORGE_B6_RELATIONSHIP_LIFECYCLE_REPORT.md`.

## B6 Exit Gate

**PASS.** B6 is committed at `ce6caf0`.

## B7 Result

**PASS — automated exit gates satisfied.** The second real Forge Further action is saved Character → Forge Their Faction: `character.forge-faction` v1 / `character-faction-v1` v1. One bounded immutable Character context, canonical FactionForge Candidate, Generated Candidate MemberOf, final reviewed DNA parity and atomic entity/relationship/operation save. No source mutation, graph traversal or inverse duplicate. Independent B6 Connections/lifecycle remains green.

Final evidence: **196 Rust tests PASS, one existing fixture writer ignored; 84 frontend tests PASS**. Explicit workspace no-fail-fast and sequential `scripts/verify.ps1` pass, including the 1,000-sequence gauntlet. Frontend format/lint/typecheck/build, Rust format/strict all-target Clippy, offline Cargo metadata, optimized Tauri no-bundle build and final `git diff --check` PASS. Exact executable: `C:\dev\CharacterForge\target\mission-b7-verify\release\character-forge-desktop.exe`, ProductVersion/FileVersion 1.0.0. Global npm shim remains unavailable; configured Corepack/pnpm checks pass.

Quality: 44 generated/content-author examples, four per non-Chaos preset plus four Chaos; exact Character hint/desire/secret relevance in all 44, distinct complete creative bodies and zero generated raw UUID/hash leakage. Repeated quotations and plausible internal reform/secret/exit-support tensions are documented. This is not independent human/native operator acceptance. No B7 P0/P1/P2 remains known. Existing P3 UI-001 and Mission 08's incomplete automated transitive advisory scan remain open residual limitations. Schema 2, archive v1, app 1.0.0, old generator/content versions and stable Hometown contracts remain unchanged.

Branch `master`; accepted B7 implementation commit `78c0e19` (`feat: add Character faction Forge Further`). Four unrelated files remain untracked and preserved. See `docs/missions/THE_FORGE_B7_CHARACTER_FACTION_REPORT.md` and `docs/quality/THE_FORGE_B7_CHARACTER_FACTION_SAMPLES.md`.

## B7 Exit Gate

**PASS.** B7 is committed at `78c0e19`.

## B8 Result

**PASS — automated exit gates satisfied.** The third real Forge Further action is saved Character → Forge Their Signature Item: `character.forge-signature-item` v1 / `character-signature-item-v1` v1. Fifteen exact bounded frozen Character facts, canonical ItemForge, gear-prioritized intent, Generated Candidate Owns, final reviewed DNA parity and atomic Item/relationship/typed immutable operation save. Source context/hash and stable identities remain frozen. No source mutation, graph traversal or inverse duplicate; B6 lifecycle remains independent.

Final evidence: **208 Rust tests PASS, one existing fixture writer ignored; 90 frontend tests PASS**. Explicit workspace no-fail-fast and sequential `scripts/verify.ps1` pass, including the 1,000-sequence gauntlet. Frontend format/lint/typecheck/build, Rust format/strict all-target Clippy, full offline Cargo metadata, optimized Tauri no-bundle build and final `git diff --check` PASS. Exact executable: `C:\dev\CharacterForge\target\mission-b8-verify\release\character-forge-desktop.exe`, ProductVersion/FileVersion 1.0.0. Machine npm shim remains unavailable; configured Corepack/pnpm tests pass.

Quality: 44 individually classified generated/content-author examples, four per ten non-Chaos presets plus four Chaos; all usable with minor editing. Principal gear profile/function, source role/skills/abilities, custody/history/secret/hooks reviewed. All full creative bodies distinct; no generated UUID/hash/game-stat leakage found. Independent executable replay reproduced all 44 snapshots exactly. No independent native/human operator smoke claimed. No known B8 P0/P1/P2 remains. Existing P3 UI-001 and Mission 08's incomplete automated transitive advisory scan remain documented limitations. Schema 2, archive v1, app 1.0.0 and existing generator/content versions unchanged.

Branch `master`; accepted B8 implementation commit `7b22eb9` (`feat: add Character signature item Forge Further`). Four unrelated files remain untracked and preserved. See `docs/missions/THE_FORGE_B8_CHARACTER_SIGNATURE_ITEM_REPORT.md` and `docs/quality/THE_FORGE_B8_CHARACTER_SIGNATURE_ITEM_SAMPLES.md`.

## B8 Exit Gate

**PASS.** B8 is committed at `7b22eb9`.

## B9 Result and Exit Gate

**PASS — automated exit gates satisfied. STOP at B9.** Five saved visual entity families have bounded deterministic provider-neutral recipes/context/briefs, exact current persisted source provenance, secret-safe visible projections, selectable prompt and safe JSON/Markdown export. Stale previews remain frozen and require explicit recompile; export rechecks current source. Quest is unsupported. No provider, credentials, binary asset persistence, Visual Canon, schema/archive/app-version changes or later work.

Verification: **216 Rust tests PASS**, one existing fixture writer ignored; **107 frontend tests PASS**; full format/lint/typecheck/build, strict Clippy, Cargo metadata, explicit no-fail-fast workspace suite, 1,000-sequence / 32,000-operation gauntlet, `scripts/verify.ps1`, optimized isolated Tauri no-bundle build and `git diff --check` PASS. Native operator smoke is not claimed. The existing npm shim limitation remains; configured Corepack/pnpm suites pass. Existing UI-001 and accepted Mission 08 dependency-review limitation remain open.

See `docs/missions/THE_FORGE_B9_VISUAL_FORGE_FOUNDATION_REPORT.md`. No next mission starts. Actual provider/persistence direction requires operator choice.

## Still Open

- Manual Link Existing Candidate vs Canon default
- bundled Entity + Relationship Canon promotion UX
- richer multi-source Forge
- generalized relationship cardinality UX
- Visual Canon
- World/Lore epistemic truth

None are required for B9.

## Deferred After B9

- Creature Forge Further
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

2026-10-01
