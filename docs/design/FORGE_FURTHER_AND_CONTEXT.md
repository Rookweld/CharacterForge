# Forge Further & Context

> Status: PROVISIONAL — Discovery 01
> Product baseline: v1.0.0 FOUNDATION

## 1. Purpose

Define how one piece of authored world content can intentionally produce another without turning The Forge into a collection of hardcoded buttons.

This document owns:

- Forge Action grammar
- relationship vocabulary and semantics
- Context Compiler behavior
- Context Recipe behavior
- Context Snapshot behavior
- Forge Result Bundle semantics
- first six-entity Forge Further matrix
- user review / persistence boundaries

It does not define the physical SQLite schema; see ENTITY_SUBSTRATE_AND_MIGRATION.md.

## 2. Forge Further Product Principle

**PROVISIONAL**

Any useful entity should be able to become a creative launch point for related entities.

Examples:

    Character
      → their faction
      → their hometown
      → their signature item
      → their rival
      → a creature hunting them
      → a quest involving them

The important abstraction is not a button called Generate Sword. It is:

    source set
    + Forge Action
    + target entity type
    + optional target profile
    + creative intent
    + proposed relationship(s)
    + Context Recipe

Forge Further must be data-driven and versioned.

## 3. Forge Action Grammar

**PROVISIONAL**

A Forge Action consists conceptually of:

    Source Set
    + Action Definition
    + Target Entity Type
    + Optional Target Profile
    + Intent
    + Context Recipe
    + Optional Proposed Relationship Template

Example:

    Source:
    Character Vaela Thorn

    Action:
    Forge their hometown

    Target:
    Location

    Profile:
    Settlement

    Intent:
    Hometown

    Proposed relationship:
    Character --born_in--> Location

    Context recipe:
    character-hometown-v1

## 4. ForgeActionDefinition

**PROVISIONAL logical contract**

A Forge Action definition should eventually include at least:

- stable action_id
- action_version
- allowed source specification
- target entity type
- optional target profile
- intent
- user-facing label/template
- Context Recipe reference
- optional proposed relationship template
- preset inheritance policy
- creative-axis inheritance policy
- default constraints
- supporting-candidate policy

Definitions are app-owned/versioned product definitions initially, not mutable project records.

Example conceptual definition:

    action_id: character.forge-hometown
    action_version: 1
    source: Character
    target_type: Location
    target_profile: Settlement
    intent: Hometown
    relationship: born_in
    context_recipe: character-hometown-v1
    inherit_preset: true
    inherit_axes: true

## 5. Entity Type vs Profile vs Intent

Entity Type chooses the typed payload.

Profile chooses a useful subtype/configuration.

Intent expresses why this target is being created in the current context.

Examples:

    Item / Weapon / SignatureWeapon
    Location / Ruins / ArtifactLocation
    Character / Rival / PersonalRivalForSelectedCharacter

Intent must not become another entity taxonomy.

## 6. Standalone Forge

**PROVISIONAL**

Existing Character and Quest generation already prove the standalone pattern:

    Forge a Character
    Forge a Quest

New types must also support standalone generation where useful:

    Forge a Location
    Forge a Faction
    Forge an Item
    Forge a Creature

Standalone Forge has no required source entity set.

## 7. Forge Further

**PROVISIONAL**

Forge Further begins from one or more existing source entities and produces related Candidate output.

The source entity must not be mutated merely because it was used as context.

Example:

    Vaela Canon R7
      → Forge their rival
      → Captain Morvane Candidate
      → proposed rival_of relationship Candidate

Vaela R7 remains untouched.

## 8. Forge From Selection

**PROVISIONAL FUTURE DIRECTION**

The action model must not assume forever that exactly one source exists.

A future workflow may select:

    Vaela Thorn
    + Ashen Concord
    + Greyhaven

then choose:

    Forge Quest

The Context Compiler receives an explicit Source Set containing exact selected revisions.

Multi-source UI is deferred until the one-source proof slice is stable.

## 9. Link Existing

**PROVISIONAL; CANON DEFAULT IS OPEN**

When a Forge Action expects a related entity, the user should eventually be able to choose:

- Forge New
- Link Existing

This prevents duplicate entities such as multiple versions of the same faction being created because the generator could not see existing structured content.

Link Existing must validate:

- both entities exist
- both belong to the same project
- endpoint types are compatible with the relationship definition
- duplicate/symmetry rules
- cardinality rules
- stale-write state

**OPEN:** whether an explicit manual Link Existing creates Candidate or Canon relationship state by default.

