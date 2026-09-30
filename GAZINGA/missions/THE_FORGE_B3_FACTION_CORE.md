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
   - docs/design/THE_FORGE_ENTITY_UNIVERSE.md sections 3, 5-9, 14-19
   - docs/design/ENTITY_SCHEMA_REFERENCE.md sections 1, 2, 6, 9, 10
   - docs/design/ENTITY_SUBSTRATE_AND_MIGRATION.md sections 2-7, 18-21, 31-33
7. Read the B1 and B2 reports to preserve the proven new-entity and Forge Further baselines.
8. Inspect the actual repository before changing contracts.

During work:

- preserve Character, Quest, Location, Relationship, and Hometown behavior
- preserve schema version 2
- preserve all existing deterministic replay contracts and stable field keys
- add tests with implementation
- do not weaken golden outputs/tests
- do not implement any Faction Forge Further action
- do not create Relationships as a side effect of standalone Faction generation
- do not implement Item, Creature, World, Ability, Visual, AI-provider, engine-adapter, Graph, Timeline, Cloud, or Collaboration features
- do not begin the next mission

At the end:

- run every required verification gate
- create docs/missions/THE_FORGE_B3_FACTION_CORE_REPORT.md
- create docs/quality/THE_FORGE_B3_FACTION_SAMPLES.md
- update GAZINGA/DISCOVERY_STATE.md
- update GAZINGA/PROJECT_STATE.md only where repository facts materially change
- report PASS only if every automated exit criterion is satisfied
- STOP

# THE FORGE B3 — FACTION CORE
## Standalone deterministic Faction Forge

# Objective

Implement **Faction** as the second new post-v1 typed Forge entity and prove that the universal lifecycle scales beyond Location without creating a parallel architecture.

B3 delivers a complete standalone Faction Forge experience:

    Forge Faction
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

B3 deliberately does NOT implement Character → Faction Forge Further, faction headquarters, leaders, rivals, or other relationship-driven actions.

# Starting baseline

Expected baseline:

- branch: master
- HEAD includes accepted B2/B2.R1 Hometown flow
- database schema = 2
- EntityType::Faction already exists in the substrate vocabulary
- first-class Relationships and B2 Forge Further remain green
- UI-001 remains known P3 debt for intermittent Location lock visual feedback

Re-inspect actual state. If the baseline materially differs, stop and report instead of silently adapting scope.

# Product contract — Faction schema v1

Implement a strongly typed Faction contract matching the canonical design.

Required fields:

1. name
2. faction_profile
3. identity
4. ideology
5. primary_goal
6. methods
7. culture
8. resources
9. public_face
10. hidden_truth
11. symbols
12. reputation
13. story_hooks

Expected value shapes:

- name: non-empty String
- faction_profile: stable typed profile enum/representation
- identity: non-empty String
- ideology: non-empty String
- primary_goal: non-empty String
- methods: non-empty String
- culture: non-empty String
- resources: non-empty String
- public_face: non-empty String
- hidden_truth: non-empty String
- symbols: non-empty String
- reputation: non-empty String
- story_hooks: non-empty list, target 3 values

Contract quality requirements:

- deny unknown fields where repository convention supports it
- envelope EntityType must be Faction
- validate required text/list minimums
- preserve Unicode
- preserve stable serde names
- Canon/Origin/Favorite/tags/notes/provenance use the shared EntityEnvelope lifecycle

Do not add member lists, leader IDs, headquarters IDs, relationship IDs, treasury numbers, military strength stats, or gameplay reputation numbers to the core payload. Those belong to Relationships or later game-specific schemas.

# Faction Profiles v1

Support stable serialized profiles:

- guild
- order
- cult
- government
- kingdom
- military
- corporation
- gang
- rebellion
- religious-order
- tribe
- secret-society
- other

Human-facing labels may use spaces/title case; serialized values must be stable and tested.

Do not create separate Entity Types for profiles.

# Generation kind and versions

Add Faction as its own GenerationKind/category.

