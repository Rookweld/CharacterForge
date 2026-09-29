# EXECUTION CONTRACT

You are executing exactly one bounded post-v1 The Forge mission.

Before coding:

1. Read GAZINGA/CANON_CORE.md.
2. Read GAZINGA/THE_FORGE_CANON_CORE.md.
3. Read GAZINGA/PROJECT_STATE.md.
4. Read GAZINGA/DISCOVERY_STATE.md.
5. Read this mission completely.
6. Read these canonical design sections:
   - docs/design/THE_FORGE_DESIGN_INDEX.md
   - docs/design/THE_FORGE_ENTITY_UNIVERSE.md sections 3, 5–9, 14–19
   - docs/design/ENTITY_SCHEMA_REFERENCE.md sections 1, 2, 5, 9, 10
   - docs/design/ENTITY_SUBSTRATE_AND_MIGRATION.md sections 2–7, 18–21, 31–33
7. Inspect the actual repository before changing contracts.

During work:

- preserve all passing v1.0 Character/Quest behavior
- preserve Mission A schema-2/Relationship behavior
- add tests with implementation
- do not weaken golden outputs or existing tests
- do not implement Character → Hometown yet
- do not implement Forge Further, Context Compiler, Context Snapshots, Result Bundles, relationship review UI, or multi-source Forge
- do not implement Faction, Item, Creature, World, Ability, Visual, AI-provider, engine-adapter, Graph, Timeline, Cloud, or Collaboration features
- do not start B2

At the end:

- run all required verification gates
- create docs/missions/THE_FORGE_B1_PLACE_CORE_REPORT.md
- update GAZINGA/DISCOVERY_STATE.md
- update GAZINGA/PROJECT_STATE.md only where current repository facts materially changed
- report PASS only if every exit criterion is satisfied
- STOP

# THE FORGE B1 — PLACE CORE
## Standalone deterministic Location Forge

# Objective

Implement **Location** as the first new post-v1 typed Forge entity and prove that the universal Entity lifecycle works for a genuinely new domain type.

B1 must deliver a complete standalone Location Forge experience:

    Forge Location
      → deterministic Candidate
      → field locks / rerolls / manual edits
      → undo / redo
      → save Candidate
      → favorite
      → promote to Canon
      → revise Canon
      → inspect history/revisions
      → close/reopen
      → JSON/Markdown export

B1 deliberately does NOT implement the Character → Hometown relationship-driven proof slice. That is B2.

# Baseline

Expected starting baseline:

- branch: master
- HEAD: 8edc134 — feat: add The Forge entity relationship substrate
- database schema: 2
- v1.0.0 release tag remains at 98777fe
- four unrelated untracked operator/log files may remain and must not be staged

Re-inspect actual state and stop if the baseline materially differs.

# Product contract — Location schema v1

Implement a strongly typed Location contract matching the canonical schema.

Required fields:

1. name
2. location_profile
3. summary
4. visual_identity
5. atmosphere
6. purpose
7. history
8. inhabitants_summary
9. notable_features
10. danger
11. secret
12. story_hooks

Expected value shapes:

- name: non-empty String
- location_profile: non-empty String or typed profile representation with stable serialization
- summary: non-empty String
- visual_identity: non-empty String
- atmosphere: non-empty String
- purpose: non-empty String
- history: non-empty String
- inhabitants_summary: non-empty String
- notable_features: non-empty list, target 3 values
- danger: non-empty String
- secret: non-empty String
- story_hooks: non-empty list, target 3 values

Location validation must follow existing contract quality:
- deny unknown fields where appropriate
- validate envelope as EntityType::Location
- validate required text
- validate list minimums
- preserve Unicode
- preserve stable serde names

Do not add gameplay/runtime fields such as level, enemy density, map coordinates, fast travel, damage, loot tables, etc.

# Location Profiles v1

Support these profiles as authored generation possibilities:

- Settlement
- District
- Building
- Dungeon
- Ruins
- Wilderness
- Landmark
- Interior
- Site
- Other

Choose a stable serialized representation and test it.

Do not create separate Entity Types for these profiles.

# Generation kind and compatibility

Add Location as a new GenerationKind or equivalent versioned generation category.

Hard compatibility rule:

- Character serialized key stays character
- Quest serialized key stays quest
- all existing Character/Quest ForgeDNA replay remains byte/semantic compatible
- adding Location must not alter Character/Quest field-stream derivation
- do not renumber old enums through custom numeric serialization
- no existing golden output may be updated merely because Location exists

Location uses its own explicit generation/content version constants.

Do not bump Character or Quest versions.

Do not bump database schema beyond 2 for B1 unless an unavoidable compatibility requirement is proven and documented. Schema 2 already admits EntityType::Location.

# LocationCandidate

Implement the generated Candidate wrapper with the same proven capabilities expected from Character/Quest:

- typed Location payload
- ForgeDNA
- per-field origin metadata
- validation that all canonical generated fields are registered
- generation provenance
- lock state
- nonce state
- deterministic reroll support
- manual field-origin tracking

Do not add Context Snapshot or Forge Further provenance in B1.

