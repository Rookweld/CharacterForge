# Mission 08 — ARMOR Report

## Status

**PASS — source/configuration review, automated gates, and packaged offline/security operator smoke complete.** The human operator accepts the unavailable complete automated transitive dependency advisory scan as a non-blocking documented residual maintenance/release risk. No production implementation changes or dependency changes were made during this mission.

## Baseline and scope

- Branch: `master`
- Starting HEAD: `d42eb3cff27655432ecb3481c74c99dfb4a880a8` (`test: complete Mission 07 gauntlet validation`)
- Starting worktree: clean; Mission 07 completion is committed. The checked-in project state text still had stale historical worktree/HEAD details, corrected below.
- Environment: Rust 1.93.1 / Cargo 1.93.1; Node 24.14.0 / pnpm 11.25.0. The global `npm` launcher is still broken because its configured `npm-cli.js` is missing.
- Scope: trust boundaries, command validation, Windows paths, archive/database handling, render/export behavior, external opening, offline/telemetry, logging, direct dependencies and available advisories, CSP/build/dev surface, errors/temp/recovery, data ownership, and relevant existing regression coverage.

## Threat-model summary

Assets and boundaries are recorded in [THREAT_MODEL_V1.md](../security/THREAT_MODEL_V1.md). Tauri commands remain the only UI-to-Rust application boundary. Rust owns filesystem and SQLite operations. Project/archive/user prose is data, rendered as React text, and there is no rich-text or raw HTML renderer. Threat assumptions exclude same-user concurrent filesystem rewriting and OS/kernel compromise.

## Security review results

### Tauri permissions and command boundary

`tauri.conf.json` has one main window and no capability files, plugins, file-dialog, shell, process, opener, clipboard, or updater dependency. The registered app commands are the product API exposed to the app frontend; there is no generic filesystem read/write or SQL command. No permission was removed because no unnecessary plugin grant was found. Core Tauri window/runtime functionality remains required.

Every registered handler was reviewed. Grouped inventory:

| Commands | Inputs / effects | Validation owner and destructive behavior |
|---|---|---|
| `default_project_directory`, `get_user_settings`, `recent_projects`, `get_project` | App config/document directory; read-only project/config state | Rust derives config paths; settings size/version checked; recent list bounded to eight. |
| `update_user_settings`, `forget_recent_project` | Typed settings or recent path; local config write | Rust checks settings version and writes only the fixed config file; forget removes a matching recent-list entry, not project data. |
| `create_project`, `open_project`, `close_project` | User directory/name or existing project path; create/open/close active SQLite project | Rust rejects blank input, canonicalizes directories/paths, sanitizes filename, reserves new database with create-new, rejects archive as DB, and validates opened schema/project state. Create writes a new user-selected project; close discards transient session only. |
| `export_project_command`, `create_backup_command`, `export_entity_command` | User-selected destination, entity ID, fixed `json`/`markdown` enum string | Rust requires active project, parses ID and format, validates destination/no-clobber; only writes chosen new export/backup file. |
| `import_project_command`, `restore_backup_command` | Archive path and selected destination directory / active project | Rust validates archive, fixed members, bounds, checksum, schema and identity before activation. Import refuses collision. Restore is destructive to active project only after explicit UI confirmation and retains recovery/rollback handling. |
| `forge_character`, `forge_quest` | Preset, seed, axes/constraints | Rust generation and contracts validate identifiers, ranges and DNA before session replacement. No persistent write. |
| `set_field_lock`, `reroll_field`, `reroll_unlocked`, `edit_field`, `undo_session`, `redo_session` | Field/value or session operation | Rust maps supported field enums and scalar/list shapes, validates the resulting candidate, and mutates transient session only. |
| `save_candidate`, `revise_canon`, `promote_to_canon`, `toggle_favorite` | Active session, ID, revision reason / state | Rust validates entity contracts and uses storage transactions/optimistic versions. Canon transitions are explicit; revise is destructive to current canon while retaining a prior revision. |
| `list_entities`, `get_entity`, `list_history`, `get_history_snapshot`, `get_canon_revisions` | Optional/required IDs and event/revision identifiers | Rust parses IDs and queries active project only; reads do not mutate. |
| `use_history_as_working_copy`, `use_canon_revision_as_working_copy` | Entity ID plus event/revision selection | Rust fetches the selected immutable snapshot, checks its entity, and changes transient working state only. |

