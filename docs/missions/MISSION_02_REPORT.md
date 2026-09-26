# Mission 02 — SOUL

## Status

**PASS** — Mission 02 exit gate satisfied. Mission 03 has not started.

## Scope delivered

- Added Character-specific fields to the shared typed contract: title, faction hint, signature gear, relationship hook, and backstory hook. Required personality, skills, abilities, and story-hook list cardinalities are validated.
- Added the versioned original content corpus at `content/characters/v1/core.json` for all eleven canonical v1 presets. The corpus is compiled into the Rust core, validated at load, and uses the existing `ContentVersion(1)`. ForgeDNA wire format and version tuple behavior were not changed; no dependencies were added.
- Added a deterministic Character generation pipeline in `forge-app-core`. It normalizes and validates DNA, derives field streams, selects weighted content influenced by creative axes, filters supported constraints, composes the Character candidate, and attaches versioned DNA and provenance.
- Added structured errors for unsupported presets, malformed/unsupported constraints, incompatible filters, content errors, and unsupported content versions.
- Added field reroll, reroll-all-unlocked, lock/unlock, and explicit per-field generated/user-modified origin metadata. Reroll-all protects manual edits while preserving the caller's lock configuration. Relationship, backstory, and story hooks explicitly depend on desire; rerolls use Mission 01's dependency and locked-field firewall behavior.
- Added a developer CLI harness that accepts a seed, preset, and repeated creative-axis overrides and emits candidate plus ForgeDNA as JSON. Example: `cargo run -p forge-app-core --example character_forge -- --seed 42 --preset cyberpunk --axis technology=100`.
- Added architecture notes. No UI, persistence, AI, provider runtime, Quest generation, or Mission 03 work was added.

## Quality evidence

- 33 `forge-app-core` tests and 10 `forge-contracts` tests passed: **43 Rust unit tests total**, 0 failed. Workspace and doc-test targets also completed successfully; doc-test count was 0.
- Golden seed 42 across every core preset produced these archetypes:

| Preset | Generated archetype |
|---|---|
| dark-fantasy | Lantern Warden |
| pirate | Mutiny-Script Quartermaster |
| steampunk | Pressure-Ledger Inspector |
| cyberpunk | Civic Algorithm Auditor |
| nordic-myth | Boundary-Stone Keeper |
| post-apocalyptic | Water-Ration Mediator |
| occult | Missing-Persons Archivist |
| monster-hunter | Molt-Pattern Tracker |
| rogue-assassin | Ledger-Page Infiltrator |
| contemporary-weird | Municipal Oddity Clerk |
| chaos-blend | Tide-Court Signal Witch |

- A 64-seed sweep for each preset verifies at least 24 distinct names and at least 40 distinct name/archetype pairs. Each generated candidate validates, including list cardinalities.
- Repeated generation with identical DNA is byte-for-byte equivalent as a Rust value and preserves caller DNA bytes. Changing the seed changes candidate output. JSON serialization is stable.
- Creative-axis behavior is tested at extreme values. In a 256-seed cyberpunk sample, Technology 100 selected the technology-biased Memory-Bank Salvager archetype more often than Technology 0. Chaos Blend passed generation/validation sweeps at Chaos 0 and 100 and several other extreme axes, with at least three distinct archetypes in a 32-seed sample.
- Reroll tests cover target-field changes, dependency-triggered desire dependents, a locked dependent firewall, explicit locks, reroll-all, manual-edit preservation/provenance, and unchanged locked values.
- Invalid presets and constraints return structured errors. Impossible filters fail without mutating a candidate. Unicode inspiration input and `u64::MAX` seed generate successfully.
- Supported semantic filters (`must_be_non_human`, `urban_only`, `no_magic`, and `no_firearms`) were exercised against tagged content choices across seed samples.

## Commands and outcomes

- `cargo check --workspace` — PASS before the new Character modules were registered; this only checked the prior workspace module graph.
- `cargo check -p forge-app-core` — first run exposed a wrong deterministic ID conversion and preset map lookup type; both were corrected. The rerun passed.
- `cargo test -p forge-app-core` — initial runs exposed a missing test import, an invalid `order` axis in the corpus, then a reroll-all manual-edit preservation defect. The import/content were corrected and reroll-all now protects user-edited fields. Final focused run: **33 passed, 0 failed**.
- `cargo test -p forge-contracts` — PASS, 10 passed, 0 failed.
- `cargo fmt --all` — PASS; formatted Rust sources.
- `cargo fmt --all -- --check` — PASS.
- `cargo clippy --workspace --all-targets -- -D warnings` — initial run reported three style warnings; corrected. Final run PASS.
- `./scripts/verify.ps1` — PASS after final source and test changes. Frontend formatting, lint, typecheck, React test (1/1), frontend production build, Rust formatting, clippy, all workspace tests, and optimized Tauri no-bundle build completed successfully. Tauri output: `target/release/character-forge-desktop.exe`.
- `$presets='dark-fantasy','pirate','steampunk','cyberpunk','nordic-myth','post-apocalyptic','occult','monster-hunter','rogue-assassin','contemporary-weird','chaos-blend'; foreach($preset in $presets){ cargo run -q -p forge-app-core --example character_forge -- --seed 42 --preset $preset }` — PASS for all eleven presets; output parsed as JSON and recorded in the quality matrix above.
- `cargo run -q -p forge-app-core --example character_forge -- --seed 42 --preset cyberpunk --axis technology=100` — PASS; emitted a valid candidate and ForgeDNA JSON.
- `cargo metadata --no-deps --format-version 1` — PASS.
- `git diff --check` — PASS (exit code 0).

## Repository state at closeout

- Branch: `master`.
- HEAD: `573f81a1cb0293fc45983c326e0657987ff669fd` (`test: harden Forge DNA dependency invariants`).
- Working tree: dirty; Mission 02 implementation and documentation are uncommitted. `git diff --check` reported no whitespace errors. No existing user changes were discarded.

Mission 02 is closed. Stop here; do not begin Mission 03.