## 10. Expand Existing

**DEFERRED**

A future action may use an existing sparse entity as context and enrich it rather than creating a new identity.

This must not be implemented as silent mutation. It requires the normal revision/Canon flow.

## 11. Relationship Vocabulary

The first relationship registry should remain intentionally small and useful. These are PROVISIONAL vocabulary entries, not an instruction to implement all of them in the first substrate mission.

### Geographic

| Canonical relation | Typical source → target | Inverse/display concept | Symmetry |
|---|---|---|---|
| located_in | Item/Location → Location | contains | directed |
| born_in | Character → Location | birthplace_of | directed |
| lives_in | Character → Location | resident_of / has_resident | directed |
| based_in | Faction → Location | hosts_faction | directed |
| inhabits | Creature → Location | habitat_of | directed |

### Social

| Canonical relation | Typical source → target | Inverse/display concept | Symmetry |
|---|---|---|---|
| member_of | Character → Faction | has_member | directed |
| leads | Character → Faction | led_by | directed |
| serves | Character/Creature → Character/Faction | served_by | directed |
| governs | Character/Faction → Location | governed_by | directed |
| worships | Character/Faction → future Religion/Deity | worshipped_by | directed |

### Political

| Canonical relation | Typical source → target | Inverse/display concept | Symmetry |
|---|---|---|---|
| allied_with | Faction → Faction | same | symmetric |
| rival_of | Character/Faction → same-family peer | same | normally symmetric |
| enemy_of | Character/Faction → same-family peer | same | PROVISIONAL symmetric for structural world use |

### Personal

| Canonical relation | Typical source → target | Inverse/display concept | Symmetry |
|---|---|---|---|
| knows | Character → Character | known_by | directed |
| related_to | Character → Character | same | symmetric |
| mentors | Character → Character | mentored_by | directed |
| loves | Character → Character | loved_by | directed |
| hates | Character → Character | hated_by | directed |
| hunts | Character/Creature → Character/Creature | hunted_by | directed |

### Possession / creation

| Canonical relation | Typical source → target | Inverse/display concept | Symmetry |
|---|---|---|---|
| owns | Character/Faction → Item | owned_by | directed |
| wields | Character → Item | wielded_by | directed |
| created_by | Item → Character/Faction | created | directed |
| guards | Character/Creature/Faction → Item/Location | guarded_by | directed |
| desires | Character/Faction → Item | desired_by | directed |
| provides | Creature/Location/Faction → Item | provided_by | directed |

### Narrative

| Canonical relation | Typical source → target | Inverse/display concept | Symmetry |
|---|---|---|---|
| involved_in | Character/Faction/Item/Creature → Quest | involves | directed |
| given_by | Quest → Character/Faction | gives_quest | directed |
| opposed_by | Quest → Character/Creature/Faction | opposes_quest | directed |
| targets | Quest → Character/Creature/Item/Location/Faction | targeted_by | directed |
| rewards | Quest → Item | reward_for | directed |
| takes_place_in | Quest → Location | hosts_quest | directed |
| caused_by | Quest/future Event → future Event/Lore | causes | directed |
| leads_to | Quest → Quest | follows_from | directed |

Vocabulary must be reviewed for ambiguity before code implementation. One stable key should correspond to one semantic meaning.

## 12. Relationship Definition Contract

**PROVISIONAL**

Each registered relationship needs:

- stable key
- allowed source Entity Types
- allowed target Entity Types
- inverse display/query key or label where useful
- symmetric flag
- semantic class
- cardinality policy
- default context priority
- user-facing display grammar
- whether Candidate/Canon lifecycle applies
- optional hard validation constraints

Relationship definitions are initially app-owned/versioned definitions.

## 13. Directionality

**PROVISIONAL**

Store one canonical assertion direction.

Do not persist both:

    Character --owns--> Item

and:

    Item --owned_by--> Character

as separate facts when they represent one assertion.

The inverse is query/presentation behavior.

Directed relationships remain meaningfully one-way:

- owns
- member_of
- created_by
- hunts
- mentors
- hates
- serves

## 14. Symmetry

**PROVISIONAL**

Symmetric relationships store one assertion.

Examples:

    Faction A allied_with Faction B
    Character A rival_of Character B

The query layer resolves either endpoint.

The physical implementation should normalize or otherwise prevent duplicate mirrored symmetric assertions.

## 15. Cardinality

**PROVISIONAL**

Cardinality is a policy on Relationship Definitions, not automatically a hard database rule.

