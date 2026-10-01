# EXECUTION CONTRACT

You are executing exactly one bounded post-v1 The Forge mission.

Before coding:

1. Read GAZINGA/CANON_CORE.md.
2. Read GAZINGA/THE_FORGE_CANON_CORE.md.
3. Read GAZINGA/DISCOVERY_STATE.md.
4. Read GAZINGA/KNOWN_ISSUES.md.
5. Read this mission completely.
6. Read:
   - docs/design/THE_FORGE_DESIGN_INDEX.md
   - docs/design/FORGE_FURTHER_AND_CONTEXT.md sections 9-19
   - docs/design/ENTITY_SUBSTRATE_AND_MIGRATION.md sections 10-20
   - docs/design/ENTITY_SCHEMA_REFERENCE.md relationship opportunities for all six current Entity Types
7. Read Mission A / A.R1, B2 / B2.R1, B3, B4, and B5 reports.
8. Inspect the actual repository before changing relationship contracts, persistence, or UI.

During work:

- preserve Character, Quest, Location, Faction, Item, Creature, Hometown, and all current deterministic generation behavior
- preserve database schema version 2
- preserve forgeproject archive format v1
- preserve application version 1.0.0
- preserve the current four persisted RelationshipType keys exactly: born_in, member_of, owns, opposed_by
- preserve Relationship identity, optimistic write_version protection, immutable revision history, and current storage semantics
- add tests with implementation
- do not weaken existing tests
- do not implement Link Existing
- do not choose a default Candidate-vs-Canon policy for manual Link Existing
- do not implement bundled Entity + Relationship Canon promotion
- do not add new RelationshipType enum variants in this mission
- do not implement symmetric relationships, cardinality rules, graph UI, multi-source Forge, or any new Forge Further generation action
- do not implement Visual Forge yet
- do not begin B7 or later work

At the end:

- run every required verification gate
- create docs/missions/THE_FORGE_B6_RELATIONSHIP_LIFECYCLE_REPORT.md
- update GAZINGA/DISCOVERY_STATE.md
- update GAZINGA/PROJECT_STATE.md only where repository facts materially change
- report PASS only if every automated exit criterion is satisfied
- STOP

# THE FORGE B6 — RELATIONSHIP LIFECYCLE
## Universal Connections + explicit Relationship Canon controls

# Objective

Turn the already-proven first-class Relationship substrate into a usable six-entity product surface before adding more Forge Further actions.

The current repository already has typed Relationship records, four stable RelationshipType values, schema-2 Relationship persistence, Candidate / Canon / Archived states, optimistic write_version conflict protection, immutable Relationship revisions, Candidate → Canon promotion, explicit Canon revision support in storage, non-destructive archive, B2-generated born_in Relationship Candidates, and a minimal Character-only Connections readout.

B6 must expose and harden that lifecycle without changing the underlying world model.

After B6, any current entity detail view must be able to show its first-class Connections and the user must be able to explicitly establish or retire Relationship truth.

# Product result

For any saved Character, Quest, Location, Faction, Item, or Creature:

Detail
→ Connections
→ relationship-aware readable label
→ related Entity name/type
→ Candidate / Canon / Archived state
→ origin
→ inspect relationship history
→ Promote to Canon when Candidate
→ Archive when Candidate or Canon

The same assertion must read correctly from either endpoint.

Example source view:
Character Vaela → Member of Ashen Concord · Candidate

Inverse target view:
Faction Ashen Concord → Member: Vaela · Candidate

Do not persist an inverse duplicate edge.

# Starting baseline

Expected baseline:

- branch: master
- HEAD: 821dbeb (feat: add Creature Forge core)
- database schema: 2
- archive format: v1
- app version: 1.0.0
- worktree contains only the four pre-existing unrelated untracked files
- RelationshipType currently has exactly born_in, member_of, owns, opposed_by
- relationship storage already supports get, list project, list for entity, promote, revise, archive, list revisions
- desktop currently exposes only list_relationships_for_entity
- current Connections UI is Character-only and special-cases born_in

If the baseline materially differs, STOP and report rather than silently broadening scope.

# Scope A — Relationship presentation metadata

Enrich the app-owned Relationship Definition metadata for the EXISTING four relationship types only.

Each definition must provide enough stable product metadata for universal presentation:

- key
- RelationshipType
- allowed source EntityType
- allowed target EntityType
- directed flag
- source-side user-facing phrase/grammar
- target-side inverse user-facing phrase/grammar
- concise display label
- semantic class/category if useful for current UI

