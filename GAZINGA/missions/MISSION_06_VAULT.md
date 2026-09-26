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

# MISSION 06 — VAULT
## Export, import, backup, `.forgeproject`, security

# Objective

Make project data portable without making it fragile or dangerous.

# Entity export

Implement canonical:
- JSON export
- Markdown export

Exports must clearly distinguish structured/canonical values and useful human-readable presentation.

# `.forgeproject`

Implement the portable project archive per canonical format, including:
- manifest
- format/schema version
- project identity
- database snapshot/data payload as defined
- pack/content lock/version metadata
- integrity hashes/checks
- forward-compatibility treatment of unknown metadata where safe

# Export safety

- snapshot from a consistent database state;
- write to temporary target first;
- verify where applicable;
- atomic/finalize behavior;
- never leave a successful-looking truncated archive.

# Import safety

Before mutating active user state:
- validate archive structure;
- validate manifest/schema;
- verify integrity metadata;
- reject unsafe paths;
- reject archive bombs according to sane count/size bounds;
- do not overwrite active project implicitly;
- stage import;
- preserve source archive;
- fail closed.

Test:
- `../` path traversal
- absolute paths
- duplicate conflicting entries
- missing manifest
- corrupt DB/payload
- hash mismatch
- unsupported future major format
- oversized/unreasonable archive metadata

# Backup

Implement explicit project backup flow and the automatic backup behaviors required before risky migration/import operations.

Test restore.

# UI wiring

Complete export/import/backup flows in the desktop UI with comprehensible errors and success confirmation.

# EXIT GATE

PASS only if a project can roundtrip through export/import with canon/history/Forge DNA intact and malicious/corrupt archives cannot damage arbitrary filesystem/project state.
