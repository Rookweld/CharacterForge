# THE FORGE A — SUBSTRATE REPORT

**Status: PASS — schema-2 compatibility foundation, first-class Relationship persistence, and R1 revision-history parity verified**

## Scope delivered

- Advanced the SQLite database schema from 1 to 2. Fresh projects are schema 2; schema-1 projects receive a no-clobber sibling recovery copy, WAL checkpoint, transactional mechanical `entities` table rebuild, empty Relationship tables, row-count and foreign-key checks, and a normal schema-2 reopen/domain validation.
- The project-row schema field now reports the database schema version. Entity payload schema, generator version, content version, ForgeDNA, and forgeproject archive format were not bumped.
- Expanded `EntityType` serialization to `character`, `quest`, `location`, `faction`, `item`, and `creature`. `StoredEntity` deliberately remains limited to validated Character/Quest payloads; no placeholder payloads, runtime rows, or new generators were added.
- Added UUID-backed Relationship identity and typed relationship/state contracts. The app-owned directed registry contains only `born_in` (Character→Location), `member_of` (Character→Faction), `owns` (Character→Item), and `opposed_by` (Quest→Creature).
- Added project-scoped Relationship create/get/list/endpoint-list, Candidate promotion, explicit Canon revision, non-destructive archive, immutable revision snapshots, optimistic write-version checks, duplicate assertion rejection, same-project foreign keys, and endpoint-type validation. No Relationship rows are inferred from Character link hints.
- R1 closeout corrected explicit Canon revisions to snapshot the accepted submitted state: promotion A creates R1=A, revisions A→B and B→C create R2=B and R3=C. Prior snapshots remain immutable; stale writes do not append or change current state. Close/reopen preserves A/B/C and current C. Archive behavior remains unchanged.
- Kept the `.forgeproject` container at format version 1. Its manifest reports the stored DB schema version; imports of schema-1 manifests may upgrade through the supported migration path.
- Updated Biome excludes for generated `target` and Tauri `gen` directories. Existing isolated Cargo target output had caused Biome to inspect generated fingerprint JSON ignored by Git.

## Frozen schema-1 fixture and migration evidence

`crates/storage/tests/fixtures/schema1-populated.sqlite3` is a frozen populated schema-1 database. It was seeded through the repository’s actual project/entity repositories with fixed Character/Quest seeds, then restored to the audited schema-1 DDL and `user_version=1`. It contains a locked Character Candidate with manual field origin, favorites, tags/notes and Unicode; Character and Quest Canon revision chains; a Quest v3 Candidate with clue/briefing metadata and a locked objective; forge-run history; and persisted settings. The one-time ignored fixture writer is `tests::write_frozen_schema1_fixture_once`; migration tests only copy the fixture to temporary paths.

The migration test compares all project/entity/revision/forge-run/settings row values before and after (excluding only the project DB-schema metadata field), verifies that metadata and `PRAGMA user_version` become 2, confirms the new Relationship tables are empty, and reopens with clean `integrity_check` and `foreign_key_check`. Deterministic Character/Quest replay signatures from stored DNA match across migration; Character Name and Quest Objective locks remain stable through unrelated field rerolls.

After migration, explicit compatibility replay signatures also cover Character content v1 and current v2 plus Quest content v2 and current v3. The frozen fixture SHA-256 is `4A259117B5D872430C1050CDC12F4E0AE0145A7668B2B29B3E0FD3319B84DD0E`.

An injected table-name collision fails the schema transaction after backup creation. The original remains `user_version=1`, its legacy table rows match the before snapshot, SQLite integrity remains `ok`, and the pre-migration recovery copy remains present. No partial schema-2 state is committed.

The existing repository has no public operation that emits a `restore_history` forge-run event, so the frozen fixture covers persistent generated/action history without that optional event. The migration preserves every stored `forge_runs` row mechanically.

## Verification

- Rust workspace tests: **121 passed, 0 failed** (12 desktop, 55 app-core, 12 contracts, 19 export, 23 storage; one ignored one-time fixture writer).
- Frontend tests: **36 passed**.
- `scripts/verify.ps1`: PASS, including frontend format/lint/typecheck/test/build, Rust fmt, strict Clippy, workspace tests, and optimized Tauri no-bundle build.
- Optimized Tauri output: `target/release/character-forge-desktop.exe`.
- `cargo metadata --format-version 1 --no-deps`: PASS.
- `git diff --check`: PASS.
- Focused schema-1 migration/rollback, Relationship lifecycle and endpoint checks, and schema-2 `.forgeproject` export/import, backup/restore suites: PASS.
- R1 focused `relationship_candidate_lifecycle_is_revisioned_and_stale_safe`: **1 passed**. `cargo test -p forge-storage`: **23 passed, 0 failed, 1 ignored**. `cargo test --workspace --no-fail-fast`: **121 passed, 0 failed, 1 ignored**. `scripts/verify.ps1`: PASS after applying `cargo fmt --all`; includes frontend checks/build, strict Clippy, workspace tests, and optimized Tauri no-bundle build.
- R1 `git diff --check`: PASS.

## Exit review

- Schema 2 and schema-1 migration/recovery: PASS.
- Frozen fixture and semantic preservation: PASS.
- Character/Quest replay and lock/reroll compatibility: PASS.
- Typed Relationship contracts, four definitions, project isolation, lifecycle/revisions/stale-write/duplicate policy: PASS.
- Fresh schema-2 creation, close/reopen, Vault compatibility, archive format v1, future-schema rejection: PASS.
- Unfinished Forge types/features exposed in UI: none.
- Known P0/P1/P2 data-loss or compatibility defect: none identified.
- New dependencies or product UI changes: none.

## Repository state

- Branch: `master`
- HEAD: `31f69ac800c07ffdf5ab5f567f80d5593b9684e2` (`docs: canonicalize The Forge entity universe discovery`)
- Worktree: mission source/tests/config/docs/fixture changes are uncommitted and unstaged. The pre-existing untracked operator exports and Mission 09 logs remain untouched. No commit, tag, push, publication, or next mission was started.
