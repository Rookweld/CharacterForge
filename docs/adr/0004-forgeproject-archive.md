# ADR 0004: Versioned `.forgeproject` archive

- **Status:** Accepted for Mission 06 — VAULT
- **Date:** 2026-09-27

## Context

Mission 06 makes the existing one-database-per-project SQLite store portable. The package must include a consistent snapshot, preserve project identity and all stored state, reject hostile archive metadata, and remain separate from the live database. Archive structure is a long-lived file compatibility boundary.

## Decision

- Use ZIP 8.6.0 with only `deflate-flate2-zlib-rs` enabled. V1 is a closed archive containing exactly `manifest.json` and `project.sqlite3`, both regular files. Unknown files and forward portable format versions are rejected.
- The manifest identifies `forgeproject` format version 1 independently of SQLite schema, generator, content, and ForgeDNA recipe versions. It records application identity/version, project identity/name, stored version tuple, export time, and BLAKE3/size metadata for the exact SQLite member bytes.
- BLAKE3 detects accidental or deliberate byte modification but does not establish authenticity or publisher identity. No signature or key management is implied.
- Export snapshots the active SQLite connection with `VACUUM INTO`, which includes committed WAL pages without mutating the live store. Both snapshot and archive are validated before an atomic no-clobber finalization.
- Import uses fixed member names and bounded streaming; it never invokes extract-all. Archive limits are 2 GiB input, two members, 1 GiB database, 64 KiB manifest, and a 200:1 expansion ratio above a 1 MiB minimum threshold. Path validation also rejects Windows device names, drive/UNC/absolute paths, alternate separators, trailing-dot/space aliases, case collisions, and non-regular entries.
- Imports validate in a system-temp staging directory, then publish the checked database via a sibling temporary file and no-clobber hard link. Restore requires an explicit operation, validates first, snapshots the existing target, closes/reopens the application session, and restores the prior snapshot if final validation fails.
- One format pipeline serves portable export and local backup. Entity JSON uses a versioned structured envelope; Markdown contains a readable header plus the full structured entity payload.

## Consequences

The format avoids extracting arbitrary archive paths and keeps SQLite as the only project source of truth. V1 intentionally has no arbitrary attachments or extension members. Future formats require an explicit version and compatibility implementation. Hard-link finalization assumes the destination filesystem supports same-volume hard links; failure is reported without publishing a partial output.
