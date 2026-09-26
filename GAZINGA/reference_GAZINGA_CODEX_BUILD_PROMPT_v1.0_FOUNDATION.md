# GAZINGA CODEX BUILD PROMPT
## Character Forge / Quest Forge — v1.0 FOUNDATION

> **Mission class:** Autonomous production build
> **Primary platform:** Windows 11 x64
> **Target:** A real, shippable v1.0 FOUNDATION release — not a scaffold, demo, mockup, or prototype
> **Canonical product specification:** `Character_Forge_Quest_Forge_WORLD_BIBLE_Platinum_Roadmap.md`

---

# 0. READ THIS FIRST — YOUR ROLE

You are the autonomous principal engineer, product engineer, test engineer, release engineer, and technical writer responsible for taking **Character Forge / Quest Forge** from an empty or early repository to a **fully working v1.0 FOUNDATION release**.

You are not being asked to brainstorm.
You are not being asked to create a proof of concept.
You are not being asked to scaffold a future application and stop.
You are being asked to **build, run, test, package, verify, and document the first production release**.

The project has a long-term roadmap through a Platinum release, but your implementation target in this mission is **v1.0 FOUNDATION only**.

The World Bible is the product constitution. Read it in full before making architecture decisions.

## Canonical input

Locate and read:

`Character_Forge_Quest_Forge_WORLD_BIBLE_Platinum_Roadmap.md`

Search the current workspace/repository if necessary. If it is external to the repository but accessible, copy it unchanged into:

`docs/WORLD_BIBLE.md`

Preserve the original file as canonical source material.

If the World Bible is genuinely unavailable, **stop before implementing product behavior** and report the missing canonical input. Do not invent a substitute product spec.

---

# 1. AUTHORITY ORDER

When instructions appear to conflict, use this priority order:

1. Data safety and integrity
2. The World Bible's explicit product rules and v1.0 scope
3. This GAZINGA build prompt
4. Existing tested repository behavior, if compatible with 1–3
5. Reasonable engineering judgment

Never silently reinterpret a product rule to make implementation easier.

If a technical constraint forces a deviation, document it in an ADR under `docs/adr/`, explain the tradeoff, implement the smallest compatible deviation, and include it in the final report.

---

# 2. OPERATING MODE — AUTONOMOUS, NOT CHAOTIC

Work autonomously through the entire mission.

Do **not** stop after each phase to ask for permission.
Do **not** ask cosmetic preference questions when the Bible already defines direction.
Do **not** wait for the user to tell you to run tests, fix failures, or package the app.

You should only interrupt for user input when one of these is true:

- a destructive action would affect unrelated user data;
- a credential, certificate, or external secret is required and cannot be avoided;
- the canonical World Bible cannot be found;
- the environment lacks a mandatory system dependency that you cannot install or work around safely;
- a genuine product ambiguity would materially change persistent data compatibility.

Otherwise, make the best defensible decision, record it, and continue.

## No false completion

Never write `DONE`, `COMPLETE`, `SHIPPED`, `PASS`, or equivalent for the whole release unless every mandatory release gate has actually been verified.

If something is blocked, say exactly what is blocked, what is complete, and what remains. A partial success is acceptable. A false green release is not.

---

# 3. THE PRODUCT IN ONE SENTENCE

**Character Forge / Quest Forge is a local-first creative construction system that lets users generate structured characters and quests, lock the parts they love, reroll only what they do not, edit freely, preserve generation history, promote results into canon, and export or back up their work — with deterministic generation and no required network or AI service.**

The core promise is:

> **Generate something cool. Keep what you love. Reroll only what you do not. Promote it into canon. Grow it into a world.**

v1.0 must make the first half of that promise feel excellent.

---

# 4. NON-NEGOTIABLE PRODUCT PRINCIPLES

These rules apply to every implementation decision.

## 4.1 Structure first, prose second

The offline engine creates valid structured entities. Prose is a presentation of structured state, not the source of truth.

## 4.2 AI is not required in v1.0

v1.0 must work completely offline after installation.

Do not add cloud AI, local LLM integration, provider settings, API key UX, embeddings, or prompt-provider infrastructure in this release.

Future architecture must remain possible, but future features must not be implemented prematurely.

## 4.3 Canon is sacred

Random generation or reroll operations may never silently overwrite canon.

Any mutation of a canon entity must be explicit and revision-safe.

## 4.4 Determinism matters

Seeds, generator version, recipe version, universe preset, active content version, creative axes, and field reroll state must be sufficient to explain/reproduce deterministic generation under the same versioned engine/content.

## 4.5 Lock-and-reroll is the heart of the product

Locked fields must never change during field reroll or `reroll unlocked`.

This invariant is release-critical.

## 4.6 Local-first and data-safe

Projects live locally.
Backups are user-controlled.
There is no telemetry by default.
There is no account requirement.
There is no network dependency.

## 4.7 Chaos may surprise creatively, never structurally

Chaos Blend may break genre expectations, but may never break schemas, IDs, persistence, migrations, exports, or lock invariants.

---

# 5. EXACT v1.0 FOUNDATION SCOPE

Build exactly these pillars to release quality.

## 5.1 Project system

The user can:

- create a project;
- name it;
- open an existing local project;
- see recent/local projects;
- close and reopen the app without losing work;
- manually create a portable project backup;
- import a valid project backup;
- receive clear errors for invalid/corrupt/incompatible imports;
- preserve data through migration 0 → 1.

Projects should have stable UUIDs and created/updated timestamps.

## 5.2 Character Forge v1

A generated Character contains, at minimum:

- `name`
- `title`
- `archetype`
- `role`
- `appearance_summary`
- `personality`
- `desire`
- `fear`
- `flaw`
- `secret`
- `signature_gear`
- `faction_hint`
- `story_hooks[3]`

No required field may be empty after successful generation.

## 5.3 Quest Forge v1

A generated Quest contains, at minimum:

- `title`
- `quest_archetype`
- `premise`
- `patron`
- `objective`
- `stakes`
- `antagonist_or_opposition`
- `complications[2]`
- `twist`
- `reward`
- `failure_consequence`
- `follow_up_hook`

No required field may be empty after successful generation.

## 5.4 Shared Forge features

Both Character Forge and Quest Forge must support:

- universe preset selection;
- Chaos Blend;
- creative axes;
- explicit deterministic seed;
- Forge action;
- per-field lock/unlock;
- reroll a single field;
- reroll all unlocked fields;
- manual field editing;
- in-session undo;
- in-session redo;
- persistent generation history;
- favorite/star;
- candidate/canon state;
- Markdown export;
- JSON export;
- local SQLite persistence;
- portable `.forgeproject` backup;
- settings;
- dark signature theme;
- light-theme-compatible design tokens.

