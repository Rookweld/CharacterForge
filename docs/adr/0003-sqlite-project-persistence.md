# ADR 0003: SQLite project persistence and lifecycle

- **Status:** Accepted for Mission 04 — MEMORY
- **Date:** 2026-09-27

## Context

Mission 01 defined versioned contracts and deterministic ForgeDNA but deliberately left persistence unimplemented. Mission 04 requires durable projects, candidate/canon lifecycle, generation history, immutable canon revisions, migration recovery, and real close/reopen behavior without coupling generation to storage or Tauri.

## Decision

- Implement the storage boundary in `forge-storage`, which consumes validated `forge-contracts` entities and `forge-app-core` candidate values. Generator APIs remain pure with respect to persistence; the desktop shell and frontend do not access SQLite directly.
- Use `rusqlite = 0.40.2` with `default-features = false` and only `bundled`. This gives the Windows build an embedded SQLite implementation without adding async runtime/ORM features or a separately installed SQLite runtime.
- Use one SQLite database per local project, schema versions in `PRAGMA user_version`, and explicit transactional schema-0-to-1 migration. Copy an existing database to `<database>.pre-migration.bak` after checkpoint and before any schema mutation. Never reset or overwrite a database after migration failure.
- Store queryable envelope columns alongside validated JSON snapshots. Candidate JSON retains the complete candidate, field origins, ForgeDNA, generation metadata, and all current Character/Quest contract fields. History events use an independent event identity so repeated deterministic generation IDs are not dropped.
- Use SQLite transactions for entity plus history/revision changes. Each entity has a monotonic write version; mutations require the version the caller loaded. Promotion and canon revision are separate explicit operations, and revisions retain immutable prior payloads.
- Enable foreign keys, WAL, `synchronous=FULL`, a bounded busy timeout, and a deliberate auto-checkpoint threshold. Provide integrity and foreign-key diagnostics.
- Keep editing-session undo/redo in transient in-memory history. Persistent ForgeRuns and canon revisions remain distinct records with different lifetimes and purposes.

## Consequences

The project repository can close and reopen an actual database while preserving generated and canon state. Future schema changes must add explicit migrations and fixtures. Portable backups/import remain a later mission; pre-migration copies are limited to migration recovery and do not implement `.forgeproject`.
