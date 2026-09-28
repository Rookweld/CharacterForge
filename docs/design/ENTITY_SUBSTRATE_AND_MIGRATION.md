# Entity Substrate & Migration

> Status: PROVISIONAL — Discovery 01
> Current physical-schema audit completed against the v1.0 repository.
> No implementation is authorized by this document alone.

## 1. Purpose

Define the data and ownership substrate that allows Character Forge / Quest Forge to grow into The Forge without sacrificing v1.0 identity, determinism, Canon history, portability, or data safety.

The design intentionally extends the existing architecture instead of replacing it.

Primary goals:

- preserve every v1.0 Character and Quest identity/state exactly
- introduce additional typed Entity Types
- add first-class Relationships
- add versioned Forge Action / Context provenance
- preserve current persistent generation history
- keep Canon explicit and revision-safe
- make future Forge Further operations atomic and inspectable
- preserve one-project-per-SQLite-database portability

## 2. Compatibility Requirements

### LOCKED v1.0 requirements

The following must survive any schema migration unchanged in meaning:

- Project ID
- Entity IDs
- Character and Quest payloads
- Candidate/Canon/Archived state
- favorites
- tags and notes
- write-version concurrency semantics
- generation provenance
- ForgeDNA
- field nonces
- lock state
- manual-edit origins
- generation-history snapshots
- Canon revision history
- project settings
- Character v1 replay
- Character current v2 content behavior
- Quest v2 replay
- Quest current v3 content behavior
- JSON/Markdown supported exports
- forgeproject backup/import semantics

Migration is structural. It must not reinterpret creative content.

## 3. Logical Model

The target logical model is:

    Project
    │
    ├─ Entity
    │   ├─ current persisted state
    │   └─ immutable Canon revision history
    │
    ├─ Relationship
    │   ├─ current persisted assertion state
    │   └─ immutable revision/history state
    │
    ├─ Forge Operation
    │   ├─ Context Snapshot
    │   └─ Output mappings
    │
    └─ Settings

Later subsystems may add Visual Assets, Game Schema Packs, or engine export profiles, but they are not part of the first substrate implementation.

### Important post-audit correction

Earlier discovery sketches described explicit current_revision_id and canon_revision_id pointers as one possible logical representation.

The v1.0 physical store does not use that model. It stores the current entity state in entities and immutable Canon revision snapshots in entity_revisions.

**PROVISIONAL DESIGN DECISION:** preserve the existing current-state + immutable-revision-log model unless implementation evidence shows a pointer model is necessary.

Do not force a pointer-based rewrite merely because it looked elegant on a whiteboard.

## 4. Stable Identity

**LOCKED by existing behavior and PROVISIONAL for all new Entity Types**

Entity identity is immutable.

Current storage already rejects updates that change:

- entity ID
- project ID
- entity type

New entity types must preserve the same rule.

A schema migration must never generate replacement IDs for v1 entities.

## 5. Immutable Revision Contract

**LOCKED foundation principle**

Current Canon revisions remain immutable historical snapshots.

The existing v1 model:

- generated work persists as Candidate/Draft
- promotion is explicit
- promotion snapshots Canon revision 1
- later Canon mutation requires revise_canon with a reason
- old revisions remain inspectable
- working-copy/history restoration does not rewrite old history

New Entity Types and Relationships must follow equivalent non-destructive semantics.

The exact Relationship revision UX may initially be simpler than Entity revision UX, but persistence must not require destructive overwrite of historical truth.

## 6. Typed Payload Contract

**PROVISIONAL**

Domain code uses typed payloads.

Current v1 storage already serializes a tagged StoredEntity union:

- CharacterCandidate
- QuestCandidate
- Character
- Quest

The future union may extend to Location, Faction, Item, Creature and their generated candidate wrappers.

JSON is acceptable as a storage representation, but app-core must validate/deserialise into typed domain contracts before using payload data.

The target is not a generic arbitrary JSON Entity model.

## 7. Version Boundaries

These versions are independent compatibility contracts and must not be collapsed:

- application release version
- SQLite database schema version
- entity payload/schema version
- generator version
- content version
- ForgeDNA recipe version
- Forge Action version
- Context Recipe version
- Context Snapshot format version
- relationship-definition registry version if needed
- forgeproject archive format version
- future visual recipe/provider metadata versions
- future engine export schema versions

