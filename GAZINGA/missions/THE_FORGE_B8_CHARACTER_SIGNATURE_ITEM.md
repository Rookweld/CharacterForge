# EXECUTION CONTRACT

Execute exactly one bounded post-v1 The Forge mission.

Before coding:
1. Read GAZINGA/CANON_CORE.md.
2. Read GAZINGA/THE_FORGE_CANON_CORE.md.
3. Read GAZINGA/DISCOVERY_STATE.md.
4. Read GAZINGA/KNOWN_ISSUES.md.
5. Read this mission completely.
6. Read docs/design/FORGE_FURTHER_AND_CONTEXT.md, especially the six-entity matrix and Context Compiler sections.
7. Read docs/design/ENTITY_SCHEMA_REFERENCE.md Character and Item sections.
8. Read B2/B2.R1, B4, B6, and B7 reports.
9. Inspect the actual repository before changing Forge Further contracts.

During work:
- preserve all six standalone entity generators and deterministic replay
- preserve B2 Hometown and B7 Character→Faction behavior
- preserve B6 Relationship lifecycle
- preserve database schema 2, forgeproject v1, app version 1.0.0
- reuse existing ItemForge; do not create a second Item generator
- generation creates Candidate Item + Candidate Relationship only
- source Character is never mutated
- no auto Canon
- no Link Existing
- no bundled Canon promotion
- no new relationship vocabulary
- use existing RelationshipType::Owns exactly as currently defined: Character → Item
- do not add wields in this mission
- no Visual Forge yet
- no multi-source graph traversal
- no later mission

At the end:
- run full verification
- create docs/missions/THE_FORGE_B8_CHARACTER_SIGNATURE_ITEM_REPORT.md
- create docs/quality/THE_FORGE_B8_CHARACTER_SIGNATURE_ITEM_SAMPLES.md
- update GAZINGA/DISCOVERY_STATE.md
- update GAZINGA/PROJECT_STATE.md only for material facts
- PASS only if every exit criterion is satisfied
- STOP

# THE FORGE B8 — CHARACTER → SIGNATURE ITEM
## Forge their signature item

# Objective

Implement the third real Forge Further action:

    Saved Character
      → Forge Their Signature Item
      → bounded Character context
      → deterministic Item Candidate
      → proposed Character --owns--> Item Candidate relationship
      → review/edit/reroll
      → atomic bundle save
      → universal B6 Connections lifecycle

This mission should prove that Forge Further now generalizes across Location, Faction, and Item while retaining one stable provenance/relationship model.

# Starting baseline

Expected:
- master HEAD includes accepted B7 implementation commit
- schema 2
- RelationshipType::Owns already exists as Character → Item
- standalone Item Forge is accepted
- B6 Connections / Candidate→Canon / Archive / history are accepted
- B7 Character→Faction is accepted
- worktree contains only the four preserved unrelated untracked files

If materially different, stop and report.

# Forge Action

Stable action:

    action_id: character.forge-signature-item
    action_version: 1
    source_type: Character
    target_type: Item
    intent: Signature Item
    relationship_type: owns
    context_recipe_id: character-signature-item-v1
    context_recipe_version: 1

Policy:
- one source Character
- one primary Item Candidate
- zero supporting Candidates
- one proposed owns Relationship Candidate
- inherit source preset
- inherit source Creative Axes
- explicit deterministic action seed
- no whole-project context
- no graph traversal
- no relationship traversal

Reuse/generalize the B7 Forge Further architecture only as far as necessary. Avoid a new parallel framework.

# Context recipe

Compile a bounded immutable Character context from the exact persisted source state.

Include at minimum:
- source entity ID
- source write_version
- source Canon state
- exact source generation kind/version/content provenance when available
- Character name
- title
- archetype
- role
- appearance
- personality
- desire
- fear
- flaw
- signature_gear
- skills
- abilities
- relationship_hook
- backstory_hook
- secret
- inherited preset
- inherited Creative Axes
- canonical source body hash
- deterministic context hash

