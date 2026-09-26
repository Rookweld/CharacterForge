# Character Forge / Quest Forge
## WORLD BIBLE & Platinum Roadmap

**Project status:** Version 1 design baseline — no production build is assumed to exist yet.  
**Document intent:** This is the canonical product bible, architecture plan, release history-as-roadmap, expansion strategy, design philosophy, engineering playbook, and Platinum quality bar for the full application.

---

# 0. The one-sentence vision

**Character Forge / Quest Forge is a local-first creative engine that turns a spark into a structured, reusable universe: characters, quests, factions, locations, artifacts, enemies, lore, images, music prompts, campaign state, and exportable project files — with deterministic generation, AI augmentation, canon memory, lock-and-reroll control, and genre expansions.**

The core promise is simple:

> **Generate something cool. Keep what you love. Reroll only what you do not. Promote the result into canon. Grow it into a world.**

The application is not a prompt box with a fancy skin. It is a **creative construction system**.

---

# 1. Product identity

## Working product name
**Character Forge / Quest Forge**

The slash is intentional. The product began as two complementary tools and matured into one creative system with two primary entry points.

### Character Forge
Creates and evolves people, creatures, archetypes, rivals, companions, villains, factions leaders, merchants, hunters, captains, shamans, hackers, engineers, monsters, and stranger things.

### Quest Forge
Creates and evolves missions, contracts, mysteries, expeditions, hunts, heists, journeys, faction conflicts, moral dilemmas, encounters, and campaign arcs.

Everything else in the application exists to deepen those two loops.

## Tagline
**Forge a spark. Build a world.**

Alternative release/store tagline:
**Characters. Quests. Worlds. Canon. One forge.**

---

# 2. Product principles — the rules that never change

These became the non-negotiable rules of the project.

### 2.1 Generate, then shape
The user should never have to throw away an entire result because one field is bad. Every meaningful field can be locked, rerolled, edited, expanded, or promoted independently.

### 2.2 Structure first, prose second
The engine creates a valid structured object before any model is asked to embellish it. A character has an identity, role, motive, flaw, faction, gear, relationships, and hooks before it has a pretty paragraph.

This made generation more consistent, testable, and model-independent.

### 2.3 AI is optional, not foundational
The application remains useful without cloud AI. Three generation modes exist:

- **Forge Core** — deterministic/rule-based generation, fully offline.
- **Local Forge** — local LLM through Ollama or another OpenAI-compatible local endpoint.
- **Cloud Forge** — configured cloud provider through a provider abstraction.

AI improves nuance. It does not own project state.

### 2.4 Canon is sacred
“Inspiration” and “canon” are separate states. Random generation can never silently overwrite established lore.

### 2.5 Every result should create new doors
A generated artifact may imply an owner. A character may imply a rival. A quest may imply a faction. A location may imply a secret. Every entity exposes contextual **Forge Further** actions.

### 2.6 Local-first
Projects, canon, settings, packs, generation history, and exports live locally by default. Cloud features are optional additions, never a requirement to open the application.

### 2.7 Reproducibility matters
A result stores its seed, recipe version, active content packs, generator version, and provider metadata. The user can reproduce, branch, or explain where a result came from.

### 2.8 Chaos is allowed; corruption is not
Chaos Mode can break genre expectations. It may never break schemas, IDs, project links, migrations, or canon integrity.

---

# 3. Who the application is for

The product deliberately supports several overlapping audiences instead of narrowing itself to one niche.

- tabletop GMs and players
- writers and worldbuilders
- game designers and indie developers
- visual artists looking for concepts
- musicians creating concept albums or narrative releases
- roleplayers and LARPers
- people who simply enjoy generating fictional worlds
- creative technologists who want structured outputs for downstream tools

The key design decision: **the interface speaks in creative concepts rather than game-system jargon**. RPG-specific mechanics are content packs, not assumptions baked into the core.

---

# 4. The core user loop

The final core workflow became:

1. Open or create a Project.
2. Choose **Character Forge** or **Quest Forge**.
3. Select a universe preset or blend several.
4. Set a few high-value creative axes.
5. Forge.
6. Lock the parts that work.
7. Reroll or edit the parts that do not.
8. Use **Forge Further** to expand relationships, gear, places, factions, enemies, scenes, or lore.
9. Promote chosen results to Canon.
10. Export, visualize, continue, or branch.

The decisive UX breakthrough was **lock-and-reroll at field level**. It became the feature people understood immediately and missed in ordinary chatbot workflows.

---

# 5. Creative controls

Instead of thirty dropdowns, the application uses a handful of axes and optional advanced constraints.

## Primary axes

- **Tone:** Hopeful ↔ Bleak
- **Reality:** Grounded ↔ Mythic
- **Order:** Ordered ↔ Chaotic
- **Scale:** Intimate ↔ Epic
- **Morality:** Clear ↔ Ambiguous
- **Strangeness:** Familiar ↔ Surreal
- **Technology:** Primitive ↔ Advanced
- **Mysticism:** None ↔ Dominant

## Optional constraints

The user can add freeform or structured constraints such as:

- must be non-human
- no magic
- urban only
- no firearms
- protagonist cannot be royalty
- reward must create another problem
- one comedic element
- must connect to existing faction X

