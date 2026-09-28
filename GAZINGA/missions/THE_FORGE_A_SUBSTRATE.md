# EXECUTION CONTRACT

You are executing exactly one bounded post-v1 The Forge mission.

Before coding:

1. Read GAZINGA/CANON_CORE.md.
2. Read GAZINGA/THE_FORGE_CANON_CORE.md.
3. Read GAZINGA/PROJECT_STATE.md.
4. Read GAZINGA/DISCOVERY_STATE.md.
5. Read this mission completely.
6. Read these canonical design documents:
   - docs/design/THE_FORGE_DESIGN_INDEX.md
   - docs/design/THE_FORGE_ENTITY_UNIVERSE.md sections 3, 6–9, 14, 19
   - docs/design/ENTITY_SUBSTRATE_AND_MIGRATION.md completely
   - docs/design/FORGE_FURTHER_AND_CONTEXT.md sections 11–18 only
7. Inspect the real repository and git status. Treat repository behavior as evidence; do not assume this mission file perfectly describes implementation details.

During work:

- stay inside mission scope
- preserve all v1.0 passing behavior
- add tests with implementation
- do not weaken existing tests
- do not implement Location/Faction/Item/Creature generators
- do not implement Forge Further UI
- do not implement Visuals, World Forge, AI providers, engine adapters, Graph UI, Game Schema Packs, cloud, collaboration, or timeline
- do not silently change ForgeDNA, field streams, content selection, or v1 Character/Quest contracts
- do not infer new world content from existing prose
- do not auto-create Relationship rows from existing Character link/hint fields
- do not begin the next mission

At the end:

- run every verification gate
- create docs/missions/THE_FORGE_A_SUBSTRATE_REPORT.md
- update GAZINGA/DISCOVERY_STATE.md
- update GAZINGA/PROJECT_STATE.md only where repository baseline facts genuinely changed
- report PASS only if every exit criterion is satisfied
- STOP

# THE FORGE A — SUBSTRATE
## Schema-2 compatibility foundation + first-class Relationship persistence

# Objective

Extend the proven v1.0 persistence/domain substrate just enough to support future The Forge entity families and first-class Relationships, while proving that a real schema-1 project migrates to schema 2 with zero semantic loss.

This mission does NOT deliver a new user-facing Forge type.

The product should behave the same for Character and Quest after the mission.

# Baseline

Expected current Git baseline before post-v1 design docs are committed:

- branch: master
- v1.0.0 release commit/tag already exists
- release commit observed during discovery: 98777fefa0eea7de11371602c3a496f854065ef9
- unrelated operator exports/logs may remain untracked and must not be absorbed

Re-inspect and report actual state rather than trusting these values blindly.

# Current schema reality

The actual v1 repository was audited during Discovery 01.

Re-verify before editing:

- CURRENT_SCHEMA_VERSION = 1
- entities is already a generic envelope + body_json table
- entity_type CHECK currently permits character/quest only
- entity_revisions stores immutable Canon snapshots
- forge_runs stores persistent generation history
- settings exists
- Character includes gear_links/faction_links/relationship_links but current generation leaves them empty
- migration backup/recovery behavior already exists

If repository reality differs, STOP and report the discrepancy before redesigning scope.

# Scope A — Entity Type future-proofing

Extend the typed EntityType vocabulary to include the initial six universe types:

- Character
- Quest
- Location
- Faction
- Item
- Creature

Requirements:

- existing serialized Character/Quest names remain unchanged
- existing validation still rejects payload/type mismatch
- adding enum variants must not create placeholder runtime rows or fake payloads
- do not add Location/Faction/Item/Creature generators
- do not invent empty generic payload structs just to satisfy storage
- if the existing StoredEntity union cannot yet carry new typed entities, leave that boundary explicit and covered by tests/documentation for the next missions

The goal is to make the type vocabulary and relationship registry possible without pretending future entity payloads already exist.

# Scope B — Relationship domain contracts

Introduce strongly typed Relationship contracts in the appropriate non-UI crate.

At minimum provide:

- RelationshipId with the same stable UUID principles as ProjectId/EntityId
- relationship key/type identifier with validated stable serialization
- Relationship envelope/state sufficient for persistence and validation
- source EntityId
- target EntityId
- project identity
- Canon/Candidate/archive lifecycle semantics consistent with current product principles
- origin/provenance fields needed to distinguish manual/generated/imported where appropriate
- created/updated timestamps
- optimistic write-version handling at storage boundary

