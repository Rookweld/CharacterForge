# EXECUTION CONTRACT

Execute exactly one bounded post-v1 The Forge mission.

Before coding:
1. Read GAZINGA/CANON_CORE.md.
2. Read GAZINGA/THE_FORGE_CANON_CORE.md.
3. Read GAZINGA/DISCOVERY_STATE.md.
4. Read GAZINGA/KNOWN_ISSUES.md.
5. Read this mission completely.
6. Read docs/design/FORGE_FURTHER_AND_CONTEXT.md, especially the six-entity matrix and Context Compiler sections.
7. Read docs/design/ENTITY_SCHEMA_REFERENCE.md Character and Faction sections.
8. Read B2/B2.R1, B3, B5 and B6 reports.
9. Inspect the actual repository before changing Forge Further contracts.

During work:
- preserve all six standalone entity generators and deterministic replay
- preserve B2 Hometown behavior byte-for-byte where its stable contracts require it
- preserve B6 Relationship lifecycle behavior
- preserve database schema 2, forgeproject v1, app version 1.0.0
- reuse existing Faction Forge; do not create a second Faction generator
- generation creates Candidate Entity + Candidate Relationship only
- source Character is never mutated
- no auto Canon
- no Link Existing
- no bundled Canon promotion
- no new relationship vocabulary
- no Visual Forge yet
- no multi-source graph traversal
- no later mission

At the end:
- run full verification
- create docs/missions/THE_FORGE_B7_CHARACTER_FACTION_REPORT.md
- create a compact generated quality packet if useful
- update GAZINGA/DISCOVERY_STATE.md
- update GAZINGA/PROJECT_STATE.md only for material facts
- PASS only if every exit criterion is satisfied
- STOP

# THE FORGE B7 — CHARACTER → FACTION
## Forge their faction

# Objective

Implement the second real Forge Further action:

    Saved Character
      → Forge Their Faction
      → bounded Character context
      → deterministic Faction Candidate
      → proposed Character --member_of--> Faction Candidate relationship
      → review/edit/reroll
      → atomic bundle save
      → universal B6 Connections lifecycle

This is the first proof that the B2 Forge Further architecture generalizes beyond Location.

# Starting baseline

Expected:
- master HEAD includes accepted B6 implementation
- schema 2
- RelationshipType::MemberOf already exists as Character → Faction
- standalone Faction Forge is accepted
- B6 Connections / Candidate→Canon / Archive / history are accepted
- worktree contains only the four unrelated preserved files

If materially different, stop and report.

# Forge Action

Stable action:

    action_id: character.forge-faction
    action_version: 1
    source_type: Character
    target_type: Faction
    intent: Character Faction
    relationship_type: member_of
    context_recipe_id: character-faction-v1
    context_recipe_version: 1

Policy:
- one source Character
- one primary Faction Candidate
- zero supporting Candidates
- one proposed member_of Relationship Candidate
- inherit source preset
- inherit source Creative Axes
- explicit deterministic action seed
- no whole-project context
- no traversal through unrelated Relationships

If existing ForgeActionDefinition is too Hometown-specific, generalize only enough to support both Hometown and Faction without breaking B2 serialization/tests. Do not build a plugin framework.

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
- personality
- desire
- fear
- flaw
- faction_hint
- relationship_hook
- backstory_hook
- secret
- inherited preset
- inherited Creative Axes
- canonical source body hash
- deterministic context hash

The compiler is read-only.

The recipe must strongly prioritize faction_hint as direct authored/generated intent, while using identity/role/desire/secret/etc. to create a coherent organization around the Character.

Do not silently copy the faction_hint string into the final Faction name as a hard requirement; it is context, not a foreign-key truth.

No unrelated entity or Relationship context in B7.

# Deterministic generation

Use the accepted FactionForge generator.

Derive target Faction DNA deterministically from:
- action ID/version
- context hash
- explicit action seed
- inherited preset
- inherited axes

Do not alter standalone Faction replay.

No hidden clock/randomness.

Same source state + same action seed = exact same Faction Candidate and proposed relationship identity.

Different action seed should deterministically vary output where expected.

# Candidate relationship

Propose exactly:

    Character --member_of--> generated Faction

Requirements:
- RelationshipType::MemberOf
- state Candidate
- origin Generated
- stable identity during review
- no inverse duplicate
- passes existing definition validation
- source Character unchanged
- Faction Candidate remains independent Candidate truth

After bundle save, B6 Connections must show:
- Character side: Member of <Faction> · Candidate
- Faction side: Member: <Character> · Candidate

Relationship can then be independently promoted/archived using B6 without changing either entity Canon state.

# Review finalization

The unsaved Faction Candidate must support its normal:
- locks
- reroll field
- reroll all
- manual edits
- undo/redo

Before Save Bundle, finalize the Forge Further recipe so persisted target DNA equals the exact final reviewed Candidate DNA.

Preserve frozen source context/hash/action/seed and stable target Entity ID / Relationship ID.

