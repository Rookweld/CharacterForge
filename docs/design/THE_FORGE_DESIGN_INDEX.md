# The Forge — Design Index

> Canonical entry point for post-v1.0 product and architecture discovery.

## Status

**Discovery 01 — Entity Universe: CANONICALIZED FOR DESIGN REVIEW**

Implementation has not started.

Product baseline:
- Character Forge / Quest Forge v1.0.0 FOUNDATION
- release commit exists: 98777fe
- local v1.0.0 tag exists
- post-v1 design work is separate from the release commit

See GAZINGA/DISCOVERY_STATE.md for the compact live handoff.

## Purpose

This design package defines how Character Forge / Quest Forge can grow into **The Forge**: a structured creative pre-production engine for connected game-world content.

The documents exist to prevent architecture decisions from being rediscovered ad hoc during implementation.

Chat is the exploration surface.
The repository is the canonical project memory.

## Authority

When design or implementation documents disagree, use this order:

1. Data safety, integrity, and non-destructive compatibility
2. Existing v1.0 FOUNDATION contracts and tested behavior
3. Explicit LOCKED decisions in this design package
4. PROVISIONAL Discovery 01 decisions
5. Individual mission specifications
6. Existing implementation details that are not compatibility contracts
7. Engineering judgment

A PROVISIONAL decision may be revised when repository evidence or implementation testing proves a better design is required. It must not be silently changed.

## Product Boundary

The Forge creates structured, connected, revisioned, canonical game-world and game-design content.

It is not itself a game engine.

Long-term:
- The Forge owns authoring truth
- runtime engines consume exports
- generation proposes
- humans establish Canon

## Canonical Design Documents

### The Forge Entity Universe
THE_FORGE_ENTITY_UNIVERSE.md

Owns:
- product vision
- entity taxonomy
- core design laws
- Entity Type vs Profile vs Intent
- Candidate/Canon philosophy
- future world-truth concepts
- expansion order
- long-term studio vision

### Entity Schema Reference
ENTITY_SCHEMA_REFERENCE.md

Owns:
- field classification vocabulary
- exact audited v1 Character/Quest contracts
- proposed Location/Faction/Item/Creature schemas
- Semantic Field Roles
- relation opportunities
- Forge Further opportunities
- future Game Schema Pack boundary

### Entity Substrate & Migration
ENTITY_SUBSTRATE_AND_MIGRATION.md

Owns:
- logical persistence model
- audited current schema-1 physical model
- stable identity/revision rules
- Relationship persistence direction
- Context/Forge operation persistence
- transaction/concurrency rules
- schema 1 → 2 migration invariants
- recovery and fixture requirements
- crate ownership

### Forge Further & Context
FORGE_FURTHER_AND_CONTEXT.md

Owns:
- Forge Action grammar
- initial Relationship vocabulary
- Relationship semantics
- six-entity Forge Further matrix
- Context Compiler
- Context Recipes
- context ranking/budget/depth
- frozen Context Snapshots
- Forge Result Bundles
- first proof slice

### Visuals & Engine Boundary
VISUALS_AND_ENGINE_BOUNDARY.md

Owns:
- future Visual Asset boundary
- revision-linked visuals
- Visual Canon direction
- provider boundary
- canonical project export snapshot
- Unity/Unreal/Godot boundary
- future Game Schema Packs
- explicit deferred implementation list

### Discovery State
../../GAZINGA/DISCOVERY_STATE.md

Owns:
- current phase
- compact design locks
- open questions
- deferred work
- exact next action

This file must remain concise enough to use as a new-chat handoff.

## Core Vocabulary

### Entity
Stable identity for a fictional or design concept in one project.

### Entity Type
Typed domain schema such as Character, Quest, Location, Faction, Item, Creature.

### Profile
Useful subtype/configuration inside an Entity Type, such as Item/Weapon or Location/Dungeon.

### Intent
Why an entity is being created in the current Forge Action, such as Signature Weapon or Character Hometown.

### Revision
Immutable historical state of an Entity or Relationship.

### Candidate
A possibility saved in the authoring space but not established as Canon truth.

### Canon
Explicit established project truth, changed only through revision-safe operations.

