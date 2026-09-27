# Architecture

## Mission 00 baseline

The application is a local-first Windows desktop product. The desktop shell hosts a TypeScript user interface and delegates application operations to Rust. Rust owns application services, deterministic domain logic, and persistence boundaries. SQLite is the authoritative project store once persistence is implemented in its mission.

## Dependency direction

```text
React UI (presentation and user input)
    │ invokes explicit application commands
    ▼
Tauri desktop boundary
    │ delegates; does not own domain rules
    ▼
Rust application/core ─────► shared, versioned contracts
    │                              ▲
    ├──► generator/domain          │ content data (versioned)
    ├──► persistence (SQLite)      │
    └──► export                    │
```

The UI must not directly own persistence. Generator/domain logic must not depend on Tauri or visual components. Persistence must not know about visual components. Shared contracts describe data crossing boundaries; they do not become a miscellaneous utility module.

## Boundaries and seams

- **Desktop:** window lifecycle and narrow command transport.
- **Frontend:** rendering, accessible interaction, and transient view state.
- **Application/core:** use cases and orchestration, independent of UI components.
- **Domain/generator:** versioned deterministic rules, independent of Tauri and storage.
- **Persistence:** SQLite access behind application-facing interfaces; no UI dependencies.
- **Content:** declarative, versioned data consumed by domain logic.
- **Export:** explicit portable representations; it does not replace SQLite as the source of truth.

Future provider or expansion support may use these boundaries if later authorized. Mission 00 creates no provider runtime, plugin system, marketplace, or expansion implementation.

## Data and safety

SQLite remains the v1.0 project source of truth when persistence is implemented. Mission 01 establishes versioned project/entity contracts and ForgeDNA, but adds no persistence workflow.

## Mission 01 — contracts and deterministic field streams

- Shared contracts use explicit version wrappers. The initial supported schema, generator, content, and recipe versions are each `1`; unsupported ForgeDNA versions return structured validation errors.
- Canonical ForgeDNA uses compact JSON: struct declaration order, sorted `BTreeMap` keys, lowercase hyphenated UUID text, lowercase fixed-width hexadecimal root seeds, normalized ASCII identifiers, and trimmed constraint values. The strict decoder accepts only the canonical representation.
- A field stream uses BLAKE3 derive-key context `character-forge/forge-dna/field-stream/v1`. Its input includes the version tuple, root seed, preset, normalized axes and constraints, target field and nonce, and sorted explicit dependency field/nonces. It excludes time, process state, UI ordering, unrelated field nonces, and global randomness.
- Lock/unlock changes only the lock flag. A single-field reroll increments the target and its transitive, explicitly dependent unlocked fields. A locked dependent and its descendants are preserved. Reroll-all increments all unlocked fields. Nonce and generated-value changes commit together only after every requested field succeeds.
- The core owns deterministic stream derivation and reroll orchestration; callers supply a pure field generator. These contracts do not add content packs, persistence, UI workflows, or AI.

See [ADR 0002](adr/0002-forge-dna-and-field-streams.md) for the serialization and stream compatibility decision.

## Mission 02 — Character generation

- Character generation lives in `forge-app-core`, independent of Tauri, UI, and storage. It returns a validated Character candidate together with the normalized ForgeDNA and generation provenance.
- Character content is compiled into the Rust core and validated at load time. `content/characters/v1/core.json` is the frozen Mission 02 baseline; `content/characters/v2/core.json` plus its declarative identity-depth additions is the current expanded corpus. DNA content version selects the corresponding corpus, preserving replay for v1 saves. ForgeDNA canonical encoding and field-stream derivation semantics are unchanged.
- Character generation owns a canonical field registry and explicit dependency graph. Archetype is the shared context dependency; relationship, backstory, and story hooks also depend on desire. Existing ForgeDNA field streams and reroll orchestration remain authoritative for deterministic selection, nonce advancement, lock behavior, and atomic updates.
- Direct user edits are represented by per-field origin metadata and retained generation provenance. Reroll-all temporarily protects manually edited fields, then restores their original lock settings.
- The developer harness is `cargo run -p forge-app-core --example character_forge -- --seed 42 --preset cyberpunk --axis technology=100`; it emits the complete candidate and ForgeDNA as JSON. This is a developer demonstration, not UI functionality.

## Mission 03 — Quest generation (closed)

- Quest generation is implemented in `forge-app-core` and reuses the existing ForgeDNA streams, nonce controls, and lock/reroll functions. The developer harness is `cargo run -p forge-app-core --example quest_forge -- --seed 42 --preset cyberpunk --axis technology=100`.
- Quest content version 2 remains available for replay; new Quest recipes use content version 3. The v3 corpus is `content/quests/v3/core.json`; the Rust loader validates it and deterministic selection/composition is separate from the JSON data. ForgeDNA wire encoding and field-stream derivation are unchanged.
- V3 adds internal thread selection plus generated clue and briefing fields on `QuestCandidate` without changing the shared Quest entity contract. Mission 03 is closed PASS; its final quality evidence and sample packet are recorded in the Mission 03 reports.
- **Remediation compatibility decision:** the committed content-version-2 Quest output is treated as replay behavior and will remain frozen. New declarative Quest content uses content version 3. No ForgeDNA serialization, stream context, nonce, lock, or dependency semantics are changed.

## Mission 04 — MEMORY persistence

