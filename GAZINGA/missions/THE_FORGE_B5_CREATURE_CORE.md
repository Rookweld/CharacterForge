# EXECUTION CONTRACT

You are executing exactly one bounded post-v1 The Forge mission.

Before coding:
1. Read GAZINGA/CANON_CORE.md.
2. Read GAZINGA/THE_FORGE_CANON_CORE.md.
3. Read GAZINGA/DISCOVERY_STATE.md.
4. Read GAZINGA/KNOWN_ISSUES.md.
5. Read this mission completely.
6. Read docs/design/ENTITY_SCHEMA_REFERENCE.md sections 1, 2, 8, 9, 10.
7. Read the accepted B3 and B4 reports to preserve the proven universal new-entity baseline.
8. Inspect the actual repository before changing contracts.

During work:
- preserve Character, Quest, Location, Faction, Item, Relationship, and Hometown behavior
- preserve schema version 2 and forgeproject format v1
- preserve every existing generation kind/version and stable field key
- add tests with implementation
- do not weaken golden tests
- do not implement Creature Forge Further actions
- do not create Relationships as a side effect of standalone Creature generation
- do not implement World, Ability, Visual, AI-provider, engine-adapter, Graph, Timeline, Cloud, Collaboration, or Game Schema Pack features
- do not begin the next mission

At the end:
- run every required verification gate
- create docs/missions/THE_FORGE_B5_CREATURE_CORE_REPORT.md
- create docs/quality/THE_FORGE_B5_CREATURE_SAMPLES.md
- update GAZINGA/DISCOVERY_STATE.md
- update GAZINGA/PROJECT_STATE.md only where repository facts materially change
- report PASS only if every automated exit criterion is satisfied
- STOP

# THE FORGE B5 — CREATURE CORE
## Standalone deterministic Creature Forge

# Objective

Implement **Creature** as the fourth new post-v1 typed Forge entity and complete the first standalone six-entity authoring foundation around Character, Quest, Location, Faction, Item, and Creature.

B5 delivers:

    Forge Creature
      → deterministic Candidate
      → locks / rerolls / manual edits
      → undo / redo
      → save Candidate
      → favorite
      → promote to Canon
      → revise Canon
      → history/revisions
      → close/reopen
      → JSON/Markdown export

B5 deliberately does NOT implement Creature Forge Further, habitat/prey/master/guarded-item relationships, Ability entities, gameplay stat packs, Visual Forge, or combat systems.

# Starting baseline

Expected:
- branch master
- HEAD includes accepted B4 Item Forge
- schema 2
- forgeproject v1
- app version 1.0.0
- EntityType::Creature already exists in substrate vocabulary
- Character/Quest/Location/Faction/Item/Hometown regressions green
- UI-001 remains known P3 debt

If baseline materially differs, STOP and report.

# Creature schema v1

Required fields:

1. name
2. creature_profile
3. appearance
4. habitat
5. behavior
6. ecology
7. threat
8. abilities_summary
9. weakness
10. origin
11. signs_of_presence
12. loot_or_resource
13. secret
14. story_hooks

Value shapes:
- name: non-empty String
- creature_profile: typed profile
- appearance: non-empty String
- habitat: non-empty String
- behavior: non-empty String
- ecology: non-empty String
- threat: non-empty String
- abilities_summary: non-empty String
- weakness: non-empty String
- origin: non-empty String
- signs_of_presence: non-empty String
- loot_or_resource: non-empty String
- secret: non-empty String
- story_hooks: non-empty list, target 3

Contract requirements:
- EntityType::Creature
- shared EntityEnvelope lifecycle
- deny unknown fields where current convention supports it
- Unicode-safe
- stable serde names
- validate required text/list minimums
- no embedded foreign entity IDs
- no combat stat block

# Profiles v1

Stable serialized profiles:
- beast
- monster
- undead
- construct
- aberration
- spirit
- elemental
- humanoid-creature
- swarm
- boss
- other

