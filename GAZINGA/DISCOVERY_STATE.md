# THE FORGE — DISCOVERY STATE

## Current Phase

THE_FORGE_B2_HOMETOWN — first Forge Further proof slice

## Status

**B1 ACCEPTED FOR CONTINUED DEVELOPMENT WITH KNOWN P3 UI DEBT**

The human operator approved continuing development despite one remaining non-blocking Workbench feedback defect:

- functional Location locks hold
- lock visual state can fail to show Locked / Unlock correctly
- tracked in GAZINGA/KNOWN_ISSUES.md as UI-001
- must be revisited before a release-quality gate

## Current Baseline

- branch: master
- B1 implementation is ready to be committed before B2 starts
- v1.0.0 FOUNDATION release baseline remains protected
- schema version: 2
- Character / Quest compatibility remains green
- Location standalone Forge is implemented
- first-class Relationships are implemented

Unrelated files to preserve and never absorb:

- The Ashen Claim.json
- The Ashen Claim.md
- mission09-final-verify.log
- mission09-rust-test.log

## Verified B1 Result

Location now supports:

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

Automated evidence after R2:

- 131 Rust tests passed
- 40 frontend tests passed
- 1 existing fixture-writer test ignored
- 1,000-sequence state gauntlet passed
- scripts/verify.ps1 PASS using isolated target
- strict Clippy PASS
- frontend format/lint/typecheck/build PASS
- optimized Tauri build PASS
- git diff --check PASS

Human smoke:
- all requested B1 workflows functioned
- Location Name/Profile locks functionally held
- visual locked feedback remained unreliable
- operator explicitly approved moving on

## Known Issue

See:

GAZINGA/KNOWN_ISSUES.md

UI-001 is non-blocking for B2 but must remain open.

## Next Mission

GAZINGA/missions/THE_FORGE_B2_HOMETOWN.md

Goal:

    Existing saved Character
      → Forge Hometown
      → versioned Forge Action
      → bounded Context Recipe
      → frozen Context Snapshot
      → deterministic Location Candidate
      → proposed born_in Relationship
      → atomic bundle save
      → close/reopen/provenance review

## B2 Design Decisions

For the first proof slice:

- Forge Further generation creates Candidate Entity + Candidate Relationship.
- Source Character is never mutated.
- Link Existing is NOT part of B2.
- Entity/Relationship Canon promotion UX remains separate and deferred.
- Existing forge_runs remains the durable operation/event backbone.
- B2 may extend it with structured context/output persistence rather than replacing historical forge_runs.
- Context compilation is read-only and Canon-first.
- exact source state used by generation must be frozen in provenance.

## Still Open

- Manual Link Existing Candidate vs Canon default
- bundled Entity + Relationship Canon promotion UX
- richer multi-source Forge
- generalized relationship cardinality UX
- Visual Canon
- World/Lore epistemic truth

None are required to complete B2.

## Deferred

- Faction Forge
- Item Forge
- Creature Forge
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

2026-09-29
