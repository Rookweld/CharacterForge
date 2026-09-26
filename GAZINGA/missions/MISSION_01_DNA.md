# EXECUTION CONTRACT

You are executing exactly one bounded GAZINGA mission.

Before coding:
1. Read `CANON_CORE.md`.
2. Read `PROJECT_STATE.md`.
3. Read this mission completely.
4. Read only the World Bible/spec sections mapped to this mission.
5. Inspect the existing repository and current git status.

During work:
- stay inside mission scope;
- preserve existing passing behavior;
- add tests with implementation;
- prefer explicit contracts to implicit coupling;
- document architecture decisions that materially affect later work;
- do not implement future-version features;
- do not hide failures;
- do not weaken tests to make them pass.

At the end:
- run every required verification command;
- create `docs/missions/MISSION_XX_REPORT.md`;
- update `PROJECT_STATE.md`;
- report PASS only if every exit gate is satisfied;
- STOP without starting the next mission.

# MISSION 01 — DNA
## Domain contracts + deterministic Forge DNA

# Objective

Build the pure, testable heart of FOUNDATION: canonical contracts and deterministic field generation infrastructure.

No polished UI. No persistence workflow.

# Deliver

## Canonical contracts
Implement strongly typed/versionable contracts for:
- project identity/envelope primitives
- entity identity
- candidate/canon state primitives
- Character
- Quest
- generation metadata
- universe/preset identifier
- creative axes
- field lock state
- field nonce/reroll state
- `GenerationRecipe`
- `ForgeDNA`
- generator/content/schema versions
- structured validation errors

Choose stable serialization rules.

## Forge DNA
Implement deterministic derivation using stable serialization and a stable PRNG/hash strategy.

Generation for field F must derive from explicit inputs including:
- root seed
- generator version
- content version
- preset/universe
- normalized creative axes/constraints
- field key
- field nonce
- declared dependencies

No accidental dependency on:
- hash map iteration order
- wall-clock time
- process ID
- random global singleton
- UI order
- unrelated field nonce

## Lock/Reroll semantics
Implement pure operations for:
- lock/unlock
- reroll a single field
- reroll all unlocked fields
- preserve locked fields
- explicit dependent-field invalidation only

A field's nonce may change only when semantically required.

## Tests
At minimum:
- same DNA reproduces same result;
- unrelated reroll does not mutate another field;
- locked field does not mutate;
- root seed change changes generation;
- field nonce change changes only intended stream;
- serialization roundtrip preserves DNA;
- ordering of map/object storage does not alter canonical seed derivation;
- Unicode seeds/inputs behave deterministically;
- empty/edge inputs are validated predictably.

Add property-style testing where appropriate.

# Explicitly prohibited

- final Character content pack
- final Quest content pack
- SQLite history/canon implementation
- UI beyond tiny developer harness if essential
- AI

# EXIT GATE

PASS only if Forge DNA and lock/reroll invariants are demonstrably stable under automated tests.

Treat any hidden cross-field mutation as P1.
