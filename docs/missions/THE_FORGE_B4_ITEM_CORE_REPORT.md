# THE FORGE B4 — Item Core mission report

**Status: PASS — B4 automated exit gates satisfied**

## Baseline and scope

Started from the requested `master` HEAD `983398e5c407156ab18f394b1c1ab22efcdbda2a`, including accepted B3 Faction Forge. This mission adds standalone deterministic Item Forge only. No Creature Forge, Item Forge Further action, owner/creator/location/guardian/reward linking, gameplay-stat pack, or later mission was implemented.

The four pre-existing untracked files remain outside mission ownership: `The Ashen Claim.json`, `The Ashen Claim.md`, `mission09-final-verify.log`, and `mission09-rust-test.log`. Their final SHA-256 values independently match the starting values recorded below.

## Delivered

- Added the typed `Item` contract with shared envelope plus twelve payload fields: `item_profile`, `appearance`, `purpose`, `properties`, `origin`, `history`, `power_or_function`, `cost_or_drawback`, `rarity`, `condition`, `secret`, and `story_hooks`. Name lives in the envelope. Required strings/lists validate; unknown fields and invalid profiles fail; envelope type must be Item. Payload origin is worldbuilding prose, distinct from envelope creation origin.
- Eleven stable kebab-case profiles: weapon, armor, artifact, consumable, tool, key-item, resource, treasure, book, quest-item, other. All serialized keys are tested. Item has its own `GenerationKind::Item`, generator version 1, and content version 1.
- Added `ItemCandidate`, explicit per-field origins, deterministic project-bound IDs/metadata, DNA validation, atomic manual edits, locks, field rerolls, reroll-all, and provenance synchronization. Generation reads explicit DNA/preset/axes/controls and declared dependencies; it reads no clock, filesystem, network, or shared mutable RNG.
- Added original preset narrative material with six alternatives in each major layer for all ten non-Chaos presets. Eleven profile corpora distinguish physical forms, six purposes, properties, functions, and drawbacks. Rarity describes custody/scarcity/access; condition describes fiction and protective packaging. No combat ratings, prices, durability numbers, runtime IDs, or embedded foreign entities were added.
- Chaos deterministically selects three source groups from the ten compatible corpora, using Item-specific domain-separated root-seed hashes. The themed groups are Name/Appearance/Purpose/Properties, Origin/History, and Secret/Story hooks. Profile capabilities/drawbacks and rarity/condition are compatible shared material. The graph does not pretend these source selections read other generated fields.
- Extended `StoredEntity` with ItemCandidate and canonical Item variants and existing validation/state/favorite/timestamp lifecycle handling. Candidate save, stale-write rejection, favorite, Canon promotion/revision, immutable revision/history inspection, tags/notes, DNA, lock state, and origins survive close/reopen. Standalone Item creates zero Relationship rows.
- Added Item command adapters and `forge_item`, using app-core generation and shared session undo/redo. The B2 cached Hometown bundle/finalizer path is preserved.
- Added a fifth Workbench mode, all thirteen field controls, typed Profile select, Item library/detail, and existing save/favorite/Canon/revision/history/export actions. Item Name/Profile regressions resolve actual controls, show Locked/Unlock and `aria-pressed=true`, route profile reroll to `item_profile`, and prevent invalid Profile edits. Keyboard/native-select and existing Reduced Motion behavior are inherited.
- JSON and readable Markdown include all canonical fields. Item `.forgeproject` export/import and Vault backup/restore are covered. Database schema stays 2; archive format stays v1; app version stays 1.0.0. Existing Character/Quest/Location/Faction versions and field-stream derivation are unchanged. No dependency or lockfile change.

## Frozen Item field contract

| Field/action | Stable control key |
|---|---|
| item_profile | 01-item-profile |
| name | 02-name |
| appearance | 03-appearance |
| purpose | 04-purpose |
| properties | 05-properties |
| origin | 06-origin |
| history | 07-history |
| power_or_function | 08-power-or-function |
| cost_or_drawback | 09-cost-or-drawback |
| rarity | 10-rarity |
| condition | 11-condition |
| secret | 12-secret |
| story_hooks | 13-story-hooks |

