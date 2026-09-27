# Mission 06 — VAULT

## Mission

Mission 06 — VAULT: portable projects, export/import, backup/restore, and archive security.

## Status

**PASS — all Mission 06 exit criteria are satisfied.** Automated verification and the packaged desktop operator smoke passed. No operator-visible P0/P1 VAULT defect was observed.

## Scope delivered

- Added `.forgeproject` format version 1 as a closed ZIP package containing exactly `manifest.json` and a consistent standalone `project.sqlite3` snapshot. It is distinct from the live SQLite file.
- Added a versioned manifest with application/format identity, project identity/name, database/generator/content versions, export timestamp, member size, and exact-byte BLAKE3 digest. The digest detects changes; it is not publisher authentication.
- Export snapshots the open store with SQLite `VACUUM INTO`, validates the snapshot, stages and syncs the package, then atomically finalizes without overwriting an existing destination. Export and backup use the same pipeline.
- Import validates names, file types, count/size/compression bounds, format version, manifest, checksum, database schema, project identity, and SQLite integrity before publishing into a new destination. Failed import does not overwrite or leave a partial target.
- Restore closes an active store, validates and stages the package, snapshots the prior project, replaces the database, reopens and validates it, and rolls back if post-replacement validation fails. Ordinary import refuses an existing destination.
- Added structured JSON and human-readable Markdown exports for saved entities.
- Added project actions and busy/error/success UI states, including explicit restore confirmation. Archive and filesystem logic remains behind typed Tauri commands in Rust.

## Files materially changed

- `crates/export/Cargo.toml`, `crates/export/src/lib.rs`, `crates/storage/src/lib.rs`
- `apps/desktop/src-tauri/Cargo.toml`, `apps/desktop/src-tauri/src/lib.rs`
- `apps/desktop/src/App.tsx`, `apps/desktop/src/api.ts`, `apps/desktop/src/styles.css`, `apps/desktop/tests/App.test.tsx`
- `Cargo.lock`, `docs/ARCHITECTURE.md`, `docs/adr/0004-forgeproject-archive.md`, `GAZINGA/PROJECT_STATE.md`
- `docs/missions/MISSION_06_REPORT.md`

## Commands executed

