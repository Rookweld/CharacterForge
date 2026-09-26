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

# MISSION 04 — MEMORY
## SQLite persistence, history, canon, revisions

# Objective

Make creativity durable.

Implement the authoritative local project store and candidate/canon lifecycle without changing the deterministic generator semantics.

# Database

Implement versioned SQLite schema and migration framework for v1 requirements.

Canonical capabilities:
- projects
- entities
- entity payload/state
- candidate/canon status
- entity revisions
- Forge runs / provenance
- persistent generation history
- favorites
- project/app settings where appropriate
- schema metadata

Use transactions for multi-record semantic operations.

Define foreign keys and indexes intentionally.

# Project lifecycle

Implement:
- create project
- open project
- close/switch safely
- load entities/history
- persist candidate
- manual edits
- favorite toggle
- promote candidate to canon
- create explicit canon revision
- preserve prior canon revision
- recover session/state on normal restart

# Canon law

Generation MUST NOT silently mutate canon.

Promoting/replacing canon must be explicit.

If canon is revised:
- identity remains stable where appropriate;
- revision is persisted;
- previous revision remains inspectable.

# Undo/redo

Implement the canonical v1 session undo/redo boundary.

Do not confuse transient undo with persistent revision history.

# Migration safety

Provide:
- schema 0/new DB -> current
- at least one synthetic previous-version migration fixture
- pre-migration backup/recovery behavior
- migration transaction/failure tests
- no destructive success on failure

# SQLite hardening

Use appropriate:
- foreign keys
- WAL/checkpoint policy per spec
- busy handling
- integrity check in diagnostic/test tooling

# Tests

Required:
- CRUD/project lifecycle
- candidate -> canon
- canon revision preservation
- favorites/history
- restart/reopen roundtrip
- transaction rollback
- migration success
- migration failure preserves recoverability
- generated Forge DNA survives persistence roundtrip unchanged

# EXIT GATE

PASS only if closing/reopening a project cannot silently lose generated/canon work and migration failure has a credible recovery path.
