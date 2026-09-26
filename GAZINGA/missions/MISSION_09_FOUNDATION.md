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

# MISSION 09 — FOUNDATION
## Packaging, smoke test, release certification

# Objective

Produce and independently verify the v1.0.0 FOUNDATION release artifacts.

Do not add features in this mission except fixes required to pass release gates.

# Version freeze

Set/verify:
- application version
- schema version
- generator version
- content version
- package metadata
- release notes

Confirm version compatibility expectations.

# Clean-room build

From a clean repository state/cache-appropriate environment:
- install dependencies;
- run formatting checks;
- run lint/typecheck;
- run Rust tests;
- run frontend tests;
- run integration/E2E;
- run mandatory seed/property gauntlets or verified canonical command;
- build release.

Capture exact commands and outcomes.

# Windows artifacts

Produce canonical release outputs, including the installer format(s) supported by the project and portable artifact if required by the canonical spec.

Generate checksums.

Do not call an artifact "portable" unless it was actually tested as such.

# Packaged-app smoke test

Smoke test the actual release build, not only dev mode.

Minimum workflow:
1. install/start release;
2. create project;
3. generate Character;
4. lock multiple fields;
5. reroll one unlocked field;
6. verify locks;
7. reroll unlocked;
8. manually edit;
9. favorite;
10. promote to canon;
11. create/revise candidate as defined;
12. generate Quest;
13. use alternate preset;
14. exercise Chaos Blend;
15. inspect history;
16. export JSON;
17. export Markdown;
18. create `.forgeproject`;
19. close application;
20. restart;
21. reopen project and verify state;
22. import/restore an exported project safely;
23. verify canon/history/DNA survived;
24. exit cleanly.

If any step fails, release is not PASS.

# Release documentation

Finalize:
- `README.md`
- `docs/USER_GUIDE.md`
- `docs/ARCHITECTURE.md`
- `docs/IMPLEMENTATION_JOURNAL.md`
- `docs/RELEASE_CHECKLIST.md`
- `CHANGELOG.md`
- release notes
- known issues

# Final certification

Create:
`docs/FOUNDATION_RELEASE_CERTIFICATION.md`

It must state:
- commit hash;
- version;
- artifact names;
- hashes;
- all verification commands;
- gate results;
- known P2/P3 issues;
- unsupported/out-of-scope features;
- whether release is PASS or FAIL.

# NO FALSE PASS

Release is FAIL/PARTIAL if:
- installer not tested;
- packaged app not launched;
- seed gates not run;
- lock invariant fails;
- data roundtrip fails;
- a known P0/P1 exists;
- artifacts are missing;
- tests were merely assumed from previous runs.

# EXIT GATE

PASS means **v1.0.0 FOUNDATION exists as a tested release artifact**.

After PASS:
STOP.

Do not begin v1.1 ANVIL.