Examples:

- member_of → many allowed
- owns → many allowed
- born_in → normally one, likely warning or strong rule
- current ruler → future relation could be one
- related_to → many

Some fictional worlds intentionally violate ordinary assumptions. Definitions should support:

- HARD
- WARN
- unrestricted

Exact first-version policies remain OPEN until the relationship registry is implemented.

## 16. Hard vs Warning Constraints

**PROVISIONAL**

Hard constraints prevent structurally invalid data.

Example:

    Item --member_of--> Location

should fail if member_of only permits Character → Faction.

Warning constraints flag unusual world semantics without blocking creativity.

Example:

    Character has two born_in relationships

may be a warning rather than impossible, depending on project needs.

## 17. Candidate Relationships

**PROVISIONAL**

Relationships created by generation are proposed/Candidate by default.

Example:

    Forge Vaela's hometown
    → Location Candidate
    → Vaela born_in Location Candidate relationship

The user may save/review the bundle without changing existing Canon.

## 18. Relationship Canon

**PROVISIONAL**

Relationship truth has a lifecycle separate from Entity truth.

Promoting a generated Location does not silently establish every proposed edge as Canon.

The UI may present related promotion choices together, but each truth change must remain explicit and revision-safe.

Exact bundled promotion UX is OPEN.

## 19. Six-Entity Forge Further Matrix

This matrix describes product grammar, not first-mission implementation scope.

| Source | User-facing action | Target | Proposed relation |
|---|---|---|---|
| Character | Forge their faction | Faction | Character member_of Faction |
| Character | Forge their hometown | Location | Character born_in Location |
| Character | Forge their signature item | Item | Character owns/wields Item |
| Character | Forge their rival | Character | Character rival_of Character |
| Character | Forge a creature hunting them | Creature | Creature hunts Character |
| Character | Forge a quest involving them | Quest | Character involved_in Quest |
| Quest | Forge the quest giver | Character/Faction | Quest given_by source |
| Quest | Forge the antagonist | Character/Creature | Quest opposed_by target |
| Quest | Forge where it takes place | Location | Quest takes_place_in Location |
| Quest | Forge the reward | Item | Quest rewards Item |
| Quest | Forge the faction behind it | Faction | Faction involved_in Quest |
| Quest | Forge what happens next | Quest | Quest leads_to Quest |
| Location | Forge its ruler/keeper | Character | Character governs Location |
| Location | Forge a local faction | Faction | Faction based_in Location |
| Location | Forge a notable resident | Character | Character lives_in Location |
| Location | Forge a dangerous creature | Creature | Creature inhabits Location |
| Location | Forge a hidden item | Item | Item located_in Location |
| Location | Forge a quest here | Quest | Quest takes_place_in Location |
| Faction | Forge its leader | Character | Character leads Faction |
| Faction | Forge its headquarters | Location | Faction based_in Location |
| Faction | Forge a rival faction | Faction | Faction rival_of Faction |
| Faction | Forge a notable member | Character | Character member_of Faction |
| Faction | Forge its signature artifact | Item | Faction owns Item |
| Faction | Forge a quest involving it | Quest | Faction involved_in Quest |
| Item | Forge its creator | Character/Faction | Item created_by creator |
| Item | Forge its owner | Character/Faction | owner owns Item |
| Item | Forge where it is hidden | Location | Item located_in Location |
| Item | Forge what guards it | Creature/Character | guardian guards Item |
| Item | Forge who wants it | Character/Faction | source desires Item |
| Item | Forge a quest centered on it | Quest | Quest targets Item |
| Creature | Forge its habitat | Location | Creature inhabits Location |
| Creature | Forge its prey | Character/Creature | Creature hunts target |
| Creature | Forge its master | Character/Faction | Creature serves target |
| Creature | Forge what it guards | Item/Location | Creature guards target |
| Creature | Forge a hunter of it | Character | Character hunts Creature |
| Creature | Forge a quest involving it | Quest | Quest opposed_by Creature or Creature involved_in Quest |

Some rows have more than one semantically valid relation. The concrete Forge Action must choose one stable definition rather than guessing after generation.

## 20. Context Compiler

**PROVISIONAL**

The Context Compiler is a read-only application-layer service.

It:

1. receives a Forge Action and Source Set
2. resolves exact source revisions
3. loads the referenced Context Recipe
4. traverses only allowed relationship paths
5. prefers Canon according to policy
6. extracts requested semantic roles/fields
7. ranks and budgets context
8. validates hard constraints/conflicts
9. produces an immutable Compiled Forge Context / Context Snapshot candidate