### Relationship
First-class assertion connecting stable Entity identities.

### Forge Action
Versioned recipe describing what can be forged from a Source Set and what target/relationship/context it uses.

### Source Set
One or more exact selected source entities/revisions used by a Forge Action.

### Context Recipe
Versioned rule describing which project facts are relevant to a Forge Action.

### Context Snapshot
Immutable, bounded, exact record of context actually used by a Forge operation.

### Forge Result Bundle
Reviewable output containing a primary Candidate, optional bounded supporting Candidates, proposed Relationships, Context Snapshot/provenance, and warnings.

### Semantic Field Role
Cross-entity semantic meaning such as HiddenTruth, Goal, Threat, Origin, or Hook used by Context Recipes.

### Visual Asset
Future non-authoritative image/visual output linked to the exact Entity revision/state that produced its brief.

## Compatibility Contract

### Existing v1.0 behavior is not disposable

The post-v1 architecture must not silently break:

- stable Project IDs
- stable Entity IDs
- Character/Quest stored payloads
- Character v1/current v2 deterministic behavior
- Quest v2/current v3 deterministic behavior
- ForgeDNA serialization/field-stream contracts
- lock/reroll/manual-edit semantics
- persistent generation history
- Candidate/Canon lifecycle
- immutable Canon revisions
- stale-write detection
- settings
- JSON/Markdown exports
- forgeproject portability

### Version contracts remain separate

Do not conflate:

- app release version
- SQLite schema version
- Entity/payload schema version
- generator version
- content version
- ForgeDNA/recipe version
- Forge Action version
- Context Recipe version
- Context Snapshot format version
- forgeproject archive format version
- future visual/export schema versions

## Decision Status Vocabulary

### LOCKED
Existing contract or explicitly approved architecture/product decision intended to govern implementation.

### PROVISIONAL
Strong current design. Implementation/repository evidence may justify revision through explicit review.

### OPEN
Known unresolved question. Do not invent an answer inside implementation.

### DEFERRED
Valid future concern intentionally outside current implementation scope.

## Current Discovery Result

Discovery 01 has established a coherent architecture around these primitives:

    Entity
    Revision
    Relationship
    Forge Action
    Context Recipe
    Context Snapshot
    Forge Result Bundle

Initial entity universe:

    Character
    Quest
    Location
    Faction
    Item
    Creature

First expansion priority:

    Location
    Faction
    Item
    Creature

First proof slice:

    Existing Character
      → Forge Hometown
      → Location Candidate
      → proposed born_in Relationship
      → atomic persistence
      → Candidate/Canon/history
      → close/reopen/export

## Current Repository Findings That Changed the Whiteboard Design

The actual v1 repository was audited before locking the migration direction.

Important findings:

- EntityEnvelope is already generic in many useful ways.
- The SQLite entities table is already a universal-style envelope + JSON snapshot store, but its entity_type CHECK only permits Character/Quest.
- Current Entity revisions use immutable revision snapshots rather than current_revision_id/canon_revision_id pointers.
- Persistent forge_runs already exists and must not be casually replaced.
- Character already contains legacy/reserved gear_links, faction_links, and relationship_links, currently generated empty.
- The new Relationship store must become authoritative without fabricating edges from those legacy vectors.
- Therefore The Forge should extend the current proven model rather than perform a clean-slate persistence rewrite.

See ENTITY_SUBSTRATE_AND_MIGRATION.md for the audited details.

## Implementation Gate

No post-v1 feature implementation should begin until:

1. the canonical design documents pass consistency review
2. OPEN questions required by the first implementation mission are either resolved or explicitly excluded
3. a bounded Mission A / Entity Substrate specification is written
4. the mission explicitly preserves v1.0 compatibility
5. the frozen schema-1 migration fixture requirement is included before schema-2 PASS

## How to Continue This Project

For a new chat or agent:

1. connect to Nullfeather Bridge
2. read GAZINGA/DISCOVERY_STATE.md
3. read only the canonical document(s) referenced by its Next Action
4. inspect actual repository code before changing a compatibility-sensitive contract
5. write/execute one bounded mission at a time
6. update canonical docs only when a design decision actually changes

Do not use chat history as the only project memory.
