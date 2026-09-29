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
   - docs/design/THE_FORGE_ENTITY_UNIVERSE.md sections 3, 6-10, 14-19
   - docs/design/FORGE_FURTHER_AND_CONTEXT.md sections 2-9, 17-18, 20-38
   - docs/design/ENTITY_SUBSTRATE_AND_MIGRATION.md sections 15-20
7. Inspect the current repository and actual storage/app-core/UI implementation before changing contracts.

During work:

- preserve all v1.0 Character/Quest behavior
- preserve Mission A Relationship behavior
- preserve B1 Location deterministic replay and field keys
- do not rename existing stable serialized keys
- do not silently mutate source Character
- add tests with implementation
- do not weaken existing golden outputs/tests
- do not implement Link Existing
- do not implement relationship Canon promotion UX
- do not implement Faction/Item/Creature Forge
- do not implement Visuals, AI providers, World, Ability, engine adapters, Graph, Timeline, Cloud, or Collaboration
- do not start the next mission

At the end:

- run every required verification gate
- create docs/missions/THE_FORGE_B2_HOMETOWN_REPORT.md
- update GAZINGA/DISCOVERY_STATE.md
- update GAZINGA/PROJECT_STATE.md only where current repository facts materially change
- report PASS only if every automated exit criterion is satisfied
- STOP

# THE FORGE B2 — HOMETOWN
## First full Forge Further proof slice

# Objective

Implement the first real Forge Further flow:

    Existing saved Character
      → Forge Hometown
      → versioned Forge Action
      → bounded Context Recipe
      → frozen Context Snapshot
      → deterministic Location Candidate
      → proposed born_in Relationship
      → atomic bundle save
      → close/reopen/provenance review

This mission proves the architectural chain. It is intentionally one source type, one action, one target type, one relationship type.

# Starting baseline

Expected baseline:

- branch: master
- HEAD starts from B1 accepted functional baseline
- standalone Location Forge works
- database schema remains 2
- first-class Relationship storage exists
- born_in definition already exists
- forge_runs remains persistent operation/history storage
- UI-001 is open, non-blocking known P3 Workbench feedback debt

Re-inspect actual HEAD and stop if materially different.

# Product behavior

From a SAVED Character, the user can choose:

**Forge Hometown**

The result is:

- one unsaved Location Candidate
- Location profile fixed to Settlement
- one proposed Candidate relationship:
  Character --born_in--> Location
- provenance describing exact source state, Forge Action, Context Recipe, and Context hash

The source Character is never mutated.

The user may:

- inspect the generated Hometown
- reroll/edit Location fields using the normal Location Candidate behavior
- discard it
- save the whole candidate bundle atomically

B2 does not auto-promote either the Location or Relationship to Canon.

# Source Character requirements

B2 supports a persisted generated Character whose generation provenance/ForgeDNA can be resolved.

Support both:

- saved Candidate Character
- saved Canon Character

If a manual/imported Character lacks the required deterministic provenance, the Forge Hometown action must fail/disable clearly and non-destructively.

Do not fabricate a default preset/axes silently.

Later missions may add user-supplied Forge Further settings for manual/imported sources.

# Forge Action v1

Introduce the minimum app-owned Forge Action definition machinery required for this proof.

Stable action:

    action_id: character.forge-hometown
    action_version: 1
    source_type: Character
    target_type: Location
    target_profile: Settlement
    intent: Hometown
    relationship_type: born_in
    context_recipe_id: character-hometown-v1

Policy:

- inherit source Universe preset
- inherit source Creative Axes
- do not inherit Character-only constraints blindly
- one primary output
- zero supporting entities
- one proposed Relationship

The definition must live outside React/Tauri.

Do not build a large generic registry framework beyond what is justified by B2, but the definition must be data-driven enough that future actions do not require a frontend if/else architecture.

# Context Recipe v1

Stable recipe:

    character-hometown-v1
    version 1

Primary source:

- exact selected persisted Character state
- source entity ID
- source write_version
- source Canon state
- source generation kind/version/content provenance where available

Bounded semantic facts:

- Name
- Title
- Archetype
- Role
- Faction hint
- Backstory hook
- Desire
- Secret

Preset/axes are included separately as inherited generation inputs.

Do not traverse unrelated project entities in B2.
Do not include whole-project context.
Do not include Candidate neighbors automatically.

# Context Compiler

Implement a read-only application-layer compiler.

Input:

- ProjectStore/read model
- Forge Action
- persisted Character record

Output:

a typed immutable context object containing:

- recipe ID/version
- source Entity ID
- source write_version
- source Canon state
- source type
- exact selected semantic facts
- inherited preset
- inherited Creative Axes
- full-source snapshot hash or canonical source-body hash
- canonical context hash

Requirements:

- deterministic canonical serialization
- deterministic BLAKE3 context hash with its own explicit domain/version context
- same source state => same context bytes/hash
- relevant source change => different hash
- compiler performs no writes
- compiler creates no Entity/Relationship
- compiler does not mutate Canon

# Context Snapshot persistence decision