Constraints are treated as data and validated after generation.

---

# 6. Universe presets in the core release

Version 1 begins with genre presets rather than paid expansions. Presets are lightweight recipes using core vocabulary and rules.

- Dark Fantasy
- Pirate
- Steampunk
- Cyberpunk
- Nordic Myth
- Post-Apocalyptic
- Occult
- Monster Hunter
- Rogue / Assassin
- Contemporary Weird
- **Chaos Blend**

A preset controls weights, vocabulary, archetypes, materials, hazards, quest structures, artifact families, faction styles, and visual prompt vocabulary.

Later full expansions go much deeper than presets.

---

# 7. Technical foundation

## Desktop shell
**Tauri 2**

Why it won:

- much smaller footprint than Electron
- excellent Windows packaging
- cross-platform path remains open
- Rust boundary provides a good place for file/database/security logic
- frontend remains fast to iterate

## Frontend

- React
- TypeScript
- Vite
- component-driven UI
- CSS variables/design tokens
- accessible keyboard-first controls

The project avoids coupling the product to one giant UI framework. Components own behavior; design tokens own appearance.

## Core application services

- Tauri/Rust application shell
- SQLite project database
- versioned JSON schemas
- generator engine
- provider adapter layer
- content-pack manager
- export/import system
- local asset manager
- optional embedding/search service later

## Storage

**SQLite is the source of truth.**

It holds:

- projects
- entities
- relationships
- canon state
- generator recipes
- seeds
- history
- tags
- pack versions
- assets metadata
- model/provider metadata

JSON is used for schemas, pack manifests, portability, and exports — not as a replacement for transactional project storage.

## Search
SQLite FTS is used before adding a vector database.

Embeddings are optional and introduced only when semantic lore recall becomes useful at larger project sizes.

One of the project’s strongest engineering lessons was: **do not introduce a vector database because “AI app” sounds like it should have one.**

---

# 8. Canonical domain model

Every object inherits a shared entity envelope:

```text
Entity
- id: UUID
- type
- name
- project_id
- schema_version
- canon_state: draft | candidate | canon | archived
- origin: manual | generated | imported
- created_at
- updated_at
- tags[]
- notes
- generation_provenance
```

Core entity types:

### Character
- identity
- aliases
- archetype
- role
- appearance
- personality
- desire
- fear
- flaw
- secret
- skills
- abilities
- gear links
- faction links
- relationship links
- story hooks

### Quest
- title
- quest_type
- premise
- patron
- objective
- stakes
- opposition
- complications
- twist
- moral pressure
- locations
- rewards
- failure consequences
- follow-up hooks

### World
- premise
- rules
- tone
- technology
- mysticism
- cultures
- regions
- historical pressures

### Location
- type
- description
- atmosphere
- inhabitants
- hazards
- secrets
- connected locations

### Faction
- purpose
- ideology
- methods
- resources
- rivals
- allies
- leadership
- public face
- hidden agenda

### Artifact
- category
- origin
- appearance
- function
- cost
- limitation
- history
- current owner
- hooks

### Enemy / Creature
- taxonomy
- behavior
- threat
- habitat
- strengths
- weaknesses
- signs
- loot/reward
- narrative use

### TimelineEvent
- date/order
- event
- participants
- locations
- consequences
- certainty

### Relationship
- source_id
- target_id
- relationship_type
- direction
- strength
- public_state
- hidden_state

### GenerationRecipe
Stores the exact inputs used to create or mutate an entity.

---

# 9. The Forge Engine

The generator became a pipeline rather than “send prompt, parse answer.”

## Pipeline

### Stage 1 — Input normalization
Normalizes universe, sliders, constraints, locked fields, project context, and requested entity type.

### Stage 2 — Deterministic skeleton
The local generator selects archetypes, structures, relationships, quest patterns, and semantic slots from weighted pack tables.

### Stage 3 — Constraint solve
Checks hard constraints before prose generation.

### Stage 4 — Prompt compilation
If AI mode is enabled, the structured skeleton becomes a compact provider-neutral prompt.

### Stage 5 — Provider generation
Local or cloud provider enriches specific fields.

### Stage 6 — Schema normalization
Model text is parsed into application-owned types. AI never creates persistent IDs or database links directly.

### Stage 7 — Canon validation
Checks conflicts against project facts and user locks.

### Stage 8 — Quality pass
Detects repetition, contradictions, empty fields, forbidden content defined by project constraints, and suspiciously generic phrasing.

### Stage 9 — Persist as candidate
Nothing automatically becomes canon.

This pipeline is one of the core reasons the product scales.

---

# 10. Forge DNA — reproducibility

Every generated result stores a **Forge DNA** record:

- random seed
- generator version
- schema version
- content pack IDs and versions
- universe blend weights
- creative axes
- constraints
- locked fields
- provider ID
- model ID if applicable
- prompt compiler version
- parent entity/revision if derived

This enables:

- exact reruns where possible
- controlled variants
- debugging
- pack regression testing
- project provenance
- branching

---

# 11. Version 1.0 — FOUNDATION

**This is the first build target from where the project stands today.**

