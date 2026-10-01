# THE FORGE B7 — Character → Faction report

## Exit status

**PASS — all B7 automated exit gates satisfied.**

Executed on 2026-10-01 using `GAZINGA/THE_FORGE_CODEX_LAUNCHER_PROMPT.txt` and the B7 mission contract. Starting and current branch: `master`; starting/current HEAD: `553980c1f165e294f0cee081a8bddc61642fbc5e`. The initial worktree contained only the four unrelated files listed below. Existing work was preserved.

## Implemented scope

- Stable action `character.forge-faction` v1; intent Character Faction; context recipe `character-faction-v1` v1; one saved Character, one Faction Candidate, no supporting Candidates, one Generated Candidate `member_of` relationship.
- Read-only context compiler captures exact persisted source ID/write version/Canon state/generation kind and versions, all twelve requested Character facts, inherited preset/axes, canonical source body hash and context hash. Facts are limited to 32 KiB and at most 16 personality traits; invalid/unsupported provenance is rejected without fabricated defaults. No graph traversal or unrelated project context is used.
- Canonical `FactionForge` consumes one canonical typed, versioned Character-context constraint. Explicit hint semantics guide Profile; ambiguous hints stay Other. Identity, ideology, goal, culture, public face, hidden truth, reputation and hooks incorporate frozen Character intent. Name is not forced to the hint; names/methods/resources/symbols reuse accepted content. This is an optional input path in the existing generator, not a second Faction generator.
- Target root seed derives from the stable action/version, context hash and explicit action seed. Preset and axes are inherited exactly. Entity/Relationship identities replay deterministically with no hidden clock. Generated entity and relationship timestamps are zero until existing persistence/lifecycle behavior applies.
- The finalizer accepts the exact reviewed typed Faction Candidate, preserving source context/hash, action/seed and output/relationship identities. Only the reviewed primary snapshot and recipe target DNA are replaced. Changed IDs, types, project/origin/state, action/context/mapping or provenance fail validation.
- Save Bundle uses one SQLite transaction for reviewed Faction, Candidate MemberOf and immutable `forge_runs` recipe/result. Source write version and recompilation of the exact persisted context are revalidated in the transaction. Failure injection after primary, relationship and operation insert proves complete rollback; duplicates reject.
- Saved Character details expose **Forge Their Faction** for supported generated provenance. Review shows source, Candidate action/relationship, frozen facts/hash and actual generation seed. Normal typed Profile/Name controls, lock/reroll/edit and session undo/redo work; Discard makes no writes. The save command reads the current backend working Candidate rather than stale frontend bundle DNA.
- Saved Faction appears in Factions; universal B6 Connections shows Character-side **Member of** and Faction-side **Member:**. Relationship promotion/archive/history remain independent of Entity Canon. No inverse duplicate is stored.

## Compatibility and scope boundaries

Database schema **2**, `.forgeproject` archive **v1**, application **1.0.0** remain unchanged. No existing generator/content/protocol versions were bumped. No dependencies were added.

The accepted standalone Faction field-generator body was compared against HEAD and is byte-identical after its private function-name normalization. Empty constraints take that exact original field-stream path. Existing standalone six-generator regression suites remain green. Hometown persisted v1 records, hashing, generation, review finalization and storage code retain their original formats/behavior; the action definition only gains a generic target-profile type with LocationProfile as its default. Existing B2/B2.R1 tests and Hometown frontend/lifecycle handoff pass unchanged.

No Character → Signature Item, Link Existing, new relationship vocabulary, bundled Canon promotion, Visual Forge, multi-source Forge or later mission was implemented. No reset, staging, commit, tag, push, publication or user-process termination occurred. The FOUNDATION portable release artifact was not replaced.

## Verification evidence

| Gate | Evidence/result |
| --- | --- |
| Frontend formatting / lint / TypeScript | **PASS** in final consolidated run |
| Frontend tests | **84 PASS** in final consolidated run |
| Frontend production build | **PASS** in final consolidated run |
| Rust formatting / strict all-target Clippy | **PASS** |
| `cargo test --workspace --no-fail-fast` | **196 PASS**, one existing fixture writer ignored; desktop 19, app-core 93, contracts 17, export 25, storage 42 |
| Existing 1,000-sequence / 32,000-operation gauntlet | **PASS** in both explicit workspace and final consolidated runs |
| `cargo metadata --format-version 1 --offline` | **PASS**, five workspace packages at 1.0.0 |
| `scripts/verify.ps1` | **PASS**, process exit 0 |
| Optimized Tauri `build --no-bundle` | **PASS** via consolidated script; release/optimized build |
| `git diff --check` | **PASS** at final closeout |

Evidence files are ignored local artifacts under `target/`: `b7-workspace-tests.log`, `b7-focused-rust.log`, `b7-clippy.log`, `b7-cargo-metadata.json`, `b7-final-verify.log`, `b7-npm-test.log`, `b7-samples.json`. The isolated Cargo target is `C:\dev\CharacterForge\target\mission-b7-verify`.

Final optimized executable: `C:\dev\CharacterForge\target\mission-b7-verify\release\character-forge-desktop.exe`; ProductVersion/FileVersion **1.0.0**, 14,639,616 bytes. SHA-256: `F4E67A73C07A49ED03A231670F7E7DBD180787EBCD8E64E92251EC8785EFBBB7`. No installer/package was produced. This executable was built from the verified uncommitted B7 worktree; no native operator smoke is claimed.

