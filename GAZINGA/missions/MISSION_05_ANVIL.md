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

# MISSION 05 — ANVIL
## Desktop UX + Tauri integration

# Objective

Turn the proven core into the actual usable desktop application.

This is the first mission where the full user loop must exist visually.

# Required screens/flows

## Welcome / Project Picker
- create project
- open existing
- recent projects where canonical
- clear failure states

## Main Forge workspace
Provide:
- Character / Quest mode
- universe preset selector
- creative axis controls
- seed visibility/edit/regenerate affordance
- generate action
- structured result presentation
- obvious lock control per rerollable field
- single-field reroll
- reroll unlocked
- manual edit
- favorite
- history access
- promote to canon / revision workflow
- project save state indication
- export entry points (may be wired fully in Mission 06)

# UX rules

**LOCKED MEANS VISIBLY LOCKED.**

The UI must show reroll scope before/while acting.

Users should never have to guess:
- which field changed;
- whether a field is locked;
- whether they are viewing candidate or canon;
- which seed is active;
- whether an edit is persisted.

# Tauri boundary

Expose narrow typed commands.

No giant "execute arbitrary SQL" command.
No arbitrary filesystem read/write API from the frontend.
No business logic duplicated in React/TS that already belongs in Rust.

# Accessibility baseline

Implement:
- keyboard navigation for core workflow
- proper labels/roles
- visible focus
- reasonable contrast
- no color-only lock/canon status
- reduced-motion respect if animation exists

# Error/loading/empty states

Build actual states for:
- no project
- no generation yet
- validation error
- DB error
- import/export unavailable placeholder as appropriate
- operation in progress

No raw panic dumps in normal UI.

# E2E workflow

Automate or documentably exercise:
create project -> Character generate -> lock -> reroll -> edit -> favorite -> canon -> Quest generate -> history -> close -> reopen -> verify persisted state.

# EXIT GATE

PASS only if a human can complete the core FOUNDATION loop entirely through the packaged/dev desktop UI without touching CLI or database files.