## Scope

Version 1.0 intentionally does not try to be the full dream.

It ships the two killer loops extremely well.

### Character Forge v1
Generates:

- name
- title
- archetype
- role
- appearance summary
- personality
- desire
- fear
- flaw
- secret
- signature gear
- faction hint
- three story hooks

### Quest Forge v1
Generates:

- title
- quest archetype
- premise
- patron
- objective
- stakes
- antagonist/opposition
- two complications
- twist
- reward
- failure consequence
- follow-up hook

### Shared features

- project creation
- universe presets
- Chaos Blend
- creative axes
- deterministic seeds
- Forge button
- field-level lock
- reroll field
- reroll unlocked
- manual editing
- generation history
- undo/redo within current forge session
- favorite/star
- candidate/canon toggle
- Markdown export
- JSON export
- local SQLite storage
- portable project backup
- settings
- dark/light capable design-token system, though the signature theme is dark

## Explicitly not in v1.0

- image generation
- cloud sync
- collaboration
- maps
- deep simulation
- community marketplace
- plugin code execution
- complex campaign timelines

The discipline to omit these features is what allows v1 to become real.

## v1.0 release gate

- fresh install works on supported Windows version
- create/open/save project reliably
- 1,000 seeded Character Forge test runs without schema failure
- 1,000 seeded Quest Forge test runs without schema failure
- lock/reroll never mutates locked fields
- export/import roundtrip retains all supported data
- database migration test exists from schema 0 → 1
- no hard dependency on network access
- no API key required
- no P1/P2 data-loss bugs

---

# 12. Version 1.1 — ANVIL

Version 1.1 focuses on the insight that users spend more time **shaping** than generating.

Added:

- multi-select field locks
- reroll with instruction (“make this stranger”, “less tragic”)
- pin vocabulary or concepts
- branch result into variant
- compare two revisions side-by-side
- quick duplicate
- richer history browser
- search by tags/name/type
- templates for common creative intents
- better clipboard workflow
- keyboard shortcuts

### Key UX discovery
A large “Forge Again” button was not enough. The product became much more addictive when users could say:

> Keep everything except the flaw and the faction.

That became the defining interaction.

---

# 13. Version 1.2 — RELICS & RIVALS

This release introduces contextual sub-generators.

New entity generators:

- Artifact Forge
- Enemy Forge
- Faction Forge
- Location Forge

They are not promoted as four new main applications. They appear through **Forge Further**.

Example:

A character result shows:

- Forge their weapon
- Forge their rival
- Forge their faction
- Forge their hideout
- Forge the quest that changes them

This is the point where the app stops feeling like separate generators and begins feeling like a world construction graph.

---

# 14. Version 1.5 — CHRONICLE

Version 1.5 introduces the first serious project/worldbuilding layer.

Added:

- entity browser
- relationship graph
- project canon panel
- lore notes
- timeline events
- project glossary
- canon conflict warnings
- context-aware generation using selected canon entities
- project-wide search
- bulk tagging
- entity collections
- project snapshots

## Canon rules

The application distinguishes:

- Draft — rough/manual
- Candidate — generated or edited material under consideration
- Canon — established truth
- Archived — kept for reference but excluded from normal generation context

### Major lesson
Never inject the entire project into an LLM prompt. The system selects the smallest relevant context using entity links, tags, FTS, and later optional semantic retrieval.

---

# 15. Version 2.0 — WORLDSMITH

This is the first true “world engine” release.

Added:

### World Forge
Creates a world skeleton with:

- premise
- fundamental tensions
- rules of reality
- technology/magic balance
- dominant powers
- cultures
- regions
- current crises
- myths

### Region and settlement generation
Worlds can grow hierarchically:

World → Region → Settlement → District → Site

### Faction ecology
Factions receive:

- resources
- goals
- pressures
- alliances
- rivalries
- dependency links

### Quest chaining
A quest can generate consequences that become new project state and subsequent quests.

### Chronicle timeline
Events can be arranged on a timeline and generated relative to established events.

### World constraints
Users can define laws such as:

- resurrection is impossible
- all magic has a physical cost
- no nation possesses faster-than-light travel
- gods never communicate directly

The validator checks generated material against these laws.

---

# 16. Version 2.2 — FORGE MEMORY

The product adds context selection and optional semantic retrieval.

Features:

- lore recall service
- SQLite FTS baseline
- optional embeddings index
- configurable context budget
- relevance inspection (“Why did the Forge remember this?”)
- manual pinning of canon context
- context exclusions

## Lesson
Opaque “memory” makes creative tools frustrating. The user must be able to inspect and override what lore is being supplied to generation.

---

# 17. Version 2.5 — RITUAL ENGINE

This is where AI becomes a first-class augmentation layer without becoming a dependency.

## Provider abstraction

Supported classes:

- Forge Core offline engine
- Ollama
- OpenAI-compatible local endpoints
- configured cloud providers

Each provider advertises capabilities:

- structured output
- context length
- image support
- streaming
- tool support
- embeddings

The app requests capabilities rather than hardcoding provider names throughout the product.

## Prompt compiler

Prompt logic moves into versioned templates with tests.

