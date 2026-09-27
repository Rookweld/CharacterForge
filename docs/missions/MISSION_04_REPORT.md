# Mission 04 — MEMORY

## Mission

Mission 04 — MEMORY: SQLite persistence, history, canon, and revisions.

## Status

**PASS — verified 2026-09-27.** The on-disk generated-candidate and canon close/reopen gates pass; schema migration failures preserve both the original database and its recovery copy.

## Scope delivered

- Implemented schema version 1 and the `forge-storage` Rust persistence boundary using the approved `rusqlite` 0.40.2 dependency with only `bundled` enabled and rusqlite defaults disabled.
- Added fresh schema-0 initialization and a populated synthetic legacy schema-0 to schema-1 migration. Existing databases are checkpointed and copied to `.pre-migration.bak` before migration; schema changes run in a transaction. Migration failure leaves the original fixture and backup readable. Unknown unversioned schemas, corrupt databases/payloads, and future schema versions return structured errors rather than resetting data.
- Added one-project-per-database lifecycle APIs, safe active-project switching/close, project rename with stale-write detection, entity listing, project-local JSON settings, parameterized SQL, foreign-key enforcement, WAL, `synchronous=FULL`, a five-second busy timeout, deliberate auto-checkpointing, and integrity/foreign-key diagnostics.
- Added candidate persistence and edits, favorites, durable generation history, explicit candidate-to-canon promotion, canon revision snapshots, and optimistic entity write versions. Repeated identical deterministic Forge IDs are retained as separate history events. Failed entity/history transactions roll back together.
- Persisted complete Character and Quest candidate snapshots, including canonical ForgeDNA, generation provenance, manual-edit field origins, locks/nonces, and Quest v2/v3-specific metadata. Real close/reopen tests verify Character DNA/history and current canon/revision snapshots.
- Added generic transient session undo/redo; it is intentionally separate from persistent ForgeRuns and canon revisions.

## Files materially changed

- `Cargo.toml`, `Cargo.lock`, `crates/storage/Cargo.toml`
- `crates/storage/src/lib.rs`, `crates/storage/src/tests.rs`
- `docs/ARCHITECTURE.md`, `docs/adr/0003-sqlite-project-persistence.md`
- `GAZINGA/PROJECT_STATE.md`

## Commands executed

- `cargo test -p forge-storage --offline` — 15 passed, 0 failed.
- `cargo clippy -p forge-storage --all-targets --offline -- -D warnings` — passed.
- `cargo metadata --no-deps --format-version 1` — passed; all five workspace packages resolved.
- `& .\scripts\verify.ps1` — passed, including frontend format/lint/typecheck/test/build, Rust formatting, workspace clippy, workspace tests, and optimized no-bundle Tauri build.
- `git diff --check` — passed.
- `cargo tree -e features -p rusqlite --offline` — confirmed default features are disabled; only the approved bundled SQLite feature is requested.

## Verification evidence

- Workspace tests: **77 passed, 0 failed** (52 `forge-app-core`, 10 `forge-contracts`, 15 `forge-storage`; other workspace and doc-test targets had 0 tests).
- Storage coverage includes CRUD/project isolation and rename, candidate history, favorites, settings, repeated runs, session undo/redo, safe project switching, candidate promotion, canon revision preservation and close/reopen, Character DNA and lock roundtrip, Quest v2/v3 metadata, stale writes, injected transactional rollback, fresh schema initialization, populated migration and backup, migration failure recovery, future-schema rejection, corrupt database/payload errors, and SQLite integrity/foreign-key checks.
- Frontend suite in the verification script: **1 passed, 0 failed**. No JavaScript files were changed.
- Tauri optimized no-bundle build passed and produced `target/release/character-forge-desktop.exe`.
- No generator, ForgeDNA contract/stream, nonce, lock/reroll, Quest, Character, frontend, or Tauri command behavior was changed.

## Architecture decisions

- A `ProjectStore` represents one on-disk project database. `ProjectRepository` and `EntityRepository` are Rust application-facing interfaces implemented by that store. The generator remains independent of SQLite and Tauri.
- SQLite schema version is tracked with `PRAGMA user_version`; the current version is 1. A populated synthetic previous schema is represented by `legacy_projects` at version 0.
- Stable envelope columns are queryable beside full validated JSON payloads. Each entity update compares a monotonic write version. Canon promotion and canon revision are separate explicit operations; each current canon state is retained as an immutable revision snapshot.
- Migration copies are limited to recoverability before schema changes. Portable `.forgeproject` backup/import remains outside Mission 04.

## Known defects

None found within Mission 04 scope.

## Deferred items

- User-facing project picker and Forge/history/canon controls remain for the separately authorized application UI mission.
- Portable project backup/import, AI/provider runtime, and later missions remain deferred. They are not required to implement this Mission 04 persistence boundary.

## Repository state

- Branch: `master`.
- Last committed baseline: `7a864ca8fcf64917859bb667e636c3a1825c0f54` (clean Mission 03 PASS closeout before implementation).
- Worktree: dirty only from Mission 04 implementation, tests, and documentation; no commit was created.

## Next mission readiness

**NO — stop after Mission 04.** Mission 04 is PASS. Mission 05 has not started and requires separate authorization.
