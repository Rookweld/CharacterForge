# Entity Schema Reference

> Status: PROVISIONAL — Discovery 01
> Baseline audit source: current v1.0 contracts and app-core field definitions in the repository.

## 1. Field Classification Vocabulary

A field may have multiple classifications.

| Classification | Meaning |
|---|---|
| IDENTITY | Helps identify the entity to a human |
| CREATIVE | Descriptive or narrative creative content |
| RELATION_HINT | May point toward a first-class Relationship or promotable entity |
| CONTEXT_HIGH | Usually important when the entity becomes Forge context |
| GAMEPLAY | May later map to gameplay/system data |
| LOCKABLE | Intended to participate in field locking |
| REROLLABLE | Intended to support deterministic field reroll |
| EXPORTABLE | Included in normal human/data exports |
| PROMOTABLE | Text may later be turned into or linked to an Entity |
| CANON_SENSITIVE | Canon edits should be especially explicit because they can alter continuity |

These classifications describe product semantics. They are not automatically generation dependencies.

## 2. Common Entity Characteristics

### Existing v1 EntityEnvelope

The current contract already provides a useful universal envelope:

- id
- project_id
- entity_type
- name
- schema_version
- canon_state
- origin
- favorite
- created_at_unix_ms
- updated_at_unix_ms
- tags
- notes
- generation_provenance

This is a strong starting point for The Forge substrate.

### Common future behavior

All initial six entity types are expected to share:

- stable Entity identity
- Candidate/Canon lifecycle
- revision history
- favorite/archive behavior
- tags/notes
- generation provenance where generated
- field locks/rerolls for generated creative fields
- JSON/Markdown export
- participation in first-class relationships
- bounded context compilation

### Important distinction: creative hint vs relationship

A payload may retain creative prose even after a structured relationship exists.

Example:

    faction_hint:
    "An ash-marked order of forbidden cartographers"

can coexist with:

    Character --member_of--> The Ashen Concord

The hint records the generated idea and provenance. The Relationship records structured world truth.

## 3. Character Schema

### Current v1 contract

Character is an existing FOUNDATION entity and its compatibility contract must be preserved.

The current typed Character payload contains:

- envelope
- aliases
- title
- archetype
- role
- appearance
- faction_hint
- signature_gear
- personality
- desire
- fear
- flaw
- secret
- skills
- abilities
- gear_links
- faction_links
- relationship_links
- relationship_hook
- backstory_hook
- story_hooks

The current Character generator has 17 deterministic generated fields:

1. archetype
2. role
3. name
4. title
5. appearance
6. personality
7. desire
8. fear
9. flaw
10. secret
11. skills
12. abilities
13. faction_hint
14. signature_gear
15. relationship_hook
16. backstory_hook
17. story_hooks

Aliases and the three existing link vectors are contract fields but are not part of the current generated field order. The current generator initializes the link vectors empty.

### Fields