Reject:
- reviewed Candidate with different Entity ID
- wrong entity type
- Relationship target mismatch
- relationship type mismatch
- provenance mismatch
- stale source at commit

This must follow the B2.R1 reviewed-bundle provenance principle.

# Atomic persistence

Saving accepted result is one transaction:

1. final reviewed Faction Candidate
2. Candidate member_of Relationship
3. immutable forge_runs event
4. typed recipe/context snapshot
5. typed result/output mapping

All or nothing.

Source optimistic write_version must be revalidated before commit.

Injected failure at each meaningful persistence stage must roll back the complete bundle.

Duplicate save must not silently create duplicate target/relationship/operation records.

No schema bump.

# Typed persistence records

Do not serialize arbitrary frontend state.

Persist enough typed/versioned recipe data to answer:
- which Character?
- exact source write_version/Canon state?
- which action/version?
- which context recipe/version?
- which source/context hash?
- inherited preset/axes?
- action seed?
- exact final Faction DNA?
- primary Faction ID?
- member_of Relationship ID?

Prefer a reusable Forge Further envelope if it can be introduced without breaking B2. Hometown-specific persisted records must remain readable and stable.

# Desktop UX

From a saved Character detail view, add:

**Forge Their Faction**

Requirements:
- only enabled when deterministic source provenance is supported
- unsupported manual/imported Character fails/disables clearly; do not fabricate defaults
- enters a review flow consistent with Forge Hometown
- shows source Character
- shows action name
- shows generated Faction Candidate
- shows proposed Member of relationship
- shows Candidate status
- shows compact context/provenance summary
- permits normal Faction lock/reroll/edit
- Discard persists nothing
- Save Bundle persists atomically
- after save, Faction appears in Factions library
- B6 Connections displays it from both endpoints

Avoid redesigning the Workbench.

If Hometown and Faction review state currently require duplicated frontend branches, refactor only enough to keep behavior maintainable; do not build a huge generic renderer.

# Quality expectations

Generated Faction should feel related to the Character, not like a random standalone faction.

Review at least 44 examples:
- 4 examples across each of the ten non-Chaos presets = 40
- 4 Chaos examples

Assess:
- visible relevance to faction_hint / Character identity
- no direct contradiction with source role/desire/secret unless intentionally plausible and documented
- coherent faction profile/identity/goal/methods/culture
- no exact clone pathologies
- no leakage of raw IDs/hash/provenance into creative prose

Record concise evidence in the mission report or a separate quality file.

This is generated/content-author review, not independent human playtest.

# Tests — app-core

At minimum:
- stable action ID/version/recipe ID/version
- compiler deterministic and read-only
- same source => same context bytes/hash
- relevant Character change => changed context hash
- inherited preset/axes exact
- same source/context/seed => exact same Faction
- changed seed deterministic variation
- source Character unchanged
- member_of relationship typed/endpoints valid
- standalone Faction deterministic tests unchanged
- Hometown tests unchanged
- finalizer replaces only reviewed target DNA/primary Candidate while preserving frozen source context/action/proposed Relationship identity
- invalid finalization cases reject atomically

# Tests — storage

At minimum:
- Candidate Character source save
- Canon Character source save
- bundle save creates exactly one Faction + one member_of + one forge_run
- no writes before Save Bundle
- stale source rolls back all
- injected failure rolls back all
- duplicate save rejected
- close/reopen exact
- forgeproject export/import preserves operation + Relationship
- reviewed lock/reroll-all target DNA persisted exactly
- reviewed field reroll target DNA persisted exactly
- source Character row unchanged
- B6 promotion/archive of saved member_of works independently
- no inverse duplicate row

# Tests — frontend

At minimum:
- saved supported Character shows Forge Their Faction
- unsupported source provenance explains why unavailable
- review displays Faction fields and proposed member_of
- Faction Profile/Name typed controls remain correct
- review edits/rerolls propagate to final bundle save
- discard persists nothing
- Save Bundle refreshes Factions and Connections
- Character connection wording: Member of X
- Faction inverse wording: Member: Character
- no auto-Canon
- stale save error does not fake success

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

Use isolated target dir if required. Do not kill user processes.

# Human review policy

B7 may PASS without immediate operator smoke if all automated integrity/determinism/lifecycle gates pass and no P0/P1/P2 remains.

Stop for human decision if implementation requires:
- Link Existing semantics
- bundled Canon promotion
- new relationship vocabulary
- schema migration
- new cardinality policy

# Exit gate

PASS only if:
- Character → Forge Their Faction works from a saved deterministic Character
- bounded immutable context is truthful
- Faction uses canonical FactionForge
- deterministic replay is proven
- reviewed Candidate DNA parity is proven
- Candidate member_of is proposed and saved atomically
- B6 Connections shows correct source/inverse views
- Relationship lifecycle remains independent from Entity Canon
- B2 Hometown remains green
- no schema/archive/app-version bump
- full verification green
- no known B7 P0/P1/P2 remains

STOP.

Do not begin Character → Signature Item, Link Existing, Visual Forge, multi-source Forge, or later missions.