Do not turn Relationship into arbitrary JSON without typed validation.

Do not add executable behavior to relationship definitions.

# Scope C — Minimal app-owned Relationship Definition registry

Implement only the minimum registry entries needed to prove the substrate and support the planned first expansion slices:

1. born_in
   Character → Location
   directed

2. member_of
   Character → Faction
   directed

3. owns
   Character → Item
   directed

4. opposed_by
   Quest → Creature
   directed

The full relationship vocabulary in the design document remains PROVISIONAL and is NOT mission scope.

Registry validation must reject invalid endpoint-type combinations.

Because Location/Faction/Item/Creature payload creation is deferred, registry tests may use typed IDs/envelopes or contract fixtures without inventing user-visible entities.

# Scope D — Relationship persistence

Add schema-2 persistence for first-class Relationships.

Use the existing proven storage philosophy:

- queryable columns for important envelope/state data
- validated serialized body/provenance where appropriate
- one relationship identity per assertion
- optimistic write-version conflict detection
- Candidate/Canon lifecycle
- immutable revision snapshots for Canon changes
- project isolation
- intentional foreign keys and indexes
- no silent hard-delete history loss

Required repository operations should cover at least:

- persist/create Candidate relationship
- get relationship
- list relationships for project
- list relationships for an Entity endpoint
- promote Candidate relationship to Canon
- revise Canon relationship explicitly
- archive/retire relationship through a non-destructive normal path
- stale-write rejection
- duplicate assertion handling as designed

Do not implement symmetric relationships in product vocabulary yet if not needed, but the storage design must not make future symmetry impossible.

# Scope E — Schema 1 → Schema 2 migration

Set the database schema version to 2 only after the migration is fully implemented and tested.

Migration must:

1. recognize schema 1
2. checkpoint WAL consistently
3. create the existing style of no-clobber pre-migration recovery copy
4. perform schema changes in a transaction
5. widen entity_type support safely
6. preserve every schema-1 Entity row mechanically
7. preserve every entity_revisions row mechanically
8. preserve every forge_runs row including event IDs/run IDs
9. preserve settings
10. add new Relationship structures empty
11. update only schema metadata that must change
12. commit
13. reopen normally as schema 2
14. run integrity/foreign-key/domain validation

If widening the entity_type CHECK requires a create-copy-verify-swap reconstruction, implement it as a bounded mechanical migration.

Do NOT:

- regenerate Entity IDs
- deserialize and rewrite creative body_json unless technically unavoidable
- normalize/reformat user prose
- create Faction/Item/Location/Creature entities from hints
- convert gear_links/faction_links/relationship_links into Relationship rows
- replace forge_runs with a new history model
- change ForgeDNA
- change generator/content versions because the DB schema changed
- bump forgeproject format solely because SQLite schema becomes 2

# Scope F — Frozen real schema-1 fixture

Create a frozen schema-1 regression fixture before relying on schema-2-only test setup.

It must contain representative persisted state for:

- Character Candidate
- Character lock state
- Character manual edit origin
- Character Canon with multiple revisions
- Quest v3 Candidate including supported candidate metadata
- Quest Canon with revisions
- favorite
- tags/notes where supported
- persistent forge_runs history
- restore-history event if feasible from existing APIs
- Reduced Motion or another persisted setting
- Unicode content

The fixture must be generated or captured in a reproducible way and then treated as schema-1 evidence.

Do not mutate the fixture in place during migration tests; copy it to a temp path.

# Scope G — Golden semantic migration proof

Implement a semantic migration assertion.

Compare schema-1 fixture meaning before migration against schema-2 readback.

Must prove preservation of:

- Project ID/name
- Entity IDs
- Character/Quest bodies
- Canon state
- origin/favorite
- write versions where migration is not intentionally required to alter them
- tags/notes
- GenerationMetadata
- ForgeDNA
- locks/nonces
- manual field origins
- entity revision numbers/reasons/snapshots
- forge_runs event IDs/run IDs/action/recipe/result
- settings

Allowed differences must be explicit:
- DB user_version becomes 2
- project-level schema metadata may become 2 where that field represents DB schema
- entity_type constraint admits new enum values
- new Relationship tables are present and empty

No other difference is silently accepted.

# Scope H — Determinism regression after migration

After a migrated project is read by schema 2:

- replay representative Character v1 and current v2 compatible DNA
- replay representative Quest v2 and current v3 compatible DNA
- assert expected deterministic outputs
- assert lock/reroll behavior remains unchanged

Use existing golden/fixed-output infrastructure where possible.

Do not update golden outputs to make a failure disappear.

# Scope I — New-project and Vault compatibility

Fresh projects created by the new build must use schema 2 correctly.

Verify:

- create/open/save/reopen Character/Quest project still works
- backup works
- forgeproject export/import roundtrip works
- forgeproject archive format remains version 1 if archive structure is unchanged
- manifest accurately reports the stored DB schema version
- future-schema rejection remains safe
- an old-schema fixture upgrades on open through the intended migration path

Do not add arbitrary members to forgeproject.

# Scope J — No user-visible expansion feature

The desktop UI should not expose unfinished:

- Location Forge
- Faction Forge
- Item Forge
- Creature Forge
- relationship editor
- Forge Further buttons
- world graph
- visual generation

If a minimal internal API is needed for tests, keep it behind current architecture boundaries.

# Architecture boundaries

Respect:

forge-contracts
- IDs/contracts/validation

forge-app-core
- Relationship Definition registry and domain validation

forge-storage
- SQLite schema/migration/repositories/transactions

Tauri/React
- no new product behavior required in this mission

If a different placement is clearly superior based on the repository, document the reason in the mission report before deviating.

# Required tests

At minimum add/retain evidence for:

1. EntityType Character/Quest serialization unchanged
2. new EntityType names stable
3. RelationshipId validation
4. valid born_in/member_of/owns/opposed_by definitions
5. invalid endpoint types rejected
6. Candidate Relationship persist/reopen
7. Candidate → Canon
8. Canon revision preserves prior state
9. stale Relationship write rejected
10. archive/retire does not destroy prior revision evidence
11. relationship project mismatch rejected
12. duplicate assertion policy enforced
13. fresh schema 2 DB
14. frozen schema 1 → schema 2 migration
15. migration rollback on injected/real failure
16. pre-migration backup preserved
17. future schema rejected
18. migrated Character semantics unchanged
19. migrated Quest semantics unchanged
20. forge_runs preserved exactly
21. settings preserved
22. Character deterministic replay unchanged
23. Quest deterministic replay unchanged
24. lock/reroll invariants unchanged
25. forgeproject export/import of schema 2 project
26. backup/restore of schema 2 project
27. integrity_check / foreign_key_check clean

# Verification gates

Run the repository-standard full verification, not only targeted tests.

Expected gates include:

- frontend format/lint/typecheck/build
- frontend tests
- cargo fmt check
- strict Clippy with warnings denied
- cargo test workspace
- Cargo metadata
- scripts/verify.ps1
- optimized Tauri no-bundle build if verify does not already cover it
- git diff --check

Use the repository's working pnpm/Corepack setup. Do not "fix" the global npm launcher as unrelated scope.

If Windows file locking requires an isolated CARGO_TARGET_DIR, document it rather than deleting unrelated release artifacts.

# Migration failure gate

PASS requires a credible demonstrated failure path.

A schema-1 project subjected to a failed migration must retain:

- original usable schema-1 DB state or transactional rollback
- pre-migration recovery copy
- no partial schema-2 semantic state

A migration test that only proves success is insufficient.

# Documentation

Create:

docs/missions/THE_FORGE_A_SUBSTRATE_REPORT.md

Update:

GAZINGA/DISCOVERY_STATE.md

Only update canonical design docs if implementation evidence forces a design correction. If so, clearly identify the correction in the report.

Do not rewrite FOUNDATION history.

# Exit gate

PASS only if all are true:

- schema 2 exists and is migration-safe
- frozen schema-1 fixture proves semantic preservation
- v1 Character/Quest determinism and lock/reroll behavior remain unchanged
- first-class Relationship contracts and persistence work
- the four minimal Relationship Definitions validate endpoints correctly
- Relationship Candidate/Canon/revision/stale-write behavior is proven
- schema-2 project close/reopen works
- Vault backup/export/import remains safe
- full repository verification passes
- no user-visible unfinished expansion feature leaked into the app
- no P0/P1/P2 data-loss or compatibility defect is known
- report and state are updated with concrete evidence

If any criterion is not verified, report PARTIAL or BLOCKED.

STOP. Do not begin Location Forge / THE_FORGE_B_PLACE.
