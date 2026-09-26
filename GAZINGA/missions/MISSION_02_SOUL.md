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

# MISSION 02 — SOUL
## Character Forge vertical slice

# Objective

Deliver a complete, deterministic Character Forge domain experience from inputs to structured candidate output.

Use Mission 01's contracts. Do not bypass them.

# Character fields

Implement the canonical v1 Character schema, including the required structured identity, appearance, faction, gear, ability, flaw/fear/secret, motivations and hooks defined by the canonical spec.

# Content system

Create versioned, original, curated content families for every v1 core universe preset.

Requirements:
- concrete nouns over vague filler;
- weighted families rather than one giant flat list;
- enough variation to survive seed sweeps;
- semantic families for coherent combinations;
- controlled contrast/contradiction where useful;
- deterministic selection;
- no copied franchise characters/lore;
- Chaos Blend combines compatible thematic ingredients without changing schema.

Creative axes must observably influence selection/distribution.

# Generation pipeline

Implement Character generation stages needed for FOUNDATION:
1. normalize inputs;
2. derive deterministic skeleton;
3. select content under constraints;
4. compose derived fields;
5. validate structure/coherence;
6. return candidate + Forge DNA/provenance.

No external AI call.

# Reroll behavior

Support domain-level:
- single-field reroll;
- reroll unlocked;
- locks;
- manual edit representation without destroying provenance;
- explicit treatment of composite/dependent fields.

# Developer demonstration

Provide a CLI/test fixture/dev harness capable of generating:
- a chosen seed;
- a chosen universe;
- chosen axes;
- output as structured JSON.

This is not the final UI.

# Quality tests

Include:
- golden seeds across presets;
- extreme creative-axis combinations;
- Chaos Blend cases;
- no empty required fields;
- grammar/template sanity checks where practical;
- duplicate/repetition heuristics;
- lock invariants during Character rerolls.

# EXIT GATE

PASS only if Character Forge can generate reproducible, varied, structurally valid candidates across all v1 presets and reroll behavior respects Forge DNA.
