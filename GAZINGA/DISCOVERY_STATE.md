# THE FORGE — DISCOVERY STATE

## Current Discovery

01 — Entity Universe (implemented substrate)

## Status

**CANONICALIZED — MISSION A PASS, R1 COMPLETE**

Independent review found a Relationship Canon revision-history mismatch after initial Mission A verification. R1 corrected it and all required regression gates passed.

Schema 2, frozen schema-1 migration evidence, first-class Relationship persistence, revision parity, and Vault compatibility are verified.

R1 report:

`docs/missions/THE_FORGE_A_R1_RELATIONSHIP_REVISION_PARITY_REPORT.md`

## Product Baseline

Character Forge / Quest Forge v1.0.0 FOUNDATION

Verified release baseline before post-v1 work:

- branch: master
- release HEAD: `98777fefa0eea7de11371602c3a496f854065ef9`
- local tag at release HEAD: `v1.0.0`
- release commit: `release: prepare Character Forge v1.0.0 FOUNDATION`

Discovery 01 design/process package is committed at:

- `31f69ac800c07ffdf5ab5f567f80d5593b9684e`
- `docs: canonicalize The Forge entity universe discovery`

The current Mission A implementation and R1 remediation worktree remains uncommitted after independent review passed.

Existing unrelated untracked files intentionally left untouched:

- `The Ashen Claim.json`
- `The Ashen Claim.md`
- `mission09-final-verify.log`
- `mission09-rust-test.log`

## Current Design Phase

Discovery 01 canonicalization is complete.

THE_FORGE_A_SUBSTRATE and its bounded R1 independent-review remediation are complete.

Mission A status:

**PASS — substrate and Relationship Canon revision-history parity verified**

Original mission:

`GAZINGA/missions/THE_FORGE_A_SUBSTRATE.md`

Codex report:

`docs/missions/THE_FORGE_A_SUBSTRATE_REPORT.md`

Remediation specification and evidence:

`GAZINGA/missions/THE_FORGE_A_R1_RELATIONSHIP_REVISION_PARITY.md`

`docs/missions/THE_FORGE_A_R1_RELATIONSHIP_REVISION_PARITY_REPORT.md`

## Canonical Process Files

- `GAZINGA/CANON_CORE.md` — existing FOUNDATION constitution
- `GAZINGA/THE_FORGE_CANON_CORE.md` — post-v1 The Forge constitution
- `GAZINGA/THE_FORGE_OPERATING_MODEL.md` — Rook / Bridge / Codex workflow
- `GAZINGA/THE_FORGE_CODEX_LAUNCHER_PROMPT.txt` — bounded Codex launcher

## Canonical Design Files

- `docs/design/THE_FORGE_DESIGN_INDEX.md`
- `docs/design/THE_FORGE_ENTITY_UNIVERSE.md`
- `docs/design/ENTITY_SCHEMA_REFERENCE.md`
- `docs/design/ENTITY_SUBSTRATE_AND_MIGRATION.md`
- `docs/design/FORGE_FURTHER_AND_CONTEXT.md`
- `docs/design/VISUALS_AND_ENGINE_BOUNDARY.md`

## Repository Audit Findings

Discovery 01 inspected the actual v1 repository.

Confirmed at the pre-Mission-A baseline:

- `CURRENT_SCHEMA_VERSION = 1`
- `EntityEnvelope` was already broadly universal
- `EntityType` contained Character and Quest
- `entities` already stored generic queryable envelope columns + `body_json`
- `entities.entity_type` CHECK admitted only Character/Quest
- `entity_revisions` stored immutable Canon revision snapshots
- `forge_runs` already stored persistent generation history with independent event identity
- settings were persisted
- current Character contract included `gear_links`, `faction_links`, `relationship_links`
- current Character generation initialized those vectors empty
- schema-0→1 migration already used WAL checkpoint, recovery copy, transaction, and future-schema rejection
- forgeproject format v1 was independent from SQLite schema version

Design consequence:

**Extend the proven v1 substrate. Do not perform a clean-slate persistence rewrite.**

Mission A has now implemented schema 2 in the current uncommitted worktree.

## Mission A Evidence So Far

Codex reported:

- schema 1 → 2 transactional migration
- frozen populated schema-1 fixture
- exact migration semantic-preservation checks
- six EntityType serialization keys
- first-class Relationship contracts/persistence
- four minimal Relationship Definitions:
  - `born_in`
  - `member_of`
  - `owns`
  - `opposed_by`
- Relationship Candidate/Canon/archive lifecycle
- stale-write protection
- Vault compatibility
- 121 Rust tests
- 36 frontend tests
- `scripts/verify.ps1` PASS
- `git diff --check` PASS

Independent Bridge review additionally reran and passed:

- `relationship_candidate_lifecycle_is_revisioned_and_stale_safe`
- `frozen_schema1_fixture_migrates_losslessly_and_failure_rolls_back_with_backup`

Independent review found the revision-history mismatch; R1 has now corrected it as described below.

## Independent-Review Resolution

R1 corrected the Relationship Canon revision-history mismatch.

Verified invariant:

    Revision 1 = A
    Revision 2 = B
    Revision 3 = C
    current    = C

Prior revisions remain immutable. A stale expected write is rejected without appending a revision or changing current state, and the exact A/B/C chain survives close/reopen.

Archive semantics remain unchanged: archiving preserves the last Canon assertion as historical evidence.

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
- no automatic Relationship inference during migration
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

The proof slice belongs to THE_FORGE_B_PLACE only after Mission A + R1 pass.

## Open Questions

These remain explicitly unresolved unless a mission requires them:

1. Manual Link Existing: Candidate relationship or Canon immediately?
2. Entity + Relationship Canon promotion UX?
3. Exact future mapping of existing `forge_runs` into richer ForgeOperation provenance?
4. Degree of metadata-driven Workbench rendering?
5. Hard-vs-warning cardinality policies beyond the minimal Relationship proof set?
6. Exact Visual Canon terminology/UX?
7. Future World/Lore epistemic model?

R1 remediation is intentionally scoped so none of these need speculative answers.

## Explicitly Deferred

- Location Forge implementation until Mission A PASS
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

## Next Action

Mission A and R1 are closed as PASS. THE_FORGE_B_PLACE remains not started; this closeout stopped at R1 as requested.

## Resume Instruction

A new ChatGPT conversation can resume with:

    Connect to Nullfeather Bridge.
    Read C:\dev\CharacterForge\GAZINGA\DISCOVERY_STATE.md
    and continue the current The Forge work.

## Last Updated

2026-09-29
