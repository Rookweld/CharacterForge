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

# MISSION 00 — IGNITION
## Preflight, repository bootstrap, architecture lock

# Objective

Turn an empty or unknown repository into a reproducible, documented v1.0 FOUNDATION workspace without prematurely implementing product features.

# Deliver

## Preflight
Record detected versions/capabilities for:
- Git
- Node/npm or chosen canonical package manager
- Rust/cargo
- Tauri prerequisites
- Windows target toolchain
- SQLite approach
- test runners

Do not pretend missing prerequisites exist. If a hard prerequisite is unavailable, create everything safely possible and mark the mission BLOCKED with exact remediation.

## Repository skeleton
Create the canonical project structure for:
- Tauri desktop shell
- frontend
- Rust application/core modules
- persistence
- generator/content
- shared contracts
- tests
- docs
- scripts
- release artifacts directory conventions

Avoid giant "utils" modules and premature plugin systems.

## Engineering baseline
Create/configure:
- formatter
- linter
- TypeScript strict mode
- Rust clippy policy
- frontend unit test runner
- Rust test baseline
- reproducible build scripts
- `.gitignore`
- environment/example config only where needed
- CI workflow if local repo policy supports it

## Documentation
Create:
- `README.md` skeleton
- `docs/ARCHITECTURE.md`
- `docs/IMPLEMENTATION_JOURNAL.md`
- `docs/RELEASE_CHECKLIST.md`
- `docs/adr/0001-foundation-stack.md`
- `docs/missions/`

Architecture doc must show dependency direction and boundaries.

## Architecture lock
Document the v1 boundaries:
- UI must not directly own persistence.
- deterministic generator core must not depend on Tauri UI.
- persistence must not know visual components.
- content data should be versionable.
- future provider/expansion systems must have architectural seams but no runtime implementation.

# Explicitly prohibited

- Character generation feature implementation
- Quest generation feature implementation
- production database schema beyond minimal bootstrap if required
- AI integrations
- World Forge
- packs/marketplace
- speculative microservices

# Verification

Required:
- dependency install succeeds where prerequisites exist;
- frontend typecheck baseline succeeds;
- Rust compile/test baseline succeeds;
- Tauri development shell can compile or exact blocker is documented;
- test command aliases work;
- git diff contains no generated secrets/binaries.

# EXIT GATE

PASS only if:
- repository shape is coherent and documented;
- baseline build/test commands are executable;
- architecture boundaries are written;
- no later mission has been accidentally implemented;
- PROJECT_STATE and mission report are complete.
