# THE FORGE B5 — Creature Core mission report

**Status: PASS — B5 automated exit gates satisfied**

## Baseline and scope

Started from the requested `master` HEAD `ccef11a8eae698bdc004e1bf3e4c678a3ba8db5b`, including accepted Item Core. The work adds standalone deterministic Creature Forge as the sixth authoring mode. It adds no Creature Forge Further action, World Forge, relationships, embedded foreign IDs, runtime/combat statistics, provider, dependency, migration, engine adapter, or later mission. No staging, commit, tag, push or publication is authorized or performed.

The four existing untracked operator/export/log files are outside mission ownership and remain untouched. Their hashes are recorded below.

## Delivered

- Typed `Creature` with shared envelope and thirteen payload fields; name is the fourteenth authored field in the envelope. Required strings/nonempty hook list, typed Profile, unknown-field rejection, Unicode and envelope type validation are explicit. Payload `origin` is fictional history, separate from envelope creation origin.
- Eleven kebab-case profiles: beast, monster, undead, construct, aberration, spirit, elemental, humanoid-creature, swarm, boss, other. New `GenerationKind::Creature` and independent generator/content versions 1/1. Existing kinds/versions and field-stream derivation are unchanged.
- `CreatureCandidate` records complete DNA, all fourteen field origins, deterministic project-bound identity and generation metadata. Exact control/dependency/origin sets and equality of Candidate/provenance DNA are validated. Generation uses explicit DNA, deterministic per-field streams and authored corpus, with no clock, I/O or mutable global RNG.
- Profile-specific anatomy, habitat, behavior, diet/energy ecology, threat, descriptive abilities, countermeasure, origin, tracking signs and resource material. Six original alternatives per major layer in each of ten non-Chaos presets; all eleven presets retained. Profile materially changes biological/behavioral meaning rather than only a label. Resource material describes safe collection/crafting/story use without numerical loot/combat tables.
- Three domain-separated Chaos source groups: physical/ecological (Name, Appearance, Habitat, Behavior, Ecology, Signs); threat/counterplay (Threat, Abilities, Weakness, Resource); origin/narrative (Origin, Secret, Hooks). Profile material remains compatible shared content. Source selection derives from seed/group, not generated field values, so no artificial dependency edges are claimed for it.
- Atomic locks, target/closure rerolls, reroll-all and typed manual edits. A locked dependent retains value/control/nonce. Manual fields survive unrelated/all rerolls; an explicit target reroll restores generated origin. Reroll-all increments each eligible nonce once. Field/shape/profile errors leave the prior candidate unchanged; every action synchronizes provenance DNA.
- CreatureCandidate/canonical Creature in schema-2 `StoredEntity` validation and lifecycle handling. Candidate save, favorite, Canon promotion, explicit revision, immutable snapshots, stale-write rejection, persistent action history, tags/notes, DNA/locks/origins and close/reopen use the existing store. Standalone Creature creates zero Relationships. No schema change.
- Tauri `forge_creature` and typed field adapters use app-core and shared session history. Creature is included in both historical working-copy and Canon revision helpers; complete-snapshot tests cover transitions/identity/locks and undo/redo branch disposal. B2 Hometown bundle/context handling is preserved.
- Sixth Workbench mode, typed Profile editor, fourteen controls, Creature library/detail, counts, save/favorite/Canon/revision/history and export actions. Shared key resolution uses actual suffix/control mapping. Real CreatureCandidate-shaped fixtures cover `02-name` and `01-creature-profile`, Locked/Unlock labels, `aria-pressed=true`, correct action keys, typed select values and atomic rejection of a synthetic invalid profile. Mock lock results also keep provenance DNA synchronized.
- Canonical JSON and readable Markdown include all fields; Creature `.forgeproject` export/import and Vault backup/restore are tested. Schema 2, archive v1, application 1.0.0 and previous entity deterministic versions remain unchanged. Existing keyboard/native-select and Reduced Motion behavior are inherited; no fresh native operator acceptance is claimed.

## Frozen field and dependency contract

| Action key | Control key |
|---|---|
| creature_profile | 01-creature-profile |
| name | 02-name |
| appearance | 03-appearance |
| habitat | 04-habitat |
| behavior | 05-behavior |
| ecology | 06-ecology |
| threat | 07-threat |
| abilities_summary | 08-abilities-summary |
| weakness | 09-weakness |
| origin | 10-origin |
| signs_of_presence | 11-signs-of-presence |
| loot_or_resource | 12-loot-or-resource |
| secret | 13-secret |
| story_hooks | 14-story-hooks |