For B2, retain existing forge_runs as the durable immutable Forge operation/event backbone.

Do NOT replace forge_runs or introduce a parallel full operation-history system in this mission.

Persist the B2 Context Snapshot and Forge Action provenance as strongly typed versioned JSON inside the immutable forge_runs recipe_json.

Persist B2 output mapping as strongly typed versioned JSON inside forge_runs result_json.

This is an explicit B2 architecture decision:

- forge_runs.event_id = durable operation event identity
- recipe_json = versioned Forge Further recipe + frozen Context Snapshot
- result_json = versioned Forge Result Bundle output mapping
- forge_runs.entity_id = primary output Location ID after successful bundle save

No database schema bump is required if this can be implemented safely with the current schema.

If repository evidence proves this impossible or unsafe, STOP and report before introducing schema 3.

# Typed Forge Further persistence records

Create versioned typed records, naming may follow repository conventions.

Conceptually recipe_json must contain:

- format_version
- action_id
- action_version
- context_recipe_id
- context_recipe_version
- source Entity ID
- source write_version
- source Canon state
- source snapshot/body hash
- selected semantic facts
- inherited preset
- inherited Creative Axes
- operator/action seed inputs
- context hash
- derived Location ForgeDNA or enough deterministic data to reproduce it

Conceptually result_json must contain:

- format_version
- primary output Entity ID
- primary output type = Location
- relationship IDs
- relationship type = born_in
- source Entity ID
- warnings array if needed

Do not serialize arbitrary frontend state.

# Deterministic Hometown generation

The generated Hometown must use the existing Location Forge rather than a second Location implementation.

Required derived behavior:

1. inherit source Character preset
2. inherit source Character Creative Axes
3. fix Location Profile = Settlement through an explicit supported Location generation constraint/input
4. derive deterministic Location generation from:
   - stable action ID/version
   - context hash
   - deterministic action seed input
5. generate through the canonical Location field streams

The resulting LocationCandidate must retain normal Location ForgeDNA/field controls so that post-generation locks/rerolls/manual edits work normally.

Important compatibility rule:

- existing standalone B1 Location DNA with no Hometown constraint must replay exactly unchanged
- existing Character/Quest outputs must replay exactly unchanged

# Action seed

Keep B2 UX simple.

Use a deterministic explicit action seed.

Preferred UI:

- Forge Hometown opens/enters a Forge Further review state with a numeric Seed control
- default may reuse the source Character root seed if available
- the user can change the seed before generating/re-generating

If implementing that UI would require a large redesign, a smaller deterministic seed input adjacent to the action is acceptable.

Do not use current time/randomness invisibly.

# Hometown profile constraint

Add the smallest explicit Location constraint needed for B2.

Example semantic constraint:

    location.profile = settlement

The exact serialized key must be versioned/stable and tested.

The constraint must:

- force Location profile to Settlement
- remain compatible with field locks/rerolls
- not alter outputs for existing B1 DNA where the constraint is absent
- reject invalid profile values cleanly

Do not add a generic constraint language.

# Forge Result Bundle

Introduce a minimal typed ForgeResultBundle for B2.

It contains:

- primary LocationCandidate
- zero supporting candidates
- proposed born_in Relationship Candidate
- frozen Context Snapshot
- action/recipe provenance
- warnings

Generation itself is read-only with respect to project storage.

Before Save Bundle, no Location or Relationship row exists.

# Candidate Relationship semantics

The proposed relationship is:

    source Character --born_in--> generated Location

State:

- Candidate
- origin Generated

It must pass the existing Relationship Definition registry.

The source Character state remains exactly unchanged.

No inverse duplicate edge is stored.

No relationship Canon promotion occurs in B2.

# Atomic Save Bundle

Implement one storage/application transaction that saves:

1. primary Location Candidate
2. Candidate born_in Relationship
3. immutable forge_runs operation event with typed recipe/context snapshot
4. typed result/output mapping

All or none.

Required invariants:

- Location saved but Relationship missing => impossible
- Relationship saved but operation/context provenance missing => impossible
- Forge run saved but primary Location missing => impossible

Validate source optimistic state immediately before commit.

If source write_version changed since Context compilation:

- abort save
- persist nothing
- return a clear stale-context/concurrency error

# Identity and reroll behavior

The unsaved Hometown Candidate owns a stable generated Location Entity ID for the review session.

Reroll/edit does not create a new Entity identity.

Discarding the unsaved bundle persists nothing.

Saving persists that same reviewed Location identity.

# Provenance inspection

After save, the product must be able to answer at minimum:

- forged from which Character?
- which source write_version/Canon state?
- which Forge Action/version?
- which Context Recipe/version?
- what context hash?
- what primary Location ID?
- what born_in Relationship ID?

Do not expose chain-of-thought.

This can be a compact provenance panel/read model, not a generic inspector framework.

# Desktop UX

Add the first minimal Forge Further UX without redesigning the approved Workbench.

A natural acceptable flow:

1. User opens a saved Character in Characters/Canon library.
2. Character detail exposes a **Forge Hometown** action.
3. Action navigates/opens a Forge Further review state.
4. UI clearly shows:
   - source Character
   - action: Forge Hometown
   - generated Location fields
   - proposed relationship: "Born in <Location>"
   - Candidate status
   - context/provenance summary
5. Normal Location field lock/reroll/edit controls remain available.
6. User can Discard or Save Bundle.
7. After save, Location appears in Locations.
8. Character detail or a minimal Connections section can show the persisted born_in relationship.

Do not build a full relationship editor or graph.

# Known UI debt rule

UI-001 remains open.

Do not expand B2 scope into general Workbench lock-indicator cleanup unless it blocks B2 functionality or breaks tests.

Do not mark UI-001 resolved.

# Tests — app-core

At minimum prove:

1. action definition fields are stable/versioned
2. Context Recipe ID/version stable
3. compiler is deterministic/read-only
4. same source state => same context bytes/hash
5. relevant source change => different context hash
6. inherited preset preserved
7. inherited Creative Axes preserved
8. Hometown output profile always Settlement
9. same source/context/action seed => exact same Location
10. changed action seed => deterministic different result where expected
11. source Character payload unchanged after generation
12. Location B1 standalone golden/deterministic tests unchanged
13. Character/Quest golden tests unchanged

# Tests — storage/atomicity

At minimum prove:

1. Save Bundle creates exactly one Location, one born_in Candidate Relationship, one forge_runs event
2. forge_runs recipe_json decodes to the typed B2 context/action record
3. forge_runs result_json decodes to the typed result/output record
4. source Character unchanged
5. close/reopen preserves Location, Relationship, and operation provenance
6. Vault export/import preserves the complete saved bundle because the project DB remains authoritative
7. injected failure after Location insert rolls back all writes
8. injected failure after Relationship insert rolls back all writes
9. stale source write_version aborts all writes
10. duplicate Save Bundle does not silently create duplicate identical born_in assertions
11. relationship endpoints remain same-project/type-valid

# Tests — frontend/Tauri

At minimum prove:

- Forge Hometown action available on eligible persisted generated Character
- unavailable/clear explanation for Character without required provenance
- action receives correct source Entity ID
- source/proposed relationship visible in review
- generated result uses Location UI
- Save Bundle invokes one atomic backend command
- successful save refreshes Locations and relationship/connection read model
- discard persists nothing
- Character source remains visually unchanged
- existing Character/Quest/Location standalone modes remain green

# Failure semantics

No silent fallback.

Examples:

- source disappeared => recoverable error
- source write_version changed => stale-context error
- unsupported/manual source provenance => explicit unsupported action message
- invalid Context Recipe => validation error
- invalid born_in endpoints => validation error
- atomic transaction failure => no partial writes

# Automated verification

Run repository-standard gates:

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

Use isolated CARGO_TARGET_DIR if the default executable is locked.
Do not kill the user's running app merely to replace a binary.

# Quality evidence

Create:

docs/quality/THE_FORGE_B2_HOMETOWN_SAMPLES.md

Include at least 24 deterministic Character → Hometown examples across multiple presets.

Each example should show:

- source Character name/preset
- selected context facts
- context hash
- action seed
- generated Hometown
- Location profile
- proposed born_in relationship

This is architecture/content evidence, not human acceptance.

# Human review policy for B2

The operator has explicitly authorized development to continue while UI-001 remains open and prefers a larger product review later.

Therefore B2 does NOT require an immediate human smoke gate for PASS if:

- all automated data-integrity and deterministic gates pass
- no P0/P1/P2 defect is known
- no new data-loss/security concern appears

Record any new cosmetic/UX defects in GAZINGA/KNOWN_ISSUES.md and continue only if they are clearly non-destructive.

Do NOT bypass human review for:
- data loss
- migration safety
- security boundary changes
- release/publish
- a product decision not covered by canon

# Documentation

Create:

- docs/missions/THE_FORGE_B2_HOMETOWN_REPORT.md
- docs/quality/THE_FORGE_B2_HOMETOWN_SAMPLES.md

Update:

- GAZINGA/DISCOVERY_STATE.md
- GAZINGA/PROJECT_STATE.md where appropriate
- canonical design docs only if implementation evidence requires a real correction

# Exit gate

PASS only if all are true:

- Forge Hometown exists for eligible saved Characters
- source Character is never mutated
- versioned Action + Context Recipe exist
- Context Compiler is bounded, deterministic, read-only
- Context Snapshot is frozen and persisted in immutable forge_runs provenance
- deterministic Hometown generation inherits preset/axes and forces Settlement
- Location Candidate retains normal B1 lock/reroll/edit behavior
- born_in Candidate Relationship is proposed correctly
- Save Bundle is atomic
- stale source prevents all writes
- close/reopen preserves bundle/provenance
- Vault roundtrip preserves bundle
- Character/Quest/B1 Location regressions remain green
- full verification passes
- no P0/P1/P2 defect is known

STOP.
Do not begin Faction Forge or Link Existing.