## 5.5 Universe presets in v1.0

Ship these built-in presets:

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
- Chaos Blend

These are **core presets**, not expansion packs.

Do not implement the later expansion-specific mechanics such as The Price, Crew Tension, Heat, Oaths, Scarcity Ledger, Prototype Faults, or The Hunt Loop in v1.0.

Chaos Blend is a built-in deterministic mixed recipe. Do not build a general expansion marketplace or user pack editor.

## 5.6 Creative axes

Expose these axes as understandable controls:

- Tone: Hopeful ↔ Bleak
- Reality: Grounded ↔ Mythic
- Order: Ordered ↔ Chaotic
- Scale: Intimate ↔ Epic
- Morality: Clear ↔ Ambiguous
- Strangeness: Familiar ↔ Surreal
- Technology: Primitive ↔ Advanced
- Mysticism: None ↔ Dominant

Use normalized integer values such as `0..100` in persisted recipes.

The axes must materially affect weighted generation. They are not decorative sliders.

---

# 6. EXPLICITLY OUT OF SCOPE FOR v1.0

Do not implement any of these during this mission:

- image generation;
- image provider integration;
- LLM provider integration;
- Ollama integration;
- cloud sync;
- accounts;
- collaboration;
- multiplayer;
- world maps;
- geographic simulation;
- living-world simulation;
- deep campaign timelines;
- semantic/vector search;
- embedding databases;
- community marketplace;
- downloadable community packs;
- executable plugins;
- Creator SDK;
- Scene Forge;
- music prompt cards;
- visual prompt cards;
- Asset Vault;
- campaign mode;
- World Forge;
- expansion-specific signature mechanics.

Do not create fake buttons for these features.
Do not add `Coming Soon` clutter to the main UI.
Do not implement backend stubs and call them architecture.

Future compatibility means **clean contracts and boundaries**, not speculative code.

---

# 7. REQUIRED TECHNOLOGY BASELINE

Use the World Bible baseline unless the existing repository contains a demonstrably better, already-working equivalent.

## Desktop

- Tauri 2
- Windows 11 x64 as primary release target

## Frontend

- React
- TypeScript
- Vite
- CSS variables/design tokens
- keyboard-friendly accessible components

## Backend/core

- Rust stable
- serde / serde_json
- UUIDs
- versioned domain contracts

## Storage

- SQLite
- WAL where appropriate
- atomic transactions
- explicit migrations

Prefer `rusqlite` with the bundled SQLite feature unless the existing repo has a proven alternative.

## Deterministic generation

Use a deterministic seeded RNG suitable for stable repeatable results, e.g. `rand_chacha::ChaCha8Rng`.

For deriving per-field RNG streams, use a stable hash such as BLAKE3 rather than language-runtime hashers whose output may change.

## Frontend testing

- Vitest
- React Testing Library

## Rust testing

- cargo test
- deterministic/golden fixtures
- property/invariant tests where practical

## E2E / smoke

Use the most reliable Windows-compatible method available in the environment.

Prefer real Tauri E2E with `tauri-driver`/WebDriver when feasible. If the environment cannot support that, still create browser-level workflow tests with mocked Tauri boundaries **and** perform/package a real native smoke test where possible.

Do not call the native release gate passed if the packaged application was not actually launched and smoke-tested.

---

# 8. REPOSITORY STRATEGY

If the current workspace is empty, initialize the project here rather than creating arbitrary nested repositories.

If a repository already exists, inspect it before changing architecture. Preserve useful working code if compatible with the Bible.

Use a clean monorepo separation close to:

```text
character-quest-forge/
├─ apps/
│  └─ desktop/
│     ├─ src/
│     └─ src-tauri/
├─ packages/
│  ├─ domain/
│  ├─ schemas/
│  └─ ui/                 # only if actually useful in v1
├─ crates/
│  ├─ app-core/
│  ├─ storage/
│  └─ export/
├─ packs/
│  └─ core/
├─ docs/
│  ├─ adr/
│  ├─ WORLD_BIBLE.md
│  ├─ IMPLEMENTATION_JOURNAL.md
│  ├─ RELEASE_CHECKLIST.md
│  └─ USER_GUIDE.md
├─ scripts/
├─ tests/
│  ├─ golden/
│  ├─ migrations/
│  └─ e2e/
├─ package.json
├─ Cargo.toml
├─ README.md
└─ CHANGELOG.md
```

Do not create empty future packages such as prompt compiler, provider SDK, secure-store, Creator SDK, or pack marketplace code unless v1.0 genuinely needs them.

Use npm workspaces unless the repo already standardizes on another package manager.

Use a Cargo workspace for Rust crates.

---

# 9. DOMAIN CONTRACTS

Create explicit versioned domain contracts before building UI.

## 9.1 Shared entity envelope

Every persisted creative entity must contain conceptually:

```text
id: UUID
project_id: UUID
type: character | quest
name: string
schema_version: integer
canon_state: draft | candidate | canon | archived
origin: manual | generated | imported
favorite: boolean
created_at: timestamp
updated_at: timestamp
tags: string[]
notes: string | null
generation_provenance: ForgeDNA | null
```

Generated entities should normally begin as `candidate`.

## 9.2 Character

Model every required Character Forge field explicitly.

Do not store the entire character as one untyped prose blob.

## 9.3 Quest

Model every required Quest Forge field explicitly.

Do not store the entire quest as one untyped prose blob.

## 9.4 GenerationRecipe

At minimum:

```text
generator_kind: character | quest
recipe_version
generator_version
root_seed
universe_preset
creative_axes
field_nonces
core_content_version
created_at
```

## 9.5 ForgeDNA / provenance

Persist enough information to explain and reproduce the original deterministic generation:

```text
root_seed
recipe_version
generator_version
universe_preset
creative_axes
field_nonces
core_content_version
selected_content_ids where practical
origin_action: forge | reroll_field | reroll_unlocked | manual_edit | restore_history
parent_generation_id where relevant
```

Manual edits should not lie about being generated. Record edit provenance separately or in history.

---

# 10. DETERMINISTIC GENERATION DESIGN

This is a release-critical subsystem.

## 10.1 Root seed format

Persist seeds as stable strings, preferably a fixed 64-bit hexadecimal representation, e.g.:

`8f3c2b179d4e6a01`

Avoid exposing raw JS `Number` for arbitrary 64-bit seed storage.

The UI should provide:

- visible seed;
- copy seed;
- randomize seed;
- manual seed entry with validation.

## 10.2 Per-field deterministic streams