| Field | Status | Classification | Context | Future relation/promotion |
|---|---|---|---|---|
| envelope.name | existing generated | IDENTITY, LOCKABLE, REROLLABLE, EXPORTABLE, CANON_SENSITIVE | HIGH | — |
| aliases | existing contract | IDENTITY, EXPORTABLE | LOW | — |
| title | existing generated | IDENTITY, CREATIVE, LOCKABLE, REROLLABLE, EXPORTABLE | MEDIUM | — |
| archetype | existing generated | CREATIVE, CONTEXT_HIGH, LOCKABLE, REROLLABLE, EXPORTABLE | HIGH | future Class/Archetype concept |
| role | existing generated | CREATIVE, CONTEXT_HIGH, LOCKABLE, REROLLABLE, EXPORTABLE | HIGH | future profession/class concept |
| appearance | existing generated | CREATIVE, LOCKABLE, REROLLABLE, EXPORTABLE | HIGH for visuals | Visual Forge |
| faction_hint | existing generated | CREATIVE, RELATION_HINT, PROMOTABLE, LOCKABLE, REROLLABLE, EXPORTABLE | MEDIUM/HIGH | member_of → Faction |
| signature_gear | existing generated | CREATIVE, RELATION_HINT, PROMOTABLE, LOCKABLE, REROLLABLE, EXPORTABLE | MEDIUM/HIGH | owns/wields → Item |
| personality | existing generated | CREATIVE, CONTEXT_HIGH, LOCKABLE, REROLLABLE, EXPORTABLE | HIGH | — |
| desire | existing generated | CREATIVE, CONTEXT_HIGH, LOCKABLE, REROLLABLE, EXPORTABLE | VERY HIGH | Quest context |
| fear | existing generated | CREATIVE, CONTEXT_HIGH, LOCKABLE, REROLLABLE, EXPORTABLE | HIGH | Quest/Creature context |
| flaw | existing generated | CREATIVE, CONTEXT_HIGH, LOCKABLE, REROLLABLE, EXPORTABLE | HIGH | Quest context |
| secret | existing generated | CREATIVE, CONTEXT_HIGH, LOCKABLE, REROLLABLE, EXPORTABLE, CANON_SENSITIVE | VERY HIGH | Quest/Lore |
| skills | existing generated | CREATIVE, GAMEPLAY, PROMOTABLE, LOCKABLE, REROLLABLE, EXPORTABLE | MEDIUM | future Ability entities |
| abilities | existing generated | CREATIVE, GAMEPLAY, PROMOTABLE, LOCKABLE, REROLLABLE, EXPORTABLE | MEDIUM | future Ability entities |
| gear_links | existing contract | legacy/reserved structured links | LOW | must not become authoritative relationship storage by accident |
| faction_links | existing contract | legacy/reserved structured links | LOW | same |
| relationship_links | existing contract | legacy/reserved structured links | LOW | same |
| relationship_hook | existing generated | CREATIVE, PROMOTABLE, LOCKABLE, REROLLABLE, EXPORTABLE | HIGH | Character/Quest/Relationship seed |
| backstory_hook | existing generated | CREATIVE, PROMOTABLE, LOCKABLE, REROLLABLE, EXPORTABLE | HIGH | Location/Lore/Event seed |
| story_hooks | existing generated | CREATIVE, PROMOTABLE, LOCKABLE, REROLLABLE, EXPORTABLE | MEDIUM/HIGH | Quest seeds |

### Semantic Field Roles

Provisional roles:

- envelope.name → IdentityName
- appearance → Appearance
- archetype / role → RoleIdentity
- personality → Personality
- desire → Goal
- fear / flaw → Conflict
- secret → HiddenTruth
- faction_hint → FactionHint
- signature_gear → ItemHint
- skills / abilities → AbilityHint
- relationship_hook → RelationshipHook
- backstory_hook → HistoryHook
- story_hooks → Hook

### Relation Opportunities

Likely relationships:

- member_of → Faction
- born_in → Location
- lives_in → Location
- owns → Item
- wields → Item
- knows / related_to / mentors / loves / hates / rival_of → Character
- serves → Faction
- involved_in → Quest
- hunts / hunted_by style relations with Creature where defined

### Forge Further Opportunities

- Forge their faction → Faction
- Forge their hometown → Location
- Forge their signature item → Item
- Forge their rival → Character
- Forge a creature hunting them → Creature
- Forge a quest involving them → Quest

### Compatibility Notes

- v1 Character content version 1 replay and current content version 2 behavior must remain reproducible.
- Existing field keys and dependency semantics must not be silently renamed or repurposed.
- Existing gear_links, faction_links, and relationship_links must not be interpreted into new first-class relationships during migration.
- A later Character payload version may deprecate or reinterpret those vectors only through an explicit compatibility design. Historical snapshots retain their original representation.

## 4. Quest Schema

### Current v1 contract

The current typed Quest payload contains:

- envelope, with envelope.name functioning as title
- quest_type
- premise
- patron
- objective
- stakes
- opposition
- complications
- twist
- moral_pressure
- locations
- rewards
- failure_consequences
- follow_up_hooks

The Quest generation pipeline has 15 fields in v2 and 17 generation fields in v3.

v3 adds generation fields:
- clue
- briefing

The QuestCandidate wrapper persists optional clue and briefing fields. The generation pipeline also has a Frame field used by recipe generation; Frame is not a first-class field on the typed Quest contract itself.