The frontend is not the validation boundary. Paths are intentionally user-selected; the application has no scoped native file dialog. Create/import derive project database filenames from a sanitized display name. Archive metadata does not provide an arbitrary output path. Residual same-user reparse-point/concurrent filesystem races are outside the documented attacker model.

### Vault, SQLite, and data parsing

Mission 06 ZIP v1 remains closed to exactly `manifest.json` and `project.sqlite3`; it rejects duplicate/case-colliding/unsafe names, directories and special files, unexpected members, oversized archives/members/manifests, excessive compression ratio, unsupported versions, checksum mismatches, inconsistent metadata, and invalid SQLite before publish. Reads are streamed to staging; there is no extract-all. Export/import/restore use no-clobber or recoverable flows already covered by archive tests. The archive digest is integrity detection, not authenticity.

SQLite is opened through bundled `rusqlite`; SQL is application-owned and values are parameterized. No extension-loading API, user-supplied SQL, or project-driven `ATTACH` path was found. Typed Serde/domain validation rejects unsupported data rather than dynamically loading it. Archive sizes are bounded, but there is no separate global text/JSON entity-size limit; none was added without evidence that a limit fits canonical v1 data.

### Rendering, URLs, offline behavior, and telemetry

No `dangerouslySetInnerHTML`, HTML/Markdown renderer, project-controlled URL opener, iframe/object/embed, frontend fetch/XHR, WebSocket, or EventSource was found. User prose renders through React text interpolation. Markdown export is data and deliberately preserves content for external Markdown viewers.

No production network client/plugin, telemetry/analytics/crash uploader, remote font/image/script, or update client was found in application code/manifests. `devUrl` is loopback-only; production frontend assets are generated locally into `dist` and embedded by Tauri. Static review is complemented by the operator's packaged offline smoke: launch, project create/open, Character and Quest Forge, save/history/canon all remained usable without network connectivity or account/API-key setup. This does not claim that the OS/WebView itself is incapable of networking.

### Logs, errors, temporary files, and recovery

No production logging crate, persistent log writer, or telemetry export was found in the repository. There is therefore no application log-folder path in the reviewed app configuration; recent-project metadata does persist project names, absolute paths and timestamps in app config. Errors are structured by kind and user messages; some lower-level storage/archive errors retain technical details. The operator reported no obvious private prose in observed errors/diagnostics and no routine creative-prose logging from any logs available to inspect. No project prose or complete entity payload is intentionally written to logs.

Archive staging uses OS temp directory plus unique process/counter names and exclusive directory creation; sibling files use create-new and no-clobber finalization. Staging cleanup is RAII. Restore keeps a pre-restore snapshot and rollback/displaced recovery behavior. Settings/recent JSON writes use PID-based temporary names and remove old target before rename; because files are per-user config and no same-user concurrent attacker is in scope, this was not elevated to a finding. This behavior is documented as an assumption, not presented as race-proof atomic replacement.

### CSP, devtools, secrets, and package surface

CSP is `default-src 'self'; style-src 'self' 'unsafe-inline'; script-src 'self'; img-src 'self' data:`. It has no wildcard or `unsafe-eval`; inline styles support current app styling and remain the narrow exception. Release frontend uses local Vite output, not the dev server. `bundle.active` is false because this mission builds the no-bundle executable; no updater or devtools enablement was found in app config. No tracked secret-shaped filename was found. The repository source scan was pattern-based and did not dump potential values; no external profile or unrelated directory was scanned.

### Dependency security

See [DEPENDENCY_REVIEW_V1.md](../security/DEPENDENCY_REVIEW_V1.md). Direct/runtime dependencies and relevant features/capabilities were reviewed. `cargo-audit`/`cargo-deny` are absent. `pnpm audit --offline` could not reach npm's advisory endpoint (`EACCES`), so a complete automated transitive advisory scan was unavailable. The operator accepts this as a non-blocking residual maintenance/release risk; this report does not claim the transitive graph was verified clean. RustSec's ZIP advisory applies only to `zip >=1.3,<2.3`; locked 8.6.0 is outside that range and this code does not use extraction. No confirmed applicable dependency vulnerability was identified in the available review. No package was installed or upgraded.