# Canonical Location fields and stable keys

Define one stable FieldKey for each generated Location field.

The key set and order become a compatibility contract once B1 passes.

Choose a consistent numbered naming convention matching current project practice.

Every field above must be independently lockable/rerollable at field granularity.

Lists such as notable_features and story_hooks may be treated as one generated field each in B1.

# Dependency graph

Design a conservative acyclic dependency graph.

Suggested semantic direction:

    location_profile
        ↓
    name
    visual_identity
    atmosphere
    purpose
        ↓
    summary
    history
    inhabitants_summary
    notable_features
    danger
        ↓
    secret
    story_hooks

This sketch is not mandatory if repository patterns support a better dependency graph.

Requirements:

- graph must be acyclic
- dependencies must reflect fields actually read by generation
- rerolling a field changes only the required dependency closure
- locked downstream fields form the same dependency firewall already proven in Character Forge
- locked values and locked nonces remain unchanged
- no hidden global RNG

Document the final graph in the mission report.

# Core presets

Location Forge supports the same 11 core universe presets:

- dark-fantasy
- pirate
- steampunk
- cyberpunk
- nordic-myth
- post-apocalyptic
- occult
- monster-hunter
- rogue-assassin
- contemporary-weird
- chaos-blend

Reuse preset IDs and creative-axis semantics where they make sense.

Do not force Character-only constraints into Location.

If Location needs constraints, keep B1 minimal and explicitly version them.

# Location content corpus

Create dedicated Location content rather than borrowing Character/Quest prose pools.

Content must be substantial enough that B1 does not repeat the early Mission 03 mistake of shipping obviously tiny pools.

For each non-Chaos preset:

- every core Location field must have meaningful authored combinatorial diversity
- avoid two-choice pools
- target at least 6 meaningful authored options/fragments at each major selection layer
- content should support the Location profiles naturally rather than making every preset feel like the same settlement with nouns swapped

Chaos Blend must structurally mix compatible material across presets without collapsing into one dominant preset.

Do not sacrifice coherence purely to maximize combinatorics.

# Deterministic generation quality

Location generation must be deterministic from:

- ForgeDNA
- field controls/nonces
- preset
- creative axes
- declared constraints

No global mutable RNG.
No current time.
No filesystem state.
No network.

Same DNA = same Location Candidate.

# Creative axes

Apply the existing creative axes where semantically useful.

The implementation must prove:

- axes can influence Location output
- extremes remain valid/coherent
- neutral output remains stable
- Character/Quest axis behavior is unchanged

Do not invent a second Location-only axis system in B1.

# Lock / reroll / edit semantics

Implement:

- lock
- unlock
- reroll one field
- reroll all unlocked
- manual edit
- field origins
- nonce behavior

Required invariants mirror Character/Quest:

- locked field values never change during unrelated rerolls
- locked field nonces never change during unrelated rerolls
- manual edit origin survives persistence
- rerolled fields return to generated origin
- dependency closure is correct
- reroll all unlocked preserves every locked field

# Storage integration

Extend StoredEntity and storage helpers to support generated Location Candidate safely.

Requirements:

- schema 2 remains authoritative
- entity_type column = location
- typed body_json decodes/validates as Location Candidate
- payload/type mismatch is corruption/validation failure
- existing Character/Quest records remain readable unchanged
- Location Candidate save/reopen works
- favorite works
- Candidate → Canon works
- Canon revision works
- history remains inspectable
- stale-write protection works
- close/reopen preserves DNA, locks, origins, tags/notes, Canon state and revisions

Do not create or infer Relationships in B1.

# History integration

Location Forge actions must participate in the existing persistent history model.

Support the same meaningful actions as the existing generated Candidate lifecycle:

- forge
- reroll_field
- reroll_all_unlocked
- manual_edit
- restore_history when existing generic UI/history flow applies

Do not replace forge_runs.

Do not implement richer ForgeOperation provenance in B1.

# Export integration

Add Location support to current JSON and Markdown entity export.

Markdown should produce a human-usable Location sheet using the canonical fields.

JSON must preserve the typed contract and provenance expected by existing export policy.

forgeproject backup/export/import remains format version 1.

Verify a project containing Location survives:
- close/reopen
- backup/restore
- forgeproject export/import

# Desktop / Tauri integration

Add the minimum complete desktop commands required to make Location a first-class standalone Forge mode.

Do not duplicate application logic in Tauri.

The Rust application layer remains authoritative for generation and mutation.

# Workbench UI

Extend the current approved Workbench rather than redesigning it.

Requirements:

- Character and Quest UX remain intact
- add Location as a third Forge mode
- user can choose preset, seed and existing creative axes
- user can Forge a Location
- all 12 canonical fields are visible
- fields can be locked/unlocked
- fields can be rerolled
- text fields can be manually edited using the current editing pattern
- reroll all unlocked works
- undo/redo works
- save Candidate works
- favorite works
- Canon promotion/revision works
- history/revision inspection works
- JSON/Markdown export works
- saved Location library/list is available in a natural place
- UI remains keyboard-usable and compatible with Reduced Motion
- no Forge Further/Hometown button in B1