- `cargo check -p forge-export`
- `cargo test -p forge-export` — 14 passed.
- `cargo check -p character-forge-desktop`
- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace` — 101 passed, 0 failed.
- `cargo metadata --no-deps --format-version 1`
- `pnpm format:check`
- `pnpm lint` — passed with nine existing CSS specificity warnings.
- `pnpm typecheck`
- `pnpm test` — 24 passed, 0 failed.
- `pnpm build`
- `npm test` — unable to start because the global npm launcher references a missing `npm-cli.js`; repository pnpm verification passed.
- `scripts/verify.ps1` — passed, including optimized Tauri build.
- `git diff --check`

## Verification evidence

- The real on-disk archive roundtrip test preserves the same project ID, Character and Quest values, manual-edit provenance, locked fields, canon state/revision, generation history, and Quest v2/v3 metadata. It commits a setting while the original store is open and confirms that the WAL-resident value appears after import. Repeated exports do not change entities or history.
- Archive regression coverage includes traversal and absolute/drive/UNC paths, Windows separator variants, case-colliding/duplicate members, unknown and special files, oversized manifest, high compression ratio, checksum mismatch, future portable version, corrupt SQLite, future SQLite schema, import collision, successful restore, injected rollback failure, active-session restore, and staging cleanup.
- Export failure preserves an existing output and the active project. Import collision preserves the existing target. Injected restore validation failure restores the previous healthy project.
- Frontend tests cover export and backup success, export errors, import/open success, restore confirmation and failure presentation, entity JSON export, and busy/disabled action state.
- Full Rust workspace: **101 passed, 0 failed** (desktop 8, app-core 53, contracts 10, export 14, storage 16). Export-specific suite: **14 passed, 0 failed**.
- Frontend: **24 passed, 0 failed**; formatting, lint, typecheck, and production build passed. The nine CSS specificity warnings predate this mission's styling addition and are non-failing.
- Recorded automated evidence: **101 Rust tests PASS**, **24 frontend tests PASS**, `scripts/verify.ps1` PASS, optimized Tauri build PASS, formatting PASS, strict Clippy PASS, Cargo metadata PASS, `git diff --check` PASS, and `pnpm test` PASS. The global `npm test` launcher still cannot start because its configured `npm-cli.js` is missing; this is an environment issue only, and the repository pnpm suite passed.
- The operator completed the smoke workflow in `C:\dev\CharacterForge\target\release\character-forge-desktop.exe` and reported **PASS**. They exported a real persisted project containing saved Character and Quest data to `.forgeproject`; imported it to a different SQLite path; opened it and confirmed project identity/content, Character, Quest, manual edits, lock state, history, and canon state/revisions survived.
- The operator created a backup, made and saved a clearly visible project change, restored the backup, and confirmed the post-backup change disappeared and the pre-backup state returned. Character, Quest, history, and canon remained valid after restore.
- The operator fully closed and relaunched the packaged app, reopened the restored project, and confirmed the restored state remained correct after process restart. No operator-visible P0/P1 VAULT defect was observed.

## Exit criteria review

- `.forgeproject` is a versioned, closed archive with a consistent standalone SQLite snapshot: **PASS** (implementation and automated archive coverage).
- Export preserves project identity and the persisted project state without mutating the open project: **PASS** (automated roundtrip/WAL checks and operator export).
- Import validates archive and database integrity, enforces path/resource/version protections, and safely publishes to a different destination: **PASS** (automated malicious-input, collision, failure-safety, and roundtrip tests; operator import).
- Character/Quest content, manual edits, locks, history, canon state/revisions, and Quest version metadata survive portable roundtrip: **PASS** (automated roundtrip and operator-confirmed persisted state).
- Backup creates a usable project snapshot and restore returns the selected prior state with recovery safeguards: **PASS** (automated restore/rollback coverage; operator backup/change/restore).
- Desktop flows expose the supported export/import/backup/restore operations and handle confirmation, progress, and errors: **PASS** (frontend tests and packaged operator workflow).
- Automated checks and optimized packaged build pass: **PASS** (evidence listed above).
- Packaged Windows desktop operator acceptance passes, including close/relaunch persistence: **PASS** (operator result recorded above).
- No known P0/P1 VAULT defect remains: **PASS** (no such defect found in automated coverage or observed by the operator).

No Mission 06 exit criterion remains unsatisfied.

## Architecture decisions

- Accepted ZIP 8.6.0 with only `deflate-flate2-zlib-rs`; format structure and compatibility rules are recorded in [ADR 0004](../adr/0004-forgeproject-archive.md).
- Archive v1 is closed to exactly two regular-file members. Limits are 2 GiB input, 2 members, 1 GiB database, 64 KiB manifest, and 200:1 expansion ratio above a 1 MiB minimum threshold.
- The portable format version remains independent of SQLite schema, generator, content, and ForgeDNA recipe versions. Saved ForgeDNA remains in the SQLite payload.
- Export hashes exact member bytes with the existing BLAKE3 dependency. No signature/authenticity claim is made.
- `VACUUM INTO` provides the standalone WAL-consistent snapshot. Staged import validates through the real `ProjectStore`; restore uses a recoverable SQLite snapshot and a close/replace/reopen lifecycle.

## Known defects

- No P0/P1 archive or data-loss defect was found in automated verification or the operator smoke.

## Deferred items

- Cloud backup/sync, sharing, arbitrary attachments, plugin execution, and Mission 07 GAUNTLET remain out of scope.
- No native file-dialog dependency was introduced. Project and archive paths use labeled path inputs because no approved dialog dependency existed and the brief authorized no additional production dependency besides ZIP.

## Repository state

- Branch: `master`.
- HEAD: `99cc0e7e60eab143f7e16ec37ba00682c537f873` (`fix: preserve Character field independence across desktop workflows`).
- Worktree: dirty with Mission 06 implementation, tests, and documentation; this closeout updates only this report and `GAZINGA/PROJECT_STATE.md`; no commit created.

## Next mission readiness

**NO — stop at the Mission 06 boundary.** Mission 06 is PASS and closed. Mission 07 has not been started, per instruction.
