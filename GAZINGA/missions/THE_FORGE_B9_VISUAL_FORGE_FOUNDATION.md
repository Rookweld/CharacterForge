# EXECUTION CONTRACT

Execute exactly one bounded post-v1 The Forge mission.

Before coding:
1. Read GAZINGA/CANON_CORE.md.
2. Read GAZINGA/THE_FORGE_CANON_CORE.md.
3. Read GAZINGA/DISCOVERY_STATE.md.
4. Read GAZINGA/KNOWN_ISSUES.md.
5. Read this mission completely.
6. Read docs/design/VISUALS_AND_ENGINE_BOUNDARY.md completely.
7. Read docs/design/ENTITY_SCHEMA_REFERENCE.md for Character, Location, Faction, Item, Creature.
8. Read B6, B7, and B8 reports.
9. Inspect the actual repository before changing contracts.

During work:
- preserve all current Entity, Relationship, Forge Further, Canon, persistence, export, and deterministic generation behavior
- preserve database schema 2
- preserve forgeproject archive v1
- preserve app version 1.0.0
- do not add a real cloud/local image provider yet
- do not add provider secrets/configuration
- do not download or generate binary images in this mission
- do not invent Visual Canon semantics
- do not put image paths into Entity payloads
- do not change narrative Canon as a result of visual compilation
- do not change archive format or add binary members to .forgeproject v1
- do not begin B10 or later work

At the end:
- run full verification
- create docs/missions/THE_FORGE_B9_VISUAL_FORGE_FOUNDATION_REPORT.md
- update GAZINGA/DISCOVERY_STATE.md
- update GAZINGA/PROJECT_STATE.md only for material facts
- report PASS only if every exit criterion is satisfied
- STOP

# THE FORGE B9 — VISUAL FORGE FOUNDATION
## Provider-neutral visual context + generation brief

# Objective

Create the architecture and usable product surface that sits immediately before a real image provider.

B9 does NOT call an external image model.

B9 must make The Forge able to take an exact saved Entity state and compile a deterministic, provider-neutral Visual Generation Brief that a future adapter can send to OpenAI, fal, ComfyUI/local diffusion, or another provider without contaminating core authoring truth.

Conceptually:

    saved Entity state
      + visual role
      + optional operator art direction
      → Visual Context Snapshot
      → deterministic Visual Generation Brief
      → provider adapter boundary (interface only)
      → future image provider

The user should be able to preview/copy/export the exact brief now.

# Starting baseline

Expected:
- master HEAD includes accepted B8 implementation
- schema 2
- archive v1
- app 1.0.0
- six Entity Types exist
- B6 Connections and B7/B8 Forge Further green
- only four preserved unrelated files remain untracked

If materially different, STOP and report.

# Supported entity families in B9

Visual Forge v1 supports:
- Character
- Location
- Faction
- Item
- Creature

Quest visual generation is deferred until a deliberate visual role is defined rather than guessed.

# Visual roles v1

Use stable typed roles.

Character:
- portrait
- full_body
- character_sheet

Location:
- establishing_shot
- environment_concept
- interior_concept

Faction:
- emblem
- banner
- uniform_concept

Item:
- item_concept
- inventory_icon
- prop_sheet

Creature:
- creature_concept
- full_body
- anatomy_detail

Do not use arbitrary free-form role strings in persisted/core contracts.

# Provider-neutral Visual Context Snapshot

Compile from the exact selected saved Entity state.

Include:
- format_version = 1
- visual_recipe_id
- visual_recipe_version
- entity_id
- entity_type
- source write_version
- source Canon state
- exact Canon revision number when the source is an explicitly selected Canon revision and the repository can identify it safely; otherwise represent the exact current persisted state without fabricating a revision number
- entity generator/content provenance where available
- selected visual role
- selected semantic visual facts
- preset/Creative Axes where available
- operator art-direction input
- canonical source body hash
- canonical visual-context hash