The compiler is read-only.

The recipe must strongly prioritize signature_gear as direct authored/generated intent and use role/archetype/skills/abilities/appearance/desire/secret as coherence context.

Do not treat signature_gear prose as an authoritative foreign-key relationship.
Do not silently hard-copy the signature_gear string as the final Item name.
Do not traverse unrelated project entities.

# Deterministic generation

Use the accepted canonical ItemForge generator.

Derive target Item DNA deterministically from:
- stable action ID/version
- context hash
- explicit action seed
- inherited preset
- inherited Creative Axes

Do not alter standalone Item replay.

Same source state + same action seed = exact same Item Candidate and proposed Relationship identity.

Different action seed should deterministically vary output where expected.

The contextual Item path may use a typed/versioned Character context constraint/input if needed, but empty/no context must preserve the exact standalone Item field-stream behavior.

# Contextual Item coherence

The generated Item should feel like this Character's signature item rather than a random inventory object.

Context should influence, where appropriate:
- profile
- appearance
- purpose
- properties
- origin
- history
- power_or_function
- cost_or_drawback
- rarity
- condition
- secret
- story_hooks

The implementation must remain worldbuilding-oriented.

Do NOT add:
- damage
- armor rating
- DPS
- level
- cooldown
- weight
- equip slot
- rarity numbers
- engine/runtime IDs

Do not invent universal gameplay stats.

# Candidate relationship

Propose exactly:

    Character --owns--> generated Item

Requirements:
- RelationshipType::Owns
- state Candidate
- origin Generated
- stable identity during review
- no inverse duplicate
- passes current Relationship Definition validation
- source Character unchanged
- Item remains independent Candidate truth

After bundle save, B6 Connections must show:
- Character side: Owns <Item> · Candidate
- Item side: Owned by <Character> · Candidate

Relationship can then independently promote/archive without changing either Entity Canon state.

Do not add wields in B8. If the item is a weapon, ownership is still the single structural assertion created by this action.

# Review finalization

The unsaved Item Candidate must support normal:
- locks
- reroll field
- reroll all
- manual edits
- undo/redo

Before Save Bundle, finalize persisted recipe provenance so target DNA exactly equals the final reviewed Item Candidate DNA.

Preserve:
- frozen source context/hash
- action/version
- action seed
- stable target Item Entity ID
- stable owns Relationship ID/type/endpoints
- result mapping

Reject:
- changed target Entity ID
- wrong entity type
- project mismatch
- Relationship target mismatch
- Relationship type mismatch
- unsupported provenance
- recipe/action mismatch
- stale source at commit

Follow B2.R1/B7 reviewed-DNA parity.

# Atomic persistence

Saving accepted result is one transaction:

1. final reviewed Item Candidate
2. Candidate owns Relationship
3. immutable forge_runs event
4. typed recipe/context snapshot
5. typed result mapping

All or none.

Source optimistic write_version must be revalidated immediately before commit.

Injected failure after every meaningful stage must roll back all rows.

Duplicate save must reject rather than silently create duplicate Item/Relationship/operation.

No schema bump.

# Typed persistence and compatibility

Persist versioned typed recipe/result data sufficient to answer:
- source Character ID/write version/Canon state
- action ID/version
- context recipe ID/version
- source/body/context hash
- inherited preset/axes
- action seed
- final Item DNA
- primary Item ID
- owns Relationship ID

Prefer sharing/generalizing the existing B7 envelope where safe.

Existing Hometown and Faction persisted JSON must remain readable and unchanged in meaning.

Do not serialize arbitrary frontend state.

# Desktop UX

From a saved supported Character detail view add:

**Forge Their Signature Item**