### Fields

| Field | Status | Classification | Context | Future relation/promotion |
|---|---|---|---|---|
| envelope.name | existing generated title | IDENTITY, CREATIVE, LOCKABLE, REROLLABLE, EXPORTABLE | MEDIUM | — |
| quest_type | existing generated | CREATIVE, CONTEXT_HIGH, LOCKABLE, REROLLABLE, EXPORTABLE | HIGH | — |
| premise | existing generated | CREATIVE, CONTEXT_HIGH, LOCKABLE, REROLLABLE, EXPORTABLE | VERY HIGH | — |
| patron | existing generated | CREATIVE, RELATION_HINT, PROMOTABLE, LOCKABLE, REROLLABLE, EXPORTABLE | HIGH | given_by → Character/Faction |
| objective | existing generated | CREATIVE, CONTEXT_HIGH, LOCKABLE, REROLLABLE, EXPORTABLE | VERY HIGH | targets → Entity where appropriate |
| stakes | existing generated | CREATIVE, CONTEXT_HIGH, LOCKABLE, REROLLABLE, EXPORTABLE | HIGH | — |
| opposition | existing generated | CREATIVE, RELATION_HINT, PROMOTABLE, LOCKABLE, REROLLABLE, EXPORTABLE | VERY HIGH | opposed_by → Character/Creature/Faction |
| complications | existing generated | CREATIVE, PROMOTABLE, LOCKABLE, REROLLABLE, EXPORTABLE | MEDIUM/HIGH | supporting entities/events |
| twist | existing generated | CREATIVE, CONTEXT_HIGH, LOCKABLE, REROLLABLE, EXPORTABLE, CANON_SENSITIVE | HIGH | — |
| moral_pressure | existing generated | CREATIVE, CONTEXT_HIGH, LOCKABLE, REROLLABLE, EXPORTABLE | HIGH | — |
| locations | existing generated | CREATIVE, RELATION_HINT, PROMOTABLE, LOCKABLE, REROLLABLE, EXPORTABLE | HIGH | takes_place_in → Location |
| rewards | existing generated | CREATIVE, RELATION_HINT, PROMOTABLE, LOCKABLE, REROLLABLE, EXPORTABLE | HIGH | rewards → Item |
| failure_consequences | existing generated | CREATIVE, CONTEXT_HIGH, PROMOTABLE, LOCKABLE, REROLLABLE, EXPORTABLE | HIGH | Event/Lore later |
| follow_up_hooks | existing generated | CREATIVE, PROMOTABLE, LOCKABLE, REROLLABLE, EXPORTABLE | MEDIUM | leads_to → Quest |
| clue | v3 candidate field | CREATIVE, CONTEXT_HIGH, LOCKABLE/REROLLABLE in v3 generation | HIGH | Lore/Item/Location hint |
| briefing | v3 candidate field | CREATIVE, LOCKABLE/REROLLABLE in v3 generation | MEDIUM | presentation/narrative |
| frame | v2/v3 generation field | generator structure, not typed Quest payload | internal generation | do not promote into payload without explicit version change |

### Semantic Field Roles

- envelope.name → IdentityName
- premise → Summary/Premise
- patron → CharacterOrFactionHint
- objective → Goal
- stakes / failure_consequences → Consequence
- opposition → Threat/OppositionHint
- twist → HiddenTruth/Complication
- locations → LocationHint
- rewards → Reward/ItemHint
- follow_up_hooks → Hook
- clue → Clue
- briefing → Briefing

### Relation Opportunities

- given_by → Character/Faction
- takes_place_in → Location
- opposed_by → Character/Creature/Faction
- involved_in → Character/Faction
- rewards → Item
- targets → Character/Creature/Item/Location where semantically appropriate
- leads_to → Quest
- caused_by → Event/Lore later

### Forge Further Opportunities

- Forge the quest giver → Character or Faction
- Forge the antagonist → Character or Creature
- Forge where it takes place → Location
- Forge the reward → Item
- Forge the faction behind it → Faction
- Forge what happens next → Quest

### Compatibility Notes

