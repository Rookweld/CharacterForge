# Character Forge v1 Threat Model

**Scope:** local-first Windows desktop application, committed Mission 07 baseline `d42eb3cff27655432ecb3481c74c99dfb4a880a8`.

## Assets

- Local project SQLite databases and their WAL/SHM files.
- `.forgeproject` archives, selected export destinations, backups, restore recovery copies, and entity JSON/Markdown exports.
- Characters, Quests, history, canon/revisions, project identity, application settings, and recent-project paths.
- Application binaries, packaged frontend, build configuration, dependency lockfiles, and user-visible error/diagnostic detail.

## Trust boundaries

| Boundary | Trust rule and controls observed |
|---|---|
| React → Tauri commands | Frontend is not authoritative. Explicit command handlers deserialize typed inputs; Rust checks enums, IDs, paths, candidate/domain contracts, and active-project state. There is no generic filesystem command. |
| Tauri/Rust → filesystem | Rust performs project, archive, export, settings, and recent-project I/O. Paths are user-selected strings because no dialog plugin is installed; create/import derives the database filename from the display name and refuses collisions. |
| Application → SQLite | SQLite is bundled in Rust. SQL is application-owned and values are bound parameters. Project open/import runs integrity and schema checks. No extension-loading call or SQL sourced from project prose was found. |
| Application → `.forgeproject` | Closed two-member ZIP format, streaming bounded reads, exact member allowlist, checksum/manifest/database consistency validation, staging before activation, no-clobber publication, and recoverable restore. |
| Imported JSON/project data → typed domain | Serde typed structs and domain validation are used. Unknown fields are denied on key envelopes; unsupported schema/format versions fail. Content files are compile-time application assets, not dynamically executed plugins. |
| Application → native OS/dialogs | No dialog, shell, opener, or process plugin is configured. User supplies paths through ordinary inputs. |
| Application → local settings/config | Versioned settings and recent-project JSON live under Tauri's app config directory. Recent-project data stores at most eight names, paths, and timestamps; it does not copy project prose. |
| Application → logging | No production logger, telemetry exporter, or log file writer was found. Diagnostics are currently returned through structured command errors and UI messages. |
| Application → dependencies/runtime | Lockfiles pin Rust and frontend dependency resolution. Tauri embeds built frontend assets for production; dev URL is loopback-only and build configuration runs a local build. |

## Untrusted inputs

Imported `.forgeproject` archives and their metadata; existing SQLite project files; recent-project/settings files; user-entered project names and paths; entity IDs, field names, values, and revision reasons; export/backup destinations; and all generated or manually authored Character/Quest prose.

## Attacker assumptions and exclusions

The review addresses malformed or malicious application inputs and accidental hostile files. It does not claim protection from an administrator controlling the machine, kernel compromise, malware with unrestricted access to the user profile, tampered OS trust roots, or a same-user attacker who can concurrently rewrite selected directories and reparse points. OS/WebView general network capability is outside application control; this review checks that the product has no required network path.

## Security properties sought

Project content is inert data; validation is repeated in Rust; imports do not choose arbitrary destinations from archive metadata; failed operations preserve prior user data; no hidden network/telemetry path is required; logs do not duplicate private project prose; and failures remain structured and read-safe.

## Residual assumptions

- User-selected directories and the per-user application config directory are trusted against concurrent same-user filesystem manipulation.
- Current project/schema payloads have format/archive bounds but no additional global JSON text-size ceiling beyond those archive limits. Enforcing a smaller entity limit needs canonical v1 size evidence and compatibility analysis; none was justified in this pass.
- There is no application log location because production logging is absent. Structured error details are local UI/command responses, not persistent diagnostics.
- Static inventory and offline source review cannot prove the operating system or WebView makes no network activity. Packaged offline operator smoke remains required.