Requirements:
- available only when deterministic source provenance can be resolved
- unsupported manual/imported source is clearly disabled/explained
- enters review flow consistent with Hometown/Faction
- shows source Character
- shows action
- shows generated Item Candidate
- shows proposed Owns relationship
- Candidate status
- compact frozen context/provenance summary
- normal Item Profile/Name and field lock/reroll/edit controls
- Discard persists nothing
- Save Bundle persists atomically
- saved Item appears in Items library
- B6 Connections shows both endpoint views

Do not redesign Workbench.
Keep action naming clear and human-readable.

# Quality review

Generate and review at least 44 examples:
- 4 examples across each of ten non-Chaos presets = 40
- 4 Chaos examples

Assess:
- visible relevance to signature_gear
- relevance to role/archetype/skills/abilities
- no severe contradiction with appearance/desire/secret
- profile appropriateness
- Item purpose/properties/function coherency
- useful origin/history/drawback/secret/hooks
- no raw UUID/hash/provenance leakage
- no full-content clones
- no accidental game-stat system

Classify each as:
- usable as-is
- minor edit
- weak

This is generated/content-author review, not independent human playtest.

# Tests — app-core

At minimum:
- stable action/recipe identifiers and versions
- compiler deterministic/read-only
- same source state => same context bytes/hash
- relevant source change => context hash changes
- inherited preset/axes exact
- same source/context/seed => exact same Item
- changed seed deterministic variation
- source Character unchanged
- owns Relationship valid
- standalone Item replay/goldens unchanged
- B7 Faction path unchanged
- B2 Hometown unchanged
- finalizer reviewed target DNA parity
- bad ID/type/action/mapping/provenance rejected

# Tests — storage

At minimum:
- Candidate source
- Canon source
- exactly one Item + owns + forge_run
- no writes before Save Bundle
- stale source => complete rollback
- injected stage failures => complete rollback
- duplicate save rejected
- close/reopen exact
- forgeproject export/import preserves operation/Relationship
- reviewed lock+rereoll-all Item DNA persisted exactly
- reviewed field reroll/manual edit Item DNA persisted exactly
- source Character row unchanged
- B6 promote/archive saved owns independently
- no inverse duplicate

# Tests — frontend

At minimum:
- supported Character shows Forge Their Signature Item
- unsupported provenance clearly explained
- review displays Item and Owns
- Item Name/Profile controls correct
- reroll/edit reflected in save
- discard no persistence
- save refreshes Items/Connections
- Character wording: Owns X
- Item inverse wording: Owned by Character
- no auto Canon
- stale save does not fake success

# Full verification

Run:
- frontend format
- frontend lint
- frontend typecheck
- frontend tests
- frontend production build
- cargo fmt --all -- --check
- strict Clippy -D warnings
- cargo test --workspace --no-fail-fast
- existing 1,000-sequence gauntlet
- Cargo metadata
- scripts/verify.ps1
- optimized Tauri build --no-bundle
- git diff --check

Use isolated target dir if needed. Do not kill user processes.

# Human review policy

B8 may PASS without immediate operator smoke if all automated integrity/determinism/lifecycle gates pass and no P0/P1/P2 remains.

Stop for human decision if implementation requires:
- adding wields
- Link Existing semantics
- bundled Canon promotion
- new vocabulary
- schema migration
- cardinality policy

# Exit gate

PASS only if:
- Character → Forge Their Signature Item works from saved deterministic Character
- bounded context is truthful and immutable
- canonical ItemForge is used
- deterministic replay proven
- final reviewed Item DNA parity proven
- Candidate owns relationship is atomic and correct
- B6 Connections shows correct source/inverse views
- relationship lifecycle remains independent from Entity Canon
- B2 Hometown and B7 Character→Faction remain green
- no schema/archive/app-version bump
- full verification green
- no known B8 P0/P1/P2 remains

STOP.

Do not begin Visual Forge, Link Existing, multi-source Forge, Creature/Item/Faction reverse Forge Further, or any later mission.