Requirements:

- existing Character/Quest/Location serialized generation keys unchanged
- adding Faction does not alter old field-stream derivation
- no old enum serialization changes
- Faction has explicit generator/content version constants
- no Character/Quest/Location version bump
- no DB schema bump

# FactionCandidate

Implement a generated Candidate wrapper following the proven Location/Character pattern:

- typed Faction payload
- ForgeDNA
- per-field origin metadata
- validation that all generated fields are registered
- GenerationMetadata
- lock state
- nonce state
- deterministic reroll support
- manual field-origin tracking

Do not attach Relationships during standalone generation.

# Stable Faction field keys

Define one stable FieldKey per generated Faction field.

The exact set/order becomes compatibility contract when B3 passes.

Use a consistent domain-prefixed/profile-safe convention.

Important UI contract lesson from Location:

- payload property names and typed action field keys must be intentionally mapped
- frontend must not send a payload property name when Rust expects another domain field name
- Name/Profile lock rendering must resolve through the real current field-control key, not a Character/Location-specific hardcoded numeric key

Add explicit regressions for these boundaries.

# Dependency graph

Design an acyclic graph matching actual generation reads.

A sensible semantic shape is:

    faction_profile
        ↓
    name
    identity
    ideology
        ↓
    primary_goal
    methods
    culture
    resources
    public_face
        ↓
    hidden_truth
    symbols
    reputation
    story_hooks

This sketch is not mandatory. The implemented graph must reflect real dependencies, not aspirational semantics.

Requirements:

- acyclic
- every dependency corresponds to fields actually read
- rerolling a field changes only its required dependency closure
- locked downstream fields form the existing dependency firewall
- locked values and nonces remain unchanged
- no hidden global RNG

Document the final graph.

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

Do not invent a separate Faction-only preset system.

# Faction content corpus

Create dedicated authored Faction material.

Every non-Chaos preset needs meaningful diversity for all 13 fields.

Quality requirements:

- avoid two-choice pools
- target at least 6 meaningful authored alternatives/fragments at each major selection layer
- profiles and prose must fit the preset rather than noun-swapping one generic faction template
- Kingdom/Government/Military/Gang/etc. should feel structurally different where selected
- public_face and hidden_truth must not simply negate each other mechanically
- culture/ideology/methods/goal should cohere often enough to be usable
- story hooks should create actionable narrative possibilities

Chaos Blend must structurally mix compatible material across presets without one source preset dominating.

Coherence matters more than maximum combinatorics.

# Deterministic generation

Faction generation must be a pure function of:

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

Reuse the current Creative Axes where they are semantically meaningful.

Prove:

- axis changes can influence Faction output
- extremes remain valid
- neutral remains deterministic
- existing Character/Quest/Location axis behavior unchanged

Do not add a new Faction-only axis model in B3.

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

- FactionCandidate
- Canon Faction snapshot where current entity lifecycle expects typed non-candidate values

Requirements:

- schema 2 remains authoritative
- entity_type column = faction
- typed body_json validates as Faction
- payload/type mismatch fails
- Candidate save/reopen
- favorite
- Candidate → Canon
- Canon revision
- history/revisions
- stale-write protection
- close/reopen preserves DNA, locks, field origins, tags/notes, state/revisions
- existing Character/Quest/Location/B2 rows remain unchanged and readable

No relationship rows are created by standalone Faction Forge.

# History

Faction operations participate in existing forge_runs lifecycle using the proven action vocabulary where applicable:

- forge
- reroll_field
- reroll_all_unlocked
- manual_edit
- restore_history

Do not replace forge_runs.

Do not introduce a second history system.

# Export

Add Faction support to:

- JSON entity export
- Markdown entity export

Markdown should be a useful Faction sheet containing all canonical fields.

Verify project/Vault roundtrip with at least one saved Faction.

forgeproject archive format remains v1 unless repository evidence proves the structure itself must change. A new Entity Type alone is not a reason to bump the archive format.