## Findings and severity

Confirmed findings entered: P0 0, P1 0, P2 0, P3 0. No unresolved P0/P1/P2 was confirmed. This count describes the reviewed evidence, not an assertion of exhaustive proof. Advisory review and operator gates remain open as verification limitations rather than invented vulnerabilities.

## Automated evidence

Mission 07 retains hostile archive path/member/size/checksum, import collision, restore rollback, settings corruption, future schema, malformed input, Unicode, and cross-feature coverage. Mission 08 reran the complete verification successfully with an isolated target directory.

| Check | Result |
|---|---|
| `scripts/verify.ps1` | PASS with isolated `CARGO_TARGET_DIR=target/mission08-security-review`; default target first hit Windows `Access denied` replacing the running packaged executable. |
| Rust workspace tests | PASS — 114 (12 desktop, 53 app-core, 10 contracts, 19 export, 20 storage). |
| Frontend tests (`pnpm test`) | PASS — 36. |
| Formatting/lint/typecheck/frontend build/strict Clippy | PASS. |
| Cargo metadata | PASS — `cargo metadata --no-deps --format-version 1`. |
| Optimized no-bundle Tauri build | PASS — `C:\dev\CharacterForge\target\mission08-security-review\release\character-forge-desktop.exe`. |
| `git diff --check` | PASS. |
| Online complete dependency advisory scan | Not available: registry endpoint denied; no installed Rust audit tool. Human accepted as non-blocking documented residual risk; no clean-transitive-graph claim. |
| Packaged offline/security operator smoke | PASS — human operator verified offline launch and core workflows, export/import/backup/restore, close/reopen, safe corrupt-import rejection, no unexpected external execution, and no observed private-prose leakage. |

## Packaged operator verification

**Operator result: PASS** in `C:\dev\CharacterForge\target\mission08-security-review\release\character-forge-desktop.exe`.

- Offline launch/use passed: project create/open, Character Forge, Quest Forge, save/history/canon worked without network, login, account, or API key.
- Project/data safety passed: entity export; `.forgeproject` export/import using a disposable project; backup/restore; close/reopen state persistence; malformed/corrupt import rejected safely while the app remained usable.
- No ordinary workflow unexpectedly opened a shell, browser, external program, or project-controlled URL.
- No obvious private prose appeared in observed errors/diagnostics. Where logs were available for inspection, the private-content sentinel did not reveal routine creative-prose logging.
- No operator-visible P0/P1/P2 security defect was found.

The operator accepts the unavailable automated transitive advisory scan as a non-blocking residual maintenance/release limitation; see [DEPENDENCY_REVIEW_V1.md](../security/DEPENDENCY_REVIEW_V1.md).

## Mission 08 PASS criteria re-evaluation

| Criterion | Result |
|---|---|
| Threat model and trust boundaries documented | PASS |
| Tauri capabilities reviewed; shell/process execution absent | PASS |
| Rust command-side validation and Windows path boundary reviewed | PASS |
| Vault hostile-input defenses and pre-activation validation retained | PASS |
| SQLite extension/code-execution boundary reviewed | PASS |
| Project prose inert in UI/export handling; external URL opening absent | PASS |
| No required hidden network path or telemetry; packaged offline smoke | PASS |
| Logging privacy reviewed; operator sentinel result recorded | PASS |
| Tracked repository secret-pattern sweep completed; no confirmed secret | PASS |
| Direct dependency/features reviewed; applicable ZIP advisory not applicable | PASS with accepted residual: no complete transitive automated advisory scan |
| CSP, local production assets, dev/release surface reviewed | PASS |
| Staging, backup, restore, and recovery safety reviewed | PASS |
| No unresolved P0/P1/P2; finding totals P0 0, P1 0, P2 0, P3 0 | PASS |
| Existing security regression and normal repository tests pass | PASS |
| Optimized Windows build and packaged operator smoke | PASS |

## Files and mission boundary

- [Threat model](../security/THREAT_MODEL_V1.md)
- [Findings log](../security/ARMOR_FINDINGS.md)
- [Dependency review](../security/DEPENDENCY_REVIEW_V1.md)
- [Security notes](../SECURITY.md)
- Mission 09 and release/publishing work have not started.