It must not:

- create entities
- create relationships
- mutate Canon
- write revisions
- directly persist output

## 21. Context Recipe Contract

**PROVISIONAL logical contract**

A versioned Context Recipe should define:

- stable recipe_id
- recipe_version
- primary source requirements
- allowed relation paths
- semantic field roles requested
- maximum traversal depth
- context item budget
- Canon/Candidate policy
- importance overrides
- constraint policy
- optional type/profile-specific requirements

Example:

    recipe: character-rival-v1

    PRIMARY
    - source Character exact selected revision

    DIRECT RELATED
    - member_of → Faction, max 1
    - lives_in → Location, max 1
    - owns → Item, max 2

    SECOND HOP
    - source Faction enemy_of → Faction, max 1

    SEMANTIC ROLES
    - Goal
    - HiddenTruth
    - Conflict
    - RoleIdentity

    CANON POLICY
    - prefer Canon
    - exclude unrelated Candidates

    LIMITS
    - max depth 2
    - bounded total context items

## 22. Canon-First Context

**PROVISIONAL**

Default context policy:

- exact selected source revision is always explicit
- current Canon is preferred for related entities
- unrelated Candidates are excluded
- historical revisions are not silently substituted
- a user-selected Candidate may itself be a primary source

This prevents idea-space content from accidentally becoming assumed world truth.

## 23. Candidate Context Rules

**PROVISIONAL**

Candidates may be included when:

- the Candidate is explicitly selected as a source
- the Forge Action explicitly permits Candidate neighborhood context
- the user explicitly opts into Candidate context

Candidate inclusion must be visible in provenance.

## 24. Semantic Field Roles

Context recipes should prefer semantic roles to hardcoded entity-specific field names where possible.

Example:

    Request semantic role HiddenTruth

can select:

- Character.secret
- Faction.hidden_truth
- Location.secret
- Item.secret
- Creature.secret

The semantic-role vocabulary remains intentionally small; see ENTITY_SCHEMA_REFERENCE.md.

## 25. Context Importance

**PROVISIONAL**

Context items may be classified:

- MANDATORY — generation must preserve or satisfy it
- HIGH — strongly relevant
- NORMAL — useful context
- FLAVOR — inspiration, lowest authority

Locks and explicit hard constraints normally elevate facts toward MANDATORY.

Importance is separate from generation dependency graphs.

## 26. Context Ranking

**PROVISIONAL**

Relationship definitions may have default context priorities, while each Context Recipe may override them for its purpose.

Example:

For Forge Rival:
- rival/social facts: very high
- faction: high
- location: medium
- unrelated owned items: low

For Forge Hometown:
- origin/backstory/location hints: very high
- family/culture hints: high
- faction: medium
- gear: low

Ranking must be deterministic for deterministic Forge operations.

## 27. Graph Traversal Depth

**PROVISIONAL**

Traversal depth must be explicit and bounded.

Example graph:

    Vaela
      → member_of Ashen Concord
      → based_in Greyhaven
      → contains Sunken Observatory

A Character rival recipe may need depth 1.

A Quest recipe may reasonably use depth 2.

No unbounded recursive traversal.

## 28. Context Budget

**PROVISIONAL**

Use semantic/item budgets before token budgets.

Illustrative policy:

- full primary selected revision
- max 8 direct related Canon items
- max 4 second-hop items
- max 3 relevant Lore items when Lore exists
- bounded total context records

Exact numbers belong to individual recipes and tests.

The architecture must remain useful without any LLM/token concept.

## 29. Context Validation

**PROVISIONAL**

The compiler may reject or warn before generation.

Examples:

- Action conflicts with a hard user constraint
- Required source type missing
- Relationship endpoint type impossible
- selected source no longer exists
- a recipe requires Canon context that is unavailable

A soft narrative contradiction may become a warning rather than a structural failure.

## 30. Frozen Context Snapshot

**PROVISIONAL**

Before generation, the exact context is frozen.

A snapshot records at least:

- Source Entity IDs
- exact source revision/version identifiers available in the storage model
- included related entity revisions
- included relationships
- extracted semantic fields/facts
- constraints and importance
- Context Recipe ID/version
- deterministic canonical serialization
- context hash

Generation must not repeatedly query a changing project state mid-run.

## 31. Context Hash

**PROVISIONAL**

The canonical Context Snapshot is hashed for provenance and reproducibility.

