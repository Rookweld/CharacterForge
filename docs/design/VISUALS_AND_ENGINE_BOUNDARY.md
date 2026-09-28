# Visuals & Engine Boundary

> Status: DEFERRED IMPLEMENTATION — Discovery 01 design framing
> Purpose: reserve clean boundaries now so future visual/engine work does not contaminate the core authoring model.

## 1. Purpose

Define how future visual assets and engine exports attach to The Forge without becoming authoritative replacements for Entity/Canon data.

This document is intentionally future-facing. Nothing here authorizes image-provider integration, binary-asset schema changes, or engine adapters in the first Entity Substrate implementation.

## 2. Narrative Content vs Visual Asset

**PROVISIONAL**

Creative payload fields such as:

- Character appearance
- Location visual_identity
- Item appearance
- Creature appearance
- Faction symbols

are narrative/design briefs.

Generated or imported images are separate Visual Assets.

Do not make a binary/image file path part of an Entity's core fictional truth merely because the user generated a picture.

## 3. VisualAsset Concept

**PROVISIONAL logical model**

A future Visual Asset may include:

- asset_id
- project_id
- linked Entity ID
- source Entity revision/state reference
- visual role
- provider
- provider model
- visual recipe/compiler version
- seed where available
- prompt/context snapshot or hash
- local asset path
- content hash
- created_at
- selection/status metadata

The exact physical schema is DEFERRED until Visual Forge is an active mission.

## 4. Revision-Linked Assets

**PROVISIONAL**

A generated visual must know which Entity revision/state produced its brief.

Example:

    Portrait P3
    linked to Vaela Thorn Canon revision 4

If Vaela later reaches Canon revision 7, P3 remains historically valid but may be stale relative to current Canon.

This allows accurate provenance rather than silently pretending old images represent new lore.

## 5. Visual Roles

Possible roles:

### Character
- Portrait
- Full Body
- Turnaround
- Character Sheet
- Costume/Outfit Concept

### Location
- Establishing Shot
- Environment Concept
- Interior Concept
- Moodboard

### Creature
- Creature Concept
- Turnaround
- Encounter Illustration
- Anatomy/Detail Sheet

### Item
- Item Concept
- Inventory Icon
- Orthographic/Prop Sheet

### Faction
- Emblem
- Banner
- Uniform Concept
- Symbol Sheet

Additional roles require explicit design rather than arbitrary free-form behavior in core code.

## 6. Visual Context Compiler

**PROVISIONAL**

Visual generation should reuse the same context philosophy as Forge Further.

Conceptually:

    selected Entity revision
    + relevant Canon
    + project Visual Direction
    + Visual Recipe
        ↓
    Visual Context
        ↓
    provider-specific prompt compiler
        ↓
    Image Provider

The Visual Context Compiler is provider-neutral.

It should prefer exact structured facts over copying all narrative prose blindly.

## 7. Project Visual Direction

**PROVISIONAL FUTURE FEATURE**

A project may later define reusable art direction such as:

- medium/rendering style
- palette language
- architecture cues
- materials
- character rendering approach
- camera/lens language
- atmosphere
- prohibited/avoid traits

Example:

    Medium: painterly concept art
    Palette: muted iron and amber
    Architecture: late gothic industrial
    Character rendering: semi-realistic
    Camera: cinematic 35mm
    Avoid: anime, high saturation, modern clothing

This becomes shared context, not hardcoded provider prompt text.

## 8. Image Provider Boundary

**PROVISIONAL**

Image providers are adapters.

Possible future providers may include cloud or local systems. The Forge core must not know provider-specific request objects.

Flow:

    Visual Context
      → Prompt Compiler / Provider Adapter
      → Provider
      → Result
      → Visual Asset metadata

Provider configuration belongs outside the core Entity model.

## 9. Generated Visual vs Selected Visual

**PROVISIONAL**

Generating an image creates a Visual Asset Candidate.

It does not automatically become the selected representation of the Entity.

The user may:

- keep multiple variants
- select one preferred visual
- reject/discard a visual
- generate again
- later mark one as a Canon Visual if that concept is retained

## 10. Visual Canon

**PROVISIONAL PRODUCT DIRECTION**

Narrative Canon and Visual Canon/selection are separate concerns.

Example:

Narrative Canon says:
- white braided hair
- silver prosthetic left hand

Generated portrait accidentally shows:
- black hair

The existence of that portrait must not change Narrative Canon.

A later Canon Visual concept should mean the user deliberately selected a visual representation, not that image output overwrote narrative facts.

Exact terminology and UX remain OPEN.

## 11. Stale Visual Detection

**PROVISIONAL**

Because Visual Assets retain their source revision/state, The Forge can later flag:

    This visual was generated from Character revision 4.
    Current Canon is revision 7.

The system does not automatically delete or regenerate stale visuals.

A future consistency check may also compare structured visual facts against asset-generation metadata, but actual computer-vision fact validation is DEFERRED.

## 12. Asset Provenance

**PROVISIONAL**

