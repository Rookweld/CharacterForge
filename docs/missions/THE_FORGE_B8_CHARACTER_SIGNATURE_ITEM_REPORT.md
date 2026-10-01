# THE FORGE B8 — Character → Signature Item report

## Exit status

**PASS — all B8 automated exit gates satisfied.**

Executed on 2026-10-01 using `GAZINGA/THE_FORGE_CODEX_LAUNCHER_PROMPT.txt` and `GAZINGA/missions/THE_FORGE_B8_CHARACTER_SIGNATURE_ITEM.md`. Starting/current branch: `master`; starting/current HEAD: `ecdd143061d2e66b4dbfb5667f1fe5b944328235`. Initial worktree contained only four unrelated operator/export/log files. Existing accepted B2–B7 work was preserved.

## Implemented scope

- Action `character.forge-signature-item` v1, intent Signature Item, context recipe `character-signature-item-v1` v1. Exactly one saved Character → one Item Candidate, no supporting Candidates, one Generated Candidate `owns` relationship.
- Read-only bounded context captures fifteen exact authored Character facts: name, title, archetype, role, appearance, personality, desire, fear, flaw, signature_gear, skills, abilities, relationship_hook, backstory_hook and secret. It records source ID/write version/Canon state/kind/generator/content versions, inherited preset/axes, exact canonical source body hash and context hash. Facts are at most 32 KiB, with at most sixteen entries per list. Unsupported provenance is rejected without invented defaults; no relationship/graph/project traversal occurs.
- Canonical ItemForge consumes one canonical typed optional context constraint. Principal gear nouns guide Profile; decorations do not replace the main object. Appearance/Purpose/Properties/Function/Drawback and provenance/hooks use frozen Character intent. Character skills and abilities are explicitly separated from powers granted by an Item. Name is not forced to gear; old name/rarity/condition and preset origin/history content are retained. No game-stat system is added.
- Stable target root seed derives from action ID/version, context hash and explicit action seed. Exact preset and nonneutral axes inheritance, stable Item/Owns identities, same-input replay and different-seed variation are tested. No hidden time changes generation.
- Review uses normal typed Item Name/Profile controls, lock/reroll/reroll-all/edit and session undo/redo. The finalizer accepts the current backend reviewed Item Candidate, synchronizes exact target DNA into the typed operation recipe, and freezes original source context/hash/action/seed and Item/relationship identities/endpoints/mapping. ID/type/project/origin/state/action/context/mapping/provenance drift rejects.
- Save Bundle atomically inserts the reviewed Item, Candidate Owns and immutable typed `forge_runs` recipe/result. The transaction checks source version, recompiles the exact persisted context, and checks source version again immediately before commit. Faults after primary, relationship and operation inserts fully roll back; stale source and duplicate save reject.
- Saved deterministic Candidate/Canon Character exposes **Forge Their Signature Item**. Manual/imported/unsupported provenance has a disabled action and explanation. Shared review displays source, frozen facts/hash, explicit seed, reviewed Item name and proposed Owns. Discard persists nothing; stale save leaves review pending. Dedicated save refreshes Items, without automatic Canon promotion.
- Universal B6 Connections shows **Owns** from Character and **Owned by** from Item. Explicit relationship promotion/archive/history are independent of both entities; one edge serves both views. Frozen operation/source/entity snapshots remain unchanged by relationship lifecycle.

## Compatibility and scope

Database schema **2**, forgeproject archive **v1**, application **1.0.0**, existing generator/content/protocol versions remain unchanged. No dependencies added. The original standalone Item field-generator body is byte-identical to HEAD after private function-name normalization; empty constraints follow that exact path without extra entropy consumption. Existing standalone goldens/replay/sweeps and B2/B2.R1/B7 regression tests remain intact. Existing Hometown and Faction bundle storage method bodies were independently compared to HEAD and are unchanged. New typed Signature Item operation and Owns survive archive export/import.

No Visual Forge, Link Existing, multi-source Forge, Creature/Item/Faction reverse Forge Further, new relationship vocabulary, bundled Canon promotion or later mission was implemented. No reset, staging, commit, tag, push, publication or user-process termination occurred. The FOUNDATION portable ZIP was not replaced.

## Verification evidence

