# PROJECT STATE

## Character Forge / Quest Forge — v1.0 FOUNDATION

> Codex: maintain this file after every mission. Keep it concise; do not turn it into a build log.

**Current mission:** 03 — CALLING
**Current status:** PARTIAL — Quest Forge prototype added; mission gates remain open
**Target release:** v1.0.0 FOUNDATION
**Last committed baseline:** `8395f07647d823681f1fcd4c9fec628a7f324942` (`wip: checkpoint Mission 03 declarative Quest Forge`)
**Branch:** `master` (Remediation II changes are currently uncommitted in the working tree)
**Schema version:** 1
**Generator version:** 1
**Content version:** 3 for new Quest recipes; Quest v2 replay retained; Character v1/v2 unchanged

## Architecture decisions currently locked

- Desktop: Tauri 2
- Frontend: React, TypeScript, Vite; strict TypeScript configuration
- Core/application logic: Rust; four core crates plus desktop shell
- Storage: SQLite is the planned source of truth; no persistence implementation in Mission 00
- Generation: versioned ForgeDNA contracts and deterministic field-stream/reroll infrastructure underpin the Mission 02 deterministic Character Forge core; no generation UI
- AI dependency in v1.0: none
- Project format: `.forgeproject`
- Frontend package manager: pnpm 11.25.0; npm launcher is misconfigured
- UI must not own persistence; generator/domain must not depend on Tauri UI; persistence must not depend on visual components

## Completed mission gates

- Repository structure, architecture documentation, package manifests and lockfiles, React desktop shell, strict TypeScript, formatter/linter, and Mission 00 report are present.
- Frontend formatting, lint, typecheck, one React UI test, and production frontend build passed.
- At Mission 00 closeout, Rust formatting, clippy with warnings denied, workspace test harness, and Cargo metadata passed; Mission 01 has since added 27 Rust tests.
- Tauri optimized no-bundle build passed and produced the Windows executable.
- Human operator independently launched the built application from the normal interactive Windows desktop and verified a visible usable Tauri/WebView window, the expected title, and rendered React/Vite branding. This satisfies the Mission 00 GUI smoke gate.
- Mission 01 added typed, versioned project/entity, Character/Quest, provenance, recipe, ForgeDNA, and validation contracts. Canonical serialization and BLAKE3 field streams are deterministic over explicit inputs; lock/reroll operations preserve locked and unrelated fields and commit atomically.
- Mission 01 verification passed: frontend checks and React test, Rust formatting, clippy with warnings denied, 27 workspace tests, Cargo metadata, and optimized Tauri build. The consolidated script passed using a temporary Cargo target directory because the existing release executable was held open.
- Pre-Mission-02 audit passed: explicit three-node cycle rejection/rollback and locked dependency-firewall tests; Rust formatting, clippy, consolidated verification, and 29 workspace tests passed.
- Mission 02 added the typed Character candidate pipeline, declarative original content for all eleven v1 presets, axis-weighted deterministic selection, supported Character constraints, provenance-preserving manual edits, field rerolls, lock-aware reroll-all, and a JSON CLI harness. At that closeout the corpus used content version 1; the later diversity audit added v2 while preserving v1 replay.
- Mission 02 quality tests cover same-DNA reproducibility, no input DNA mutation, fixed seed outputs across all presets, 64-seed repetition sweeps, creative-axis extremes and distribution shifts, Chaos Blend, field cardinalities, structured invalid-input errors, dependency rerolls/firewalls, explicit locks, manual edits, and stable JSON serialization.
- Mission 02 verification passed: `./scripts/verify.ps1` (frontend checks/test/build, Rust fmt/clippy/workspace tests, optimized Tauri build); 43 Rust unit tests passed (33 app-core, 10 contracts); `git diff --check` passed. Details are in `docs/missions/MISSION_02_REPORT.md`.
- Mission 03 prototype adds a deterministic Quest candidate pipeline, CLI, all eleven core preset identifiers, and shared lock/reroll integration. Current quality evidence shows substantial exact multi-field similarity (up to 13/14 matching fields in seed sweeps), sparse three-choice fields, incomplete axis semantics, and coherence defects in reviewed preset samples. Mission 03 is **PARTIAL** and must not be closed.
- Mission 03 remediation adds `content/quests/v3/core.json`, a deterministic declarative Quest path with semantic threads, clue, and briefing. Quest v2 generation remains available for replay; new Quest DNA uses v3. Character content versions 1 and 2 are unchanged.
- Mission 03 Remediation II remains PARTIAL. Chaos Blend v3 now records zero exact clones, 23 >=12/14 pairs, 1,162 >=10/14 pairs, 18,071 >=8/14 pairs, mean 3.409, median 3, and max 13 over 1,024 seeds. Mythic shifts the intended family from 99/256 to 194/256; Chaos moves secondary location source from spine in 256/256 low cases to compatible alternate in 256/256 high cases while primary-thread counts remain similar. Four-value ordinary identity pools, remaining same-spine similarity, Chaos seam review, broader semantic constraint policy, and human acceptance remain open. Quest v2 replay is preserved; unfinished v3 is refined in place without a v4. No Mission 04 work has begun. See `docs/QUEST_CONTENT_QUALITY_REPORT.md` and `docs/missions/MISSION_03_REPORT.md`.
- Pre-Mission-03 SOUL diversity audit expanded the current corpus to content version 2 while preserving the v1 corpus and ForgeDNA stream/reroll semantics. Deterministic field-level, archetype-cluster, Chaos Blend, controlled-axis, and UTF-8 evidence is recorded in `docs/CONTENT_QUALITY_REPORT.md`; Mission 02 remains PASS after audit verification.

## Mission 00 closeout

- **GUI result:** VERIFIED GUI SUCCESS by operator verification. Automated Codex GUI inspection could not observe the operator's interactive desktop; the earlier 16×16 event-target-only result described the Codex execution/inspection context and did not represent the actual desktop launch. The prior APPLICATION FAILURE classification is superseded and incorrect.
- All Mission 00 exit criteria are satisfied. Mission 00 is closed.

## Mission 01 closeout

- All Mission 01 exit criteria are satisfied. Mission 01 is closed.
- **Architecture:** canonical ForgeDNA is compact JSON with deterministic map order, normalized identifiers/constraints, and supported version tuple 1/1/1/1. Per-field streams use BLAKE3 derive-key context `character-forge/forge-dna/field-stream/v1` and only explicit dependencies. See `docs/adr/0002-forge-dna-and-field-streams.md`.
- **Git at Mission 01 closeout:** branch `master`, HEAD `573f81a1cb0293fc45983c326e0657987ff669fd` (`test: harden Forge DNA dependency invariants`).
- **Blockers/P0/P1:** none found within Mission 01 scope.

## Deferred by design

Persistence schema/migrations, AI/provider runtime, World Forge, marketplace, cloud sync, expansions, and all other later-mission features. Quest Forge remains PARTIAL; do not begin Mission 04 until the remaining Mission 03 gates are resolved. See `docs/missions/MISSION_03_REPORT.md`.

## Handoff

Mission 00 report: `docs/missions/MISSION_00_REPORT.md`. Mission 01 report: `docs/missions/MISSION_01_REPORT.md`. Mission 02 report: `docs/missions/MISSION_02_REPORT.md`. Mission 03 report: `docs/missions/MISSION_03_REPORT.md`. Do not begin Mission 04 until Mission 03's open gates are resolved.