Do not create separate Entity Types for profiles.

# Explicit non-goal

Creature Core describes worldbuilding/game-design semantics, not universal combat math.

Do NOT add:
- hit points
- armor class
- damage
- DPS
- level
- challenge rating
- speed numbers
- aggro radius
- loot tables
- encounter XP
- cooldowns
- engine IDs

Those belong to future Game Schema Packs or specialized runtime systems.

# Generation kind and versions

Add Creature as its own GenerationKind.

Requirements:
- old GenerationKind serialized values unchanged
- Creature generator/content version constants = explicit v1
- no old entity version bump
- no DB schema bump
- no archive format bump

# CreatureCandidate

Follow Item/Faction pattern:
- typed Creature
- ForgeDNA
- per-field origin metadata
- deterministic IDs
- complete field-control registration
- validation
- locks
- rerolls
- reroll-all
- manual edits
- provenance sync

# Stable field keys

Freeze one stable FieldKey per generated field.

Recommended:
- 01-creature-profile
- 02-name
- 03-appearance
- 04-habitat
- 05-behavior
- 06-ecology
- 07-threat
- 08-abilities-summary
- 09-weakness
- 10-origin
- 11-signs-of-presence
- 12-loot-or-resource
- 13-secret
- 14-story-hooks

Action keys must intentionally map typed domain fields:
- creature_profile
- name
- appearance
- habitat
- behavior
- ecology
- threat
- abilities_summary
- weakness
- origin
- signs_of_presence
- loot_or_resource
- secret
- story_hooks

Do not hardcode Character/Location/Faction/Item numeric keys in UI logic.

# Dependency graph

Implement an acyclic graph that matches actual generation reads.

Reasonable shape:
    creature_profile
      → appearance
      → behavior
      → ecology
      → threat
      → abilities_summary
      → weakness
      → loot_or_resource

Other fields may remain independent if generation does not read profile/other fields.

Requirements:
- graph reflects real reads only
- locked downstream fields form dependency firewall
- locked value and nonce remain stable
- reroll-all increments eligible controls exactly once
- no hidden mutable RNG

Document final graph in report.

# Presets

Support the same 11:
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

# Creature content corpus

Author dedicated material.

Per non-Chaos preset:
- meaningful diversity for all fields
- avoid two-choice pools
- target at least 6 meaningful alternatives/fragments at major layers
- preserve preset identity
- profile structure matters materially
- habitat/ecology/behavior/threat/weakness should cohere
- signs_of_presence should feel useful for discovery/tracking
- loot_or_resource must remain descriptive worldbuilding, not numeric loot-table semantics
- secret/story_hooks should create usable narrative hooks

Chaos Blend:
- deterministic multi-source composition across compatible corpora
- avoid one-source domination
- preserve basic coherence
- measure source/profile distribution if architecture exposes it

# Determinism

Pure function of:
- ForgeDNA
- preset
- Creative Axes
- field controls/nonces
- declared dependencies
- supported constraints, if any

No time, filesystem, network, mutable global RNG.

Same DNA = exact same Candidate.

# Lock/reroll/edit

Implement:
- lock/unlock every field
- reroll field
- reroll all unlocked
- manual edit
- field origins
- nonce behavior
- undo/redo

Invariants:
- locked values unchanged
- locked nonces unchanged
- manual edit → manual origin
- explicit target reroll → generated origin
- dependency closure exact
- reroll-all preserves all locks

# Storage

Extend StoredEntity for CreatureCandidate / canonical Creature.

Verify:
- schema stays 2
- entity_type = creature
- typed body validation
- type/payload mismatch fails
- Candidate save/reopen
- favorite
- Candidate → Canon
- Canon revision
- history/revisions
- stale-write protection
- close/reopen preserves DNA/locks/origins/tags/notes
- no Relationship rows from standalone generation

# History

