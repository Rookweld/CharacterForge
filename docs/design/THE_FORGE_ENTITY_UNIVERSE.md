# The Forge Entity Universe

> Status: PROVISIONAL — Discovery 01
> Product baseline: Character Forge / Quest Forge v1.0.0 FOUNDATION

## 1. Vision

The Forge is a local-first creative pre-production engine for building structured, connected, revisioned, canonical game-world content before gameplay code needs to exist.

Character Forge and Quest Forge are the first two authoring tools inside a broader system where people can build characters, quests, locations, factions, items, creatures, lore, abilities, worlds, visual concepts, and the relationships between them.

A studio should be able to begin with a thin idea and progressively turn it into a coherent design package without surrendering creative control.

The Forge creates the world and its design truth. Unity, Unreal, Godot, or another runtime may consume exports later; they are not the source of authoring truth.

## 2. Product Principle

**PROVISIONAL — Build the world before you build the game.**

Generation is not authority. Generation proposes possibilities. Humans decide what becomes Canon.

The product should optimize for creative momentum without destructive overwrite, deterministic and inspectable generation where deterministic engines are used, stable identity, revision history, explicit relationships, bounded relevant context, provider-neutral architecture, portability, and gradual progression from loose hints to structured entities.

## 3. Core Design Laws

### 3.1 Generation proposes; humans establish Canon
**PROVISIONAL**

A generated entity, relation, visual, or supporting concept is a Candidate unless the user explicitly establishes it as Canon. Forge Further must never silently alter an existing Canon entity or relationship.

### 3.2 Stable identity is sacred
**PROVISIONAL**

An Entity ID identifies the fictional/design concept across revisions. Editing or revising an entity does not create a new identity.

### 3.3 Revisions are immutable historical states
**PROVISIONAL**

New states create new revisions. Existing Canon revisions are never rewritten in place. This extends the v1.0 FOUNDATION principle that Canon is revision-safe.

### 3.4 Relationships are first-class data
**PROVISIONAL**

When both sides exist as entities, important world connections should be represented as relationships rather than only duplicated prose fields.

### 3.5 No entity explosion
**PROVISIONAL**

Create entity types because their data and behavior are meaningfully different, not because a noun sounds important.

Examples:
- Merchant, Villain, Companion, Quest Giver → Character profiles/roles
- Dungeon, Village, Ruins, Building → Location profiles
- Weapon, Armor, Artifact, Consumable → Item profiles
- Skill, Spell, Passive, Ritual → likely Ability profiles

### 3.6 Typed payloads, universal lifecycle
**PROVISIONAL**

Entities share identity, revision, Candidate/Canon, provenance, tags, relationship, and export behavior while retaining strongly typed payloads. The core architecture must not collapse into an unvalidated generic JSON domain model.

### 3.7 Canon-first context
**PROVISIONAL**

Context compilation uses current Canon by default. Candidate content is included only when the action or user explicitly requests it. Canon is established world truth; Candidates are idea space.

### 3.8 Bounded context
**PROVISIONAL**

Forge actions receive the smallest useful version-specific context set. They do not read the whole project by default.

### 3.9 Provider neutrality
**PROVISIONAL**

The domain, context, and relationship models do not depend on one AI or image provider.

### 3.10 The Forge owns authoring truth
**PROVISIONAL**

Engine integrations export from The Forge. Runtime engines do not become the authoritative editor of Canon or Forge history.

## 4. Entity Taxonomy

Only the initial six entities are in the first expansion scope.

### Actors

#### Character
A socially or narratively modeled person or person-like actor where identity, personality, motivation, relationships, and role matter most.

Possible profiles/roles include NPC, Companion, Villain, Merchant, Leader, Hero, Quest Giver, Boss, Civilian, and Rival.

#### Creature
A being modeled primarily through appearance, habitat, behavior, ecology, threat, abilities, and weaknesses.

Possible profiles include Beast, Monster, Undead, Construct, Aberration, Spirit, Elemental, Swarm, and Boss.

Boundary rule: if personality and social identity are the design center, prefer Character. If physiology, ecology, and threat identity are the design center, prefer Creature.

### World

#### World
Future high-level container/composition entity. **DEFERRED**

#### Region
Future geographic or political sub-area. **DEFERRED**

#### Location
A place at any practical design scale.

Profiles may include Settlement, District, Building, Dungeon, Ruins, Wilderness, Landmark, Interior, Site, or Other.