Changing one does not automatically change the others.

Example: a SQLite schema 2 project can still contain Character payload semantics and ForgeDNA that reproduce v1 behavior.

## 8. Entity Storage Model

### Current v1 physical model

The current entities table stores:

- id — text primary key
- project_id
- entity_type — currently CHECK limited to character or quest
- name
- canon_state — draft/candidate/canon/archived
- origin — manual/generated/imported
- favorite
- schema_version
- body_json — complete validated StoredEntity JSON
- generation_json — optional GenerationMetadata JSON
- write_version
- created_at
- updated_at

Indexes exist for project/state and project/type.

This is already substantially closer to a universal Entity store than the earlier discovery discussion assumed.

### PROVISIONAL target

Keep the universal envelope/query-column pattern.

Required evolution:

- permit the initial six Entity Types
- retain validated typed body_json snapshots
- retain write_version optimistic concurrency
- retain origin/favorite/state/timestamps
- do not duplicate authoritative relationship truth inside new payload foreign-key fields

The exact schema-2 DDL is an implementation mission concern.

### SQLite CHECK caveat

Because entity_type currently has a CHECK limited to character/quest, adding new types may require a controlled table reconstruction in SQLite rather than a simple ALTER.

That is acceptable if it is:

- transactional
- mechanically lossless
- backed up first
- proven by frozen-fixture tests

It must not become a semantic data rewrite.

## 9. Entity Revision Storage Model

### Current v1 physical model

entity_revisions stores:

- integer revision_id
- entity_id
- revision_number
- reason
- body_json
- generation_json
- created_at

It enforces unique entity_id + revision_number and cascades when the owning entity is deleted.

### PROVISIONAL target

Preserve immutable body snapshots and monotonic per-entity revision numbers.

New Entity Types should use the same revision mechanism where possible.

A new global UUID revision identity is not currently required. Do not invent one unless later cross-reference requirements justify it.

## 10. Relationship Storage Model

**PROVISIONAL new substrate**

Relationships are new first-class project records.

Logical fields:

- relationship identity
- project ID
- source Entity ID
- relationship type/key
- target Entity ID
- current relationship state
- provenance/origin
- write version
- created/updated timestamps

Relationship identity represents one assertion, not an arbitrary mutable edge.

Example:

    Vaela member_of Ashen Concord

is one Relationship identity.

If a later world fact says:

    Vaela enemy_of Ashen Concord

that is a different assertion/Relationship rather than changing the old relationship type in place.

Endpoint Entity IDs must belong to the same project.

## 11. Relationship Revision Model

**PROVISIONAL**

Relationship revision persistence should mirror the proven Entity pattern as closely as practical:

- immutable prior snapshots
- explicit reason for Canon mutation where appropriate
- monotonic revision number
- current write-version conflict protection
- Candidate/Canon lifecycle
- Retired state instead of destructive overwrite for ordinary use

A minimal first implementation may expose less revision UI, but storage should not require history loss.

## 12. Referential Integrity

**PROVISIONAL**

Rules:

- Relationships reference existing Entities.
- Source and target belong to the same project.
- Relationship endpoint types are validated against the app-owned Relationship Definition registry.
- Project databases remain self-contained.
- New foreign keys must be enabled under the same current PRAGMA foreign_keys=ON policy.

The current schema uses project cascade deletion and entity revision cascade deletion. Project deletion is not currently the normal end-user operation.

For new relationship/history tables, destructive cascade behavior must be reviewed carefully so that ordinary Entity archive/retire actions do not erase history.

## 13. Delete / Archive / Retire Rules

**PROVISIONAL**

Normal product operations should favor:

- Archive Entity
- Retire Relationship

rather than hard delete.

Hard delete, if ever exposed, should be a deliberate maintenance operation with inbound-reference/history handling.

Archive/retire must not silently destroy Canon history or provenance.

## 14. Symmetric Relationship Storage

**PROVISIONAL**

Symmetric relationships are stored once.

Examples:

- allied_with
- rival_of

The application layer should normalize endpoints or otherwise enforce duplicate prevention so A↔B is not persisted twice as A→B and B→A.

The database must not rely on duplicated mirrored edges for querying.

## 15. Forge Operation Persistence

### Current v1 physical history

The current forge_runs table stores:

- event_id — independent AUTOINCREMENT event identity
- run_id — generation identity
- project_id
- entity_id nullable
- action
- parent_run_id nullable
- recipe_json
- result_json
- created_at