Where available, preserve:

- linked Entity ID
- source revision/state
- visual role
- Visual Recipe/compiler version
- project Visual Direction version/hash
- provider
- model
- seed
- prompt/context snapshot or hash
- generation timestamp
- output content hash

Provenance explains where an asset came from. It is not a claim of authenticity or copyright ownership.

## 13. Local Asset Ownership

**PROVISIONAL**

The Forge is local-first.

When Visual Assets are introduced, local project ownership and portability must be designed deliberately.

The current forgeproject v1 archive contains exactly manifest.json and project.sqlite3 and intentionally rejects arbitrary extra members.

Therefore Visual Assets CANNOT simply be shoved into the existing v1 archive without an explicit archive-format decision.

Possible future strategies:
- forgeproject format v2 with bounded asset members
- adjacent managed asset directory plus export packaging
- explicit asset bundle subsystem

**OPEN for future Visual Forge discovery.**

## 14. Human Export

Current/future human-readable exports may include:

- Markdown
- PDF world bible
- Character sheets
- Quest sheets
- Location sheets
- Faction briefs
- Creature briefs
- Item sheets
- concept-art briefs
- production handoff documents

PDF/document exports are presentation outputs, not authoritative project stores.

## 15. Data Export

Core machine-readable exports may include:

- JSON
- CSV where data shape is appropriate
- future structured project snapshots

Stable Entity IDs and Relationship IDs/keys should survive export where useful.

## 16. Canonical Project Export Snapshot

**PROVISIONAL**

Engine adapters should consume a deliberate Project Export Snapshot rather than raw SQLite tables.

Typical snapshot content:

- selected/current Canon Entity revisions
- selected/current Canon Relationships
- stable IDs
- typed payloads
- project-level schema metadata needed by consumers
- optional project-specific gameplay extensions later

Typically excluded:
- full internal editing history
- discarded Candidates
- transient undo/redo
- internal UI state
- unrelated provider secrets

This preserves the boundary between authoring history and runtime consumption.

## 17. Engine Boundary

**PROVISIONAL**

The Forge owns authoring truth.

Engines consume transforms of that truth.

Conceptually:

    The Forge
       ↓
    Canonical Project Export Snapshot
       ├─ JSON
       ├─ CSV
       ├─ Unity Adapter
       ├─ Unreal Adapter
       └─ Godot Adapter

The internal domain model must not become Unity ScriptableObject-shaped, Unreal DataTable-shaped, or Godot Resource-shaped.

## 18. Unity Adapter

**DEFERRED**

Possible future output:
- ScriptableObjects
- JSON runtime assets
- generated C# data models where explicitly requested

No design is locked yet.

## 19. Unreal Adapter

**DEFERRED**

Possible future output:
- DataTables
- DataAssets
- JSON/CSV import packages

No design is locked yet.

## 20. Godot Adapter

**DEFERRED**

Possible future output:
- Resources
- JSON runtime data
- generated GDScript/C# data bindings where explicitly requested

No design is locked yet.

## 21. What Engines Must Never Own

**PROVISIONAL**

Without a future explicit two-way-sync design, engine adapters must not become authoritative for:

- Entity identity creation
- Canon promotion
- revision history
- ForgeDNA
- Context Snapshot provenance
- relationship truth
- authoring Candidate state

One-way export is the safe default.

Two-way synchronization is a separate high-risk design problem and is DEFERRED.

## 22. Future Game Schema Packs

**DEFERRED, but architectural direction reserved**

Core The Forge schemas describe reusable fictional/game-design meaning.

A specific game may need production stats.

Example Action RPG Item extension:

- damage
- attack_speed
- rarity_tier
- equip_slot
- level_requirement

Example Survival Item extension:

- weight
- durability
- stack_size
- crafting_value
- nutrition

Game Schema Packs may later provide typed project-specific production metadata without polluting the universal Item schema.

They must be data/schema extensions, not arbitrary executable plugin code.

## 23. Production Exports

**DEFERRED**

Possible future outputs:

- full Game Design Document
- World Bible
- Character roster
- Quest dependency sheets
- localization source sheets
- concept-art briefs
- voice briefs
- faction sheets
- item catalogs
- creature catalogs
- relationship graph data
- engine-ready content packages

These should be derived from authoritative project state.

## 24. Deferred Implementation

The following are intentionally out of the Entity Substrate implementation scope:

- image generation
- image editing
- image provider credentials
- Visual Asset database tables
- forgeproject visual-asset packaging
- Visual Canon UX
- stale visual UI
- Unity adapter
- Unreal adapter
- Godot adapter
- Game Schema Packs
- production export automation
- two-way engine synchronization

---

See also:
- THE_FORGE_ENTITY_UNIVERSE.md
- ENTITY_SCHEMA_REFERENCE.md
- FORGE_FURTHER_AND_CONTEXT.md
- ENTITY_SUBSTRATE_AND_MIGRATION.md
- ../../GAZINGA/DISCOVERY_STATE.md