Required semantics:

born_in:
- storage: Character → Location
- source: Born in <Location>
- inverse: Birthplace of <Character>

member_of:
- storage: Character → Faction
- source: Member of <Faction>
- inverse: Member: <Character> or another equally clear documented inverse phrase

owns:
- storage: Character → Item
- source: Owns <Item>
- inverse: Owned by <Character>
- do NOT expand owns to Faction → Item in B6

opposed_by:
- storage: Quest → Creature
- source: Opposed by <Creature>
- inverse: Opposes quest <Quest>
- do not add alternate endpoints in B6

# Scope B — Universal Connection read model

Replace the Character-only assumption with a universal read model for all six current Entity Types.

The UI/API boundary should have a typed view that includes at minimum:

- relationship
- relationship write_version
- whether the currently viewed entity is source or target
- related Entity ID
- related Entity type
- related Entity display name
- source/inverse readable label or enough typed metadata to render it safely

Requirements:

- one stored Relationship appears from either endpoint
- no inverse Relationship row is created
- missing/archived related Entity handling is safe and explicit
- no cross-project lookup
- ordering is deterministic
- repeated refresh does not duplicate rows
- state shown is actual persisted Relationship state
- write_version shown/used is actual persisted Relationship write_version
- prefer deriving display grammar from app-owned definitions, not scattered string conditionals in React

# Scope C — Explicit Relationship Canon promotion

Expose Candidate → Canon promotion through Tauri/API/UI.

Requirements:

- user action is explicit
- requires relationship ID + expected write_version
- stale expected version fails without mutation
- Candidate → Canon appends/preserves correct revision history per Mission A.R1 semantics
- current Relationship becomes Canon
- source/target/type/ID remain unchanged
- related Entity records are not mutated
- promoting an Entity does NOT auto-promote its relationships
- promoting a Relationship does NOT auto-promote either Entity
- B2 Hometown Candidate relationship can be promoted independently after bundle save
- successful action refreshes Connections from persisted state

Use an unambiguous UI label such as Promote connection to Canon.

# Scope D — Non-destructive archive

Expose existing Relationship archive semantics.

Requirements:

- Candidate or Canon may be archived through an explicit action
- requires expected write_version
- stale write fails without mutation
- archive is non-destructive
- immutable history/revisions remain available
- Entity records remain untouched
- archived Relationship does not silently disappear from all inspectability
- UI must make clear this is Archive, not hard delete

Do not implement restore/unarchive in B6 unless existing storage contract already safely supports it without inventing new semantics.

# Scope E — Relationship history/revision inspection

Expose read-only Relationship revision history using the existing storage model.

The user must be able to inspect at least:

- revision number
- reason
- stored Relationship state
- relationship type
- source/target identities
- origin
- timestamp

Requirements:

- prior snapshots remain immutable
- inspecting history cannot mutate current state
- current state and historical state are clearly distinguished
- archive history remains inspectable
- no relationship revision working-copy flow is required
- no endpoint/type editing is required

# Scope F — Connections UX

Add a Connections section to saved detail views for Character, Quest, Location, Faction, Item, and Creature.

It must NOT be Character-only.

Each active connection should show:

- readable relationship phrase based on direction
- related entity name
- related entity type where useful
- Candidate / Canon state
- origin when useful
- actions valid for current state

Archived connections should remain inspectable in a clearly secondary/read-only presentation.

If no connections exist, show a calm empty state such as No connections yet.

Do not infer relationships from prose/hints.
Do not redesign the Workbench.
Do not build graph/timeline/relationship-creation UI.

# Scope G — Relationship command/API surface

Add only commands required by this mission.

Likely operations:

- list relationship connection views for entity
- relationship revision history
- promote relationship to Canon
- archive relationship

Exact command names may follow repository conventions.

Do not expose arbitrary relationship mutation.
Do not expose endpoint/type editing because a changed assertion should be a different Relationship identity.

# Scope H — Storage/contract safety

Preserve all current persistence rules:

- same-project endpoints
- endpoint Entity Types match app-owned definitions
- source != target
- one Relationship identity = one assertion
- optimistic write_version
- immutable prior revisions
- explicit Canon changes
- archive instead of destructive ordinary delete
- no migration of legacy Character gear_links/faction_links/relationship_links
- no automatic inference from prose