### Society

#### Faction
An organized social or political group: guild, order, cult, government, kingdom, military, corporation, gang, rebellion, tribe, secret society, religious order, etc.

Culture, Species, Religion, and Deity are **DEFERRED** until their real schemas justify separate entities.

### Narrative

#### Quest
Existing FOUNDATION entity.

#### Event
Historical/current narrative occurrence. **DEFERRED**

#### Lore
Structured world fact, rule, rumor, prophecy, legend, law, or historical assertion. **DEFERRED**

Dialogue is not initially a foundational entity.

### Gameplay

#### Ability
Likely future family for Skill, Spell, Passive, Talent, Attack, Ritual, or Status Effect. **DEFERRED**

#### Class / Archetype
Possible future gameplay/design entity or schema extension. **DEFERRED**

### Objects

#### Item
A physical or fictional object with design relevance.

Profiles may include Weapon, Armor, Artifact, Consumable, Tool, Key Item, Resource, Treasure, Book, Quest Item, or Other.

Artifact Forge is expected to be a specialized Item Forge mode rather than a separate foundational entity type.

## 5. Entity Type vs Profile vs Intent

**PROVISIONAL**

The architecture distinguishes:

- Entity Type — stable domain schema, for example Item
- Profile — subtype/configuration inside that schema, for example Weapon
- Intent — why generation is being performed, for example Signature weapon for the selected Character

Example:

    Entity Type: Item
    Profile: Weapon
    Intent: Signature weapon for Vaela

Intent affects generation and context without multiplying entity types.

## 6. Universal Entity Mental Model

**PROVISIONAL**

An Entity is the stable identity of one concept in one project.

Conceptually:

    Entity
    - entity_id
    - project_id
    - entity_type
    - display_name
    - current state / revision
    - Canon state / revision
    - favorite
    - archived
    - tags
    - created/updated metadata

Physical storage is defined separately and must respect the existing v1 schema.

## 7. Revision Mental Model

**PROVISIONAL**

A revision is an immutable state of an entity.

    Vaela Thorn
      Revision A — Candidate
      Revision B — manually edited
      Revision C — Canon
      Revision D — working-copy origin
      Revision E — new Canon after explicit confirmation

The Entity remains Vaela throughout. Generation provenance belongs to the produced state, not to an abstract identity forever.

## 8. Relationship Mental Model

**PROVISIONAL**

A Relationship is an explicit assertion between Entity identities.

Examples:

    Character --member_of----> Faction
    Character --owns---------> Item
    Character --born_in------> Location
    Faction   --based_in-----> Location
    Quest     --takes_place_in-> Location
    Quest     --opposed_by---> Creature
    Quest     --rewards------> Item

Relationships reference stable identities. Provenance may additionally record which revisions supplied context when a relationship was proposed.

Inverse presentation/query behavior does not require duplicate stored assertions.

## 9. Candidate and Canon

**PROVISIONAL**

Forge Further from Canon creates Candidate output and Candidate/proposed relationships. It does not mutate the source Canon.

    Canon Entity
       │ Forge Further
       ▼
    Candidate Entity
    +
    Candidate Relationship(s)

Promotion is explicit and revision-safe.

Entity truth and relationship truth are separate. Promoting an Entity must not silently Canonize unrelated proposed relationships.

## 10. World Truth

The architecture should eventually distinguish:

- Entity Truth — facts in an entity revision
- Relationship Truth — explicit connections
- World Truth — broad setting rules/facts

World Truth is expected to belong to future Lore/World Rule structures rather than being forced into unrelated payloads.

**DEFERRED beyond conceptual reservation.**

## 11. Epistemic Truth

**DEFERRED**

Canon status answers whether content is established in the project. It does not necessarily mean every in-fiction statement is objectively true.

A future model may distinguish known true, known false, believed, rumored, disputed, and unknown. No implementation is authorized by this section.

## 12. Unresolved Concepts

**PROVISIONAL**

Creative payloads may name concepts that do not yet exist as entities.

Examples:
- Character faction_hint: The Ashen Concord
- Item origin: Forged beneath the Cathedral of Glass
- Creature loot_or_resource: Heartglass shard

These are unresolved concepts, not broken references. The Forge must allow useful creative content before the entire world has been normalized into entities.

## 13. Promote Concept to Entity

**PROVISIONAL PRODUCT DIRECTION; IMPLEMENTATION DEFERRED UNTIL SUBSTRATE EXISTS**

