# EXECUTION CONTRACT

You are executing exactly one bounded post-v1 The Forge mission.

Before coding:

1. Read GAZINGA/CANON_CORE.md.
2. Read GAZINGA/THE_FORGE_CANON_CORE.md.
3. Read GAZINGA/DISCOVERY_STATE.md.
4. Read GAZINGA/KNOWN_ISSUES.md.
5. Read this mission completely.
6. Read these canonical design sections:
   - docs/design/THE_FORGE_DESIGN_INDEX.md
   - docs/design/THE_FORGE_ENTITY_UNIVERSE.md sections 8-19
   - docs/design/ENTITY_SCHEMA_REFERENCE.md sections 1, 2, 7, 9, 10
   - docs/design/ENTITY_SUBSTRATE_AND_MIGRATION.md sections 2-7, 15-21, 31-33
7. Read the B1/B2/B3 reports to preserve the proven Location, Forge Further, and universal new-entity baselines.
8. Inspect the actual repository before changing contracts.

During work:

- preserve Character, Quest, Location, Faction, Relationship, and Hometown behavior
- preserve schema version 2
- preserve all existing deterministic replay contracts and stable field keys
- add tests with implementation
- do not weaken golden outputs/tests
- do not implement Item Forge Further actions
- do not create Relationships as a side effect of standalone Item generation
- do not implement Creature, World, Ability, Visual, AI-provider, engine-adapter, Graph, Timeline, Cloud, Collaboration, or Game Schema Pack features
- do not begin the next mission

At the end:

- run every required verification gate
- create docs/missions/THE_FORGE_B4_ITEM_CORE_REPORT.md
- create docs/quality/THE_FORGE_B4_ITEM_SAMPLES.md
- update GAZINGA/DISCOVERY_STATE.md
- update GAZINGA/PROJECT_STATE.md only where repository facts materially change
- report PASS only if every automated exit criterion is satisfied
- STOP

# THE FORGE B4 — ITEM CORE
## Standalone deterministic Item Forge

# Objective

Implement **Item** as the third new post-v1 typed Forge entity and prove that the universal lifecycle continues to scale cleanly after Location and Faction.

B4 delivers a complete standalone Item Forge experience:

    Forge Item
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

B4 deliberately does NOT implement Item Forge Further, owner/creator/location/guardian relationships, reward linking, Visual Forge, Ability entities, or gameplay-stat packs.

# Starting baseline

Expected baseline:

- branch: master
- HEAD includes accepted B3 Faction Forge
- database schema = 2
- EntityType::Item already exists in the substrate vocabulary
- Character/Quest/Location/Faction/Hometown regressions green
- first-class Relationships remain available but unused by standalone Item generation
- UI-001 remains known P3 debt for intermittent Location lock visual feedback

Re-inspect actual state. If the baseline materially differs, stop and report rather than silently adapting scope.

# Product contract — Item schema v1

Implement a strongly typed Item contract matching the canonical design.

Required fields:

1. name
2. item_profile
3. appearance
4. purpose
5. properties
6. origin
7. history
8. power_or_function
9. cost_or_drawback
10. rarity
11. condition
12. secret
13. story_hooks

Expected value shapes:

- name: non-empty String
- item_profile: stable typed profile enum/representation
- appearance: non-empty String
- purpose: non-empty String
- properties: non-empty String
- origin: non-empty String
- history: non-empty String
- power_or_function: non-empty String
- cost_or_drawback: non-empty String
- rarity: non-empty descriptive String
- condition: non-empty descriptive String
- secret: non-empty String
- story_hooks: non-empty list, target 3 values

Important product boundary:

**Item v1 is a worldbuilding/design schema, not a universal inventory-stat schema.**

Do not add:
- damage
- armor rating
- weight
- item level
- cooldown
- equip slot
- DPS
- durability numbers
- price/currency values
- universal rarity tiers as gameplay truth
- engine-specific IDs/stats

If content uses words like rare/common/legendary, they remain authored descriptive worldbuilding unless a later Game Schema Pack explicitly owns game semantics.

Contract quality requirements:

- deny unknown fields where repository convention supports it
- envelope EntityType must be Item
- validate required text/list minimums
- preserve Unicode
- preserve stable serde names
- Canon/Origin/Favorite/tags/notes/provenance use the shared EntityEnvelope lifecycle

Do not add owner IDs, creator IDs, location IDs, guardian IDs, Quest IDs, relationship IDs, or embedded foreign entities to the Item payload. Those belong to first-class Relationships or later Forge Further actions.

# Item Profiles v1

Support stable serialized profiles:

- weapon
- armor
- artifact
- consumable
- tool
- key-item
- resource
- treasure
- book
- quest-item
- other

Human-facing labels may use spaces/title case; serialized values must be stable and tested.

Do not create separate Entity Types for Item profiles.

# Generation kind and versions

Add Item as its own GenerationKind/category.

Requirements:

- existing Character/Quest/Location/Faction serialized generation keys unchanged
- adding Item does not alter old field-stream derivation
- no old enum serialization changes
- Item has explicit generator/content version constants
- no Character/Quest/Location/Faction version bump
- no DB schema bump