# Desktop / Tauri integration

Add the minimum complete commands/adapters needed for Faction as a first-class standalone mode.

Do not duplicate generator logic in Tauri.

Do not disturb B2 Hometown session/bundle behavior.

# Workbench UI

Extend the approved Workbench rather than redesigning it.

Requirements:

- Character / Quest / Location modes remain intact
- add Faction as a fourth Forge mode
- preset, seed, and Creative Axes available
- all 13 fields visible
- lock/unlock
- field reroll
- reroll all unlocked
- manual edit for text/list fields through current pattern
- profile editing must remain typed/valid; do not allow arbitrary invalid enum text
- undo/redo
- save Candidate
- favorite
- Canon promotion/revision
- history/revision inspection
- JSON/Markdown export
- Faction library/list/detail
- keyboard usability
- Reduced Motion remains respected

No Forge Further buttons for Faction in B3.

# UI field-control parity gate

B3 must explicitly avoid repeating Location's payload/action/control mismatch.

Automated frontend regressions must use a REAL FactionCandidate-shaped payload and prove at least:

### Name
- Lock Name sends the correct domain action field
- the returned candidate's actual Name field control is locked
- visual state shows Locked / Unlock Name
- aria-pressed = true

### Faction Profile
- Lock Faction Profile sends the correct typed action key
- actual profile field control is locked
- visual state shows Locked / Unlock Faction profile
- profile reroll sends the correct domain action key

These tests do not close Location UI-001. That issue remains separately tracked until human verification.

# Automated quality gates

## A. Contract and determinism

At minimum:

- valid/invalid Faction contract
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
- Character/Quest/Location deterministic regressions unchanged
- B2 Hometown regressions unchanged

## B. Preset sweep

Generate at least:

- 256 seeds × 11 presets = 2,816 candidates

Assert:

- every Candidate valid
- exact deterministic replay
- no exact full Faction clones inside each preset sweep unless explicitly justified
- required fields/lists complete

## C. Chaos Blend

Generate at least:

- 1,024 Chaos Blend seeds

Measure/report:

- exact duplicate count
- profile distribution
- meaningful source/pattern diversity where the architecture exposes it
- obvious source-dominance failure if measurable

Do not invent unsupported numerical quality metrics.

## D. Sample audit

Create:

docs/quality/THE_FORGE_B3_FACTION_SAMPLES.md

Include:

- 4 representative Factions from each non-Chaos preset = 40
- 12 Chaos Blend Factions
- total = 52

Assess:

- full clones
- malformed prose
- contradictions
- severe repetition
- preset identity
- coherence between identity/ideology/goal/methods/culture/public face/hidden truth

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

The operator explicitly authorized continued development through non-destructive feature work and prefers a broader review later.

Therefore B3 may PASS without immediate human smoke if:

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

- docs/missions/THE_FORGE_B3_FACTION_CORE_REPORT.md
- docs/quality/THE_FORGE_B3_FACTION_SAMPLES.md

Update:

- GAZINGA/DISCOVERY_STATE.md
- GAZINGA/PROJECT_STATE.md where appropriate
- GAZINGA/KNOWN_ISSUES.md only if a real new known issue is discovered

Canonical design docs change only if implementation evidence forces a real design correction.

# Exit gate

B3 PASS requires all:

- typed Faction contract
- all 13 fields
- all 13 profiles
- deterministic Faction Forge
- all 11 presets
- lock/reroll/manual-edit/undo-redo semantics
- lifecycle persistence / Candidate / favorite / Canon / revisions / history
- JSON/Markdown export
- close/reopen and Vault roundtrip
- UI mode/library/detail
- Name/Profile typed action/control regressions
- preset + Chaos sweeps pass
- 52-sample quality evidence produced with no unresolved severe content defect
- Character/Quest/Location/B2 regressions green
- full repository verification green
- no known P0/P1/P2 defect

STOP.
Do not begin Item Forge or any Faction Forge Further action.