Actual acyclic reads:

```text
creature_profile -> appearance, habitat, behavior, ecology, threat,
                    abilities_summary, weakness, origin,
                    signs_of_presence, loot_or_resource
```

Name, Secret and Story Hooks read their own streams and themed corpus. The Origin edge is intentional: fictional origin reads profile-specific genesis material. All ten profile-dependent fields read the current generated/reviewed Profile; the validated graph matches these reads exactly. Profile reroll invalidates this closure while preserving locked dependents and independent fields. Cycle/replaced graph, missing controls/origins, unsupported versions/preset/constraints and provenance mismatch fail.

## Quality evidence

- 2,816 Candidates (256 × 11): complete, valid, exact replay, no DNA input mutation, zero full-content clones within a preset.
- 1,024 Chaos seeds: valid/exact replay, zero full-content duplicates, all eleven profiles, 640 source patterns, 1,013 mixed-source results. All ten eligible corpora occur in each of the three groups. Maximum group/source share: 121/1,024 = 11.82%.
- Profile counts: Beast 106; Monster 83; Undead 75; Construct 94; Aberration 104; Spirit 117; Elemental 85; Humanoid Creature 95; Swarm 85; Boss 100; Other 80.
- 52 complete generated samples: **17 usable as-is, 35 minor edit, 0 weak**. This is Codex generated/content-author review, not independent human playtesting. The fixed sample seeds show ten profiles; all eleven are covered by separate profile/axis and Chaos tests. Repeated local-refuge/boundary/guide/survey plots remain a bounded-corpus limitation, not hidden by the clone check. Minor notes request explicit remembered duties/events, witness agreements, Boss counterplay, sharper Other visuals or bridges between Chaos cultural sites.
- Final editorial repairs removed attribution filler, corrected anatomy sentence punctuation/articles, replaced inappropriate single-body anatomy modifiers for Spirit/Swarm/Elemental, and removed doubled shelter wording. The packet captures UTF-8 directly; local shell decoding damage was avoided without changing product Unicode semantics.
- Complete records, classifications, repetition metrics, source table and reproduction command: [THE_FORGE_B5_CREATURE_SAMPLES.md](../quality/THE_FORGE_B5_CREATURE_SAMPLES.md).

## Verification

All required final gates passed against the final implementation, with `CARGO_TARGET_DIR=C:\dev\CharacterForge\target\mission-b5-verify` and `RUST_TEST_THREADS=1` for the final Rust suites:

- Frontend formatting/lint/TypeScript typecheck/production build: **PASS**.
- Frontend tests: **52 passed**.
- Rust formatting and strict workspace/all-target Clippy (`-D warnings`): **PASS**.
- Rust tests: **182 passed, 1 ignored** (existing one-time frozen fixture writer). Suite counts: desktop 18; app-core 89; contracts 17; export 24; storage 34 passed.
- `cargo test --workspace --no-fail-fast`: **PASS**, final explicit run in the same serial configuration.
- Existing 1,000-sequence deterministic state gauntlet: **PASS** in both final consolidated and explicit no-fail-fast runs.
- Full offline Cargo metadata: **PASS**, saved in `target/b5-cargo-metadata.json`.
- `scripts/verify.ps1`: **PASS**; complete log `target/b5-serial-verify.log`.
- Optimized Tauri `build --no-bundle`: **PASS**. Exact executable: `C:\dev\CharacterForge\target\mission-b5-verify\release\character-forge-desktop.exe`; ProductVersion/FileVersion 1.0.0 observed.
- `git diff --check`: **PASS** after closeout. New files/state also pass explicit UTF-8/replacement-character/trailing-whitespace checks.

Final explicit no-fail-fast log: `target/b5-serial-workspace.log`. Build/test logs and sample-capture scratch data are ignored local evidence, not release artifacts. The dedicated B5 target preserves the existing B4/default executable; no user process was terminated. No independent native operator smoke is claimed.

The machine's `npm test` was attempted as required by the working agreement; its launcher cannot locate `C:\Users\pontu\AppData\Roaming\npm\node_modules\npm\bin\npm-cli.js`. This is an environment limitation. Configured pnpm tests supply the frontend test evidence; no npm PASS is claimed.