Required `npm test` was attempted after TypeScript/TSX edits. The machine npm shim cannot locate `C:\Users\pontu\AppData\Roaming\npm\node_modules\npm\bin\npm-cli.js`; this is the existing environment limitation. Repository Corepack/pnpm 11.25.0 tests pass; no global tooling was installed.

Focused initial failures were corrected before final verification: a Rust MutexGuard adapter coercion; two test assumptions about the source's existing history event and accessible reroll label; copied relationship fixture IDs; an editor accessible label; a Clippy failpoint naming warning. Assertions were corrected to match actual existing behavior, and the focused rerun passed. Initial quality review exposed random institutional profile fallback; the new contextual path was repaired and the final 44 samples regenerated. No unresolved B7 P0/P1/P2 remains known.

## Quality review

See `docs/quality/THE_FORGE_B7_CHARACTER_FACTION_SAMPLES.md`: **44 generated/content-author reviewed examples**, four across each of ten non-Chaos presets and four Chaos. All include exact source facts and all thirteen Faction fields. 44/44 visibly relate Identity to faction_hint, Goal to desire and Hidden Truth to secret. All complete creative bodies differ; no generated raw UUID/hash leakage was found. Names can repeat from the unchanged name corpus without complete clones.

All 44 are usable with minor editorial tightening of quoted source facts. Specific plausible tensions (concealed fare-taking against free-passage ethics; internal Civic Pattern Office reform; helping someone exit another guild; mixed union/crew organization in Chaos) are documented. Chaos retains mixed preset secondary imagery around one shared Character intention. These are content-author observations, not independent human playtesting or native operator acceptance.

## Exit criteria re-evaluation

| Required criterion | Evidence |
| --- | --- |
| Saved deterministic Character → Forge Their Faction | Frontend action/review/save tests; real command-boundary persistence test |
| Truthful bounded immutable context | Core compiler/replay/source rejection tests; persisted-source exact recompilation in storage |
| Canonical FactionForge / deterministic replay | Core exact canonical generation/replay, inherited-input and seed-variation assertions; unchanged standalone generator body |
| Reviewed DNA parity | Core finalizer; storage lock/reroll-all and field-reroll/manual edit exact recipe roundtrip; command session undo/redo |
| Atomic Candidate MemberOf | Three transaction-stage rollbacks, stale context/version, duplicate and reopen tests |
| Correct B6 source/inverse Connections | Storage metadata views plus frontend Faction handoff across Candidate → Canon → Archived |
| Independent relationship lifecycle | Unchanged source/target snapshots and immutable forge operation across promotion/archive |
| B2/B2.R1 green | Existing Hometown core/storage/export/frontend/lifecycle tests pass |
| Stable schema/archive/app versions | Schema 2; archive v1 export/import test; workspace metadata and optimized executable ProductVersion/FileVersion 1.0.0 verified |
| Full verification green | **PASS**, consolidated script exit 0 and final diff/UTF-8 checks |
| No known B7 P0/P1/P2 | Zero known; residuals below remain accurately scoped |

## Residual limitations

- No independent native operator smoke is claimed; B7 explicitly permits automated PASS without an immediate smoke. Fresh-binary human review remains useful.
- Existing P3 **UI-001** Location lock visual feedback remains open; automated passing lock tests do not close it.
- Mission 08's accepted incomplete automated transitive dependency advisory scan remains maintenance/release verification risk, not a confirmed product vulnerability. Direct/runtime and feature/capability review was completed; ZIP 8.6.0 is outside the reviewed advisory's affected range below 2.3.0; no confirmed applicable vulnerability was identified. The whole transitive graph is not claimed automatically verified clean.
- FOUNDATION packaging facts remain: unsigned portable package, WebView2 Runtime required, no MSI/NSIS installer claimed. This mission builds an isolated no-bundle executable only.

## Worktree ownership and preserved files

Seventeen mission-owned files (unstaged/uncommitted; five newly created):

- `GAZINGA/DISCOVERY_STATE.md`
- `GAZINGA/PROJECT_STATE.md`
- `apps/desktop/src/App.tsx`
- `apps/desktop/src/api.ts`
- `apps/desktop/tests/App.test.tsx`
- `apps/desktop/src-tauri/src/lib.rs`
- `crates/app-core/src/forge_further.rs`
- `crates/app-core/src/faction.rs`
- `crates/app-core/src/lib.rs`
- `crates/app-core/src/character_faction.rs` (new)
- `crates/app-core/src/faction_context.rs` (new)
- `crates/app-core/examples/character_faction_samples.rs` (new)
- `crates/storage/src/lib.rs`
- `crates/storage/src/tests.rs`
- `crates/export/src/lib.rs`
- `docs/quality/THE_FORGE_B7_CHARACTER_FACTION_SAMPLES.md` (new)
- `docs/missions/THE_FORGE_B7_CHARACTER_FACTION_REPORT.md` (new)

Unrelated files remain untracked and excluded; baseline SHA-256 values independently rechecked unchanged at final closeout:

| File | SHA-256 |
| --- | --- |
| `The Ashen Claim.json` | `32BC3B62555F32A7AB4A8CA936B52F4979305C08027937274E636E6E8033A844` |
| `The Ashen Claim.md` | `A298F7A2DAFDFB4AC4FE354607C67CB565B0CFD38D9D45D9085D1AAA8FEDFC0D` |
| `mission09-final-verify.log` | `958E927350F940EA6075A0E2A4452E922A1C780ADF9EE97052EC59C3DD8FE6AD` |
| `mission09-rust-test.log` | `8E16F7D11EF7432BB341CC22512D59A384321AAD0CFFCF10F9133B315234F0D2` |

## B7 exit gate

**PASS. STOP at B7.** No later action or mission was begun.