Compiler requirements:
- read-only
- deterministic
- bounded
- no whole-project context
- no relationship traversal in B9
- no network
- no filesystem write
- no mutation of Entity/Relationship/Canon
- exact same source state + role + art direction => exact same snapshot/hash

# Semantic facts per entity

Use structured fields, not blind whole-payload dumping.

Character:
- name
- title
- archetype
- role
- appearance
- personality summary
- signature_gear
- key abilities summary
- faction_hint only as contextual design language, not authoritative emblem/clothing truth

Location:
- name
- location_profile
- summary
- visual_identity
- atmosphere
- purpose
- notable_features
- danger where it materially affects scene mood

Faction:
- name
- faction_profile
- identity
- culture
- public_face
- symbols
- ideology only where it affects visible design language

Item:
- name
- item_profile
- appearance
- purpose
- properties
- origin
- power_or_function
- condition
- rarity as descriptive worldbuilding only

Creature:
- name
- creature_profile
- appearance
- habitat
- behavior
- ecology
- threat
- abilities_summary
- weakness only when it produces useful visible anatomy/design cues
- signs_of_presence only for scene/encounter roles, not isolated concept roles

Keep secrets/hidden truths OUT of visual prompts by default unless the selected role explicitly needs them. B9 should favor visible facts.

# Operator art direction

B9 may accept optional transient art-direction fields:
- medium/style
- palette
- materials
- camera/composition
- atmosphere
- avoid/prohibited traits

Requirements:
- typed, bounded strings
- optional
- included in snapshot/hash
- not project-global persistence yet
- not provider-specific syntax
- no secret/API-key fields

Do not invent a permanent Project Visual Direction storage model in B9.

# Visual Recipe Definitions

Create app-owned versioned definitions for each entity family + visual role.

Each recipe should define:
- stable recipe_id
- version = 1
- source EntityType
- VisualRole
- required semantic fields
- optional semantic fields
- composition intent
- default aspect intent (semantic, not provider parameter)
- whether transparent-background intent is meaningful
- whether background/environment should be present
- forbidden secret-field categories by default

Do not put provider model names or API request fields in recipes.

# Visual Generation Brief

Compile a deterministic provider-neutral brief with at minimum:

- brief format_version = 1
- context hash
- recipe ID/version
- entity identity/type
- visual role
- concise subject description
- structured must-include facts
- optional supporting details
- art direction
- composition/camera intent
- background/environment intent
- avoid list
- consistency notes
- provenance summary

The brief should also expose a human-readable prompt text generated deterministically from the same structured data.

Requirements:
- stable ordering
- no raw UUID/hash leakage inside creative prompt text
- IDs/hashes allowed in provenance metadata only
- no hidden chain-of-thought
- no provider-specific flags
- no "best quality, masterpiece" boilerplate spam
- no silent contradictions with source facts
- no secret field leakage by default

# Provider adapter boundary

Define an interface/trait/contract only.

Conceptually:
    ImageProviderAdapter
      capabilities()
      compile_request(VisualGenerationBrief, ProviderSettings)
      generate(...)   // NOT IMPLEMENTED/CALLED IN B9

B9 may define typed capability metadata sufficient for future providers:
- supported aspect intents
- transparent background support
- reference-image support
- seed support
- variation support

But do not implement a real provider or call network/local model.

Core must not know provider-specific request JSON.

# Visual Asset concept

Define only the provider-neutral metadata contract needed for the future result, without persisting binary assets yet.

A VisualAssetCandidate metadata contract may include:
- asset_id
- project_id
- linked entity_id
- source state reference
- visual role
- visual recipe/version
- context hash
- provider/provider_model optional future fields
- seed optional
- prompt/brief hash
- content hash optional until binary exists
- created_at optional/zero for pure planning
- status Candidate