A single field reroll must not perturb other fields.

Implement deterministic stream derivation conceptually as:

```text
field_seed = HASH(
  root_seed |
  generator_version |
  recipe_version |
  core_content_version |
  universe_preset |
  field_key |
  field_nonce
)
```

Seed a field-local RNG from that hash.

Each generated field has a nonce/revision counter beginning at `0`.

### Reroll one field

- increment only that field's nonce;
- regenerate only that field;
- never mutate locked fields;
- never mutate unrelated unlocked fields;
- append generation history.

### Reroll unlocked

- for each unlocked generated field, increment that field's nonce;
- regenerate those fields;
- locked fields remain byte-for-byte semantically unchanged;
- append one grouped history event.

This design is strongly preferred because it makes lock behavior deterministic, testable, and explainable.

## 10.3 Derived/composite fields

If a field depends on several other fields, explicitly document dependencies.

Do not let rerolling `fear` silently rewrite `name` or `appearance`.

For v1.0, prefer field generators with limited dependencies and use context only where it materially improves coherence.

## 10.4 Coherence without hidden mutation

When generating dependent fields, read current locked/current state as context but write only the requested target field.

Example:

- `story_hooks` may consider role, desire, fear, secret, faction hint;
- rerolling `story_hooks` may change only `story_hooks`.

---

# 11. FORGE ENGINE PIPELINE

Implement a concrete, testable pipeline inspired by the Bible:

1. validate/normalize forge input;
2. resolve universe preset;
3. resolve creative-axis weight modifiers;
4. derive deterministic field RNG streams;
5. generate structured skeleton;
6. apply coherence rules;
7. validate required fields and invariants;
8. attach ForgeDNA/provenance;
9. return result;
10. persist candidate/history transactionally when requested.

There is no model/provider stage in v1.0.

The offline engine is not a fallback. It is the v1 product engine.

---

# 12. CONTENT SYSTEM — ORIGINAL, VERSIONED, CURATED

Put built-in content under `packs/core/` with an explicit version, e.g. `1.0.0`.

Content should be data-driven enough that generator logic is not littered with hard-coded strings.

A sensible structure might include:

```text
packs/core/
├─ manifest.json
├─ shared/
├─ dark-fantasy/
├─ pirate/
├─ steampunk/
├─ cyberpunk/
├─ nordic-myth/
├─ post-apocalyptic/
├─ occult/
├─ monster-hunter/
├─ rogue-assassin/
└─ contemporary-weird/
```

Do not implement third-party pack loading in v1.0.

## 12.1 Content quality

Do not fill tables with repetitive synonyms.

Use compositional semantic families and weighted tables.

Prefer original genre vocabulary. Do not copy protected character names, settings, factions, or signature terms from commercial franchises.

## 12.2 Minimum useful variety

Each universe should have enough curated material that ordinary short sessions do not immediately repeat obvious phrases.

As a quality target, provide meaningful content families for:

### Character

- name components/patterns;
- titles;
- archetypes;
- roles;
- physical descriptors;
- personality traits;
- desires;
- fears;
- flaws;
- secrets;
- gear;
- faction styles/hints;
- hook structures.

### Quest

- quest archetypes;
- patrons;
- objectives;
- stakes;
- opposition;
- complications;
- twists;
- rewards;
- failure consequences;
- follow-up hooks;
- premise templates.

Do not chase arbitrary gigantic table sizes at the expense of quality, but avoid tiny 5-item lists that make repetition obvious.

## 12.3 Creative axes must affect generation

Implement weight modifiers so axes change likely content families.

Examples:

- high Bleak increases tragic costs, desperate stakes, compromised patrons;
- high Mythic increases legendary motifs and supernatural framing;
- high Chaos increases volatile factions/complications and unusual combinations;
- high Epic increases scope and stakes;
- high Ambiguity increases morally mixed motives/rewards;
- high Surreal increases unusual imagery/logic while preserving schema validity;
- high Technology shifts materials, roles, opposition, and gear;
- high Mysticism increases ritual/supernatural vocabulary.

Do not make every axis affect every table equally.

Document the weighting strategy.

## 12.4 Chaos Blend

Chaos Blend should deterministically blend vocabulary/weights from multiple core universes.

A good v1 strategy:

- deterministically choose 2–4 source universes from the root seed;
- apply a dominant universe plus one or more secondary influence weights;
- persist the chosen blend composition in ForgeDNA;
- preserve coherent nouns/materials rather than concatenating random genre words.

Chaos Blend must still pass all schema and required-field tests.

---

# 13. DATABASE DESIGN

SQLite is the source of truth.

Prefer one project database per local project directory.

A project directory may conceptually contain:

```text
<ProjectRoot>/
├─ project.sqlite3
├─ project.json          # optional lightweight metadata if useful
└─ backups/
```

The database should include tables conceptually covering:

- project metadata;
- entities;
- entity revisions;
- forge/generation history;
- settings as appropriate;
- migration metadata.

A practical normalized/hybrid design is acceptable: stable envelope columns plus JSON payloads for versioned entity bodies/provenance, provided schema validation remains explicit and queryable metadata is not hidden unnecessarily.

## 13.1 Suggested tables

### projects

- id
- name
- schema_version
- created_at
- updated_at

### entities

- id
- project_id
- entity_type
- name
- canon_state
- origin
- favorite
- schema_version
- body_json
- generation_json nullable
- created_at
- updated_at

### entity_revisions

Immutable historical snapshots:

- revision_id
- entity_id
- revision_number
- reason/action
- body_json
- generation_json
- created_at

### forge_runs

Persistent generation history:

- run_id
- project_id
- generator_kind
- entity_id nullable
- action
- parent_run_id nullable
- recipe_json
- result_json
- created_at

### app/project settings

Use a clear versioned key/value model only where appropriate.

## 13.2 Transaction rules

Any operation that updates an entity and history/revisions must be transactional.

Never persist a half-updated entity with missing history.

## 13.3 WAL and checkpoints

Use WAL when suitable for reliability/responsiveness.

Checkpoint deliberately before creating a portable snapshot.

## 13.4 Migration system

Implement schema versioning from day one.

Create migration `001` and a migration runner.

Test:

- fresh database → v1;
- schema 0 fixture → v1;
- migration failure does not destroy original database;
- pre-migration backup is created for existing user DBs.

Never silently reset a database after a migration error.

---

# 14. REVISION, HISTORY, UNDO, AND CANON RULES

These concepts are related but not identical.

## 14.1 Session undo/redo

Undo/redo applies to the current forge editing session.

Operations that should push session state include:

- forge result replacement;
- field reroll;
- reroll unlocked;
- manual edit;
- history restore into current session.