Separate prompt stages exist for:

- expansion
- rewrite
- contradiction check
- style shift
- summarization
- relationship inference
- quest consequence generation

## Model-independent persistence
Model responses are normalized to the product schema immediately. The database never becomes a graveyard of provider-specific chat blobs.

---

# 18. Expansion architecture introduced after 2.5

A full expansion is not a bag of adjectives.

An expansion may contain:

- content tables
- archetypes
- quest patterns
- faction templates
- artifact families
- creature families
- visual vocabulary
- music/style vocabulary
- world laws
- generator modifiers
- validation rules
- UI icon/theme tokens
- sample projects
- tutorial scenarios

## File format

First-party and community content use a versioned `.forgepack` container.

A pack contains:

```text
manifest.json
schemas/
data/
recipes/
prompts/
assets/
localization/
README.md
```

Packs are declarative by default. Arbitrary executable code is not permitted in ordinary content packs.

This kept modding powerful without making “install a genre pack” equivalent to “run somebody’s code.”

---

# 19. Expansion I — RAVENBOUND
## Dark Fantasy & Occult

The first full expansion deliberately deepens the most natural genre for the product.

Adds:

- curses with costs and escalation
- occult factions
- forbidden orders
- relic corruption
- ritual quests
- omen system
- monster signs and hunts
- gothic settlements
- dark pilgrimage structures
- bargains and consequences
- secret societies
- ritual visual prompt vocabulary

### Signature mechanic: **The Price**
Powerful artifacts, abilities, bargains, and solutions may carry a generated narrative cost that persists in canon.

This mechanic later became available to other packs.

---

# 20. Expansion II — BLACK TIDE
## Pirates, Privateers & Haunted Seas

Adds:

- crews
- ships as entities
- ports
- sea regions
- treasure chains
- rival captains
- mutiny pressure
- naval rumors
- cursed cargo
- privateer contracts
- ghost-ship events

### Signature mechanic: **Crew Tension**
Named crew members develop loyalties, grudges, fears, and ambitions that can modify quests.

### Key expansion lesson
The best expansions add **new verbs**, not just new nouns.

Black Tide does not merely generate pirate-flavored names. It lets the player **crew, sail, bargain, mutiny, hunt, smuggle, raid, and chart**.

---

# 21. Expansion III — BRASS & SMOKE
## Steampunk & Industrial Fantasy

Adds:

- inventions
- patents and stolen designs
- airships
- guilds
- industrial cities
- class tension
- clockwork entities
- unstable prototypes
- workshop locations
- resource dependencies

### Signature mechanic: **Prototype Faults**
Inventions can be powerful but carry physical, social, legal, or mechanical failure modes.

---

# 22. Expansion IV — NEON ASHES
## Cyberpunk & Tech-Noir

Adds:

- megacorps
- street crews
- augmented characters
- networks
- digital locations
- data artifacts
- reputation
- identity layers
- synthetic entities
- surveillance pressures
- corporate contracts

### Signature mechanic: **Heat**
Actions can increase attention from factions, law enforcement, corporations, rivals, or networks.

Heat is project state and can influence generated quests.

---

# 23. Expansion V — FROST & IRON
## Nordic Myth & Saga

Adds:

- clans
- oaths
- blood debts
- halls
- voyages
- mythic beasts
- seers
- runic artifacts
- winter pressures
- honor conflicts

### Signature mechanic: **Oaths**
An oath creates an explicit canon obligation. Breaking one can generate consequences and faction reactions.

---

# 24. Expansion VI — WASTELAND TESTAMENT
## Post-Apocalyptic Survival

Adds:

- settlements
- scarcity
- salvage
- contamination zones
- vehicles
- routes
- ruins
- survival factions
- strange weather
- old-world artifacts

### Signature mechanic: **Scarcity Ledger**
Locations and factions can have pressure around water, power, medicine, ammunition, fuel, food, trust, or technology.

Quests emerge naturally from shortages rather than only from random mission tables.

---

# 25. Expansion VII — HUNTER’S MARK
## Monster Hunter / Dark Expedition

Adds:

- contracts
- tracks/signs
- creature ecology
- preparation stages
- trophies
- specialist gear
- hunter lodges
- rival hunters
- evolving monsters

### Signature mechanic: **The Hunt Loop**

Discover → Research → Prepare → Track → Confront → Consequence

Quests in this pack can use the entire loop or only selected phases.

---

# 26. Free meta-expansion — CHAOS GATE

Chaos Blend existed in the core from the beginning, but Chaos Gate formalizes cross-pack fusion.

Features:

- weighted multi-pack recipes
- incompatibility resolver
- genre collision prompts
- cross-pack artifact mutation
- “one wrong thing” mode
- escalating weirdness
- remix seeds

Example:

> A Nordic oathbound salvage crew pilots a failing brass submarine beneath a cyberpunk megacity while hunting a saint-shaped machine that only appears during electrical storms.

The important implementation rule:

**Chaos combines semantics, not databases.** Entity structure remains valid no matter how bizarre the fiction becomes.

---

# 27. Version 3.0 — FORGE STUDIO

Version 3 turns structured worlds into creative production material.

Added:

## Visual prompt cards
Every entity can generate provider-neutral visual briefs:

- subject
- composition
- materials
- lighting
- environment
- camera/lens language
- palette
- negative constraints

## Music prompt cards
Produces structured prompts for music tools:

- genre blend
- instrumentation
- tempo/energy
- vocal character
- narrative function
- arrangement arc
- texture

## Scene Forge
Generates scenes using canon characters, location, conflict, and desired purpose.

## Storyboard cards
Scene beats with shot suggestions and continuity notes.

## Export bundles
Exports project subsets for:

- Markdown
- JSON
- CSV
- prompt packs
- image brief packs
- game-dev data

A Unity/Godot-friendly JSON profile is introduced without making either engine a runtime dependency.

---

# 28. Version 3.2 — ASSET VAULT

Adds a real local asset layer:

- attach images
- attach audio references
- attach PDFs/text notes
- entity-linked assets
- thumbnails
- provenance
- export packaging

Generated or imported media remains separate from the canonical textual entity data.

This prevents heavy binaries from polluting project migrations.

---

# 29. Version 3.5 — CREATOR SDK

The pack ecosystem becomes officially supported.

Includes:

- Forge Pack Builder
- schema validator
- recipe debugger
- preview runner
- seeded regression tests
- localization checker
- package signing metadata
- pack dependency declarations
- semantic version compatibility checks

## Community packs
Community packs are declarative and can be imported from files.

The application never silently downloads or executes community code.

---

# 30. Version 4.0 — LIVING WORLDS

The application gains simulation without pretending to be a full game engine.

## Pressure simulation
Factions and locations have pressures such as:

- resources
- fear
- influence
- instability
- heat
- loyalty
- conflict

Simulation ticks produce **proposals**, not irreversible canon changes.

Example:

> The Iron Choir is likely to blockade Greyhaven because fuel pressure reached critical and relations with the harbor guild collapsed.

The user can accept, reject, modify, or branch the event.

## NPC intention system
Important characters can hold:

- goal
- current plan
- obstacle
- relationship pressure
- secret

The engine can propose plausible next actions.

### Critical principle
The simulation never becomes an autonomous black box that rewrites the user’s world. It suggests history; the user canonizes history.

---

# 31. Version 4.2 — CAMPAIGN MODE

Adds long-form play and session support.

- session log
- unresolved threads
- clocks/pressures
- quest status
- character status
- consequences queue
- “What changed since last session?”
- recap generator
- next-session hooks

The product becomes viable as a GM companion without sacrificing its broader creative identity.

---

# 32. Version 4.5 — VAULT SYNC & COLLABORATION

Cloud arrives late, intentionally.

Local projects remain canonical. Optional sync adds:

- encrypted project backup
- version history
- device sync
- shared workspaces
- comments
- role permissions
- branch/merge workflows for canon edits

## Why this waited until 4.5
Collaboration built before the domain model stabilizes multiplies every design mistake. By waiting, sync moves stable entities instead of synchronizing chaos.

---

# 33. Version 5.0 — PLATINUM

**Character Forge / Quest Forge: Platinum** is the point where the application is treated as a mature creative platform rather than an ambitious generator.

## Platinum includes

- Character Forge
- Quest Forge
- World Forge
- Artifact / Enemy / Faction / Location Forge
- Chronicle
- Forge Memory
- provider abstraction
- visual prompt cards
- music prompt cards
- Scene Forge
- Asset Vault
- living-world proposals
- Campaign Mode
- Creator SDK
- all first-party core universe presets
- expansion compatibility layer
- polished import/export
- accessibility pass
- backup/restore
- disaster recovery
- stable migrations
- full offline mode

## Platinum UX pass

- onboarding project with guided examples
- contextual help rather than tutorial walls
- command palette
- keyboard navigation
- consistent empty states
- explainable context/memory
- diff view for every mutation
- non-destructive history
- high-DPI support
- reduced motion setting
- screen-reader labels

## Platinum reliability gate

Release is blocked unless:

- zero known P1 data-loss issues
- zero known P1 security issues
- all migrations pass clean and populated database fixtures
- backup/restore verified
- import/export roundtrip verified
- pack compatibility tests pass
- deterministic golden seeds pass
- lock invariants pass
- offline mode works after fresh install
- provider outages degrade gracefully
- corrupted optional assets cannot corrupt project DB
- application can reopen the last valid project after an interrupted write

## Platinum packaging

Windows is the primary supported platform first.

Ship:

- signed installer when economically practical
- portable build
- release notes
- checksums
- sample projects
- user guide
- pack author guide
- migration guide

macOS/Linux releases follow the same core but receive their own packaging test matrices rather than being claimed “supported” just because Tauri can compile them.

---

# 34. The expansion business model — optional, not architectural

The codebase must not assume one monetization model.

Possible commercial model at Platinum:

- paid core application or supporter edition
- major first-party expansions as paid packs
- free utility/meta packs
- community packs free by default
- optional marketplace later

Do not architect DRM into v1.

Licensing and commerce sit outside the creative engine. This keeps open-source or hybrid distribution options possible until product-market fit exists.

---

# 35. UI architecture

The final app uses five major spaces.

