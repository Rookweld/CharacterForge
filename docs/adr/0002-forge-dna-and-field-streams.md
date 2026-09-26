# ADR 0002: Canonical ForgeDNA and deterministic field streams

- **Status:** Accepted for v1.0 FOUNDATION
- **Mission:** 01 — DNA

## Context

ForgeDNA must serialize reproducibly and support field-local generation and rerolls across runs. Hash-map iteration, timestamps, process state, UI ordering, and unrelated field rerolls must not alter a field's deterministic inputs.

## Decision

ForgeDNA uses strict compact JSON with declared struct field order and sorted `BTreeMap` keys. UUIDs serialize as lowercase hyphenated text, root seeds as exactly 16 lowercase hexadecimal digits, identifiers use normalized lowercase ASCII slugs, and constraint values are trimmed. The canonical decoder rejects valid JSON that is not already in this representation.

Each field stream uses BLAKE3 derive-key with context `character-forge/forge-dna/field-stream/v1`. Its stable input consists of the schema/generator/content/recipe versions, root seed, universe preset, normalized creative axes and constraints, target field key and nonce, plus sorted explicitly declared dependency field keys and nonces. It excludes timestamps, process state, global randomness, UI ordering, and unrelated field nonces.

Single-field reroll advances the target and its transitive explicitly dependent unlocked fields. Locked dependents and their descendants are preserved. Reroll-all advances every unlocked field. All planned nonce updates and generated values commit atomically after successful generation.

## Consequences

- Changing canonical field serialization or stream inputs requires a compatible version/context change and updated determinism fixtures.
- BLAKE3, serde, and serde_json are direct Rust dependencies of the pure contracts/core crates; no runtime service or global RNG is introduced.
- The field generator remains supplied by callers. This decision does not define Character/Quest content or add persistence/UI workflows.