| Gate | Evidence/result |
| --- | --- |
| Frontend format / lint / TypeScript | **PASS**, final consolidated run |
| Frontend tests | **90 PASS**, final consolidated run |
| Frontend production build | **PASS** |
| Rust format / strict workspace all-target Clippy `-D warnings` | **PASS** |
| Explicit `cargo test --workspace --no-fail-fast` | **208 PASS**, one existing fixture writer ignored; desktop 20, core 100, contracts 17, export 26, storage 45 |
| Existing 1,000-sequence / 32,000-operation gauntlet | **PASS** in explicit and consolidated workspace runs |
| Full `cargo metadata --format-version 1 --offline` | **PASS**, all five workspace packages 1.0.0 |
| `scripts/verify.ps1` | **PASS**, process exit 0; repeated all 208 Rust and 90 frontend tests |
| Optimized Tauri `build --no-bundle` | **PASS**, isolated release build |
| Independent sample executable replay | **PASS**, all 44 exact source/bundle snapshots identical |
| Final `git diff --check` / UTF-8 / preserved-file hashes | **PASS** |

Final optimized executable: `C:\dev\CharacterForge\target\mission-b8-verify\release\character-forge-desktop.exe`; ProductVersion/FileVersion **1.0.0**, 14,809,600 bytes. SHA-256: `0874325948C14D4795ABAC4A79908222C4322219884D8500A1C722A40826DA30`. Built from this verified uncommitted B8 worktree; no installer produced and no native operator smoke claimed.

Evidence files are ignored local artifacts under `target/`: `b8-focused-core.log`, `b8-focused-rust-final.log`, `b8-workspace-tests-final.log`, `b8-clippy.log`, `b8-cargo-metadata.json`, `b8-final-verify.log`, `b8-frontend-tests.log`, `b8-npm-test.log`, `b8-samples.json`. Isolated target: `C:\dev\CharacterForge\target\mission-b8-verify`.

Required `npm test` was attempted after TypeScript/TSX modifications. The existing machine npm shim cannot locate `C:\Users\pontu\AppData\Roaming\npm\node_modules\npm\bin\npm-cli.js`. Repository Corepack/pnpm 11.25.0 is available and the configured tests pass; no global tooling installed.

Initial failures corrected before final verification: copied test field names/helper duplication and save notice wording; Rust string-pattern API misuse; two malformed test inputs that violated existing normalization/minimum-list rules. The canonical constraint test now checks internal pretty-printed JSON, since existing DNA normalization trims outer whitespace. Initial generated quality review exposed decoration-based profile errors and arbitrary generic functions; the new context path was corrected and examples regenerated. An attempted concurrent log overwrite failed because a prior verification process still owned the output file; final runs use separate logs and wait for Cargo completion.

## Quality review

`docs/quality/THE_FORGE_B8_CHARACTER_SIGNATURE_ITEM_SAMPLES.md` contains **44 generated/content-author reviewed examples**, four per ten non-Chaos presets plus four Chaos. Each includes all fifteen frozen facts and all thirteen Item fields, a classification and individual review note. **0 usable as-is; 44 usable with minor edit; 0 weak/unusable.** Repeated source quotation needs editorial tightening; individual notes cover physical/function clarifications and plausible mixed-era custody. No severe Character/Item contradiction found in this set. Principal objects retain gear-relevant profiles; contextual Profile rerolls may yield the same strongly constrained value while advancing DNA nonce.

All 44 incorporate exact signature gear in Appearance/Purpose/Function, source skills/abilities in Function (without transferring Character powers), backstory in History and secret in Secret. Complete creative bodies are distinct; no generated UUID/hash or conventional game-stat leakage found. Chaos retains mixed secondary imagery around a common Character intention. This review is not independent human/native operator acceptance.

## Residual limitations

- No independent native operator smoke claimed; B8 permits automated PASS when all integrity/determinism/lifecycle gates are green and no known P0/P1/P2 remains. A fresh-binary operator review remains useful.
- Existing P3 **UI-001** Location lock visual feedback remains open; passing lock tests do not close it.
- Mission 08's accepted incomplete automated transitive dependency advisory scan remains maintenance/release verification risk, not a confirmed vulnerability. Direct/runtime and enabled-feature/capability review was completed; locked ZIP 8.6.0 lies outside the reviewed advisory range below 2.3.0. No confirmed applicable vulnerability identified; the whole transitive graph is not claimed automatically verified clean. No security tooling installed.
- FOUNDATION package remains unsigned, requires WebView2 Runtime, and has no claimed MSI/NSIS installer. B8 produces only an isolated no-bundle executable.