IMPORTANT:
- no Entity payload field gets an asset path
- no binary path is persisted
- no Visual Canon field/state introduced
- no DB schema bump
- no archive change
- no fake generated asset rows without an actual image

If even this metadata contract would force premature persistence semantics, keep it as an in-memory/serializable domain contract only.

# Desktop UX

On saved detail views for Character, Location, Faction, Item, Creature add:

**Visual Forge**

Flow:
1. open a saved Entity
2. choose supported visual role
3. optionally enter art direction
4. Compile Visual Brief
5. preview:
   - source entity and state
   - visual role
   - context facts
   - context hash/provenance
   - deterministic human-readable prompt/brief
6. copy prompt/brief to clipboard if current Tauri capabilities support it safely without new dependency; otherwise provide selectable text and Export Brief
7. export Visual Brief as JSON and/or Markdown through existing safe file-save patterns

There must be an explicit notice:
- "No image provider is configured in this foundation build."
or equivalent.

Do not show a fake Generate Image button that pretends generation occurred.

Quest detail must not show Visual Forge in B9.

# Stale-source semantics

The compiled brief refers to an exact source state.

If the source Entity changes after compilation:
- existing compiled brief remains truthful to old source snapshot
- UI should be able to detect/recompile or warn before export/use if current persisted source write_version differs
- do not silently relabel the old brief as current
- do not mutate/delete the old in-memory snapshot automatically

Persistence of historical visual briefs is NOT required in B9 unless repository structure supports it without schema/archive changes. Prefer transient compile + export.

# Security/privacy boundaries

- no API keys
- no provider credentials
- no network
- no shelling out to image tools
- no arbitrary prompt templates executing code
- no loading remote images
- no automatic inclusion of hidden secret fields
- no arbitrary file paths from Entity text

# Tests — contracts/app-core

At minimum:
- stable VisualRole serialization
- role validity per EntityType
- stable recipe IDs/version
- deterministic snapshot/hash
- relevant visible field change changes hash
- non-visual secret change does not affect default visual snapshot unless recipe includes it
- art-direction change changes hash
- same source+role+direction exact replay
- compiler read-only
- Character recipe facts correct
- Location recipe facts correct
- Faction recipe facts correct
- Item recipe facts correct
- Creature recipe facts correct
- Quest rejected
- bounded string/list behavior
- prompt text stable/order deterministic
- no UUID/hash leakage in creative prompt
- no secret leakage by default
- existing entity goldens/replay unchanged
- B2/B7/B8 regressions unchanged

# Tests — frontend/Tauri

At minimum:
- Visual Forge action visible for all five supported families
- absent for Quest
- role options are family-specific
- compile shows correct visible facts
- art direction affects preview
- no provider configured notice
- no fake success/image state
- stale source warning/recompile path
- export brief works through safe path
- current entity save/Canon/Connections behavior unaffected

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

# Human/product decision boundary

B9 must STOP and ask for operator choice before implementing any of these:
- actual image provider
- API-key storage
- local ComfyUI/Stable Diffusion integration
- OpenAI/fal/other cloud adapter
- binary Visual Asset persistence
- adjacent asset directory layout
- forgeproject v2
- Visual Canon/selected-image semantics
- provider billing/credit behavior

Those are B10+ product decisions.

# Exit gate

PASS only if:
- provider-neutral VisualRole and recipe contracts exist
- five current visual entity families compile deterministic bounded snapshots
- deterministic provider-neutral Visual Generation Brief + human prompt exists
- no secret leakage by default
- no raw identity/hash leakage into creative prompt
- exact source state/provenance is represented
- stale-source semantics are safe
- desktop Visual Forge preview/export exists
- Quest is deliberately unsupported
- no network/provider/binary asset persistence occurs
- no schema/archive/app-version bump
- B2/B6/B7/B8 and all standalone generators remain green
- full verification green
- no known B9 P0/P1/P2 remains

STOP.

Do not implement an actual image provider until the operator chooses the provider/persistence direction.