Actual acyclic reads:

```text
item_profile -> appearance
             -> purpose
             -> properties
             -> power_or_function
             -> cost_or_drawback
```

Other fields read their own deterministic stream and authored source material. The graph is validated exactly; a graph replacement/cycle is rejected. Profile reroll invalidates only this closure. A locked dependent preserves its value/nonce. Reroll-all increments each eligible field once, preserving locked controls. Manual fields are protected from unrelated/all rerolls; explicit target reroll returns them to generated origin. Tests cover these transitions and atomic rejection of invalid edits.

## Quality evidence

- **2,816 Candidates (256 × 11 presets):** all valid, complete, exact replay, input DNA unchanged, no exact full-content duplicates within a preset.
- **1,024 Chaos seeds:** exact duplicates 0; all eleven profiles; 635 distinct source patterns; 1,011 mixed-source Candidates. Every source group uses all ten corpora. Maximum single group/source share: 118/1,024 = 11.52%.
- Profile counts: Weapon 97; Armor 91; Artifact 95; Consumable 85; Tool 99; Key Item 78; Resource 91; Treasure 85; Book 107; Quest Item 91; Other 105.
- **52 final samples:** 40 non-Chaos plus 12 Chaos, classified 22 usable as-is, 30 minor edit, 0 weak. This is Codex generated/content-author review, not independent human playtesting. Repeated names, profile sentences, and hook groups remain bounded corpus characteristics; no exact full clone or unresolved severe content defect was observed. Minor editing may connect custody events, specify a repair accessory/technique, or bridge Chaos cultures.
- See [THE_FORGE_B4_ITEM_SAMPLES.md](../quality/THE_FORGE_B4_ITEM_SAMPLES.md) for all thirteen fields in every sample, per-record judgments, source count table, reproduction command, and review repairs.

## Verification

All required final gates passed against the completed implementation:

- Frontend format, lint, TypeScript typecheck, production build: **PASS**.
- Frontend tests: **48 passed**.
- Rust format and strict workspace/all-target Clippy (`-D warnings`): **PASS**.
- Workspace Rust tests: **166 passed, 1 ignored** (existing one-time frozen schema fixture writer). Suite counts: desktop 15; app-core 80; contracts 16; export 23; storage 32 passed.
- `cargo test --workspace --no-fail-fast`: **PASS**, rerun after the final adapter repair.
- Existing 1,000-sequence deterministic state gauntlet: **PASS** in the final verification and final no-fail-fast run.
- Cargo metadata: **PASS**, saved in `target/b4-cargo-metadata.json`.
- `scripts/verify.ps1`: **PASS**, with absolute `CARGO_TARGET_DIR=C:\dev\CharacterForge\target\mission-b4-verify`.
- Optimized Tauri `build --no-bundle`: **PASS**. Exact executable: `C:\dev\CharacterForge\target\mission-b4-verify\release\character-forge-desktop.exe`.
- `git diff --check`: **PASS** at closeout.

Final consolidated log: `target/b4-final-complete-verify.log`. Final explicit no-fail-fast log: `target/b4-final-complete-workspace.log`. These are ignored local build evidence, not release artifacts. The absolute isolated target avoids replacing any existing default-target executable; no user process was terminated.


`npm test` was attempted as required by the working agreement. The machine's launcher cannot locate `C:\Users\pontu\AppData\Roaming\npm\node_modules\npm\bin\npm-cli.js`. Configured repository pnpm tests run successfully; the launcher failure is an environment limitation, not a passing npm result.

## Review repairs and residual limitations