This already provides durable generation history and deliberately allows repeated deterministic generation IDs to remain separate events.

Current actions include:

- forge
- reroll_field
- reroll_all_unlocked
- manual_edit
- restore_history

### PROVISIONAL future ForgeOperation model

Do not destroy or rewrite forge_runs.

Future relationship/context-aware Forge operations need additional structured provenance:

- Forge Action ID/version
- Context Snapshot reference
- source Entity/revision set
- resulting Entity/Relationship outputs
- warnings/conflict metadata where useful

**OPEN:** whether forge_runs is evolved, wrapped by a new operation table, or retained as legacy FOUNDATION history while new ForgeOperation records coexist.

This must be decided from implementation simplicity and backward compatibility, not aesthetics.

## 16. Context Snapshot Persistence

**PROVISIONAL new substrate**

Context Snapshots are immutable.

Logical fields:

- context_snapshot_id
- format_version
- Context Recipe ID/version
- canonical serialized context
- context hash
- created timestamp

The snapshot contains only context actually used, including exact source/revision references and included facts/relationships.

It must not blindly copy the whole project.

## 17. Atomic Forge Result Persistence

**PROVISIONAL**

Saving an accepted Forge Result Bundle is one transaction.

Conceptually:

    BEGIN
      persist Context Snapshot if not already durable
      persist Forge Operation
      persist primary Candidate
      persist supporting Candidates allowed by action
      persist Candidate Relationships
      persist output mappings
      persist generation/history records
    COMMIT

Failure at any point rolls back the entire bundle.

The system must never leave:

- Entity saved but proposed Relationship missing
- Relationship saved but source Context/operation provenance missing
- partial supporting Candidates

## 18. Optimistic Concurrency

**LOCKED existing behavior; extend to new records**

v1 entities use monotonic write_version and require callers to provide the loaded expected version.

This behavior continues.

Relationship mutations should adopt equivalent optimistic conflict protection.

Frontend state does not get to overwrite a newer persisted state silently.

## 19. Stale Context Semantics

**PROVISIONAL**

Generation occurs from a frozen exact Context Snapshot.

If a source Entity changes after generation but before the user saves/reviews the result:

- provenance remains truthful to the old exact source revision/state
- the result is not relabeled as if it used the new state
- UI may warn that source context has changed
- persistence must revalidate structural assumptions before commit

A result is not automatically invalid just because its source later changed.

## 20. Project Isolation

**PROVISIONAL**

No cross-project Relationships in the first architecture.

Every forgeproject remains self-contained.

A future shared library/reference system is a separate design problem and is DEFERRED.

## 21. Crate Ownership

Current architecture boundaries remain strong.

### forge-contracts
Owns:
- stable IDs
- versioned domain contracts
- Entity Types
- typed payloads
- validation primitives
- serializable Context/Relationship contracts where appropriately domain-level

Must not depend on UI or SQLite.

### forge-app-core
Owns:
- deterministic generators
- Forge definitions
- Relationship Definition registry
- Context Compiler
- Context Recipes
- Forge Further orchestration
- bundle validation
- Canon/world consistency rules
- domain-level provenance logic

Must not directly become Tauri UI code.

### forge-storage
Owns:
- SQLite connection lifecycle
- schema versions/migrations
- transactions
- persistence/repositories
- integrity checks
- WAL-consistent snapshot support

Must not decide creative semantics such as which Character secret is relevant to a Quest.

### forge-export
Owns:
- human/data export transformations
- future canonical project export model
- later engine adapter transformations

Must not become authoritative project storage.

### Tauri desktop shell
Owns:
- native command boundary
- desktop dialogs/lifecycle
- transport between UI and application services

Must not become a second domain layer.

### React frontend
Owns:
- presentation
- interaction
- review/confirmation UX
- selected working state

Must not access SQLite directly or own integrity decisions.

## 22. Current v1 Physical Schema Audit

**AUDITED 2026-09-28 AGAINST THE ACTUAL REPOSITORY.**

Current storage implementation: crates/storage/src/lib.rs.

### Database-level behavior

- CURRENT_SCHEMA_VERSION = 1
- one SQLite database per project
- PRAGMA user_version holds DB schema version
- future schema versions are rejected
- foreign keys enabled
- WAL enabled after migration
- synchronous=FULL
- 5-second busy timeout
- wal_autocheckpoint=1000
- integrity_check and foreign_key_check support exist
- pre-migration recovery copy exists for schema-0 migration