BLAKE3 is a natural fit because the project already uses it, but the context hashing domain/keying must be independently versioned and must not silently reuse ForgeDNA stream semantics.

The hash is provenance/integrity metadata, not authenticity/signature proof.

## 32. Provider-Neutral Context

**PROVISIONAL**

Compiled context is a domain object, not an OpenAI prompt, Ollama prompt, or image prompt.

Conceptually:

    CompiledForgeContext
        ├─ deterministic generator
        ├─ local model adapter
        ├─ cloud model adapter
        └─ visual prompt compiler

Provider-specific serialization belongs at the adapter boundary.

## 33. Forge Result Bundle

**PROVISIONAL**

A Forge operation returns a reviewable bundle:

    ForgeResultBundle
    - operation provenance
    - primary Candidate
    - zero or more supporting Candidates
    - proposed relationships
    - frozen Context Snapshot
    - warnings

The bundle is the universal product result abstraction. It is not required to be stored as one giant JSON record.

## 34. Supporting Candidate Rules

**PROVISIONAL**

Default: one primary Entity per Forge Action.

Supporting entities are permitted only when the Action Definition explicitly allows them.

Example rich action:

    Primary:
    Quest — Beneath the Ash

    Supporting:
    Location — The Ember Crypt

    Proposed relationships:
    Quest takes_place_in Ember Crypt
    Ember Crypt located_in Greyhaven

This must remain bounded so one click does not silently populate dozens of entities.

## 35. User Review

The review UI should eventually show:

- generated primary entity
- supporting candidates
- proposed relationships
- relevant source/context provenance
- warnings/conflicts
- Save Candidate
- Forge Again
- Discard
- later explicit Canon actions

Provenance should be human-readable without exposing any model chain-of-thought.

## 36. Persistence Transaction

**PROVISIONAL**

Accepting/saving a Forge Result Bundle must be atomic.

Conceptually:

    BEGIN
      persist Context Snapshot
      persist Forge Operation
      persist primary Entity/revision
      persist allowed supporting Entity revisions
      persist proposed Relationship revisions
      persist operation-output mappings
      persist relevant history/provenance
    COMMIT

All or none.

Context compilation and expensive generation occur before the write transaction. Before commit, optimistic/stale assumptions must be revalidated.

## 37. Provenance Inspection

The product should eventually answer:

**Why did The Forge create this?**

The answer is structured provenance such as:

    Generated from:
    Vaela Thorn — Canon revision X
    Ashen Concord — Canon revision Y
    Greyhaven — Canon revision Z

    Forge Action:
    character.forge-rival v1

    Context Recipe:
    character-rival-v1

    Context hash:
    ...

This is data provenance, not hidden reasoning.

## 38. First Proof Slice

**PROVISIONAL IMPLEMENTATION PROOF**

The first end-to-end Forge Further proof should be:

    Existing v1 Character
        ↓
    Forge Hometown
        ↓
    Context Recipe
        ↓
    frozen Context Snapshot
        ↓
    deterministic Location generation
        ↓
    Location Candidate
        +
    proposed born_in Relationship
        ↓
    atomic save
        ↓
    history / Candidate / Canon lifecycle
        ↓
    close + reopen
        ↓
    export

This slice exercises the substrate while minimizing domain breadth.

## 39. Expansion Sequence

Provisional implementation order:

1. Entity substrate and version boundaries
2. first-class relationships
3. Context Snapshots and recipe contracts
4. Forge Action registry
5. Location Forge
6. Character → Hometown proof slice
7. Faction Forge
8. Item Forge
9. Creature Forge
10. full six-entity Forge Further matrix
11. multi-source richer context
12. later Visual Forge
13. later engine bridges

## 40. Open Product Questions

1. Manual Link Existing: Candidate relationship or Canon immediately?
2. When promoting a Forge Result Bundle, how are Entity and Relationship Canon decisions grouped in the UI?
3. Should some manually authored relationships bypass Candidate state while generated relationships never do?
4. Which initial relationship cardinality constraints are hard versus warnings?
5. How much context detail should the operator inspect by default versus behind a provenance view?
6. What is the first user-facing behavior when a Forge result was created from an older source revision that has since changed?

---

See also:
- THE_FORGE_ENTITY_UNIVERSE.md
- ENTITY_SCHEMA_REFERENCE.md
- ENTITY_SUBSTRATE_AND_MIGRATION.md
- VISUALS_AND_ENGINE_BOUNDARY.md
- ../../GAZINGA/DISCOVERY_STATE.md