Reuse existing forge_runs vocabulary:
- forge
- reroll_field
- reroll_all_unlocked
- manual_edit
- restore_history

Do not create another history system.

# Export / Vault

Add Creature:
- JSON
- Markdown
- project export/import
- Vault backup/restore

Markdown should read like a useful Creature sheet.

Keep archive format v1 unless actual format incompatibility proves otherwise.

# Desktop/Tauri/UI

Add Creature as sixth authoring mode overall / next standalone Workbench mode.

Requirements:
- all 14 fields visible
- typed Profile select
- locks/rerolls/edit
- undo/redo
- save/favorite/Canon/revision/history/export
- library/list/detail
- keyboard usability
- Reduced Motion preserved
- no Creature Forge Further controls

# UI control parity gate

With a REAL CreatureCandidate payload prove:

Name:
- correct action key
- actual 02-name control locked
- Locked/Unlock visible
- aria-pressed true

Creature Profile:
- correct creature_profile action key
- actual 01-creature-profile locked
- Locked/Unlock visible
- reroll uses creature_profile
- invalid profile manual edit rejected atomically

# Automated gates

## Contract/determinism
At minimum:
- valid/invalid Creature contract
- stable profile serialization
- stable GenerationKind serialization
- stable field-key set
- acyclic dependency graph
- deterministic replay
- input DNA not mutated
- lock/dependency firewall
- reroll-all lock invariants
- manual edit origin
- Unicode roundtrip
- unsupported version rejection
- invalid preset rejection
- all previous entity regressions unchanged
- B2 Hometown regressions unchanged

## Preset sweep
Generate:
- 256 seeds × 11 presets = 2,816 candidates

Assert:
- every Candidate valid
- exact replay
- no exact full-content duplicates inside a preset unless explicitly justified
- required fields complete

## Chaos
Generate:
- 1,024 Chaos seeds

Report:
- exact duplicate count
- profile distribution
- source/pattern diversity if exposed
- any source-dominance concern

## Sample review
Create:
docs/quality/THE_FORGE_B5_CREATURE_SAMPLES.md

Include:
- 4 × each non-Chaos preset = 40
- 12 Chaos
- total 52

Assess:
- malformed prose
- full clones
- contradictions
- severe repetition
- preset identity
- profile coherence
- habitat/ecology/behavior/threat/weakness coherence
- signs/loot/secret/hooks usefulness

Classify:
- usable as-is
- minor edit
- weak

Label accurately as generated/content-author review, not independent human playtest.

# Verification

Run:
- frontend format
- frontend lint
- frontend typecheck
- frontend tests
- frontend production build
- cargo fmt --all -- --check
- strict Clippy with warnings denied
- cargo test --workspace --no-fail-fast
- existing 1,000-sequence gauntlet
- Cargo metadata
- scripts/verify.ps1
- optimized Tauri no-bundle build
- git diff --check

Use isolated CARGO_TARGET_DIR if Windows locking requires it.
Do not kill the user's running app just to replace a binary.

# Human review policy

B5 may PASS without immediate operator smoke when:
- automated behavior/data-integrity gates pass
- no P0/P1/P2 remains
- no migration/security/product-decision gate arises

New cosmetic P3s may be logged and deferred.

Do NOT bypass human review for:
- data loss
- schema/migration safety
- security boundaries
- release/publish
- unresolved product/design decision

# Exit gate

PASS requires:
- typed Creature contract
- all 14 fields
- all 11 profiles
- deterministic generation
- all 11 presets
- lock/reroll/manual-edit/undo-redo
- Candidate/favorite/Canon/revisions/history
- JSON/Markdown
- close/reopen + project/Vault roundtrip
- Workbench/library/detail
- Name/Profile control parity
- 2,816 preset sweep
- 1,024 Chaos sweep
- 52-sample quality packet
- all previous regressions green
- full repository verification green
- no known P0/P1/P2

STOP.
Do not begin Creature Forge Further, World Forge, or any post-Creature mission.