Promotable fields may later offer Link Existing or Forge as Entity actions.

Promotion is always a human action. Migration must never auto-create entities or relationships from text hints.

## 14. Initial Six-Entity Universe

1. Character — existing
2. Quest — existing
3. Location — first expansion
4. Faction — second expansion
5. Item — third expansion
6. Creature — fourth expansion

These six form the first proof that the universal entity architecture works.

Each must be able to:
- exist independently
- support deterministic generation
- support locks/rerolls/manual edits where applicable
- participate in revision-safe Canon
- link through first-class relationships
- serve as bounded Forge context
- Forge Further into multiple other types
- export without engine-specific assumptions

## 15. First Expansion Types

**PROVISIONAL ORDER**

### 1. Location Forge
Largest immediate multiplier because Characters, Quests, Factions, Creatures, and Items all gain physical/world placement.

### 2. Faction Forge
Adds social and political structure around Characters, Locations, Quests, and Items.

### 3. Item Forge
Adds objects that can be owned, rewarded, hidden, guarded, desired, or used as story centers.

### 4. Creature Forge
Adds ecological, threat, and opposition structure.

Skills, Spells, and Abilities follow after the world graph is rich enough to give gameplay concepts useful context.

## 16. Future Entity Families

**DEFERRED**

Likely later additions:
- Ability / Skill / Spell / Talent / Status Effect
- World / Region
- Culture / Species / Religion / Deity
- Lore / World Rule / Rumor / Prophecy / Law
- Event / Historical Event
- Class / Archetype
- economy/crafting concepts where justified by real schemas

New types must pass the data-model-not-cool-noun rule.

## 17. World Graph

**DEFERRED**

A graph visualization may later display relationships between entities. The graph is a view over authoritative data, not the foundational storage model.

## 18. Long-Term Studio Vision

The Forge should become a creative pre-production engine capable of producing a structured design package containing:

- world and region structure
- locations and dungeons
- characters, factions, creatures
- items and artifacts
- quests and narrative hooks
- later abilities/classes/gameplay concepts
- lore/history/world rules
- visual briefs and concept assets
- canonical relationship graph
- human-readable world bible / design package
- JSON/CSV and later engine-specific exports

The goal is not one-button game generation. The goal is to let humans create, lock, revise, connect, inspect, and intentionally establish a coherent game universe.

A future Forge RPG Skeleton workflow should orchestrate normal Forge primitives rather than introduce a second architecture.

## 19. Design Locks

### LOCKED by existing v1.0 FOUNDATION

- Canon changes are explicit and revision-safe.
- Character/Quest deterministic behavior and historical ForgeDNA/version contracts must not be silently changed.
- SQLite is the authoritative local project store in the current product architecture.
- UI does not own persistence.
- Generator/domain logic is not coupled to Tauri UI.
- The portable forgeproject archive is a separate compatibility boundary.

### PROVISIONAL Discovery 01 locks

- stable Entity identity
- typed payloads
- profiles instead of entity explosion
- first-class relationships
- data-driven Forge Further
- generation proposes; humans establish Canon
- Canon-first bounded context
- frozen version-specific context snapshots
- provider-neutral context
- Forge Result Bundles
- no automatic concept promotion
- The Forge remains the authoring source of truth
- first expansion order: Location → Faction → Item → Creature

## 20. Open Questions

1. Does manual Link Existing create a Candidate relationship or immediately express Canon truth?
2. What is the exact UX when an Entity and relationships from the same Forge Result Bundle are promoted?
3. How should existing v1 persistent generation history map to future ForgeOperation provenance without falsifying history?
4. How much Workbench UI should be metadata-driven versus specialized per entity type?
5. Which future relationship cardinality rules are hard constraints versus warnings?
6. When World/Lore arrives, what exact representation owns global fictional rules and epistemic status?

---

See also:
- Design Index: THE_FORGE_DESIGN_INDEX.md
- Entity Schema Reference: ENTITY_SCHEMA_REFERENCE.md
- Entity Substrate & Migration: ENTITY_SUBSTRATE_AND_MIGRATION.md
- Forge Further & Context: FORGE_FURTHER_AND_CONTEXT.md
- Visuals & Engine Boundary: VISUALS_AND_ENGINE_BOUNDARY.md
- Discovery State: ../../GAZINGA/DISCOVERY_STATE.md
