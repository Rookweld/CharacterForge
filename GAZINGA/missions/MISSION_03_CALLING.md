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

# MISSION 03 — CALLING
## Quest Forge vertical slice

# Objective

Deliver the complete deterministic Quest Forge domain experience, reusing shared systems rather than cloning Character Forge architecture.

# Quest fields

Implement the canonical v1 Quest schema for:
- title
- region/location
- quest giver
- objective
- stakes
- complication/twist
- antagonist/threat
- required clue/condition
- artifact/reward
- consequence
- moral pressure
- concise mission briefing/presentation fields
- all other v1 fields defined by canonical contracts

# Content/coherence

Build universe-aware, versioned Quest content families.

A good Quest result should create actionable story pressure:
- somebody wants something;
- something blocks it;
- failure matters;
- the twist changes interpretation/action;
- reward/consequence fits the premise.

Avoid vague "go to dangerous place and retrieve mysterious object" repetition.

Creative axes must matter.

Chaos Blend must surprise thematically while remaining internally usable.

# Shared engine

Reuse Forge DNA, constraints, content helpers and lock semantics from Missions 01–02.

Do not create a second RNG architecture.

# Optional Character linkage

If canonical v1 contracts support referencing a Character/entity, implement only the minimal structured reference required by FOUNDATION.

Do not implement World simulation or campaign chaining.

# Tests

Required:
- golden Quest seeds;
- every preset;
- extreme axes;
- Chaos Blend;
- field lock/reroll invariants;
- structural validation;
- repetition/quality heuristics;
- deterministic output.

# EXIT GATE

PASS only if Character Forge and Quest Forge coexist on the same deterministic foundation with no duplicated incompatible generation systems.
