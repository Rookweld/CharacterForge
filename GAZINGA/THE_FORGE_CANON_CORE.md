# THE FORGE — CANON CORE

## Purpose

This is the always-loaded post-v1.0 constitution for The Forge expansion work.

It does NOT replace GAZINGA/CANON_CORE.md as the historical v1.0 FOUNDATION constitution.

For post-v1 implementation, read both:

1. GAZINGA/CANON_CORE.md for existing FOUNDATION contracts
2. this file for The Forge expansion rules
3. GAZINGA/DISCOVERY_STATE.md for current discovery/implementation state
4. the current bounded mission file
5. only the canonical design documents referenced by that mission

## Product

The Forge is a local-first creative pre-production engine for structured, connected, revisioned, canonical game-world and game-design content.

Character Forge and Quest Forge are the proven FOUNDATION.

The first expansion universe is:

- Character
- Quest
- Location
- Faction
- Item
- Creature

Long-term, The Forge may grow into broader worldbuilding, gameplay-design, visual, and engine-export capabilities.

It is not itself a game engine.

## Non-Negotiable FOUNDATION Inheritance

Existing v1.0 behavior is a compatibility contract, not disposable prototype code.

Do not silently change:

- Project IDs
- Entity IDs
- Character/Quest typed payload semantics
- Character v1/current v2 deterministic behavior
- Quest v2/current v3 deterministic behavior
- ForgeDNA canonical serialization
- field-stream derivation
- field keys/nonces/dependency behavior
- lock/reroll semantics
- manual-edit provenance
- Candidate/Canon lifecycle
- immutable Canon history
- stale-write protection
- persistent generation history
- SQLite data safety
- forgeproject portability/safety

If a future feature requires a compatibility change, version it explicitly and preserve supported old replay.

## The Forge Product Laws

### Generation proposes; humans establish Canon

No generator, Forge Further action, AI provider, import, migration, or visual provider may silently establish or overwrite Canon.

### Stable identity

Entity identity persists across revisions.

Migration does not regenerate existing identities.

### Typed payloads

Shared infrastructure must not turn the domain into arbitrary unvalidated JSON.

### Profiles instead of entity explosion

Create new Entity Types when their data model meaningfully differs, not for every noun/role.

### Relationships are first-class

Important connections between existing entities belong in structured Relationship data.

Creative hints may remain in payloads for provenance and early-world authoring.

### Canon-first bounded context

Context is relevant, version-specific, bounded, and Canon-first by default.

Do not dump the entire project into a generator/provider.

### Frozen context provenance

A Forge operation records the exact context it used.

Do not pretend a result used newer source revisions after the fact.

### Provider neutrality

The domain/context model does not depend on a specific AI/image provider.

### The Forge owns authoring truth

Unity, Unreal, Godot, and other runtimes consume exports.

They do not become authoritative Canon stores by accident.

## Initial Expansion Order

PROVISIONAL:

1. Entity/Relationship/Context substrate
2. Location Forge
3. Character → Forge Hometown proof slice
4. Faction Forge
5. Item Forge
6. Creature Forge
7. wider six-entity Forge Further matrix
8. richer/multi-source context
9. later Visual Forge
10. later engine bridges

## First Proof Slice

The first architecture proof is:

    Existing v1 Character
      → Forge Hometown
      → Location Candidate
      → proposed born_in Relationship
      → atomic persistence
      → Candidate/Canon/history behavior
      → close/reopen
      → export

Do not implement four new Forge types simultaneously to prove the substrate.

## Storage and Migration

SQLite remains the authoritative project store for the current architecture.

Schema migrations must:

- be explicit
- create recovery backup before destructive schema mutation
- be transactional
- preserve existing content/IDs/history
- reject future unsupported schemas
- never auto-promote prose concepts into entities
- never infer new Canon relationships from legacy hints

The actual schema-1 repository has been audited in docs/design/ENTITY_SUBSTRATE_AND_MIGRATION.md.

## Relationship Rules

- one structured assertion has one authoritative relationship record
- inverse display/query does not require duplicate edges
- symmetric relations must not be stored twice in mirrored directions
- endpoint types are validated against versioned app-owned definitions
- cross-project relationships are out of scope
- generated relationships are Candidate/proposed by default

Exact manual Link Existing Canon behavior remains OPEN until explicitly resolved.

## Context Rules

Context Compiler is read-only.

It may:
- resolve exact source revisions
- traverse allowed relationship paths
- rank facts
- apply bounded recipes
- validate constraints
- produce immutable context

It may not:
- mutate entities
- mutate Canon
- create relationships
- write to SQLite as a side effect

## Scope Control

A future idea being architecturally possible is not permission to implement it.

Explicitly DEFERRED until their own missions:

- Visual generation implementation
- World Forge
- Ability/Spell Forge
- Game Schema Packs
- engine adapters
- Graph UI
- Timeline
- collaboration
- cloud sync
- arbitrary plugin code execution

## Evidence Rule

No PASS without evidence.

Mission reports must distinguish:

- automated evidence
- operator/manual evidence
- known residual limitations
- unverified assumptions

Do not report a design assumption as an implemented fact.

## Repository-as-Memory Rule

The project must not depend on ChatGPT, Codex, or another agent remembering previous sessions.

State lives in:

- GAZINGA/DISCOVERY_STATE.md
- canonical docs/design files
- bounded mission files
- mission reports
- tested repository behavior

Chat is the whiteboard. The repository is the memory.
