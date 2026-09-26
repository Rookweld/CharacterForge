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

# MISSION 08 — ARMOR
## Release hardening, diagnostics, accessibility, performance

# Objective

Remove "developer prototype" behavior before packaging.

# Error handling

Audit:
- Rust panics reachable from user input
- frontend unhandled rejections
- raw database errors shown to users
- filesystem errors
- invalid project state
- corrupt settings
- missing/unwritable paths

Provide actionable user messages and diagnostic detail separately.

# Logging/diagnostics

Implement canonical diagnostic facilities:
- application version
- schema version
- generator/content versions
- platform/runtime information
- log location
- safe recent diagnostics export if in canonical scope
- no secrets/private project contents dumped unnecessarily

# Performance

Measure and enforce reasonable budgets for:
- cold/warm startup
- project open
- Character generate
- Quest generate
- single field reroll
- history list
- save/canon operation
- export

Optimize actual bottlenecks only.

# Accessibility audit

Verify:
- keyboard-only core loop
- focus visibility/order
- accessible control names
- lock/canon state not color-only
- dialogs focus/trap/escape behavior
- readable scaling
- reduced motion where applicable

# Data-safety failure injection

Where practical simulate:
- disk write failure
- DB busy/locked
- interrupted export
- corrupt settings
- unavailable backup path

Ensure failures do not create false success or destroy good state.

# Dependency/security hygiene

- audit dependencies;
- remove unused high-risk packages;
- ensure no debug endpoints/devtools are unintentionally production-exposed;
- verify CSP/Tauri permissions follow least privilege;
- verify file dialogs/paths are scoped.

# EXIT GATE

PASS only if there are no known P0/P1 defects and the app behaves like a release candidate rather than a dev harness.