Two parallel consolidated runs reproduced the documented Relationship lifecycle timestamp fluctuation: `updated_at_unix_ms` 1790807300077 versus 1790807300078. Failure evidence is retained in `target/b5-final-verify.log` and `target/b5-complete-verify.log`. The correctly filtered isolated test passed (one test, not an empty filter) in `target/b5-relationship-isolated-final.log`. Relationship code and its baseline assertion were not modified. The final consolidated and no-fail-fast gates run with `RUST_TEST_THREADS=1`, explicitly reducing timing interference while retaining every test/assertion. The prior parallel no-fail-fast run passed 182 tests in `target/b5-final-workspace.log`. The timestamp fluctuation is not claimed fixed by serial execution; final gate results/configuration are recorded above.

Early new-test failures were repaired: the copied lifecycle assertion expected thirteen origins rather than Creature's fourteen; the first Chaos metric used fields from only two source groups. The corrected metric samples Name/Threat/Secret from all three groups. These repairs did not weaken baseline regressions or change old generator behavior.

## Residual limitations

- Existing P3 UI-001 remains open; automated Creature controls do not close the operator-observed Location visual-feedback issue.
- Mission 08's accepted incomplete automated transitive advisory scan remains residual maintenance/release verification risk. No complete graph-clean claim, new advisory scan or tooling installation is made.
- No fresh independent native operator smoke or human content playtest is claimed. B5 permits an automated PASS when its behavior/data-integrity gates pass and no P0/P1/P2 or migration/security/product-decision gate remains.
- The corpus favors local observable encounters and repeats some motifs. The sample packet records the creative limits and required minor edits explicitly.

## Exit-gate review

| Criterion | Evidence/status |
|---|---|
| Typed Creature / fourteen fields / eleven stable profiles | PASS — Contract and invalid-shape/key tests |
| Deterministic standalone generation / all eleven presets / axes | PASS — Replay/sweep/profile/axis tests |
| Lock/reroll/edit and dependency firewall | PASS — Core action/nonce/all-lock/atomic tests |
| Undo/redo and historical/Canon adapters | PASS — Desktop complete-snapshot regressions |
| Candidate/favorite/Canon/revision/history/stale writes/reopen | PASS — On-disk Creature storage tests |
| JSON/Markdown / project export-import / Vault restore | PASS — Creature export roundtrip regression |
| Workbench/library/detail / Name/Profile parity | PASS — 52 frontend tests |
| 2,816 sweep / 1,024 Chaos / 52-sample packet | PASS — Quality evidence above |
| Prior entities/Relationships/B2 preserved | PASS — Full workspace and frontend regressions |
| Required full verification / optimized build / diff check | PASS |
| Known outstanding P0/P1/P2 | PASS — 0/0/0 in B5; existing residual limitations documented above |

## Mission-owned files

- `crates/contracts/src/domain.rs`, `generation.rs`, `lib.rs`
- `crates/app-core/src/creature.rs`, `creature_content.rs`, `lib.rs`; `examples/creature_samples.rs`
- `crates/storage/src/lib.rs`, `tests.rs`; `crates/export/src/lib.rs`
- `apps/desktop/src-tauri/src/lib.rs`; `apps/desktop/src/api.ts`, `App.tsx`; `tests/App.test.tsx`
- This report, the Creature sample packet, `GAZINGA/DISCOVERY_STATE.md`, `GAZINGA/PROJECT_STATE.md`

## Preservation and repository state

| Unrelated file | Starting SHA-256 |
|---|---|
| The Ashen Claim.json | 32BC3B62555F32A7AB4A8CA936B52F4979305C08027937274E636E6E8033A844 |
| The Ashen Claim.md | A298F7A2DAFDFB4AC4FE354607C67CB565B0CFD38D9D45D9085D1AAA8FEDFC0D |
| mission09-final-verify.log | 958E927350F940EA6075A0E2A4452E922A1C780ADF9EE97052EC59C3DD8FE6AD |
| mission09-rust-test.log | 8E16F7D11EF7432BB341CC22512D59A384321AAD0CFFCF10F9133B315234F0D2 |

Branch `master`; starting/current HEAD `ccef11a8eae698bdc004e1bf3e4c678a3ba8db5b`. Mission changes are unstaged/uncommitted; the four unrelated files remain untracked. All four final SHA-256 values equal the starting values above. No commit/tag/push/publication occurred.

**B5 exit gate: PASS. STOP. Creature Forge Further, World Forge and later missions remain unstarted.**