## Worktree ownership

Sixteen mission-owned files are unstaged/uncommitted (eleven tracked modifications; five new files):

- `GAZINGA/DISCOVERY_STATE.md`
- `GAZINGA/PROJECT_STATE.md`
- `apps/desktop/src-tauri/src/lib.rs`
- `apps/desktop/src/App.tsx`
- `apps/desktop/src/api.ts`
- `apps/desktop/tests/App.test.tsx`
- `crates/app-core/src/item.rs`
- `crates/app-core/src/lib.rs`
- `crates/app-core/src/character_signature_item.rs` (new)
- `crates/app-core/src/item_context.rs` (new)
- `crates/app-core/examples/character_signature_item_samples.rs` (new)
- `crates/storage/src/lib.rs`
- `crates/storage/src/tests.rs`
- `crates/export/src/lib.rs`
- `docs/missions/THE_FORGE_B8_CHARACTER_SIGNATURE_ITEM_REPORT.md` (new)
- `docs/quality/THE_FORGE_B8_CHARACTER_SIGNATURE_ITEM_SAMPLES.md` (new)

Final branch: `master`; HEAD: `ecdd143061d2e66b4dbfb5667f1fe5b944328235`; worktree dirty only with the sixteen B8 files plus four preserved unrelated untracked files. Index remains empty. No commit created.

Four unrelated files remain byte-identical by SHA-256:

| File | SHA-256 |
| --- | --- |
| `The Ashen Claim.json` | `32BC3B62555F32A7AB4A8CA936B52F4979305C08027937274E636E6E8033A844` |
| `The Ashen Claim.md` | `A298F7A2DAFDFB4AC4FE354607C67CB565B0CFD38D9D45D9085D1AAA8FEDFC0D` |
| `mission09-final-verify.log` | `958E927350F940EA6075A0E2A4452E922A1C780ADF9EE97052EC59C3DD8FE6AD` |
| `mission09-rust-test.log` | `8E16F7D11EF7432BB341CC22512D59A384321AAD0CFFCF10F9133B315234F0D2` |

## Exit gate re-evaluation

| Required exit criterion | Re-evaluation/evidence |
| --- | --- |
| Saved deterministic Character → Forge Their Signature Item | **PASS**; Candidate/Canon storage sources, supported UI action/review/save and real command-boundary persistence |
| Truthful bounded immutable context | **PASS**; all fifteen exact facts individually tested for hash changes, source/body hashes frozen, supported provenance/constraints and byte/list bounds enforced; exact persisted-source recompilation |
| Canonical ItemForge | **PASS**; optional typed context path, canonical generation equality, standalone field-generator body unchanged |
| Deterministic replay | **PASS**; repeated core equality, seed variation, stable IDs, exact inherited preset/nonneutral axes; all 44 sample snapshots independently reproduced |
| Final reviewed Item DNA parity | **PASS**; lock/reroll-all and field-reroll/manual edit exact storage recipe roundtrip plus session undo/redo; frozen context/IDs preserved |
| Atomic Candidate Owns | **PASS**; one Item + one Generated Candidate edge + one typed operation, three stage rollbacks, stale-source and duplicate rejection, close/reopen |
| Correct B6 source/inverse Connections | **PASS**; Owns / Owned by storage views and UI handoff across Candidate → Canon → Archived, one edge |
| Independent relationship lifecycle | **PASS**; immutable forge operation, unchanged Character and Item snapshots, explicit independent promotion/archive/history |
| B2 Hometown / B2.R1 and B7 Character → Faction remain green | **PASS**; existing core/storage/export/UI/lifecycle tests unchanged and passing; existing bundle storage method bodies identical |
| Stable schema/archive/app versions | **PASS**; schema 2, archive v1 roundtrip, unchanged manifests/contracts/locks, metadata and executable 1.0.0 |
| Full verification green | **PASS**; required gates above, final consolidated process exit 0 and final diff/encoding checks |
| No known B8 P0/P1/P2 remains | **PASS**; zero known; prior P3 UI-001 and accepted dependency-scan verification limitation remain scoped residuals |

**PASS. STOP at B8.** No Visual Forge, Link Existing, multi-source Forge, Creature/Item/Faction reverse Forge Further or later mission begun. No native acceptance is inferred from automated evidence.
