# THE FORGE — DISCOVERY STATE

## Current Discovery

01 — Entity Universe

## Status

**MISSION A PASS — ENTITY/RELATIONSHIP SUBSTRATE COMMITTED**

Mission A and bounded R1 remediation are closed as PASS.

Current implementation baseline:

- branch: `master`
- HEAD: `8edc134` — `feat: add The Forge entity relationship substrate`
- prior design baseline: `31f69ac`
- v1.0.0 FOUNDATION release baseline: `98777fe`

Only four old unrelated files remain untracked:

- `The Ashen Claim.json`
- `The Ashen Claim.md`
- `mission09-final-verify.log`
- `mission09-rust-test.log`

Do not absorb them into future commits.

## Verified Mission A Result

Schema 2 is active.

Verified:

- schema 1 → 2 migration with WAL checkpoint, no-clobber recovery copy, transaction and rollback
- frozen populated schema-1 fixture
- semantic preservation of Character/Quest content, IDs, revisions, history and settings
- Character v1/current v2 replay preserved
- Quest v2/current v3 replay preserved
- lock/reroll behavior preserved
- six EntityType serialization keys:
  - Character
  - Quest
  - Location
  - Faction
  - Item
  - Creature
- first-class Relationship persistence
- four app-owned Relationship Definitions:
  - `born_in`
  - `member_of`
  - `owns`
  - `opposed_by`
- Relationship Candidate → Canon lifecycle
- Relationship Canon revision parity:
  - R1=A
  - R2=B
  - R3=C
  - current=C
- stale Relationship writes rejected without mutating history
- Relationship chain survives close/reopen
- Vault / forgeproject format v1 compatibility retained
- no Location/Faction/Item/Creature generator or unfinished Forge Further UI exposed

Codex full verification reported:

- 121 Rust tests passed
- 36 frontend tests passed
- `scripts/verify.ps1` PASS
- `git diff --check` PASS

Independent Bridge review reran:

- focused Relationship A/B/C lifecycle — PASS
- full `forge-storage` suite — 23 passed, 1 ignored one-time fixture writer
- 1,000-sequence deterministic state gauntlet — PASS

Reports:

- `docs/missions/THE_FORGE_A_SUBSTRATE_REPORT.md`
- `docs/missions/THE_FORGE_A_R1_RELATIONSHIP_REVISION_PARITY_REPORT.md`

## Canonical Process Files

- `GAZINGA/CANON_CORE.md`
- `GAZINGA/THE_FORGE_CANON_CORE.md`
- `GAZINGA/THE_FORGE_OPERATING_MODEL.md`
- `GAZINGA/THE_FORGE_CODEX_LAUNCHER_PROMPT.txt`

## Canonical Design Files

- `docs/design/THE_FORGE_DESIGN_INDEX.md`
- `docs/design/THE_FORGE_ENTITY_UNIVERSE.md`
- `docs/design/ENTITY_SCHEMA_REFERENCE.md`
- `docs/design/ENTITY_SUBSTRATE_AND_MIGRATION.md`
- `docs/design/FORGE_FURTHER_AND_CONTEXT.md`
- `docs/design/VISUALS_AND_ENGINE_BOUNDARY.md`

## Design Locks

- stable Entity identity
- immutable revision history
- typed payloads
- profiles instead of entity explosion
- first-class Relationships
- Forge Further is data-driven
- generation proposes; humans establish Canon
- Canon-first bounded context
- frozen version-specific Context Snapshots
- provider-neutral context
- Forge Result Bundles
- no automatic concept promotion
- no automatic Relationship inference during migration
- v1 ForgeDNA remains unchanged unless explicitly versioned
- The Forge remains authoring source of truth
- engines consume exports
- schema evolution preserves v1 Character/Quest meaning

## Initial Entity Universe

1. Character — implemented
2. Quest — implemented
3. Location — next
4. Faction
5. Item
6. Creature

## Refined PLACE Sequence

To keep implementation bounded, PLACE is split:

### THE_FORGE_B1_PLACE_CORE

Standalone typed deterministic Location Forge.

Must prove Location can use the universal lifecycle:

- deterministic generation
- locks/rerolls/manual edits
- persistence
- history
- Candidate/Canon/revisions
- close/reopen
- JSON/Markdown export
- desktop Workbench integration

It must NOT implement Forge Further or Character → Hometown yet.

### THE_FORGE_B2_HOMETOWN

First full Forge Further proof slice:

    Existing saved Character
      → Forge Hometown
      → bounded Context Recipe
      → frozen Context Snapshot
      → Location Candidate
      → proposed born_in Relationship
      → atomic bundle persistence
      → review/history/close-reopen

B2 starts only after B1 PASS.

## Open Questions

Still OPEN unless a future mission requires a decision:

1. Manual Link Existing: Candidate or Canon by default?
2. Entity + Relationship Canon promotion UX?
3. Exact long-term evolution of `forge_runs` into richer ForgeOperation provenance?
4. Degree of metadata-driven Workbench rendering?
5. Hard-vs-warning cardinality policies beyond current minimal Relationship definitions?
6. Visual Canon terminology/UX?
7. World/Lore epistemic model?

B1 does not require these answers.

## Explicitly Deferred

- Forge Further until B2
- Faction Forge
- Item Forge
- Creature Forge
- full six-entity Forge Further matrix
- multi-source Forge
- Visual generation / Visual Asset persistence
- Game Schema Packs
- Unity/Unreal/Godot adapters
- World Forge
- Ability/Spell Forge
- AI providers
- Graph UI
- Timeline
- collaboration/cloud
- arbitrary plugin code execution

## Next Action

Execute:

`GAZINGA/missions/THE_FORGE_B1_PLACE_CORE.md`

using:

`GAZINGA/THE_FORGE_CODEX_LAUNCHER_PROMPT.txt`

Do not start B2 in the same Codex session.

## Resume Instruction

A new ChatGPT conversation can resume with:

    Connect to Nullfeather Bridge.
    Read C:\dev\CharacterForge\GAZINGA\DISCOVERY_STATE.md
    and continue the current The Forge work.

## Last Updated

2026-09-29