- Quest v2 replay must remain supported.
- Current v3 content version 3 is the default current corpus at the v1.0 release baseline.
- Frame, Clue, and Briefing must not be conflated with the stable typed Quest contract during migration.
- New relationships supplement existing prose/list fields; they do not erase historical generated text.

## 5. Location Schema

> New entity — PROVISIONAL schema v1 for future Location Forge.

### Fields

| Field | Classification | Context | Relation/promotion |
|---|---|---|---|
| name | IDENTITY, LOCKABLE, REROLLABLE, EXPORTABLE, CANON_SENSITIVE | HIGH | — |
| location_profile | IDENTITY, CREATIVE, LOCKABLE, REROLLABLE, EXPORTABLE | HIGH | — |
| summary | CREATIVE, CONTEXT_HIGH, LOCKABLE, REROLLABLE, EXPORTABLE | VERY HIGH | — |
| visual_identity | CREATIVE, LOCKABLE, REROLLABLE, EXPORTABLE | HIGH for visuals | Visual Forge |
| atmosphere | CREATIVE, CONTEXT_HIGH, LOCKABLE, REROLLABLE, EXPORTABLE | HIGH | visual/audio direction |
| purpose | CREATIVE, CONTEXT_HIGH, LOCKABLE, REROLLABLE, EXPORTABLE | HIGH | — |
| history | CREATIVE, PROMOTABLE, LOCKABLE, REROLLABLE, EXPORTABLE | MEDIUM/HIGH | Lore/Event |
| inhabitants_summary | CREATIVE, RELATION_HINT, PROMOTABLE, LOCKABLE, REROLLABLE, EXPORTABLE | MEDIUM | Character/Creature/Faction |
| notable_features | CREATIVE, PROMOTABLE, LOCKABLE, REROLLABLE, EXPORTABLE | HIGH | Location/Item |
| danger | CREATIVE, RELATION_HINT, PROMOTABLE, LOCKABLE, REROLLABLE, EXPORTABLE | HIGH | Creature/Character/Faction |
| secret | CREATIVE, CONTEXT_HIGH, PROMOTABLE, LOCKABLE, REROLLABLE, EXPORTABLE, CANON_SENSITIVE | VERY HIGH | Quest/Lore |
| story_hooks | CREATIVE, PROMOTABLE, LOCKABLE, REROLLABLE, EXPORTABLE | MEDIUM/HIGH | Quest |

### Profiles

Initial profiles:
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

### Intents

Examples:
- Hometown
- Faction Headquarters
- Quest Destination
- Hidden Ruin
- Creature Lair
- Artifact Location
- Dangerous Site
- Safe Haven

Intent is generation context, not an entity type.

### Semantic Field Roles

- name → IdentityName
- summary → Summary
- visual_identity → VisualIdentity
- atmosphere → Atmosphere
- purpose → Purpose
- history → History
- inhabitants_summary → PopulationHint
- notable_features → Feature
- danger → Threat
- secret → HiddenTruth
- story_hooks → Hook

### Relation Opportunities

- located_in / contains → Location/Region later
- based_in inverse query for Faction
- lives_in / born_in inverse query for Character
- inhabits inverse query for Creature
- controlled_by / governed_by → Faction/Character
- located_in relation from Item
- takes_place_in relation from Quest

### Forge Further Opportunities

- Forge its ruler/keeper → Character
- Forge a local faction → Faction
- Forge a notable resident → Character
- Forge a dangerous creature → Creature
- Forge a hidden item → Item
- Forge a quest here → Quest

## 6. Faction Schema

> New entity — PROVISIONAL schema v1 for future Faction Forge.

### Fields