- `forge-storage` implements the Rust repository interfaces using `rusqlite` 0.40.2 with only the `bundled` feature (rusqlite default features disabled). SQLite is built into the application; no separately installed SQLite runtime is required.
- Each on-disk `ProjectStore` contains one project. Versioned SQLite `user_version` migrations create schema 1 from a fresh schema-0 database or upgrade the documented synthetic `legacy_projects` schema-0 fixture. Existing databases receive a `.pre-migration.bak` copy before migration; schema changes commit transactionally, and failures preserve both original data and backup.
- Projects, structured candidate/canon payloads, generation runs, entity revisions, and project-local settings are persisted. Candidate payloads retain complete Character/Quest candidates, ForgeDNA, generation provenance, per-field manual-edit origins, locks/nonces, and Quest v3 clue/briefing fields. Quest v2 replay payloads remain intact.
- `ProjectRepository` and `EntityRepository` form the application-facing persistence contracts. The generator remains independent of storage and Tauri. Multi-record writes use transactions; entity writes use optimistic `write_version` checks. Canon promotion and revision are explicit operations; previous canon snapshots remain inspectable.
- Connections enable foreign keys, WAL, FULL synchronous mode, a 5-second busy timeout, and a 1,000-page automatic checkpoint. `ProjectSession` validates a new handle before replacing the active one. `SessionHistory<T>` provides in-memory editing-session undo/redo, separate from persistent ForgeRuns and canon revisions.
- Migration/corruption/version errors are structured. Integrity tooling runs SQLite integrity and foreign-key checks. See [ADR 0003](adr/0003-sqlite-project-persistence.md) and [Mission 04 report](missions/MISSION_04_REPORT.md).
# Mission 05 — desktop application boundary

`apps/desktop/src-tauri/src/lib.rs` now exposes typed Tauri commands for project lifecycle, Character and Quest generation, candidate shaping, persistence, browsing, favorites, canon promotion, history snapshots, and canon revisions. Commands coordinate the existing `forge-app-core` candidate APIs and `forge-storage` repositories; field generation, dependency rerolls, lock semantics, manual-edit provenance, optimistic writes, and SQLite transactions remain in Rust. The desktop shell does not expose SQL or general filesystem read/write commands.

The React frontend owns only view/session state: current section and forge mode, selected preset, seed, axes and supported constraint inputs, the current unsaved candidate presentation, current selection, and loading/error notices. It calls the command boundary through `apps/desktop/src/api.ts`. The Rust project store is authoritative for saved project/entity state; frontend candidates are accepted back by narrow shaping commands and validated by the core before changes are returned.

Project creation takes a user-selected database path and a display name, opens the Mission 04 `ProjectStore`, and inserts the versioned project envelope. Opening an existing path passes through `ProjectSession::open`, which validates storage integrity before swapping the active project. Closing releases the active store and session candidate. The first-launch screen deliberately asks for a path instead of introducing a file-picker plugin or broad filesystem capability; portable project packaging is still deferred to Mission 06.

Generation commands build canonical ForgeDNA from the selected preset, seed, axes, and validated constraints, then invoke the existing pure generator. Character and Quest candidates are attached to the active project's canonical ID; Quest Forge uses its project-aware generation entry point while retaining the previous default entry point for deterministic replay compatibility. The generator does not run in TypeScript.

Field lock, reroll, and edit commands update the active Rust candidate by calling the corresponding core operations. The UI shows the exact reroll action and locked state. Saving a new candidate inserts its candidate and history record in one storage transaction; saving later changes uses its loaded write version, so a stale write returns a structured conflict. Favorites, library reads, canon promotion, snapshots, and revision reads also delegate to ProjectStore. Canon promotion remains explicit and snapshots a revision; the frontend never silently revises canon.

Tauri command failures cross the boundary as a small typed `{ kind, message, expected, actual }` shape. The frontend maps schema, corruption, validation, storage, and stale-write cases to plain-language messages; stale writes require reload rather than retrying over newer data. Busy states disable duplicate primary actions, empty/loading states appear in each library, and keyboard focus, labeled sliders/inputs, visible focus, non-color state labels, and reduced-motion support are part of the initial accessibility baseline.

## Mission 06 — VAULT portable projects

- The live project remains one SQLite database opened through `ProjectStore`. A `.forgeproject` is a closed ZIP container with exactly `manifest.json` and `project.sqlite3`; it is never a renamed live database. ZIP handling is confined to `forge-export`, not React or Tauri commands.
- The manifest has an independent portable format version (`1`), application and database format identity, canonical project ID/name, schema/generator/content versions, export timestamp, and exact member size and BLAKE3 digest. The SQLite snapshot retains per-entity ForgeDNA recipe/content versions, locks, manual-edit origins, Quest metadata, history, settings, and canon revisions. The digest detects byte changes; it does not authenticate a publisher.
- Export uses SQLite `VACUUM INTO` on the active connection to include committed WAL state. It validates the resulting standalone database, streams it into a staged archive, syncs the completed file, then uses an atomic no-clobber hard link to the requested path. The open project is not checkpointed or rewritten as an export step.
- Import reads only the two fixed member names. It rejects unsafe/ambiguous paths, special files, duplicates, unknown members, missing members, unsupported versions, size/compression limits, integrity mismatches, SQLite corruption, schema incompatibility, and manifest/database disagreement. The database is streamed to isolated staging and opened through `ProjectStore` before a sibling temporary file is atomically published at a previously nonexistent destination.
- Restore validates the package before replacing anything, closes the active session through `restore_active`, creates a consistent snapshot of the prior destination, moves the old file aside, clears stale SQLite sidecars, installs and reopens the restored database, and rolls back from the snapshot if post-replacement validation fails. Ordinary imports never replace an existing path.
- The desktop boundary exposes typed export/import/backup/restore and JSON/Markdown entity-export commands. The UI only passes paths and intent; Rust owns archive parsing, checksums, database checks, staging, replacement, and error classification. Export/backup use a shared `.forgeproject` pipeline. See [ADR 0004](adr/0004-forgeproject-archive.md) and [Mission 06 report](missions/MISSION_06_REPORT.md).
