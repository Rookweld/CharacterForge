# THE FORGE B1 — PLACE CORE Mission Report

**Status: ACCEPTED FOR CONTINUED DEVELOPMENT WITH KNOWN P3 UI DEBT**

Standalone Location Forge is implemented and the automated gates pass. Human operator smoke passed the requested functional workflows. One remaining non-blocking Workbench feedback defect is explicitly accepted for continued development and tracked as UI-001 in GAZINGA/KNOWN_ISSUES.md.

## Delivered

- Strongly typed Location contract.
- Stable profiles: settlement, district, building, dungeon, ruins, wilderness, landmark, interior, site, other.
- Independent GenerationKind::Location with generator/content version 1.
- 12 stable Location fields.
- Deterministic generation across all 11 core presets.
- Lock/unlock, field reroll, reroll-all-unlocked, manual-edit origin.
- Undo/redo and persistent history.
- Candidate save, favorite, Canon promotion, Canon revision.
- Close/reopen persistence.
- JSON/Markdown entity export.
- forgeproject v1 and backup/restore roundtrip coverage.
- Desktop Workbench Location mode and Location library.
- No Hometown / Forge Further behavior added in B1.

## Deterministic and content evidence

- Preset sweep: 256 seeds × 11 presets = 2,816 candidates; validity, exact replay, and no full authored-field duplicates inside each preset sweep passed.
- Chaos Blend sweep: 1,024 seeds; zero authored-field tuple duplicates, 445 distinct names, all ten Location profiles, and all ten source presets represented.
- Human-readable sample audit: docs/quality/THE_FORGE_B1_LOCATION_SAMPLES.md.
- Sample audit contains 40 non-Chaos + 12 Chaos Locations.

## Verification

After B1.R2:

- Rust workspace: 131 passed, 0 failed, 1 ignored existing fixture-writer test.
- Frontend: 40 passed.
- 1,000-sequence state gauntlet: PASS.
- scripts/verify.ps1: PASS with isolated CARGO_TARGET_DIR.
- Frontend format/lint/typecheck/build: PASS.
- cargo fmt --check: PASS.
- strict Clippy: PASS.
- Cargo metadata: PASS.
- optimized Tauri Windows no-bundle build: PASS.
- git diff --check: PASS.
- Required npm test attempt remains blocked before test execution by the machine's broken global npm launcher; repository-managed corepack/pnpm tests pass.

## B1 remediations

### B1.R1 — Location Name lock indicator

Source defect found:
- Character Name key had been hardcoded as 03-name in Workbench rendering.
- Location Name uses 02-name.

Automated remediation passes and uses the shared field-control resolver.

Report:
docs/missions/THE_FORGE_B1_R1_LOCATION_NAME_LOCK_INDICATOR_REPORT.md

### B1.R2 — Location Profile action-key parity

Source defect found:
- payload property: location_profile
- typed action field: profile
- ForgeDNA control key: 01-location-profile

The Workbench now uses the typed action key for profile actions while retaining the payload property and stable DNA key.

Report:
docs/missions/THE_FORGE_B1_R2_LOCATION_PROFILE_ACTION_KEY_PARITY_REPORT.md

## Human operator result

The full requested B1 functional smoke passed.

Observed remaining issue:

- Location lock state can functionally hold while the visible Locked / Unlock indicator remains unreliable in the operator environment.

The operator explicitly approved continuing development rather than blocking B2 on this cosmetic/state-feedback issue.

This is tracked as:

GAZINGA/KNOWN_ISSUES.md — UI-001

Acceptance conditions for continued development:

- functional lock remains enforced
- rerolls respect locked values/nonces
- persistence is correct
- no data loss or silent unlock occurs

The issue must be revisited before the next release-quality gate.

## Git / scope

B1 does not include:

- Character → Forge Hometown
- Context Compiler
- Context Recipe persistence
- Context Snapshot persistence
- Forge Result Bundle persistence
- Faction / Item / Creature Forge
- Visual generation
- engine adapters

Those remain later bounded missions.

**Exit:** B1 is accepted as the functional Location baseline for B2 development, with known UI-001 debt still open.
