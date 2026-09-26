# CANON CORE
## The non-negotiable context carried into every GAZINGA mission

This file is intentionally short. It is the "always-loaded" project constitution.

# PRODUCT

**Character Forge / Quest Forge** is a local-first creative desktop application.

Core promise:

> Generate something cool. Keep what you love. Reroll only what you don't. Promote it into canon. Grow it into a world.

v1.0 FOUNDATION proves that loop with Characters and Quests.

# PRODUCT PRINCIPLES

1. **Generate, then shape.**
   Generation begins the creative process; it does not replace user authorship.

2. **Structure first, prose second.**
   Canonical entities are structured data. Prose is a representation of that data.

3. **AI is optional, not foundational.**
   v1.0 must be fully useful without an external model provider.

4. **Canon is sacred.**
   Candidate generation must never silently overwrite canon.

5. **Determinism matters.**
   Same compatible Forge DNA + versions must reproduce the same generated state.

6. **Lock-and-reroll is the heart of the product.**
   Locked fields must remain byte/semantic stable when unrelated fields reroll.

7. **Local-first and data-safe.**
   User worlds live locally and must survive crashes, updates, migrations and imports.

8. **Chaos may surprise creatively, never structurally.**
   Chaos Blend can combine themes, not corrupt contracts or identity.

# v1.0 FOUNDATION SCOPE

## Character Forge
Structured generation for:
- name
- title
- archetype
- appearance/look
- faction
- signature gear
- ability/talent
- flaw
- fear
- secret
- motivation
- relationship hook
- backstory hook
- story/image prompt helpers where defined by contracts

## Quest Forge
Structured generation for:
- title
- region/location
- quest giver
- objective
- stakes
- complication/twist
- antagonist/threat
- required clue or condition
- artifact/reward
- consequence
- moral pressure
- mission briefing / concise presentation fields

## Shared
- projects
- universe presets
- creative axes
- root seed
- deterministic field streams
- locks
- single-field reroll
- reroll-unlocked
- manual edit
- undo/redo within defined session boundary
- persistent generation history
- favorites
- candidate/canon workflow
- revision history
- JSON/Markdown export
- project backup/import
- settings
- diagnostics
- release packaging

# v1.0 UNIVERSE PRESETS

Core presets must include the canonical v1 set from the World Bible and monolithic spec, including Chaos Blend.

No expansion-only mechanics are implemented in FOUNDATION.

# FORGE DNA

Never use one mutable global RNG stream for the whole entity.

A field's generation stream must derive from stable inputs such as:

- root seed
- generator version
- content version
- universe/preset identity
- normalized creative axes/constraints
- field key
- field nonce
- relevant explicit dependencies

Rerolling one field increments only the nonce/salt state required for that field and its explicitly declared dependents.

Locked fields do not mutate.

# CANON WORKFLOW

Generation creates a **candidate**.

User may:
- lock fields;
- reroll fields;
- edit fields;
- favorite;
- promote candidate to canon;
- revise canon explicitly.

Generation never silently mutates canon.

# STORAGE

SQLite is the authoritative project store for v1.0.

Migrations must be versioned and tested.

Data-changing migration/import operations require recoverability.

# PORTABLE PROJECT

Canonical portable project extension:

`.forgeproject`

It must contain sufficient project data and metadata for safe restore/import, including integrity information and pack/version locking as defined by the build specification.

# SECURITY / SAFETY

- No arbitrary code execution from project/content files.
- Reject path traversal in archives.
- Validate schemas.
- Bound archive extraction.
- Use atomic/recoverable file operations.
- Preserve original data on failed import/migration.

# QUALITY

Generic filler is a defect.

The content system should produce concrete, varied results rather than endless permutations of:
- mysterious warrior
- ancient artifact
- dangerous enemy
- dark secret

Creative axes must observably affect output.

# RELEASE TRUTH

"Works on my machine" is not a release gate.

Claims require evidence:
- tests
- artifacts
- logs
- smoke execution

No false completion.
