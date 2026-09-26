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

## Mission 03 — Quest prototype (partial)

- Quest generation is implemented in `forge-app-core` and reuses the existing ForgeDNA streams, nonce controls, and lock/reroll functions. The developer harness is `cargo run -p forge-app-core --example quest_forge -- --seed 42 --preset cyberpunk --axis technology=100`.
- New Quest recipes use content version 2. Quest content version 1 is currently rejected; no historical Quest corpus was created before Mission 03.
- The Quest preset thematic data is currently embedded in Rust source and the prose pools remain small. The intended declarative, versioned Quest corpus and complete quality/coherence gates remain unfinished. See the Mission 03 report.
- **Remediation compatibility decision:** the committed content-version-2 Quest output is treated as replay behavior and will remain frozen. New declarative Quest content uses content version 3. No ForgeDNA serialization, stream context, nonce, lock, or dependency semantics are changed.
