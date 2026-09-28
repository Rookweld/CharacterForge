# THE FORGE — DISCOVERY STATE

## Current Discovery

01 — Entity Universe

## Status

**CANONICALIZED — READY FOR FIRST IMPLEMENTATION MISSION**

Discovery 01 design framing and repository-backed schema audit are complete enough to prepare the first bounded implementation mission.

Implementation mission THE_FORGE_A_SUBSTRATE is PREPARED but NOT STARTED.

## Product Baseline

Character Forge / Quest Forge v1.0.0 FOUNDATION

Verified current release baseline before post-v1 design work:

- branch: master
- release HEAD: 98777fefa0eea7de11371602c3a496f854065ef9
- local tag at HEAD: v1.0.0
- release commit: release: prepare Character Forge v1.0.0 FOUNDATION

Existing unrelated untracked files intentionally left untouched:

- The Ashen Claim.json
- The Ashen Claim.md
- mission09-final-verify.log
- mission09-rust-test.log

Current post-v1 design files are uncommitted/untracked until the human chooses the design commit.

## Current Design Phase

Discovery 01 canonicalization complete.

Current next phase:

**THE_FORGE_A_SUBSTRATE — schema-2 compatibility foundation + first-class Relationship persistence**

Mission file:

GAZINGA/missions/THE_FORGE_A_SUBSTRATE.md

## Canonical Process Files

- GAZINGA/CANON_CORE.md — existing FOUNDATION constitution
- GAZINGA/THE_FORGE_CANON_CORE.md — post-v1 The Forge constitution
- GAZINGA/THE_FORGE_OPERATING_MODEL.md — Rook / Bridge / Codex workflow
- GAZINGA/THE_FORGE_CODEX_LAUNCHER_PROMPT.txt — bounded Codex launcher

## Canonical Design Files

- docs/design/THE_FORGE_DESIGN_INDEX.md
- docs/design/THE_FORGE_ENTITY_UNIVERSE.md
- docs/design/ENTITY_SCHEMA_REFERENCE.md
- docs/design/ENTITY_SUBSTRATE_AND_MIGRATION.md
- docs/design/FORGE_FURTHER_AND_CONTEXT.md
- docs/design/VISUALS_AND_ENGINE_BOUNDARY.md

## Repository Audit Findings

Discovery 01 inspected the actual v1 repository.

Confirmed:

- CURRENT_SCHEMA_VERSION = 1
- EntityEnvelope is already broadly universal
- EntityType currently contains Character and Quest
- entities already stores generic queryable envelope columns + body_json
- entities.entity_type CHECK currently admits only character/quest
- entity_revisions stores immutable Canon revision snapshots
- forge_runs already stores persistent generation history with independent event identity
- settings is persisted
- current Character contract includes gear_links, faction_links, relationship_links
- current Character generator initializes those three vectors empty
- schema-0→1 migration already uses WAL checkpoint, pre-migration recovery copy, transaction, and future-schema rejection
- forgeproject format v1 is independent from SQLite schema version

Design consequence:

**Extend the proven v1 substrate. Do not perform a clean-slate persistence rewrite.**

## PROVISIONAL Design Locks

- stable Entity identity
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
- no automatic relationship inference during migration
- v1 ForgeDNA remains unchanged unless explicitly versioned
- The Forge remains authoring source of truth
- engines consume exports
- schema evolution must preserve v1 Character/Quest meaning

## Initial Entity Set

- Character
- Quest
- Location
- Faction
- Item
- Creature

## First Expansion Priority

1. Location
2. Faction
3. Item
4. Creature

## First Proof Slice

    Existing Character
      → Forge Hometown
      → Location Candidate
      → proposed born_in Relationship
      → atomic persistence
      → Candidate/Canon/history
      → close/reopen
      → export

The first proof slice is NOT part of THE_FORGE_A_SUBSTRATE. It belongs to the following PLACE mission after the substrate passes.

## Prepared First Implementation Mission

THE_FORGE_A_SUBSTRATE is bounded to:

- schema 1 → schema 2 safe migration
- frozen real schema-1 fixture
- exact semantic migration proof
- future EntityType vocabulary
- first-class Relationship contracts
- four minimal relationship definitions:
  - born_in
  - member_of
  - owns
  - opposed_by
- Relationship persistence/lifecycle/revisions/stale writes
- Character/Quest determinism regression
- Vault compatibility
- no new user-visible Forge types

## Open Questions

These remain explicitly unresolved unless a mission requires them:

1. Manual Link Existing: Candidate relationship or Canon immediately?
2. Entity + Relationship Canon promotion UX?
3. Exact future mapping of existing forge_runs into richer ForgeOperation provenance?
4. Degree of metadata-driven Workbench rendering?
5. Hard-vs-warning cardinality policies beyond the minimal relationship proof set?
6. Exact Visual Canon terminology/UX?
7. Future World/Lore epistemic model?

THE_FORGE_A_SUBSTRATE is intentionally scoped so questions 1–4 do not require speculative product answers.

## Explicitly Deferred

- Location Forge implementation until substrate PASS
- Faction Forge
- Item Forge
- Creature Forge
- full Forge Further matrix
- multi-source Forge
- Visual generation implementation
- Visual Asset persistence
- Game Schema Packs
- engine adapters
- World Forge
- Ability/Spell Forge
- AI providers
- Graph UI
- Timeline
- collaboration
- cloud
- arbitrary plugin code execution

## Design Consistency Check

Completed after canonicalization:

- no TODO placeholders remain in docs/design
- git diff --check passed
- design docs distinguish LOCKED / PROVISIONAL / OPEN / DEFERRED
- physical schema statements are grounded in the current repository audit
- current v1 storage model overrides earlier whiteboard assumptions where they differed
- no implementation files were changed during Discovery 01 canonicalization

## Next Action

Before Codex implementation:

1. review git status
2. make a clean docs/design discovery commit if desired, staging ONLY the new The Forge design/process/mission files
3. leave the four unrelated operator/log files untouched
4. then launch Codex using GAZINGA/THE_FORGE_CODEX_LAUNCHER_PROMPT.txt
5. current mission: GAZINGA/missions/THE_FORGE_A_SUBSTRATE.md

After Codex stops, Rook should inspect the actual report/diff/tests through Nullfeather Bridge rather than requiring the full report to be pasted into chat.

## Resume Instruction

A new ChatGPT conversation can resume with:

    Connect to Nullfeather Bridge.
    Read C:\dev\CharacterForge\GAZINGA\DISCOVERY_STATE.md
    and continue the current The Forge work.

## Last Updated

2026-09-28
