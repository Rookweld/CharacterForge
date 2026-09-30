# THE FORGE — DISCOVERY STATE

## Current Phase

THE_FORGE_B3_FACTION_CORE — prepared for execution

## Status

**B2 PASS — B3 FACTION CORE AUTHORIZED**

B2 Hometown and B2.R1 reviewed-bundle provenance are closed as PASS.

Independent review confirmed the B2.R1 fix:
- reviewed Location DNA is finalized from the exact current Candidate before atomic Save Bundle
- frozen Character source Context Snapshot/hash remains unchanged
- Location/Relationship identity remains stable
- stale-source/rollback/duplicate-save protections remain intact
- focused reviewed-bundle tests pass
- full Codex verification passed with 143 Rust tests, 1 existing fixture-writer ignored, and 42 frontend tests

B2 is committed at:

- `fe10c6c` — `feat: add Forge Hometown flow`

The operator has authorized continued non-destructive development through bounded missions.

## Current Repository State

- branch: `master`
- HEAD: `fe10c6c`
- database schema: 2
- Character / Quest compatibility: green
- standalone Location Forge: implemented
- first-class Relationships: implemented
- Forge Hometown: implemented and verified
- next bounded mission: Faction Forge core

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

Automated B1 evidence:
- 131 Rust tests passed
- 40 frontend tests passed
- 1 existing fixture-writer test ignored
- 1,000-sequence state gauntlet passed
- `scripts/verify.ps1` PASS
- strict Clippy PASS
- frontend format/lint/typecheck/build PASS
- optimized Tauri build PASS
- `git diff --check` PASS

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

B2 introduced the first full Forge Further proof slice:

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
- persisted `recipe.location_dna` equals the exact final reviewed Candidate DNA
- source Context Snapshot/hash remains frozen
- stable primary Entity ID and Relationship target are preserved
- close/reopen retains final reviewed state/provenance

Independent review reran the focused B2.R1 tests successfully.

## B2 Design Decisions

- Forge Further generation creates Candidate Entity + Candidate Relationship.
- Source entities are never mutated by generation.
- Link Existing is not part of B2.
- Entity/Relationship Canon promotion UX remains separate.
- `forge_runs` remains the durable operation/event backbone.
- Context compilation is bounded, deterministic, read-only, and Canon-aware.
- exact source state used by generation is frozen in provenance.
- review-time Location DNA may change; frozen source context must not.

## Next Mission

`GAZINGA/missions/THE_FORGE_B3_FACTION_CORE.md`

Scope:

**Standalone deterministic Faction Forge only.**

B3 must not implement:
- Faction Forge Further actions
- Item Forge
- Creature Forge
- Link Existing
- Visual generation
- engine adapters

## Still Open

- Manual Link Existing Candidate vs Canon default
- bundled Entity + Relationship Canon promotion UX
- richer multi-source Forge
- generalized relationship cardinality UX
- Visual Canon
- World/Lore epistemic truth

None are required for B3.

## Deferred After B3

- Item Forge
- Creature Forge
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