## 35.1 Forge
The immediate creative workspace.

- Character Forge
- Quest Forge
- contextual Forge Further panels
- locks
- rerolls
- creative axes

## 35.2 World
Structured project entities and relationships.

- entity browser
- world hierarchy
- factions
- relationships
- timeline

## 35.3 Chronicle
History and canon management.

- canon changes
- timeline
- generation history
- revisions
- branch comparisons

## 35.4 Studio
Downstream creative material.

- scene cards
- image briefs
- music briefs
- export bundles
- assets

## 35.5 Packs & Settings

- universe packs
- provider settings
- storage
- backups
- accessibility
- Creator SDK tools

The application never becomes a dashboard full of tiny status boxes. Every screen has one clear creative job.

---

# 36. Interaction details that mattered more than expected

## Lock icons must be obvious
Tiny hidden locks failed usability tests. Locks became first-class, always-visible controls on forge fields.

## Reroll must show scope
Before rerolling, UI clearly states what will change.

Example:

> Reroll 3 unlocked fields. 8 locked fields will remain unchanged.

## Diff before replacing canon
When a canon entity is mutated, the app shows old/new values and requires explicit acceptance.

## Undo is not a substitute for revision history
Undo handles recent UI operations. Revision history handles creative provenance.

## Seeds deserve visibility
Not on every screen, but available in an advanced inspector and copyable for reproducibility.

---

# 37. Prompt-engineering lessons discovered along the way

## 37.1 Do not ask models for everything in one pass
Large “create a complete character with everything” prompts become generic and contradictory.

Better:

1. deterministic structure
2. targeted field enrichment
3. consistency pass

## 37.2 Ask for contrasts
Characters improve dramatically when prompts request internal tension.

Example:

- a virtue that causes problems
- a skill paired with a blind spot
- a desire that conflicts with an obligation

## 37.3 Give the model less lore, but better lore
Relevant context beats maximum context.

## 37.4 Force concrete nouns
“Ancient mysterious artifact” is weak. The compiler requests material, construction, function, provenance, limitation, and one sensory trait.

## 37.5 Never let prose overwrite IDs
The model describes relationships. Application code resolves references.

## 37.6 Rewrite only the field being changed
When the user asks to make a character’s secret darker, do not regenerate their name, outfit, faction, and childhood.

## 37.7 Run validation after creativity
Validation before generation narrows possibilities. Validation after generation catches contradictions while preserving surprise.

---

# 38. Generator design tricks

## Weighted tables beat giant static lists
Weights can respond to universe, tone, existing canon, and novelty pressure.

## Novelty pressure
The generator records recently used archetypes and vocabulary within a project/session and temporarily reduces their weights.

This reduces “everything is ash, shadow, void, blood, raven, and forgotten kings” syndrome.

## Semantic families
Terms belong to families so the engine can avoid near-duplicates.

Example:

`smuggler`, `contraband runner`, and `black-market courier` are treated as related concepts for repetition control.

## Controlled contradiction
Some contradiction is narratively useful.

A “contradiction” can be transformed into a secret, rumor, unreliable source, or unresolved mystery rather than always rejected.

---

# 39. Content pack authoring rules

A good pack must answer:

- What new stories become possible?
- What new actions become possible?
- What new pressures exist?
- What new entity relationships exist?
- What makes this pack structurally different from a vocabulary reskin?

A pack should avoid:

- copyrighted franchise names/content
- hundreds of near-identical adjectives
- hardcoded UI behavior
- executable scripts
- assumptions that break other packs

Every first-party pack ships:

- 20+ archetype families
- 20+ quest structures
- 10+ faction models
- 10+ artifact families
- 10+ location families
- signature mechanics
- regression seeds
- at least one sample project

---

# 40. Project file strategy

## `.forgeproject`
Portable project archive.

Contains:

- SQLite database snapshot
- manifest
- optional assets
- pack lockfile
- integrity metadata

## `.forgepack`
Content pack.

## `.forgebundle`
Export bundle for sharing selected entities/scenes/prompts without sharing the entire project.

All containers are versioned and use documented internal structures.

---

# 41. Backups and data safety

Creative work can represent hundreds of hours. Data loss is treated as a product-ending bug.

Rules:

- atomic database writes
- SQLite WAL mode where appropriate
- pre-migration snapshot
- periodic local backup rotation
- manual backup command
- integrity check
- repair/export tools
- migrations are one-way in production but backups allow rollback
- assets are content-addressed or checksum-tracked where useful

The application never silently deletes unknown future-version fields during an older-version import.

---

# 42. Security rules

- API keys stored using OS credential storage where available
- never stored in project exports
- community packs are data-only by default
- HTML/Markdown previews sanitize active content
- imported archives validate paths to prevent traversal
- network access is capability-scoped
- telemetry is off by default unless explicitly enabled
- provider prompts clearly separate trusted app instructions from imported lore text

---

# 43. Testing strategy

## Unit tests

- generators
- weighting
- constraints
- locks
- schema validators
- migrations
- pack compatibility
- recipe compilation

## Golden seed tests
Known seeds must generate known structures under fixed generator/pack versions.

## Property tests
Examples:

- locked fields never change
- generated quest always has an objective
- relationship references point to existing entities
- no duplicate IDs

## Snapshot tests
Prompt compiler outputs and export formats.

## E2E tests

- install/open
- create project
- forge character
- lock fields
- reroll
- canonize
- close/reopen
- export
- import

## Failure tests

- provider unavailable
- malformed provider output
- disk full simulation
- corrupted asset
- interrupted export
- incompatible pack
- migration failure

---

# 44. Performance budgets

The structured/offline portions should feel instant.

Targets:

- app shell usable within ~2–3 seconds on a typical modern Windows machine
- offline skeleton generation under 100 ms for common entities
- lock/reroll UI response immediate
- project search under 150 ms for ordinary projects
- large lists virtualized
- AI generation streamed when provider supports it

Never make a spinner appear for work that could have been deterministic and instant.

---

# 45. Logging and diagnostics

A user-accessible diagnostics bundle contains:

- application version
- schema version
- pack list/versions
- OS/runtime info
- sanitized logs
- recent error traces

It explicitly excludes:

- API keys
- private project prose unless the user chooses to include it

Developer mode adds generator traces and prompt inspection.

---

# 46. Accessibility and usability

Platinum target:

- full keyboard navigation
- visible focus states
- scalable type
- screen-reader semantic labels
- reduced-motion mode
- non-color-only state indicators
- high-contrast friendly tokens
- tooltips for unfamiliar creative mechanics

A dark aesthetic is not an excuse for unreadable charcoal-on-black UI.

---

# 47. Visual identity

The signature visual language is **forged material + manuscript precision + modern tool clarity**.

Not full fantasy parchment.
Not generic SaaS purple gradients.
Not hacker terminal cosplay.

Elements:

- dark graphite foundation
- warm metal/ember highlights
- cool secondary accents where universes demand it
- layered cards resembling plates, pages, and components
- strong typography hierarchy
- restrained texture
- subtle forge sparks/heat only in high-impact moments

Universe packs may tint presentation but never make the application feel like a different product.

---

# 48. Sound design

Optional and restrained.

- soft forge strike when canonizing
- subtle lock click
- muted shuffle/reroll sound
- no constant ambient loop by default
- master mute

The application should still feel professional when used at work with headphones off.

---

# 49. Onboarding

The final onboarding avoids a 15-step tutorial.

It offers:

1. **Forge something now** — zero-setup demo project.
2. **Start a world** — guided project setup.
3. **Import a project**.

The demo teaches locks by generating a character and inviting the user to keep two fields and reroll the rest.

That one interaction explains the product better than several pages of documentation.

---

# 50. Documentation set

By Platinum:

- Quick Start
- Character Forge guide
- Quest Forge guide
- Canon & Chronicle guide
- World Forge guide
- AI Providers guide
- Offline mode guide
- Backup & Restore
- Content Pack authoring guide
- Forge Pack schema reference
- Export format reference
- Troubleshooting
- Migration notes

---

# 51. Things intentionally never added to core

These remain integrations or optional packs unless strong evidence says otherwise.

- full 3D map editor
- full word processor
- full game engine
- full DAW/audio editor
- social network
- cryptocurrency/NFT mechanics
- mandatory cloud accounts
- uncontrolled autonomous agents rewriting canon

Product discipline matters as much as feature ambition.

---

# 52. Tips & tricks — the lessons worth preserving

These are the “we only learned this by building it” notes that should survive into every future implementation.

### The best random generator is not very random
Pure randomness creates noise. Weighted constraints + controlled novelty creates surprise that still feels authored.

### Lock-and-reroll is the heart of the app
Treat it as infrastructure, not a UI gimmick. Every generator must support it from day one.

### Save provenance early
Seeds, pack versions, and recipe versions feel unnecessary until a user asks “how did I make that amazing result?”

### Packs should add mechanics
If a pack only changes vocabulary, it is a theme pack, not an expansion.

### Separate canon from possibility
The generator can be wild because the canon layer is deliberate.

### Do not regenerate more than necessary
Users become attached to details quickly.

### Keep provider adapters boring
All provider-specific weirdness belongs at the edge. Domain logic should never care whether text came from Ollama, a cloud model, or the offline generator.

### Normalize immediately
Never let unstructured AI output become your persistent data model.

### AI context should be inspectable
When generation uses project lore, the user should be able to see which facts were supplied.

### Use SQLite until it actually hurts
It will take the product much farther than architecture astronautics suggest.

### Build recovery before collaboration
A crash-resistant local project is more valuable than a flashy multiplayer feature.

### Every feature should answer a creative question
If the team cannot say which creative problem a feature solves, it probably does not belong.

---

# 53. Recommended repository layout

```text
character-quest-forge/
├─ apps/
│  └─ desktop/
├─ packages/
│  ├─ domain/
│  ├─ forge-engine/
│  ├─ schemas/
│  ├─ prompt-compiler/
│  ├─ pack-sdk/
│  ├─ ui/
│  └─ test-fixtures/
├─ crates/
│  ├─ app-core/
│  ├─ storage/
│  ├─ export/
│  └─ secure-store/
├─ packs/
│  └─ core/
├─ docs/
├─ scripts/
└─ tests/
   ├─ golden/
   ├─ e2e/
   └─ migrations/
```