| Field | Classification | Context | Relation/promotion |
|---|---|---|---|
| name | IDENTITY, LOCKABLE, REROLLABLE, EXPORTABLE, CANON_SENSITIVE | HIGH | — |
| faction_profile | IDENTITY, CREATIVE, LOCKABLE, REROLLABLE, EXPORTABLE | HIGH | — |
| identity | CREATIVE, CONTEXT_HIGH, LOCKABLE, REROLLABLE, EXPORTABLE | VERY HIGH | — |
| ideology | CREATIVE, CONTEXT_HIGH, LOCKABLE, REROLLABLE, EXPORTABLE | VERY HIGH | Lore |
| primary_goal | CREATIVE, CONTEXT_HIGH, LOCKABLE, REROLLABLE, EXPORTABLE | VERY HIGH | Quest |
| methods | CREATIVE, CONTEXT_HIGH, LOCKABLE, REROLLABLE, EXPORTABLE | HIGH | — |
| culture | CREATIVE, CONTEXT_HIGH, PROMOTABLE, LOCKABLE, REROLLABLE, EXPORTABLE | MEDIUM/HIGH | Culture later |
| resources | CREATIVE, RELATION_HINT, PROMOTABLE, LOCKABLE, REROLLABLE, EXPORTABLE | MEDIUM | Item/Location |
| public_face | CREATIVE, LOCKABLE, REROLLABLE, EXPORTABLE | HIGH | — |
| hidden_truth | CREATIVE, CONTEXT_HIGH, LOCKABLE, REROLLABLE, EXPORTABLE, CANON_SENSITIVE | VERY HIGH | Quest/Lore |
| symbols | CREATIVE, LOCKABLE, REROLLABLE, EXPORTABLE | MEDIUM | Visual Forge |
| reputation | CREATIVE, GAMEPLAY, CONTEXT_HIGH, LOCKABLE, REROLLABLE, EXPORTABLE | MEDIUM | future gameplay schema |
| story_hooks | CREATIVE, PROMOTABLE, LOCKABLE, REROLLABLE, EXPORTABLE | MEDIUM/HIGH | Quest |

### Profiles

Initial profiles:
- Guild
- Order
- Cult
- Government
- Kingdom
- Military
- Corporation
- Gang
- Rebellion
- Religious Order
- Tribe
- Secret Society
- Other

### Intents

Examples:
- Character Faction
- Quest Patron Organization
- Antagonist Faction
- Local Power
- Rival Faction
- Hidden Order

### Semantic Field Roles

- identity → Summary/Identity
- ideology → Belief
- primary_goal → Goal
- methods → Method
- culture → CultureHint
- resources → ResourceHint
- public_face → PublicIdentity
- hidden_truth → HiddenTruth
- symbols → VisualIdentity
- reputation → Reputation
- story_hooks → Hook

### Relation Opportunities

- led_by → Character
- member_of inverse for Character
- based_in → Location
- governs / controls → Location
- allied_with / enemy_of / rival_of → Faction
- owns → Item
- involved_in → Quest
- serves/worships future world concepts where appropriate

### Forge Further Opportunities

- Forge its leader → Character
- Forge its headquarters → Location
- Forge a rival faction → Faction
- Forge a notable member → Character
- Forge its signature artifact → Item
- Forge a quest involving it → Quest

## 7. Item Schema

> New entity — PROVISIONAL schema v1 for future Item Forge.

The first Item schema is a worldbuilding/design schema, not a game-specific inventory-stat schema.

### Fields

| Field | Classification | Context | Relation/promotion |
|---|---|---|---|
| name | IDENTITY, LOCKABLE, REROLLABLE, EXPORTABLE, CANON_SENSITIVE | HIGH | — |
| item_profile | IDENTITY, GAMEPLAY, LOCKABLE, REROLLABLE, EXPORTABLE | HIGH | — |
| appearance | CREATIVE, LOCKABLE, REROLLABLE, EXPORTABLE | HIGH for visuals | Visual Forge |
| purpose | CREATIVE, CONTEXT_HIGH, LOCKABLE, REROLLABLE, EXPORTABLE | HIGH | — |
| properties | CREATIVE, GAMEPLAY, CONTEXT_HIGH, LOCKABLE, REROLLABLE, EXPORTABLE | HIGH | Ability later |
| origin | CREATIVE, RELATION_HINT, PROMOTABLE, LOCKABLE, REROLLABLE, EXPORTABLE | HIGH | creator/location |
| history | CREATIVE, PROMOTABLE, LOCKABLE, REROLLABLE, EXPORTABLE | MEDIUM/HIGH | Lore/Event |
| power_or_function | CREATIVE, GAMEPLAY, CONTEXT_HIGH, LOCKABLE, REROLLABLE, EXPORTABLE | VERY HIGH | Ability |
| cost_or_drawback | CREATIVE, GAMEPLAY, CONTEXT_HIGH, LOCKABLE, REROLLABLE, EXPORTABLE | HIGH | Ability/Status later |
| rarity | CREATIVE, GAMEPLAY, LOCKABLE, REROLLABLE, EXPORTABLE | MEDIUM | game schema later |
| condition | CREATIVE, GAMEPLAY, LOCKABLE, REROLLABLE, EXPORTABLE | LOW/MEDIUM | — |
| secret | CREATIVE, CONTEXT_HIGH, PROMOTABLE, LOCKABLE, REROLLABLE, EXPORTABLE, CANON_SENSITIVE | HIGH | Quest/Lore |
| story_hooks | CREATIVE, PROMOTABLE, LOCKABLE, REROLLABLE, EXPORTABLE | MEDIUM/HIGH | Quest |