Redo stack clears after a new branch mutation.

## 14.2 Persistent generation history

Every Forge/reroll action gets a persistent generation-history record.

History must survive app restart.

The UI should let the user inspect prior generated snapshots and restore one into the current editing session.

Restoring history must create a new action/revision when saved rather than silently rewriting history.

## 14.3 Canon

Generated results default to `candidate`.

The user can promote a candidate to `canon` explicitly.

If a canon entity is edited/rerolled, do not silently overwrite it without a revision.

At minimum:

- create a revision snapshot before mutation;
- show a clear confirmation or explicit edit flow for canon mutation;
- retain the ability to inspect the previous revision.

## 14.4 Favorites

Favorite/star is independent of canon state.

A weird draft can be a favorite without being canon.

---

# 15. PORTABLE PROJECT FORMAT — `.forgeproject`

Implement the v1 portable project backup as a versioned archive.

A valid archive should contain at minimum:

```text
manifest.json
project.sqlite3
pack-lock.json
integrity.json
```

Optional directories such as `assets/` may be absent in v1.

## 15.1 Manifest

Include:

- format name;
- archive format version;
- application version;
- project id;
- project name;
- project schema version;
- created/exported timestamp.

## 15.2 Pack lock

Record core content pack ID/version.

## 15.3 Integrity metadata

Store hashes for included files.

Use a stable cryptographic hash such as BLAKE3 or SHA-256.

## 15.4 Export safety

Create archives through temporary paths and atomic rename where possible.

On interrupted export, never replace a previously valid backup with a corrupt partial file.

## 15.5 Import safety

Before extraction:

- reject absolute paths;
- reject `..` traversal;
- reject duplicate dangerous paths;
- reject unsupported format versions cleanly;
- verify integrity metadata;
- inspect database compatibility;
- import into a new project directory before registering it as valid.

Never import over an existing project destructively without an explicit safe workflow.

## 15.6 Unknown future fields

An older app must not silently rewrite and delete unknown future-version data.

If an archive/project version is newer than supported, open/import must fail read-safely with a clear message rather than attempting lossy downgrade.

---

# 16. ENTITY EXPORTS

## 16.1 JSON

Export a single selected entity with:

- versioned envelope;
- structured entity body;
- canon/favorite metadata;
- provenance where appropriate.

Output must be valid UTF-8 JSON and roundtrip through validation.

## 16.2 Markdown

Export human-readable Markdown with:

- title/name;
- metadata summary;
- each field under stable headings;
- story hooks/complications as lists;
- seed/provenance summary in a compact footer when generated.

Sanitize/escape content so imported/manual text cannot inject active HTML into unsafe preview contexts.

---

# 17. TAURI COMMAND BOUNDARY

Keep the frontend thin regarding persistence and filesystem operations.

The Rust side should own:

- project filesystem paths;
- SQLite connections;
- migrations;
- backup/import/export filesystem work;
- deterministic generator execution;
- archive validation;
- canonical persistence operations.

Expose small typed Tauri commands such as conceptually:

```text
list_projects
create_project
open_project
close_project
forge_character
forge_quest
reroll_field
reroll_unlocked
save_entity
set_canon_state
set_favorite
list_history
load_history_snapshot
export_entity_markdown
export_entity_json
backup_project
import_project_backup
get_settings
update_settings
```

The exact names may differ.

Do not expose a generic `execute_sql` or arbitrary filesystem command to the frontend.

---

# 18. UI / UX REQUIREMENTS

The application must feel like a focused creative instrument, not an admin panel.

## 18.1 Visual identity

Follow the Bible:

**forged material + manuscript precision + modern tool clarity**.

Avoid:

- generic SaaS purple gradients;
- full fake parchment;
- fake hacker terminal styling;
- excessive animated particles;
- illegible charcoal-on-black text.

Use:

- dark graphite foundation;
- restrained warm ember/metal accent;
- cool accent only where useful;
- layered card/plate feel;
- strong typography hierarchy;
- subtle texture, if any;
- crisp focus states.

Use system/local fonts only unless an asset is explicitly bundled legally.

## 18.2 Welcome / Project Picker

Must support:

- New Project
- Open recent/local project
- Import `.forgeproject`
- sample/demo project entry if practical

Keep it fast and obvious.

## 18.3 Main Forge workspace

A strong layout is:

### Header

- product/project identity;
- current project name;
- save/state indicator;
- backup/export/settings access.

### Left control rail

- Character Forge / Quest Forge switch;
- universe preset;
- creative axes;
- seed controls;
- Forge / Reroll Unlocked actions.

### Main result area

Structured field cards/rows.

Every meaningful field visibly exposes:

- label;
- content;
- lock state;
- reroll field action;
- edit action/state.

Do not hide lock behavior in a context menu.

### Secondary/history area

- favorite;
- canon state;
- generation metadata;
- persistent history;
- undo/redo;
- export actions.

Responsive behavior on normal laptop widths is required. No need for mobile support.

## 18.4 Lock interaction

The lock icon must be immediately understandable.

Locked state cannot be represented by color alone.

Include tooltip/accessible label.

## 18.5 Reroll scope

Before or during reroll actions, it must be visually clear what will change.

`Reroll unlocked` should communicate the number or scope of fields being regenerated.

## 18.6 Editing

Manual edits should be frictionless.

Use inline edit or a focused edit mode.

Do not force a modal for every short field.

Manual edits should survive subsequent rerolls when their field is locked. If unlocked and rerolled, they may be intentionally replaced after visible user action.

## 18.7 Seed visibility

Seeds are a feature, not debug noise.

Make the current seed discoverable and copyable without dominating the screen.

## 18.8 Empty/error/loading states

Create intentional states for:

- no project selected;
- project with no generated entities;
- invalid seed;
- generation validation failure;
- DB error;
- backup/export failure;
- import incompatibility;
- migration failure.

No blank screens.
No raw Rust panic text shown to the user.

---

# 19. ACCESSIBILITY

v1.0 does not need the entire future Platinum accessibility pass, but it must establish a good baseline:

- semantic buttons/forms;
- keyboard navigation for primary flows;
- visible focus states;
- usable contrast;
- labels for icons;
- no state conveyed only through color;
- scalable layout for Windows display scaling;
- reduced-motion-friendly transitions;
- tooltips for lock/reroll/seed concepts.

Treat a dark theme as an accessibility challenge, not an excuse.

---

# 20. SETTINGS

Keep v1 settings small and real.

Suggested settings:

- theme: dark / light / system;
- reduced motion: on/off/system where practical;
- optional sound effects: on/off, default off or restrained;
- default universe preset;
- default creative-axis values;
- backup retention count if automatic backup rotation is implemented.