# ItemCandidate

Implement a generated Candidate wrapper following the proven Faction/Location pattern:

- typed Item payload
- ForgeDNA
- per-field origin metadata
- validation that all generated fields are registered
- GenerationMetadata
- lock state
- nonce state
- deterministic reroll support
- manual field-origin tracking

Do not attach Relationships during standalone generation.

# Stable Item field keys

Define one stable FieldKey per generated Item field.

The exact set/order becomes a compatibility contract when B4 passes.

Use the proven pattern:

- profile has its own stable control key
- Name has its own stable control key
- payload property names and typed action keys are intentionally mapped
- frontend rendering resolves through the real current Item field-control key rather than hardcoded Character/Location/Faction assumptions

Add explicit regressions for these boundaries.

# Dependency graph

Design an acyclic graph that matches actual generation reads.

A reasonable semantic shape is:

    item_profile
        ↓
    name
    appearance
    purpose
        ↓
    properties
    power_or_function
    origin
        ↓
    history
    cost_or_drawback
    rarity
    condition
    secret
    story_hooks

This sketch is guidance, not a mandatory graph.

Requirements:

- graph is acyclic
- every dependency corresponds to fields actually read by generation
- rerolling a field changes only its required dependency closure
- locked downstream fields form the existing dependency firewall
- locked values and nonces remain unchanged
- no hidden global RNG

Document the final graph in the mission report.

# Presets

Support the same 11 core universe presets:

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

Reuse stable preset IDs and Creative Axes.

Do not invent an Item-only preset framework.

# Item content corpus

Create dedicated authored Item material.

Every non-Chaos preset needs meaningful diversity for all 13 fields.

Quality requirements:

- avoid two-choice pools
- target at least 6 meaningful authored alternatives/fragments at each major selection layer
- profile identity must matter: Weapon, Book, Resource, Key Item, Artifact, Consumable, etc. should not read like the same generic object with nouns swapped
- appearance, purpose, properties, function, drawback, rarity, condition, secret, and hooks should cohere often enough to be directly useful
- cost_or_drawback should create interesting tradeoffs without assuming a specific combat/stat system
- rarity should describe social/world scarcity, provenance, control, uniqueness, or accessibility rather than hard-coded game loot tiers
- condition should be descriptive and useful for fiction/design, not a durability stat
- secret should add narrative leverage rather than simply contradicting purpose
- story hooks should generate actionable narrative possibilities

Chaos Blend must structurally mix compatible material across presets without one source preset dominating.

Coherence matters more than maximum combinatorics.

# Deterministic generation

Item generation must be a pure function of:

- ForgeDNA
- preset
- Creative Axes
- supported constraints if any
- per-field controls/nonces
- declared dependencies

No current time.
No filesystem.
No network.
No mutable global RNG.

Same DNA = exact same Candidate.

# Creative Axes

Reuse the current Creative Axes where semantically meaningful.

Prove:

- axis changes can influence Item output
- extremes remain valid
- neutral remains deterministic
- existing Character/Quest/Location/Faction behavior unchanged

Do not add an Item-only axis model in B4.

# Lock / reroll / edit semantics

Implement:

- lock/unlock each field
- reroll one field
- reroll all unlocked
- manual edit
- field origins
- nonce behavior
- undo/redo through current session mechanics

Invariants:

- locked value does not change on unrelated/reroll-all operations
- locked nonce does not change
- manual edit records manual origin
- rerolled field returns to generated origin
- dependency closure is correct
- reroll-all preserves every lock

# Storage integration

Extend StoredEntity and storage validation to support:

- ItemCandidate
- Canon Item snapshot where current lifecycle expects typed non-candidate values

Requirements:

- schema 2 remains authoritative
- entity_type column = item
- typed body_json validates as Item
- payload/type mismatch fails
- Candidate save/reopen
- favorite
- Candidate → Canon
- Canon revision
- history/revisions
- stale-write protection
- close/reopen preserves DNA, locks, field origins, tags/notes, state/revisions
- existing Character/Quest/Location/Faction/B2 rows remain unchanged and readable

Standalone Item generation creates **zero Relationship rows**.

# History

Item operations participate in the existing forge_runs lifecycle using the proven action vocabulary where applicable:

- forge
- reroll_field
- reroll_all_unlocked
- manual_edit
- restore_history

Do not replace forge_runs.

Do not introduce a second history system.

# Export

Add Item support to:

- JSON entity export
- Markdown entity export

Markdown should be a useful worldbuilding Item sheet containing all canonical fields.

Verify project/Vault roundtrip with at least one saved Item.

forgeproject archive format remains v1 unless repository evidence proves the structure itself must change. A new Entity Type alone is not a reason to bump the archive format.

# Desktop / Tauri integration

Add the minimum complete commands/adapters needed for Item as a first-class standalone mode.

Do not duplicate generator logic in Tauri.

Do not disturb B2 Hometown session/bundle behavior.

# Workbench UI

Extend the approved Workbench rather than redesigning it.

Requirements:

- Character / Quest / Location / Faction modes remain intact
- add Item as a fifth Forge mode
- preset, seed, and Creative Axes available
- all 13 fields visible
- lock/unlock
- field reroll
- reroll all unlocked
- manual edit for text/list fields through current pattern
- Item Profile editing remains typed/valid; do not allow arbitrary invalid enum text
- undo/redo
- save Candidate
- favorite
- Canon promotion/revision
- history/revision inspection
- JSON/Markdown export
- Item library/list/detail
- keyboard usability
- Reduced Motion remains respected

No Forge Further buttons for Item in B4.

# UI field-control parity gate

B4 must explicitly prove that Item follows the corrected universal mapping pattern.

Automated frontend regressions must use a REAL ItemCandidate-shaped payload and prove at least:

### Name

- Lock Name sends the correct domain action field
- returned candidate's actual Name field control is locked
- visual state shows Locked / Unlock Name
- aria-pressed = true

### Item Profile

- Lock Item Profile sends the correct typed action key
- actual profile field control is locked
- visual state shows Locked / Unlock Item Profile
- profile reroll sends the correct domain action key
- typed profile editing cannot emit an invalid profile value

These tests do not close Location UI-001. That issue remains separately tracked until human verification.

# Automated quality gates

## A. Contract and determinism

At minimum:

- valid/invalid Item contract
- stable profile serialization
- stable GenerationKind serialization
- stable canonical field-key set
- acyclic dependencies
- deterministic replay
- ForgeDNA input not mutated by generation
- lock/dependency firewall
- reroll-all lock invariants
- manual edit origin
- Unicode roundtrip
- unsupported version rejection
- invalid preset rejection
- Character/Quest/Location/Faction deterministic regressions unchanged
- B2 Hometown regressions unchanged
- standalone Item generation creates no Relationships

## B. Preset sweep

Generate at least:

- 256 seeds × 11 presets = 2,816 Candidates

Assert:

- every Candidate valid
- exact deterministic replay
- no exact full Item clones inside each preset sweep unless explicitly justified
- required fields/lists complete

## C. Chaos Blend

Generate at least:

- 1,024 Chaos Blend seeds

Measure/report:

- exact duplicate count
- profile distribution
- meaningful source/pattern diversity where architecture exposes it
- obvious source-dominance failure if measurable

Do not invent unsupported numerical quality metrics.

## D. Sample audit

Create:

docs/quality/THE_FORGE_B4_ITEM_SAMPLES.md

Include:

- 4 representative Items from each non-Chaos preset = 40
- 12 Chaos Blend Items
- total = 52

Assess:

- full clones
- malformed prose
- contradictions
- severe repetition
- preset identity
- profile identity
- coherence between purpose/properties/function/drawback/origin/history/rarity/condition/secret

Classify:

- usable as-is
- minor edit
- weak

Label this accurately as generated/content-author review, not independent human playtest.

# Regression / verification

Run all repository-standard gates:

- frontend format
- frontend lint
- frontend typecheck
- frontend tests
- frontend production build
- cargo fmt --all -- --check
- strict Clippy with warnings denied
- cargo test --workspace --no-fail-fast
- Cargo metadata
- scripts/verify.ps1
- optimized Tauri no-bundle build if not already covered
- git diff --check

The existing 1,000-sequence deterministic state gauntlet must remain green.

Use isolated CARGO_TARGET_DIR if Windows locks the default executable.
Do not kill the user's app merely to replace a binary.

# Human review policy

The operator has authorized continued development through non-destructive bounded feature work and prefers a broader review later.

Therefore B4 may PASS without immediate human smoke if:

- all automated behavior/data-integrity gates pass
- no P0/P1/P2 defect is known
- no migration/security/product-decision gate appears

New clearly cosmetic P3 issues may be logged in GAZINGA/KNOWN_ISSUES.md without stopping the pipeline.

Do NOT bypass human review for:

- data loss
- schema/migration safety
- security boundary change
- release/publish
- a product decision not already covered by canon

# Documentation

Create:

- docs/missions/THE_FORGE_B4_ITEM_CORE_REPORT.md
- docs/quality/THE_FORGE_B4_ITEM_SAMPLES.md

Update:

- GAZINGA/DISCOVERY_STATE.md
- GAZINGA/PROJECT_STATE.md where appropriate
- GAZINGA/KNOWN_ISSUES.md only if a real new known issue is discovered

Canonical design docs change only if implementation evidence forces a real design correction.

# Exit gate

B4 PASS requires all:

- typed Item contract
- all 13 fields
- all 11 profiles
- deterministic Item Forge
- all 11 presets
- lock/reroll/manual-edit/undo-redo semantics
- lifecycle persistence / Candidate / favorite / Canon / revisions / history
- JSON/Markdown export
- close/reopen and Vault roundtrip
- UI mode/library/detail
- Name/Profile typed action/control regressions
- preset + Chaos sweeps pass
- 52-sample quality evidence produced with no unresolved severe content defect
- Character/Quest/Location/Faction/B2 regressions green
- full repository verification green
- no known P0/P1/P2 defect

STOP.
Do not begin Creature Forge or any Item Forge Further action.