### projects table

Columns:
- id
- name
- schema_version
- generator_version
- content_version
- created_at
- updated_at

The store enforces exactly one project per database through application logic.

### entities table

Columns:
- id
- project_id
- entity_type
- name
- canon_state
- origin
- favorite
- schema_version
- body_json
- generation_json
- write_version
- created_at
- updated_at

Important constraints:
- entity_type currently permits only character or quest
- canon_state permits draft/candidate/canon/archived
- origin permits manual/generated/imported
- body_json must be valid JSON
- generation_json, when present, must be valid JSON
- project_id references projects with ON DELETE CASCADE
- unique(project_id, id)

### entity_revisions table

Columns:
- revision_id AUTOINCREMENT
- entity_id
- revision_number
- reason
- body_json
- generation_json
- created_at

Constraints:
- unique(entity_id, revision_number)
- entity_id references entities with ON DELETE CASCADE

### forge_runs table

Columns:
- event_id AUTOINCREMENT
- run_id
- project_id
- entity_id nullable
- action
- parent_run_id nullable
- recipe_json
- result_json
- created_at

Foreign keys:
- project_id → projects ON DELETE CASCADE
- entity_id → entities ON DELETE SET NULL

Indexes exist by run, project/time, and entity/time.

### settings table

A key/value JSON settings store with updated_at timestamp.

### Current typed storage union

StoredEntity currently supports:
- CharacterCandidate
- QuestCandidate
- Character
- Quest

### Current Character link-vector caveat

The typed Character contract already contains:
- gear_links
- faction_links
- relationship_links

Current Character generation initializes all three as empty. They are not part of the 17 deterministic Character generated fields.

**PROVISIONAL migration rule:** preserve these fields exactly for v1 compatibility, but do not treat them as the new authoritative Relationship store and do not auto-convert them into Relationship rows during migration.

## 23. Target Schema

**PROVISIONAL logical target; exact DDL deferred to implementation design review**

Retain:
- projects
- entities
- entity_revisions
- forge_runs unless an explicit compatibility-safe evolution replaces/wraps it
- settings

Evolve:
- EntityType support to Character, Quest, Location, Faction, Item, Creature
- StoredEntity typed union accordingly

Add:
- relationships
- relationship_revisions
- immutable context_snapshots
- structured Forge operation/output mapping if required after forge_runs compatibility decision

Do not add yet:
- Visual Asset tables
- Game Schema Pack tables
- engine adapter state
- collaboration/cloud tables
- world graph cache tables

## 24. Schema 1 → Schema 2 Strategy

**PROVISIONAL**

Migration should be the smallest physical change that satisfies the target.

Sequence:

1. open project and read user_version
2. reject unsupported future versions
3. validate enough of schema 1 to migrate safely
4. checkpoint WAL consistently
5. create no-clobber pre-migration recovery copy
6. begin migration transaction
7. perform required schema-2 DDL
8. mechanically preserve all schema-1 rows
9. create new empty relationship/context/operation structures as required
10. verify row counts and critical identities/pointers/state
11. set user_version = 2
12. commit
13. reopen through normal schema-2 reader
14. run integrity/foreign-key/domain validation

If the entity_type CHECK requires table reconstruction, use a controlled create-copy-verify-swap pattern inside the transaction.

No creative semantics are inferred during migration.

## 25. Migration Invariants

After schema 1 → 2, all existing v1 semantics must match.

Must preserve exactly:

- Project ID and project name
- Entity IDs
- entity type
- current Character/Quest bodies
- Canon state
- origin
- favorite
- schema_version fields
- write_version
- timestamps
- tags/notes in body JSON
- generation provenance JSON
- ForgeDNA
- root seed
- locks
- field nonces
- manual field-origin metadata
- Character payload contents
- Quest payload contents including supported v2/v3 candidate metadata
- all entity revision numbers/reasons/bodies
- all forge_runs events and event IDs
- settings

Allowed intentional differences:
- PRAGMA user_version becomes 2
- entity_type constraint admits new types
- new substrate tables exist and are initially empty for migrated v1 projects
- schema-internal metadata required by the new version may exist

## 26. Pre-Migration Backup