Do not add AI/provider settings in v1.0.

---

# 21. DATA SAFETY REQUIREMENTS

Creative work may represent hundreds of hours. Treat data loss as release-blocking.

Implement or verify:

- atomic transactional writes;
- WAL where appropriate;
- migration backup;
- manual portable backup;
- local backup rotation if implemented;
- database integrity check on backup/import path;
- archive integrity hashes;
- safe temp-file exports;
- explicit failure messages;
- no silent reset/recreate after corruption;
- no destructive overwrite on import.

P1/P2 data-loss bugs block release.

---

# 22. SECURITY BASELINE

Even offline software needs sane boundaries.

Requirements:

- validate archive paths against traversal;
- sanitize Markdown/HTML preview surfaces;
- do not execute content-pack data;
- no arbitrary code execution from imported content;
- no telemetry unless explicitly added in a future version;
- no secrets in logs;
- no hidden network calls;
- Tauri capability configuration should grant only required access;
- filesystem actions should be owned by Rust/Tauri command boundaries.

Run dependency/security checks available to the toolchain and record results.

---

# 23. LOGGING AND DIAGNOSTICS

Use structured application logging.

At minimum log:

- app start/version;
- project open/close;
- migration start/result;
- forge action metadata excluding private prose where possible;
- export/backup/import start/result;
- unexpected errors with useful context.

Do not dump entire private project content into logs by default.

Provide a documented log location.

If inexpensive, add a user-facing `Open logs folder` or diagnostics action, but do not let this derail core v1.

---

# 24. PERFORMANCE BUDGETS

Targets from the Bible:

- shell usable in about 2–3 seconds on a typical modern Windows machine;
- offline generation under 100 ms for ordinary Character/Quest generation on typical hardware;
- lock/reroll UI response feels immediate;
- no spinner for deterministic work that completes nearly instantly;
- list rendering remains responsive as project history grows.

Measure the generator in tests/benchmarks enough to detect egregious regressions.

Do not prematurely micro-optimize before correctness.

---

# 25. TESTING — RELEASE CRITICAL

Tests are not a cleanup phase. Build them alongside features.

## 25.1 Rust unit tests

Cover:

- seed parsing;
- deterministic RNG derivation;
- weighted selection;
- zero/invalid weight handling;
- every Character field generator;
- every Quest field generator;
- axis weighting;
- Chaos Blend composition;
- required-field validation;
- schema/entity validation;
- field locks;
- single-field reroll;
- reroll unlocked;
- canon mutation revision behavior;
- migration logic;
- backup archive validation;
- path traversal rejection;
- integrity hash verification;
- JSON/Markdown export.

## 25.2 Mandatory 1,000-seed tests

Create deterministic automated tests that execute at least:

- 1,000 Character Forge runs;
- 1,000 Quest Forge runs.

Across the run set, cover every core universe preset and Chaos Blend.

Every run must:

- return success;
- satisfy schema;
- contain all required fields;
- contain valid stable IDs;
- contain valid provenance.

These tests are part of the v1 release gate.

## 25.3 Lock invariant property tests

Test extensively that:

> locked fields never change.

Cases:

- one lock + reroll field elsewhere;
- many locks + reroll unlocked;
- all fields locked + reroll unlocked;
- manual edit then lock then reroll unlocked;
- canon entity mutation flow;
- repeated 100+ rerolls.

A failure here is a P1 release blocker.

## 25.4 Golden seeds

Create a curated set of known seeds for each generator and several presets.

Persist expected structured output fixtures.

Golden tests must include generator/content version in fixture naming so intentional future version changes can be managed rather than silently breaking reproducibility.

## 25.5 Migration tests

At minimum:

- fresh DB → schema 1;
- schema 0 fixture → schema 1;
- populated schema 0 fixture → schema 1 preserving data;
- migration failure path preserves original/pre-migration backup.

## 25.6 Backup/import roundtrip

Automated test:

1. create project;
2. generate entities/history;
3. favorite/canonize/edit some data;
4. create `.forgeproject`;
5. import into a new temporary location;
6. verify semantic equality of all supported persisted data.

## 25.7 Frontend component tests

Cover:

- lock button state;
- reroll scope;
- field editing;
- undo/redo;
- history rendering;
- canon/favorite controls;
- seed editing;
- preset/axis controls;
- error/empty states.

## 25.8 Workflow/E2E tests

Automate as much of this as environment permits:

1. launch;
2. create project;
3. Forge Character;
4. lock two fields;
5. reroll unlocked;
6. verify locks unchanged;
7. edit a field;
8. canonize;
9. close/reopen project;
10. verify persistence;
11. export JSON and Markdown;
12. back up project;
13. import backup;
14. verify restored data;
15. repeat essential flow for Quest Forge.

## 25.9 Failure tests

At minimum where safely automatable:

- invalid seed;
- malformed archive;
- path traversal archive;
- corrupt integrity hash;
- incompatible future project/archive version;
- unwritable export target;
- migration failure fixture;
- corrupted DB copy used only in test temp data.

Never intentionally corrupt a user's real project during testing.

---

# 26. VISUAL/QUALITY TEST PASS

After functionality is working, perform a visual QA pass at common Windows sizes/scales.

Check at least:

- 1920×1080 at 100%;
- 2560×1440 at 125% if environment permits;
- normal laptop-width window;
- resized narrower desktop window.

Verify:

- no clipped field controls;
- long generated text wraps;
- lock/reroll buttons remain reachable;
- history does not destroy layout;
- focus states visible;
- dark theme readable;
- light token mode does not become unusable.

Fix obvious visual defects before release packaging.

---

# 27. RELEASE ARTIFACTS

For Windows v1.0, attempt to produce:

- Tauri NSIS installer `.exe`;
- Tauri MSI if the environment supports the required packaging toolchain;
- portable ZIP containing the release executable and minimal README;
- SHA-256 or BLAKE3 checksum file;
- release notes;
- sample project or demo project;
- user guide.

Do not claim an MSI exists if the packaging prerequisites are unavailable.

Do not invent a code-signing certificate. If artifacts are unsigned, state that clearly in release notes.

The application itself must not require admin rights for ordinary use unless Tauri packaging imposes an installer-specific requirement.

---

# 28. VERSIONING

Set product release version to:

`1.0.0`

Use semver.

The following must agree where applicable:

- frontend package version;
- Tauri package version;
- Rust crate workspace version strategy;
- archive application version metadata;
- About/settings display;
- release notes;
- changelog.

Keep separate explicit versions for:

- database schema (`1`);
- entity schema(s);
- generator (`1.x` or `1`);
- recipe (`1`);
- core content pack (`1.0.0`);
- `.forgeproject` format (`1`).

Do not conflate these version numbers.

---

# 29. BUILD / TEST COMMAND EXPERIENCE

A fresh developer checkout should have obvious root commands.

Create scripts equivalent to:

```text
npm install
npm run dev
npm run typecheck
npm run lint
npm run test
npm run test:frontend
npm run test:e2e       # if supported
npm run tauri:dev
npm run tauri:build
npm run release:verify
```

And document Rust-specific commands such as:

```text
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

The exact script names may vary slightly, but the root developer experience must be coherent.

`release:verify` should orchestrate as many non-destructive release checks as practical.

---

# 30. DOCUMENTATION THAT MUST EXIST BY RELEASE

Create and maintain:

## `README.md`

- what the product is;
- screenshot placeholder only if a real screenshot is not yet captured — do not fake product screenshots;
- development prerequisites;
- dev commands;
- build commands;
- project structure;
- license status if known, otherwise do not invent one.

## `docs/USER_GUIDE.md`

Explain:

- projects;
- Character Forge;
- Quest Forge;
- universe presets;
- creative axes;
- seeds;
- locks;
- reroll field;
- reroll unlocked;
- manual edits;
- undo/redo;
- history;
- candidate vs canon;
- favorites;
- export;
- project backup/import;
- data location;
- unsigned installer warning if applicable.

## `docs/IMPLEMENTATION_JOURNAL.md`

Maintain during work, not after.

For each milestone record:

- date/time;
- goal;
- files/areas changed;
- architecture decisions;
- tests run;
- results;
- discovered issues;
- fixes;
- remaining risk.

## `docs/RELEASE_CHECKLIST.md`

Use checkboxes with evidence/commands/results.

## `docs/adr/*.md`

Write ADRs only for meaningful decisions, e.g.:

- per-project DB strategy;
- field-local seed derivation;
- archive format;
- migration strategy.

Do not produce dozens of meaningless ADRs.

## `CHANGELOG.md`

Create a proper v1.0.0 entry describing actual shipped features.

## Release notes

Create `docs/releases/v1.0.0.md` describing actual release status and known limitations.

---

# 31. IMPLEMENTATION JOURNAL RULE

Start the journal **before** coding.

After every major milestone, update it.

Do not fabricate test results from memory at the end.

When a command fails, record the failure and resolution if it materially affected the implementation.

This journal is intended to let another engineer understand how v1.0 was built and why.

---

# 32. GIT / CHANGE HYGIENE

If Git is available:

- inspect current status before work;
- never delete unrelated user changes;
- do not reset/checkout away work you did not create;
- use logical commits if autonomous commit permissions/workflow allow it;
- otherwise maintain clean working changes and a detailed journal.

Never use destructive Git commands merely to make the workspace clean.

---

# 33. PHASED EXECUTION PLAN

You must follow this order unless the existing repository already contains verified later-phase work.

Every phase must end in a buildable/testable state.

---

## PHASE 0 — PREFLIGHT AND BASELINE

### Goals

Understand environment and repository before changing anything.

### Actions

1. Find/read the World Bible in full.
2. Inspect repository tree.
3. Inspect Git status.
4. Detect Node/npm version.
5. Detect Rust/Cargo version.
6. Detect Windows/Tauri build prerequisites.
7. Run any existing tests/build.
8. Record baseline in implementation journal.
9. Copy World Bible into `docs/WORLD_BIBLE.md` if needed/present externally.
10. Create/confirm `.gitignore` for build outputs, local DBs, logs, temp archives.

### Exit gate

- Canonical spec read.
- Existing code understood.
- Environment blockers known.
- Baseline recorded.

Do not spend hours upgrading unrelated dependencies in preflight.

---

## PHASE A — CONTRACTS BEFORE UI

### Goals

Lock down v1 domain, schemas, versions, migration baseline.

### Implement

1. workspace structure;
2. shared version constants;
3. entity envelope;
4. Character type;
5. Quest type;
6. CreativeAxes;
7. UniversePreset enum/IDs;
8. GenerationRecipe;
9. ForgeDNA;
10. validation functions;
11. JSON schema/docs where appropriate;
12. SQLite migration runner;
13. migration 001;
14. schema 0 migration fixture/test.

### Tests

- serialization roundtrip;
- required field validation;
- enum/version validation;
- migration fresh 0 → 1;
- populated migration fixture preservation.

### Exit gate

`cargo test --workspace` passes for implemented contracts/storage bootstrap.
Frontend typecheck/build still works if present.

Update journal.

---

## PHASE B — OFFLINE FORGE ENGINE

### Goals

Build the actual creative engine before polishing UI.

### Implement in order

1. root seed parser/generator;
2. stable hash-based field seed derivation;
3. deterministic RNG helper;
4. weighted table abstraction;
5. core content manifest/versioning;
6. shared content families;
7. all 10 named presets;
8. creative-axis weighting;
9. deterministic Chaos Blend;
10. Character generators for every required field;
11. Quest generators for every required field;
12. coherence/context helpers;
13. field lock model;
14. single-field reroll;
15. reroll unlocked;
16. ForgeDNA/provenance;
17. validation pipeline;
18. golden fixtures.

### Content authoring expectation

Do not leave `[TODO]` tables.
Do not ship placeholder lorem ipsum.
Do not use franchise-specific copied IP.

The output should already be fun to read from CLI/unit tests before the UI exists.

### Required tests before leaving Phase B

- same recipe/seed/version → same result;
- different field nonce changes target field deterministically;
- target-field reroll changes only target field;
- locked fields never change;
- reroll unlocked changes only unlocked fields;
- all-locked reroll is a safe no-op with clear result;
- creative axes measurably influence weighted selection;
- Chaos Blend composition persisted;
- 1,000 Character runs pass;
- 1,000 Quest runs pass;
- golden seed suite passes.

### Quality spot check

Print/log a curated sample matrix for internal development review:

- at least 2 characters per universe;
- at least 2 quests per universe;
- several Chaos Blend examples;
- extreme axis combinations.

Use this to fix obvious repetition, grammar defects, incoherent templates, accidental duplicate words, and bad article/plural handling.

### Exit gate

The offline engine is independently useful and release-gate tests pass.

Update journal.

---

## PHASE C — PERSISTENCE AND DATA SAFETY

### Goals

Make creative work durable and revision-safe.

### Implement

1. app data/project path strategy;
2. project repository;
3. entity repository;
4. revision repository;
5. forge history repository;
6. favorite/canon persistence;
7. transactional mutation flows;
8. WAL/checkpoint behavior;
9. pre-migration backup;
10. manual project backup foundation;
11. integrity helpers;
12. sample project fixture.

### Tests

- create/open project;
- save/reload Character;
- save/reload Quest;
- history survives restart/reopen simulation;
- favorite/canon survives reload;
- canon edit creates revision;
- transaction rollback on injected failure;
- DB integrity check;
- migration failure preserves original/backup.

### Exit gate

No known path exists where a normal failed operation leaves a half-written creative entity.

Update journal.

---

## PHASE D — DESKTOP SHELL AND CORE UX

### Goals

Expose the core engine as a polished desktop workflow.

### Implement in vertical slices

#### D1 — App shell/project picker

- Tauri 2 app boots;
- project picker;
- new project;
- open project;
- recent projects;
- project header;
- global error boundary/state.

#### D2 — Character Forge

- preset;
- axes;
- seed;
- Forge;
- full field display;
- locks;
- field reroll;
- reroll unlocked;
- inline editing;
- undo/redo;
- favorite;
- candidate/canon;
- persistent history.

Do not move on until Character Forge feels coherent end-to-end.

#### D3 — Quest Forge

Reuse shared components/behavior without copy-paste divergence.

Support every Quest field and identical lock/reroll/edit/history guarantees.

#### D4 — Theme/settings/polish baseline

- dark signature theme;
- light/system token capability;
- accessible focus/labels;
- reduced-motion baseline;
- settings persistence;
- intentional empty/error states.

### Frontend tests

Build interaction tests as features land.

### Exit gate

A user can create a project and use both killer loops without touching a terminal.

Update journal.

---

## PHASE E — EXPORT, BACKUP, IMPORT

### Goals

Make work portable and recoverable.

### Implement

1. entity JSON export;
2. entity Markdown export;
3. `.forgeproject` archive creation;
4. manifest;
5. pack lock;
6. integrity hashes;
7. safe temp-write/atomic finalize;
8. archive path validation;
9. integrity validation;
10. compatible import into new project location;
11. future-version rejection;
12. roundtrip tests.

### UI

Add clear export/backup/import actions without cluttering the Forge loop.

### Exit gate

Automated backup/import roundtrip passes with non-trivial project state.

Update journal.

---

## PHASE F — HARDENING, QA, PACKAGING

### Goals

Turn working software into release software.

### Actions

1. run formatter/linter/typecheck/clippy;
2. eliminate warnings where practical;
3. full unit/integration suite;
4. 1,000 + 1,000 seed gate;
5. lock invariant stress tests;
6. migration suite;
7. import/export roundtrip suite;
8. frontend tests;
9. E2E/workflow tests;
10. visual QA;
11. keyboard/accessibility smoke;
12. release build;
13. launch packaged app;
14. fresh-project smoke;
15. reopen persistence smoke;
16. installer smoke if feasible;
17. portable build smoke;
18. generate checksums;
19. finalize README/user guide;
20. finalize changelog/release notes;
21. complete release checklist with evidence.

### Exit gate

All mandatory v1.0 release gates are green, or remaining blockers are explicitly reported and release is not falsely declared complete.

---

# 34. REQUIRED v1.0 RELEASE GATES

The release is blocked unless all of these are true:

## Build/install

- clean checkout dependencies install successfully;
- production frontend builds;
- Rust workspace builds;
- Tauri production build succeeds;
- a packaged Windows artifact exists;
- packaged application launches on the build machine;
- fresh install/smoke is performed where environment permits.

## Core project flow

- create project works;
- open project works;
- close/reopen retains data;
- invalid project errors are visible and non-destructive.

## Character Forge

- all required fields generated;
- 1,000 seeded runs without schema failure;
- locks/rerolls/edit/history/canon/favorite work.

## Quest Forge

- all required fields generated;
- 1,000 seeded runs without schema failure;
- locks/rerolls/edit/history/canon/favorite work.

## Determinism

- golden seeds pass;
- same seed/recipe/content version reproduces same result;
- field-local reroll does not perturb unrelated fields.

## Locks

- locked fields never mutate in automated stress tests;
- all-locked reroll is safe.

## Persistence/migrations

- schema 0 → 1 migration test exists and passes;
- pre-migration backup path verified;
- no known P1/P2 data-loss bug.

## Backup/import

- `.forgeproject` archive generated;
- integrity metadata verified;
- traversal protection tested;
- export/import roundtrip retains all supported data.

## Offline

- app launches and core workflows work with network unavailable;
- no API key required;
- no hidden network dependency.

## Documentation

- README accurate;
- USER_GUIDE accurate;
- IMPLEMENTATION_JOURNAL current;
- RELEASE_CHECKLIST contains actual evidence;
- CHANGELOG contains actual shipped behavior;
- release notes list real limitations.

If any required item is not proven, mark it `BLOCKED` or `FAIL`, not assumed `PASS`.

---

# 35. QUALITY BAR — WHAT “GOOD” MEANS

A release can technically pass tests and still feel mediocre. Do not stop there.

The following qualitative checks matter:

## Character output

A Character should feel like an actionable story seed, not a bag of adjectives.

Good:

- role, desire, fear, flaw, secret, gear, faction hint, and hooks reinforce or interestingly contradict each other;
- the title feels earned;
- appearance is concrete enough to visualize;
- hooks suggest action.

Bad:

- generic `mysterious warrior` filler;
- repeated `ancient`, `shadow`, `forgotten` everywhere;
- every secret is betrayal;
- every flaw is distrust;
- hooks merely restate fields.

## Quest output

A Quest should provide a situation that can immediately generate a scene/session/story.

Good:

- clear objective;
- meaningful stakes;
- specific opposition;
- complications change how the objective is approached;
- twist recontextualizes rather than randomizes;
- reward has flavor;
- failure creates future consequences;
- follow-up hook opens another door.

Bad:

- `retrieve artifact from dangerous place` with no specificity;
- twist unrelated to premise;
- reward identical across genres;
- complication is just another enemy.

## UI

The lock/reroll loop should be understandable without reading a manual.

If the user has to wonder what will change, improve the UI.

---

# 36. TIPS / ENGINEERING LESSONS TO APPLY FROM DAY ONE

Carry these lessons from the World Bible into implementation.

## 36.1 The best random generator is not maximally random

Weighted semantic families produce stronger results than uniform giant lists.

## 36.2 Controlled contradiction is useful

Interesting combinations can come from tension, e.g.:

- merciful assassin;
- hopeful plague doctor;
- orderly pirate mutineer;
- mythic cybernetic pilgrim.

But contradiction must remain interpretable.

## 36.3 Novelty pressure

Within one generation, avoid repeating the same key noun/adjective family excessively.

Optionally track recently selected semantic family IDs during a single result and reduce duplicate weights.

Do not add global non-deterministic novelty memory that breaks seed reproducibility.

## 36.4 Concrete nouns beat vague mood words

Prefer:

`brass lung`, `salt-stained warrant`, `glass compass`, `bone tally`, `copper prayer wheel`

over:

`mysterious item`, `dark object`, `powerful artifact`.

## 36.5 Do not regenerate more than requested

This is both a UX rule and architecture rule.

## 36.6 Normalize immediately

Generator results become validated domain structs before persistence/UI.

## 36.7 SQLite until it actually hurts

Do not add Redis, Postgres, vector DB, Elasticsearch, or cloud services.

## 36.8 Recovery before collaboration

Backups/import integrity matter before any sync dream.

---

# 37. DO NOT LET YOURSELF WANDER OFF

The following rules are mandatory:

1. Build vertical slices; every milestone compiles/runs.
2. Never stub a critical path and declare it finished.
3. Run relevant tests after each milestone.
4. Keep schema/versioning explicit.
5. Preserve a clean migration path.
6. No cloud infrastructure in v1.
7. No AI substitution for deterministic core generation.
8. No vector DB.
9. No hidden network dependency.
10. No speculative expansion mechanics.
11. No giant dependency additions without concrete need.
12. No silent data loss.
13. Maintain implementation journal throughout.
14. Any incomplete feature is visible in docs and excluded from release claims.
15. Prefer simple reliable implementations over abstract frameworks built for imaginary future needs.
16. Fix test failures before piling more features on top.
17. Do not rewrite working architecture merely because another library is fashionable.
18. Do not spend the mission redesigning the World Bible.

---

# 38. DEFECT PRIORITY

Use this severity model:

## P1 — release blocker

- data loss/corruption;
- locked field mutation;
- cannot open project;
- broken migration;
- broken backup/restore;
- security path traversal/code execution;
- packaged app does not launch;
- deterministic generation materially non-deterministic under fixed versions.

## P2 — release blocker for v1 unless explicitly waived

- core Forge workflow broken;
- export missing/corrupt;
- history/canon persistence broken;
- major accessibility/input blocker;
- frequent crash/panic;
- one core universe systematically broken.

## P3

- non-critical visual defect;
- awkward wording;
- rare edge-case inconvenience;
- documentation polish.

Resolve all known P1/P2 defects before claiming v1.0 shipped.

---

# 39. FAILURE HANDLING DURING AUTONOMOUS WORK

When a step fails:

1. read the actual error;
2. identify root cause;
3. fix the smallest root cause;
4. rerun the failing command/test;
5. rerun nearby regression tests;
6. record meaningful failures/fixes in the journal;
7. continue.

Do not repeatedly apply random dependency changes hoping the error disappears.

If a system dependency blocks packaging and cannot be installed safely:

- complete all code/test work possible;
- document exact dependency and command/error;
- mark the packaging gate blocked;
- do not call the release fully complete.

---

# 40. FINAL RELEASE VERIFICATION RUN

At the end, perform a clean verification sequence from repository root.

Adapt command names to the implemented scripts, but cover all of these:

```text
npm ci  # or equivalent clean install where package lock supports it
npm run typecheck
npm run lint
npm run test
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
npm run release:verify
npm run tauri:build
```

Then inspect produced artifacts rather than trusting command exit alone.

Launch the packaged application.

Perform the manual/native smoke flow:

1. launch app;
2. create `Smoke Test World`;
3. forge Character with known seed;
4. lock Name and Secret;
5. reroll unlocked;
6. verify Name and Secret unchanged;
7. edit Role;
8. undo;
9. redo;
10. favorite Character;
11. canonize Character;
12. forge Quest;
13. lock Objective;
14. reroll Twist;
15. export Quest JSON;
16. export Quest Markdown;
17. close app;
18. reopen app;
19. reopen project;
20. verify both entities/history/canon/favorite persisted;
21. create `.forgeproject` backup;
22. import backup into a new project copy;
23. verify imported data;
24. close/relaunch once more.

Record this smoke result in `docs/RELEASE_CHECKLIST.md`.

---

# 41. FINAL RESPONSE FORMAT

When you have completed as much as the environment allows, provide one concise but evidence-rich final engineering report with these sections:

## Release status

One of:

- `PASS — v1.0 FOUNDATION release gates verified`
- `PARTIAL — implementation complete, release blocked by ...`
- `FAIL — core release blockers remain`

Do not use PASS unless true.

## What was built

Summarize real implemented systems.

## Repository / architecture

Give the important paths and architecture decisions.

## Tests

List commands and actual pass/fail counts where available.

Explicitly mention:

- Character 1,000-seed result;
- Quest 1,000-seed result;
- lock invariant result;
- migration result;
- backup/import roundtrip result.

## Release artifacts

List exact artifact paths and file names actually produced.

Include checksums if generated.

## Known limitations

List real limitations only.

## Deviations from Bible

List any, with ADR references.

## Next recommended action

Keep this limited to the immediate post-v1 next step. Do not begin implementing v1.1 during this mission.

---

# 42. DEFINITION OF DONE

This mission is complete only when Character Forge / Quest Forge v1.0 FOUNDATION is demonstrably a real desktop application with:

- reliable local projects;
- a strong Character Forge;
- a strong Quest Forge;
- all core universe presets;
- meaningful creative axes;
- deterministic seeds;
- robust field locks;
- targeted rerolls;
- manual editing;
- session undo/redo;
- persistent history;
- favorites;
- candidate/canon workflow;
- JSON/Markdown export;
- portable project backup/import;
- migration infrastructure;
- automated tests;
- Windows packaging;
- accurate docs;
- no known P1/P2 data-loss bug.

A scaffold is not done.
A pretty mockup is not done.
Passing frontend tests while Rust is broken is not done.
A debug build without backup/import is not done.
A release artifact that was never launched is not done.

The user should be able to install/open it, forge something interesting, lock what they love, reroll the rest, canonize it, close the application, come back later, and trust that their world is still there.

That is v1.0 FOUNDATION.

---

# 43. START NOW

Begin with Phase 0.

Read the entire World Bible.
Inspect the repository and environment.
Create the implementation journal.
Establish the baseline.
Then execute Phases A through F without waiting for routine approvals.

Do not build v1.1.
Do not build Platinum.
Do not wander into cloud AI.

**Build the foundation so well that the Platinum roadmap has something worthy to stand on.**

## FORGE. BUILD. VERIFY. SHIP.