### Profiles

Initial profiles:
- Weapon
- Armor
- Artifact
- Consumable
- Tool
- Key Item
- Resource
- Treasure
- Book
- Quest Item
- Other

### Intents

Examples:
- Signature Weapon
- Ancient Relic
- Quest Reward
- Hidden Artifact
- Faction Symbol Item
- Creature Resource
- Dangerous Tool

### Semantic Field Roles

- appearance → Appearance
- purpose → Purpose
- properties → Property
- origin → Origin
- history → History
- power_or_function → AbilityHint/Function
- cost_or_drawback → Cost/Conflict
- secret → HiddenTruth
- story_hooks → Hook

### Relation Opportunities

- owns / owned_by → Character/Faction
- wields → Character
- created_by → Character/Faction
- located_in → Location
- guards / guarded_by → Character/Creature/Faction
- rewards relation from Quest
- desired_by → Character/Faction
- involved_in / targets with Quest

### Forge Further Opportunities

- Forge its creator → Character or Faction
- Forge its owner → Character or Faction
- Forge where it is hidden → Location
- Forge what guards it → Creature or Character
- Forge who wants it → Character or Faction
- Forge a quest centered on it → Quest

### Explicit non-goal for first Item schema

Do not bake in universal damage, armor rating, weight, item level, cooldown, equip slot, or other game-specific stats. Those belong to future Game Schema Packs unless a later core schema decision proves otherwise.

## 8. Creature Schema

> New entity — PROVISIONAL schema v1 for future Creature Forge.

### Fields

| Field | Classification | Context | Relation/promotion |
|---|---|---|---|
| name | IDENTITY, LOCKABLE, REROLLABLE, EXPORTABLE | HIGH | — |
| creature_profile | IDENTITY, GAMEPLAY, LOCKABLE, REROLLABLE, EXPORTABLE | HIGH | — |
| appearance | CREATIVE, LOCKABLE, REROLLABLE, EXPORTABLE | HIGH for visuals | Visual Forge |
| habitat | CREATIVE, RELATION_HINT, PROMOTABLE, LOCKABLE, REROLLABLE, EXPORTABLE | HIGH | inhabits → Location |
| behavior | CREATIVE, CONTEXT_HIGH, LOCKABLE, REROLLABLE, EXPORTABLE | VERY HIGH | — |
| ecology | CREATIVE, CONTEXT_HIGH, LOCKABLE, REROLLABLE, EXPORTABLE | HIGH | World/Lore |
| threat | CREATIVE, GAMEPLAY, CONTEXT_HIGH, LOCKABLE, REROLLABLE, EXPORTABLE | HIGH | Quest |
| abilities_summary | CREATIVE, GAMEPLAY, PROMOTABLE, LOCKABLE, REROLLABLE, EXPORTABLE | HIGH | Ability entities later |
| weakness | CREATIVE, GAMEPLAY, CONTEXT_HIGH, LOCKABLE, REROLLABLE, EXPORTABLE | HIGH | Quest |
| origin | CREATIVE, PROMOTABLE, LOCKABLE, REROLLABLE, EXPORTABLE | MEDIUM/HIGH | Lore/Faction |
| signs_of_presence | CREATIVE, LOCKABLE, REROLLABLE, EXPORTABLE | MEDIUM | Quest/Location |
| loot_or_resource | CREATIVE, RELATION_HINT, PROMOTABLE, LOCKABLE, REROLLABLE, EXPORTABLE | MEDIUM | Item |
| secret | CREATIVE, CONTEXT_HIGH, LOCKABLE, REROLLABLE, EXPORTABLE, CANON_SENSITIVE | HIGH | Lore/Quest |
| story_hooks | CREATIVE, PROMOTABLE, LOCKABLE, REROLLABLE, EXPORTABLE | MEDIUM/HIGH | Quest |