- **Resolved P2 — working-copy/Canon adapter eligibility:** final command-boundary inspection found that the shared historical working-copy and Canon revision eligibility matches omitted Item. The accepted Faction baseline had the same omission. Added the two typed candidate cases and a shared validated Canon transition helper. A regression exercises both Item and Faction through Candidate → Canon → historical working copy → edited Canon, preserving identity, DNA/locks/origins and original snapshots. This is a bounded lifecycle parity repair; it adds no Forge Further action.
- An earlier full `--no-fail-fast` run reproduced the already documented Relationship lifecycle assertion fluctuation: `updated_at_unix_ms` differed by 1 ms (1790803534492 vs 1790803534493). The isolated test and the subsequent whole-workspace rerun passed without modifying that test or Relationship implementation. The failure remains available in `target/b4-workspace-tests.log`; isolated/rerun evidence is in `target/b4-relationship-rerun.log` and `target/b4-workspace-tests-rerun.log`.
- Existing P3 UI-001 remains an operator-observed Location visual-feedback issue. The npm launcher limitation remains environmental. Neither is claimed resolved by B4.
- Mission 08's human-accepted incomplete automated transitive advisory scan remains residual maintenance/release verification risk; B4 adds no dependency and makes no new advisory-clean claim.

## Exit-gate review

| Criterion | Evidence/status |
|---|---|
| Typed Item; 13 fields; 11 profiles; stable serialization | PASS — contract and invalid-shape tests |
| Deterministic Item Forge; 11 presets; explicit versions and axes | PASS — replay/preset/axis tests |
| Lock/reroll/edit/undo/redo and dependency firewall | PASS — app-core and desktop command/session tests |
| Candidate/favorite/Canon/revisions/history/stale writes/reopen | PASS — Item on-disk storage tests |
| JSON/Markdown and project/Vault roundtrip | PASS — Item export/import/backup/restore tests |
| Workbench mode/library/detail and typed Name/Profile parity | PASS — frontend regressions |
| Preset/Chaos sweeps and 52-sample audit | PASS — evidence above and sample packet |
| Character/Quest/Location/Faction/Relationship/B2 compatibility | PASS — full workspace/frontend regression suites |
| Complete final verification/build/diff check | PASS |
| Outstanding P0/P1/P2 defects | 0/0/0; lifecycle eligibility finding resolved |

Existing P3 UI-001 remains open in `GAZINGA/KNOWN_ISSUES.md`; Item tests do not close the operator-observed Location lock indicator issue. B4's stated policy permits an automated PASS without immediate operator smoke when no data-integrity/security/product-decision gate or P0/P1/P2 defect remains. No independent operator acceptance is claimed.

## Mission-owned files

- `crates/contracts/src/domain.rs`, `generation.rs`, `lib.rs`
- `crates/app-core/src/item.rs`, `item_content.rs`, `lib.rs`; `examples/item_samples.rs`
- `crates/storage/src/lib.rs`, `tests.rs`
- `crates/export/src/lib.rs`
- `apps/desktop/src-tauri/src/lib.rs`
- `apps/desktop/src/api.ts`, `App.tsx`; `tests/App.test.tsx`
- This report, the Item sample packet, `GAZINGA/DISCOVERY_STATE.md`, `GAZINGA/PROJECT_STATE.md`

## Preservation evidence

| Unrelated file | Starting and final SHA-256 (equal) |
|---|---|
| The Ashen Claim.json | 32BC3B62555F32A7AB4A8CA936B52F4979305C08027937274E636E6E8033A844 |
| The Ashen Claim.md | A298F7A2DAFDFB4AC4FE354607C67CB565B0CFD38D9D45D9085D1AAA8FEDFC0D |
| mission09-final-verify.log | 958E927350F940EA6075A0E2A4452E922A1C780ADF9EE97052EC59C3DD8FE6AD |
| mission09-rust-test.log | 8E16F7D11EF7432BB341CC22512D59A384321AAD0CFFCF10F9133B315234F0D2 |

## Repository state

- Branch: `master`.
- Starting/current HEAD: `983398e5c407156ab18f394b1c1ab22efcdbda2a`.
- Worktree: B4 implementation/tests/docs are unstaged/uncommitted; four unrelated pre-existing untracked files are preserved.
- No commit, tag, push, publication, or later mission was performed.

**B4 exit gate: PASS. STOP. Creature Forge and Item Forge Further remain unstarted.**