No schema bump.

The SQLite CHECK currently restricts relationship_type to the existing four keys. B6 must respect that boundary.

# Scope I — B2 Hometown compatibility

Regression-test the real existing Hometown flow:

1. save Character
2. Forge Hometown
3. save bundle
4. Candidate born_in relationship exists
5. Character Connections shows Born in <Location> · Candidate
6. Location Connections shows inverse Birthplace of <Character> · Candidate
7. promote that Relationship to Canon
8. both endpoint views show Canon
9. entities retain independent Canon/Candidate states
10. archive the Relationship
11. both endpoint views show Archived/read-only
12. relationship revision history remains intact

Do not alter Hometown generation/context/provenance.

# Explicit non-goals

B6 does NOT implement:

- Link Existing
- Manual Create Relationship
- manual default Candidate vs Canon decision
- bundled Entity + Relationship Canon promotion
- new relationship vocabulary
- cardinality policy
- warnings for two born_in assertions
- symmetry normalization
- Faction owns Item
- Character wields Item
- lives_in
- based_in
- inhabits
- rival_of
- any new Forge Further action
- graph UI
- multi-source context
- Visual Forge / image providers
- World/Lore
- Game Schema Packs
- engine adapters

# Automated verification requirements

At minimum add/extend tests proving:

Definition/read model:
- all four keys serialize unchanged
- source/target endpoint rules unchanged
- source and inverse display metadata exist for all four
- every supported Entity endpoint can resolve a connection view
- one stored relationship appears correctly from both endpoints
- related name/type resolution is correct
- no inverse duplicate is persisted

Canon lifecycle:
- Candidate → Canon success
- wrong expected write_version rejected
- repeated/stale promote rejected safely
- ID/type/endpoints preserved
- correct revision appended
- Entity rows unchanged

Archive:
- Candidate → Archived
- Canon → Archived
- stale archive rejected
- historical revisions retained
- Entity rows unchanged
- archived relationship remains inspectable

Hometown integration:
- actual B2 bundle Candidate visible from Character and Location
- explicit promotion visible from both
- archive visible from both
- Hometown provenance untouched

Frontend fixtures/tests:
- Character source-side born_in wording
- Location inverse born_in wording
- Character member_of wording
- Faction inverse member_of wording
- Character owns wording
- Item inverse owns wording
- Quest opposed_by wording
- Creature inverse opposed_by wording
- Candidate Promote action
- Canon has no Promote action
- Candidate/Canon Archive action
- Archived is read-only
- stale mutation error displayed without optimistic fake state
- empty Connections state
- Connections render for all six Entity Types

# Full verification gates

Run:

- frontend format
- frontend lint
- frontend typecheck
- frontend tests
- frontend production build
- cargo fmt --all -- --check
- strict Clippy with warnings denied
- cargo test --workspace --no-fail-fast
- existing 1,000-sequence state gauntlet
- Cargo metadata
- scripts/verify.ps1
- optimized Tauri no-bundle build
- git diff --check

Use isolated CARGO_TARGET_DIR if Windows locking requires it.
Do not kill the user's running application to replace binaries.

The known Relationship timestamp fluctuation observed in prior missions must not be hidden. If reproduced, diagnose whether B6 changed the behavior. Do not weaken the assertion merely to force green.

# Human review policy

B6 may PASS without immediate operator smoke if:

- data integrity gates pass
- relationship state changes are independently regression-tested
- no migration/schema/security/product-decision issue arises
- no P0/P1/P2 remains

Human approval IS required if implementation would need to decide:

- manual Link Existing Candidate vs Canon default
- bundled Entity + Relationship Canon promotion behavior
- new relationship vocabulary semantics
- cardinality HARD/WARN policy
- schema migration

Do not decide those silently.

# Exit gate

PASS only if:

- all six entity detail views have universal Connections
- existing four Relationship types display correctly in both directions
- Candidate Relationship promotion to Canon is explicit and stale-safe
- Archive is explicit, non-destructive, stale-safe
- Relationship revisions/history are inspectable
- B2 Hometown born_in works end-to-end through the new surface
- no inverse duplicates are created
- no Entity truth is silently mutated with Relationship truth
- no schema/archive/app-version bump
- full repository verification is green
- no known B6 P0/P1/P2 remains

STOP.

Do not begin Character → Faction Forge Further, Character → Signature Item, Link Existing, Visual Forge, or any later mission.
