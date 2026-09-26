# Mission 03 — CALLING remediation report

## Status and scope

**PARTIAL.** The Quest Forge improvement is in place, but the required exit gate is not met. Mission 04 — MEMORY is not started. No persistence or Character Forge redesign was added.

V2 generation remains available for compatibility. New recipes use v3. The shared ForgeDNA serialization shape, field-stream context, nonce semantics, locks, and dependency graph rules remain unchanged; the contracts crate now accepts content version 3. The pre-implementation HEAD was `5e84827e5605a18f3ff3b477daa86489d9055202` on `master`. At the start of this continuation the worktree had an untracked `content/quests/v3/core.json` from in-progress work and therefore was not clean. That artifact was preserved. No commit was created.

## Changes

- Added a versioned declarative corpus for all eleven presets, with semantic thread tags and distinct pools for givers, premises, objectives, stakes, threats, complications, clues, twists, rewards, consequences, moral pressures, locations, titles, and follow-ups.
- Added deterministic thread and per-field selection through existing FieldStreams. V3 field selection is bound to an explicit Frame dependency. New candidates include clue and briefing fields; the shared Quest entity contract remains unchanged.
- Added structured Quest constraint validation. `inspiration` is supported and appears in the briefing. Unsupported and Character-only constraints fail explicitly.
- Added corpus load validation and targeted v3 generation, reroll, lock, constraint, deterministic, and Unicode serialization tests.
- Kept the version-2 generator path for replay.
- Expanded the audit example to sweep every preset, compare the original 14 public fields, count field diversity and thread clusters, compare controlled axis distributions, and emit representative samples.

## Quality results

Full measurements and representative samples are in [QUEST_CONTENT_QUALITY_REPORT.md](../QUEST_CONTENT_QUALITY_REPORT.md).

| Chaos Blend metric (1,024 seeds; 523,776 pairs) | v2 baseline | v3 |
|---|---:|---:|
| Exact expanded clones | 0 | 4 |
| Pairs >=10/14 | 2,083 | 15,990 |
| Pairs >=8/14 | 30,149 | 88,856 |
| Mean shared fields | 4.668 | 3.528 |
| Maximum shared fields | 13 | 14 |
| Unique expanded candidates | 1,024 | 1,020 |

The average overlap improved, while high-overlap pairs and exact clones increased. Most narrative fields have four observed values per preset. Monster Hunter produced one exact expanded duplicate in its 256-seed sweep. This is not a successful similarity remediation.

Controlled 256-seed Chaos Blend axis comparisons shifted thread counts for Bleak, Epic, Ambiguity, Surreal, Technology, and Mysticism. Mythic and Chaos showed no measurable distribution shift. Those two axis claims remain unproven. Constraint coverage remains deliberately bounded to `inspiration`; the canonical Character-only constraints are rejected, but broader Quest content filters do not yet exist. Human coherence and wording review found coherent chains in representative samples but also repeated wording, tag-like quest types, and gaps between some clues/objectives/opposition. A broad human acceptance review remains outstanding.

## Verification

- `cargo test -p forge-app-core quest::tests`: 9 passed, 0 failed.
- `cargo test -p forge-app-core quest_content::tests`: 1 passed, 0 failed.
- `cargo run -q -p forge-app-core --example quest_quality_audit`: PASS; 256 seeds for each of ten ordinary presets, 1,024 Chaos Blend seeds, v2/v3 same-basis comparison, 256 low/high controlled pairs per canonical axis, samples for all presets, axis extremes, and an inspiration-constrained sample.
- `cargo fmt --all -- --check`: PASS.
- `cargo clippy --workspace --all-targets -- -D warnings`: PASS.
- `cargo test --workspace`: 58 passed (48 app-core, 10 contracts), 0 failed.
- `cargo metadata --no-deps --format-version 1`: PASS.
- `./scripts/verify.ps1`: PASS; frontend format/lint/typecheck, 1 frontend test, frontend build, Rust checks/tests, and optimized Tauri no-bundle build passed.
- `git diff --check`: pending final state check.

## Exit gate re-evaluation

Mission 03 remains **PARTIAL**. Declarative data and v3 clue/briefing are present, and determinism/verification pass. Material gates remain: reduce high-overlap Chaos Blend clustering; deepen field pools; address expanded exact clones; establish every relevant axis with controlled distribution evidence; provide a comprehensively reviewed Quest constraint matrix; and complete human review of coherence and wording. Do not begin Mission 04 until those gates are closed.