The exact monorepo tooling can change. The important separation is domain/generator/schema/provider/storage/UI.

---

# 54. Build order — from empty repository to v1.0

This sequence is intentional.

## Phase A — Contracts before UI

1. Define schemas.
2. Define entity envelope.
3. Define Character and Quest types.
4. Define GenerationRecipe and ForgeDNA.
5. Create schema validators.
6. Create SQLite migration 001.

## Phase B — Offline forge engine

1. Seeded RNG.
2. Weighted selection helpers.
3. core content tables.
4. Character skeleton generator.
5. Quest skeleton generator.
6. constraint validation.
7. locked-field mutation API.
8. golden tests.

## Phase C — Persistence

1. project repository.
2. entity repository.
3. revisions/history.
4. settings.
5. backup/export.

## Phase D — Desktop shell

1. Tauri app.
2. project picker.
3. Forge workspace.
4. result cards.
5. locks.
6. reroll.
7. history.
8. canon toggle.

## Phase E — Import/export

1. Markdown.
2. JSON.
3. project archive.
4. roundtrip tests.

## Phase F — Polish/release

1. errors and empty states.
2. keyboard controls.
3. installer/portable build.
4. smoke tests.
5. sample project.
6. README and release notes.

Only after this passes is v1.0 considered shipped.

---

# 55. The “do not let Codex wander off” implementation rules

These should be embedded into the later Codex build prompt.

1. Build vertical slices; every milestone must compile and run.
2. Never stub a critical path and declare it done.
3. Run tests after each milestone.
4. Preserve a clean migration path.
5. Do not introduce cloud infrastructure in v1.
6. Do not substitute AI calls for deterministic core generation.
7. Do not add libraries without a concrete need.
8. Keep schemas/versioning explicit.
9. No silent data loss.
10. Produce a changelog and implementation log as work proceeds.
11. Every incomplete feature is marked visibly and excluded from release claims.
12. Prefer reliable simple implementations over speculative abstractions.

---

# 56. Milestone acceptance checklist

Every release milestone answers:

- Does it build from a clean checkout?
- Does it launch?
- Are migrations tested?
- Are new features covered by tests?
- Can existing projects still open?
- Can data be backed up and restored?
- Is the feature discoverable in UI?
- Is failure handled visibly?
- Does documentation match reality?
- Is there a reproducible release artifact?

If any answer is “no”, the milestone is not complete.

---

# 57. Platinum end-state example

A user opens a long-running project called **The Drowned Meridian**.

They select an existing canon character, Mara Vey, and click **Forge a Quest**.

The app remembers that:

- Mara owes the Salt Choir a debt.
- Greyhaven is suffering a fuel shortage.
- the Black Tide expansion is active.
- an artifact called the Bell of Glass is canonically missing.
- Mara’s brother is secretly employed by a rival faction.

Quest Forge proposes:

> **The Bell Beneath Greyhaven** — The Salt Choir will erase Mara’s debt if she recovers the Bell of Glass from a wreck exposed only during the week’s lowest tide. Greyhaven’s fuel guild is also searching for the wreck because its engine still contains a rare catalyst. Mara’s brother has already joined their expedition.

The user likes everything except the reward.

They lock:

- title
- objective
- brother complication
- location

They reroll only **Reward** with the instruction:

> Make the reward useful but morally uncomfortable.

The app proposes a legal pardon obtained by framing an innocent smuggler.

The user edits it, promotes the quest to canon, forges the wreck location, creates a visual brief, generates a music prompt for the scene, and exports the quest packet for a game session.

That experience is the target.

---

# 58. Definition of success

The application succeeds when users stop thinking:

> “I am using a random generator.”

…and start thinking:

> “I am building my world.”

The Platinum release is successful when someone can begin with a single generated character and, without leaving the application, grow that spark into a coherent, inspectable, exportable creative universe — while always remaining the author of what becomes canon.

---

# 59. What we build first — locked decision

When implementation begins, do **not** start with expansions, AI providers, image tools, simulation, or collaboration.

The first shippable target remains:

## Character Forge / Quest Forge v1.0 FOUNDATION

With exactly these pillars:

- local project
- Character Forge
- Quest Forge
- universe presets
- creative axes
- deterministic seeds
- field locks
- field rerolls
- manual edits
- history
- canon state
- Markdown/JSON export
- project backup
- tests
- Windows release artifact

If v1.0 feels fantastic, the rest of this bible has somewhere solid to live.

If v1.0 feels mediocre, no amount of later expansion content can rescue it.

---

# 60. The next artifact after this Bible

The next document should be the **GAZINGA CODEX BUILD PROMPT**.

It should use this World Bible as canonical requirements and instruct Codex to:

- create the repository
- implement v1.0 FOUNDATION only
- maintain architecture compatible with later roadmap stages
- work milestone-by-milestone
- run tests continuously
- produce release artifacts
- maintain an implementation journal
- refuse to claim completion until release gates pass

That prompt should be huge, explicit, test-driven, and written so Codex can execute the project rather than merely scaffold it.

**That is the next step.**