**LOCKED pattern from existing storage**

Continue the existing recovery principle:

- checkpoint committed WAL content first
- create a sibling pre-migration recovery file
- refuse to overwrite an existing recovery file unexpectedly
- do not delete the recovery file merely because migration started
- never reset the user's project after a failed migration

Exact naming may preserve the current .pre-migration.bak convention unless schema-2 work proves a need to version the recovery name.

## 27. Transaction / Rollback

**LOCKED principle**

All schema mutations are transactional.

Failure before commit:
- rolls back DB changes
- leaves the original project usable
- preserves the recovery copy

A post-commit reopen/integrity failure must be treated as release-critical and recovery behavior must be explicitly tested.

## 28. Frozen Schema-1 Test Fixture

**REQUIRED BEFORE IMPLEMENTATION PASS**

Create a frozen real schema-1 fixture containing at minimum:

- Character Candidate with locks and manual edits
- Character Canon with multiple revisions
- Quest v3 Candidate
- Quest Canon with revisions
- favorites
- persistent generation history
- restore-history event
- Reduced Motion or other persisted settings
- representative Unicode
- non-empty tags/notes where supported

The fixture becomes immutable regression evidence.

## 29. Golden Semantic Migration Test

**REQUIRED**

Define a canonical semantic extraction of a schema-1 fixture.

Then:

    before = canonical_v1_semantics(schema1_fixture)
    migrate_to_schema2()
    after = canonical_v1_semantics(read_through_schema2)

Assert before == after except explicitly allowed schema metadata.

This test must catch silent changes to payload text, IDs, Canon history, ForgeDNA, field origins, and history.

## 30. Determinism After Migration

**REQUIRED**

After migration:

- replay representative Character v1/current v2 DNA
- replay representative Quest v2/v3 DNA
- compare deterministic results with pre-migration expected outputs
- validate lock/reroll invariants remain unchanged

Schema migration must not alter generator input normalization or field-stream semantics.

## 31. forgeproject Compatibility

**LOCKED archive principle from ADR 0004**

forgeproject format version is independent from SQLite schema version.

The v1 archive is a closed ZIP with:
- manifest.json
- project.sqlite3

If that package structure remains unchanged, SQLite schema 2 does not automatically require forgeproject format 2.

The manifest already records the stored version tuple. A new app can import an archive containing schema 2 if supported.

An old app should safely reject an unsupported future DB schema instead of partially interpreting it.

## 32. Old Application Compatibility

Once a project is upgraded to DB schema 2, Character Forge v1.0.0 is expected to reject it as a future unsupported schema.

That is correct fail-safe behavior.

The upgrade UX should clearly communicate:
- the project will be upgraded
- a recovery backup is created
- older app versions cannot open the upgraded copy

Do not attempt a fake backwards-compatible write mode that risks dropping new data.

## 33. Explicit Non-Goals

The first substrate migration does NOT implement:

- Visual generation
- Visual Asset persistence
- World Forge
- Ability/Spell Forge
- Game Schema Packs
- Unity/Unreal/Godot adapters
- Graph UI
- Timeline
- collaboration
- cloud sync
- plugin code execution
- automatic text-to-entity promotion
- automatic relationship inference from v1 Character hints
- automatic conversion of legacy Character link vectors into Relationship rows
- arbitrary AI provider integration

## 34. Open Architecture Questions

1. Should future structured Forge operations extend forge_runs or use a parallel ForgeOperation table linked to legacy/new history events?
2. Do Relationship revisions use integer revision_number exactly like Entity revisions, or does a later requirement justify a different identity?
3. How should manual Link Existing default between Candidate and Canon?
4. Does Entity schema version remain the existing envelope schema_version for all types, or should payload-version naming become more explicit when new type-specific versions diverge?
5. How should future new Character payload versions deprecate existing gear_links/faction_links/relationship_links without breaking old snapshots?
6. What minimal schema-2 DDL best widens the entities entity_type CHECK while preserving current indexes, constraints, row identities, and performance?

---

See also:
- ADR 0003 — SQLite project persistence and lifecycle
- ADR 0004 — Versioned forgeproject archive
- THE_FORGE_ENTITY_UNIVERSE.md
- ENTITY_SCHEMA_REFERENCE.md
- FORGE_FURTHER_AND_CONTEXT.md
- ../../GAZINGA/DISCOVERY_STATE.md