Do not redesign approved visual identity, typography, palette, or layout hierarchy merely to add Location.

# Data-driven UI boundary

B1 may factor shared field metadata where it clearly reduces duplicate Character/Quest/Location behavior.

It must NOT turn the Workbench into a giant generic dynamic form framework.

Use the canonical rule:

**Shared metadata drives shared behavior. Specialized UI expresses meaningful domain differences.**

If a refactor materially changes Character/Quest UI behavior, it is out of scope.

# Automated quality gates

## A. Contract and deterministic tests

At minimum:

- Location contract valid/invalid cases
- stable profile serialization
- stable GenerationKind serialization
- canonical Location field-key set
- acyclic dependency graph
- deterministic replay
- same DNA does not mutate during generation
- lock/reroll firewall
- reroll all unlocked
- manual edit origin
- Unicode roundtrip
- content-version rejection for unsupported versions
- invalid preset rejection
- existing Character/Quest golden replay unchanged

## B. Preset sweep

Generate at least:

- 256 seeds × 11 presets

Assert:
- every candidate valid
- deterministic exact replay
- no full exact duplicate Location Candidate inside each preset sweep unless mathematically justified and documented
- required fields/lists complete

## C. Chaos Blend sweep

Generate at least:

- 1,024 Chaos Blend seeds

Measure and report:
- exact duplicate count
- broad source/pattern diversity
- obvious dominant-source failure if measurable with the chosen architecture

Do not invent a fake numeric quality score if the corpus architecture does not expose source provenance.

## D. Human-readable sample audit

Produce:

docs/quality/THE_FORGE_B1_LOCATION_SAMPLES.md

Include at minimum:
- 4 representative Locations from each non-Chaos preset = 40
- 12 Chaos Blend Locations
- total 52 samples

Review/report:
- obvious full clones
- malformed prose
- field contradictions
- severe repetition
- preset identity
- whether fields read as one coherent Location rather than unrelated sentences

Classify samples:
- usable as-is
- minor edit
- weak

Do not claim human/operator quality review if Codex itself only generated the sample file. Label automated/content-author self-review accurately.

# Regression gates

Run all repository-standard gates:

- frontend format
- frontend lint
- frontend typecheck
- frontend tests
- frontend production build
- cargo fmt --check
- strict Clippy with warnings denied
- cargo test --workspace --no-fail-fast
- cargo metadata --format-version 1 --no-deps
- scripts/verify.ps1
- optimized Tauri no-bundle build if not already covered
- git diff --check

The existing 1,000-sequence state gauntlet must continue to pass.

# Operator smoke requirement

Automated PASS is not sufficient for final B1 PASS.

After implementation gates pass, report **PARTIAL — OPERATOR SMOKE REQUIRED** unless an actual human operator has run the built desktop app.

Required human smoke:

1. Open/create a project.
2. Choose Location mode.
3. Forge one Location with an explicit seed.
4. Confirm all Location fields render.
5. Lock:
   - Name
   - Secret
6. Reroll another field and then Reroll All Unlocked.
7. Confirm locked Name/Secret remain unchanged.
8. Manually edit one field.
9. Undo and Redo.
10. Save Candidate.
11. Favorite it.
12. Promote to Canon.
13. Create one explicit Canon revision.
14. Inspect revision/history.
15. Export JSON and Markdown.
16. Close/reopen project and confirm persisted Location.
17. Confirm Character and Quest forging still visibly work.

Record exact operator result before final PASS.

# Documentation

Create:

- docs/missions/THE_FORGE_B1_PLACE_CORE_REPORT.md
- docs/quality/THE_FORGE_B1_LOCATION_SAMPLES.md

Update:

- GAZINGA/DISCOVERY_STATE.md
- GAZINGA/PROJECT_STATE.md where appropriate

Update canonical design docs only if implementation evidence forces a real design correction.

# Explicit non-goals

B1 must NOT implement:

- Character → Forge Hometown
- Context Compiler
- Context Recipes
- Context Snapshot persistence
- Forge Result Bundle
- Forge Action registry for relationship-driven generation
- Relationship UI
- Link Existing
- Relationship Canon UX
- Faction Forge
- Item Forge
- Creature Forge
- World/Lore/Ability systems
- Visual generation
- Game Schema Packs
- engine adapters
- Graph UI
- timeline
- cloud/collaboration
- AI providers

# Exit gate

B1 final PASS requires all of:

- typed Location contract implemented
- deterministic Location Forge implemented
- all 11 presets supported
- lock/reroll/manual-edit semantics proven
- Location persistence/history/Canon/revisions proven
- JSON/Markdown export proven
- close/reopen and Vault roundtrip proven
- preset and Chaos sweeps pass
- sample audit produced with no unresolved severe quality defect
- full repository verification passes
- Character/Quest compatibility remains green
- actual operator desktop smoke passes
- no P0/P1/P2 data-loss or compatibility defect known

If automated gates pass but operator smoke has not occurred:

**PARTIAL — OPERATOR SMOKE REQUIRED**

STOP.

Do not begin THE_FORGE_B2_HOMETOWN.