### Profiles

Initial profiles:
- Beast
- Monster
- Undead
- Construct
- Aberration
- Spirit
- Elemental
- Humanoid Creature
- Swarm
- Boss
- Other

### Intents

Examples:
- Quest Antagonist
- Location Threat
- Artifact Guardian
- Character Hunter
- Faction Beast
- Local Wildlife
- Boss Creature

### Semantic Field Roles

- appearance → Appearance
- habitat → LocationHint
- behavior → Behavior
- ecology → Ecology
- threat → Threat
- abilities_summary → AbilityHint
- weakness → Weakness
- origin → Origin
- signs_of_presence → Clue
- loot_or_resource → ItemHint
- secret → HiddenTruth
- story_hooks → Hook

### Relation Opportunities

- inhabits → Location
- hunts → Character/Creature
- serves → Character/Faction
- guards → Item/Location
- opposed_by inverse/query relation with Quest
- provides → Item/Resource
- involved_in → Quest

### Forge Further Opportunities

- Forge its habitat → Location
- Forge its prey → Character or Creature
- Forge its master → Character or Faction
- Forge what it guards → Item or Location
- Forge a hunter of it → Character
- Forge a quest involving it → Quest

## 9. Cross-Entity Semantic Roles

**PROVISIONAL**

Raw field names are entity-specific. Context recipes should be able to ask for broader semantic meaning without hardcoding every concrete field.

Initial semantic roles:

- IdentityName
- Summary
- Appearance
- VisualIdentity
- RoleIdentity
- Personality
- Goal
- Purpose
- Belief
- Method
- Conflict
- Threat
- Weakness
- HiddenTruth
- History
- Origin
- LocationHint
- FactionHint
- ItemHint
- CharacterHint
- AbilityHint
- Reward
- Consequence
- Clue
- Hook
- Reputation
- Atmosphere

The list should remain small and pragmatic. The Forge is not building a general ontology system.

## 10. Shared vs Specialized UI

**PROVISIONAL**

Shared field metadata may drive common behavior:

- label
- value kind
- lockable
- rerollable
- context importance
- exportable
- promotable
- semantic role

However, the Workbench must not become a fully generic dynamic form engine.

Specialized presentation remains appropriate where meaningful:
- story hook cards
- notable features
- multi-value lists
- future ability/stat controls
- relationship panels
- revision/history views
- visual asset views

Principle:

**Shared metadata drives shared behavior. Specialized UI expresses meaningful domain differences.**

## 11. Future Gameplay Schema Extension

**DEFERRED**

Core entities describe reusable fictional/game-design semantics. Concrete runtime stats vary heavily by genre and project.

Future Game Schema Packs may extend an entity with project-specific typed production data.

Examples:

Action RPG Item pack:
- damage
- attack_speed
- rarity_tier
- equip_slot
- level_requirement

Survival Item pack:
- weight
- durability
- stack_size
- crafting_value
- nutrition

This keeps The Forge core reusable across genres while allowing later production specificity.

## 12. Deferred Schemas

The following are reserved for later discovery and must not be implemented from this document alone:

- Ability
- World
- Region
- Lore
- Event
- Culture
- Species
- Religion
- Deity
- Class / Archetype
- Crafting Recipe
- Loot Table
- Currency / Economy concepts

---

See also:
- THE_FORGE_ENTITY_UNIVERSE.md
- FORGE_FURTHER_AND_CONTEXT.md
- ENTITY_SUBSTRATE_AND_MIGRATION.md
- ../../GAZINGA/DISCOVERY_STATE.md
