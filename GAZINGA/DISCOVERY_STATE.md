# THE FORGE — DISCOVERY STATE

## Current Phase

THE_FORGE_B2_R1_REVIEWED_BUNDLE_PROVENANCE — closed

## Status

**B2 PASS — B2.R1 reviewed bundle provenance blocker resolved**

Independent review found that Location lock/reroll could change the working ForgeDNA while the cached bundle recipe retained original DNA, causing atomic Save Bundle to reject a legitimately shaped Hometown. B2.R1 now finalizes bundle provenance from the exact reviewed candidate while preserving frozen source context/hash and stable entity/relationship identity. Focused lock, reroll-all, and reroll persistence/reopen tests pass; frontend lock-before-save regression passes. Full verification passes: 143 Rust tests, 1 existing fixture-writer ignored; 42 frontend tests; formatting, lint, typecheck, production build, strict Clippy, workspace tests, optimized isolated-target Tauri build, and `scripts/verify.ps1`. `git diff --check` passes. Existing B1 Workbench feedback debt remains:

- functional Location locks hold
- lock visual state can fail to show Locked / Unlock correctly
- tracked in GAZINGA/KNOWN_ISSUES.md as UI-001
- must be revisited before a release-quality gate

## Current Repository State

- branch: master
- starting baseline: `48905176ef9fe86799c8a2cfbc3dbc6eef177b4c` (`docs: prepare Forge Hometown mission`)
- B2 implementation and closeout are uncommitted and unstaged
- v1.0.0 FOUNDATION release baseline remains protected
- schema version: 2
- Character / Quest compatibility remains green
- Location standalone Forge is implemented
- first-class Relationships are implemented
- Forge Hometown is implemented and verified for saved generated Candidate and Canon Characters

Unrelated files to preserve and never absorb:

- The Ashen Claim.json
- The Ashen Claim.md
- mission09-final-verify.log
- mission09-rust-test.log

## Verified B1 Result

Location now supports:

- typed Location payload
- 10 Location profiles
- 12 deterministic fields
- all 11 core presets
- ForgeDNA / field streams
- locks / rerolls / reroll-all
- manual edits / origins
- undo / redo
- Candidate save
- favorite
- Canon promotion / revision
- history
- JSON / Markdown export
- close / reopen
- Vault / forgeproject v1 roundtrip
- desktop Workbench Location mode and library

Automated evidence after R2:

- 131 Rust tests passed
- 40 frontend tests passed
- 1 existing fixture-writer test ignored
- 1,000-sequence state gauntlet passed
- scripts/verify.ps1 PASS using isolated target
- strict Clippy PASS
- frontend format/lint/typecheck/build PASS
- optimized Tauri build PASS
- git diff --check PASS

Human smoke:
- all requested B1 workflows functioned
- Location Name/Profile locks functionally held
- visual locked feedback remained unreliable
- operator explicitly approved moving on

## Known Issue

See:

GAZINGA/KNOWN_ISSUES.md

UI-001 is non-blocking for B2 but must remain open.

## Mission Handoff

B2 and B2.R1 are PASS. The next entity mission has not started. Do not begin it without explicit authorization.

## B2 Design Decisions

For the first proof slice:

- Forge Further generation creates Candidate Entity + Candidate Relationship.
- Source Character is never mutated.
- Link Existing is NOT part of B2.
- Entity/Relationship Canon promotion UX remains separate and deferred.
- Existing forge_runs remains the durable operation/event backbone.
- B2 may extend it with structured context/output persistence rather than replacing historical forge_runs.
- Context compilation is bounded, deterministic, and read-only over the exact persisted Character record.
- exact source state used by generation must be frozen in provenance.

## Verified B2 Result

- Versioned action `character.forge-hometown` and recipe `character-hometown-v1` live in app-core.
- The compiler includes only the eight approved semantic facts, source identity/write version/Canon state, source body hash, and inherited preset/Creative Axes; domain-separated BLAKE3 hashes the context.
- Hometown generation uses the existing Location Forge and deterministic streams with the explicit `location.profile=settlement` constraint.
- One atomic schema-2 transaction saves Location Candidate, Candidate/Generated `born_in` relationship, and typed recipe/context/result provenance in `forge_runs`; stale source and tested failure points leave no partial writes.
- Close/reopen and `.forgeproject` export/import preserve the complete bundle and provenance.
- Desktop review supports explicit action seed, inspection/edit, discard, atomic Save Bundle, and Character Connections. Unsupported manual/imported provenance is disabled with an explanation.
- Quality evidence includes 24 deterministic samples across all 11 presets.
- Verification: 140 Rust tests PASS, 1 existing fixture-writer test ignored; 42 frontend tests PASS; `scripts/verify.ps1`, formatting, lint, typecheck, frontend production build, strict Clippy, Cargo metadata, optimized Tauri no-bundle build, and `git diff --check` PASS.
- B2.R1 resolved the independent-review P2: reviewed Hometown DNA is finalized from the exact current Location Candidate before atomic Save Bundle while frozen source context/hash remains unchanged. Existing UI-001 remains open as non-blocking P3 debt.

## Still Open

- Manual Link Existing Candidate vs Canon default
- bundled Entity + Relationship Canon promotion UX
- richer multi-source Forge
- generalized relationship cardinality UX
- Visual Canon
- World/Lore epistemic truth

None are required to complete B2.

## Deferred

- Faction Forge
- Item Forge
- Creature Forge
- full six-entity Forge Further matrix
- multi-source Forge
- Visual generation
- Game Schema Packs
- engine adapters
- World / Ability Forge
- Graph UI
- Timeline
- collaboration / cloud
- arbitrary plugin code execution

## Resume

Connect to Nullfeather Bridge and read this file first.

## Last Updated

2026-09-30
