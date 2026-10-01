# THE FORGE B6 — Relationship Lifecycle Report

**Status: PASS — B6 automated exit gates satisfied**

## Scope and implementation

- Extended the existing four app-owned Relationship definitions with serialized source/inverse display prefixes. Keys and endpoint rules remain unchanged. React selects metadata by endpoint role; it does not define inverse types or duplicate assertions.
- Added a typed storage `ConnectionView`: complete persisted Relationship, actual write version, source/target role, related ID/type/name/availability, and definition metadata. Lists keep existing deterministic creation-time/ID ordering. Reads validate project/type boundaries, resolve archived endpoints explicitly and report missing endpoints without looking in other projects.
- The saved-detail component used by all six libraries and Canon now loads Connections by project/entity identity. Historical entity snapshots have no connection mutation surface. Empty state is “No connections yet.” Load failures are distinguished from emptiness. Request sequencing and unmount guards prevent late reads from replacing newer results.
- Added narrow Tauri/API promotion, archive and revision-history commands. Mutations require Relationship ID and loaded expected write version and use existing transactional storage. No arbitrary relationship creation/edit/type/endpoint mutation API was added.
- Candidate rows offer “Promote connection to Canon”; Candidate/Canon rows offer “Archive connection”. Archived assertions are secondary and read-only, with history inspection. Mutation results are read back from storage before UI truth changes. Stale conflicts explain the issue and reload; failed reloads remove old mutation controls until Refresh succeeds.
- History separates current assertion from immutable historical snapshots: revision number, reason, state, relationship type, endpoint IDs, origin, and timestamp. No restore/edit action is exposed. Existing archive semantics retain the pre-archive assertion as the archive revision snapshot.

## Four existing definitions

| Key | Allowed endpoints | Source wording | Inverse wording |
|---|---|---|---|
| born_in | Character → Location | Born in <Location> | Birthplace of <Character> |
| member_of | Character → Faction | Member of <Faction> | Member: <Character> |
| owns | Character → Item | Owns <Item> | Owned by <Character> |
| opposed_by | Quest → Creature | Opposed by <Creature> | Opposes quest <Quest> |

“Member:” names the Character belonging to the viewed Faction. `owns` still excludes Faction sources; `opposed_by` still excludes Character/Faction targets. All four keys serialize unchanged; all 36 source/target combinations per definition are checked against the existing exact endpoint rule.

## Timestamp defect

Prior B2/B4/B5 reports record an intermittent 1 ms mismatch between promotion's Canon revision snapshot and current row. Inspection confirmed separate `now_unix_ms()` calls in `mutate_relationship`; B6 did not introduce that defect. Promotion now captures one monotonic mutation timestamp and uses it for the revision snapshot, revision timestamp and current row. Archive retains its existing pre-archive snapshot contract. The original complete-snapshot equality assertion and A/B/C revision regression are unchanged. A new future-clock regression checks monotonic current/revision timestamps for promotion and archive. Final verification uses normal parallel Rust execution, without a serial workaround or weakened assertions.

## Regression evidence

- All six real generated endpoint types resolve views; four persisted assertions appear from both ends with correct names/types/roles/versions, stable ordering, reopen parity, no read side effects and no inverse duplicates.
- Archived related entities and a missing endpoint are explicit; unknown IDs/independent project stores do not resolve connections from another store.
- Actual reviewed B2 Hometown flow saves Character, locks/rerolls reviewed Location, finalizes/saves bundle, promotes the Candidate assertion, archives Canon and reopens. Both views follow persisted state/version. Entity rows, entity revisions, forge history, frozen context/hash and final reviewed DNA remain byte-identical through relationship operations.
- Candidate archive retains its snapshot. Wrong/stale/repeated promote/archive cannot change current records or revision history. Entity Canon promotion leaves the relationship Candidate; relationship promotion leaves both endpoints Candidate.
- Frontend tests cover all eight phrases, all six saved detail libraries, empty/error states, Candidate/Canon actions, archived read-only history, persisted expected versions, no optimistic fake state, refresh without duplicates, stale and readback failure handling, late read suppression, and the actual existing Hometown UI handoff using mocked persistence responses.

## Final verification

Final automated results, with `CARGO_TARGET_DIR=C:\dev\CharacterForge\target\mission-b6-verify` and normal parallel Rust execution (`RUST_TEST_THREADS` unset):

- Frontend formatting/lint/TypeScript typecheck/production build: **PASS**.
- Frontend tests: **79 passed** (59 App tests, 20 Connections tests).
- Rust formatting and strict workspace/all-target Clippy (`-D warnings`): **PASS**.
- `cargo test --workspace --no-fail-fast`: **187 passed, 0 failed, 1 ignored**. Counts: desktop 18; app-core 89; contracts 17; export 24; storage 39. The ignored test is the existing one-time frozen fixture writer.
- Existing 1,000-sequence state gauntlet: **PASS**, both final explicit and consolidated test runs.
- Full offline Cargo metadata: **PASS**.
- `scripts/verify.ps1`: **PASS**, final sequential run.
- Optimized Tauri `build --no-bundle`: **PASS**. Exact executable: `C:\dev\CharacterForge\target\mission-b6-verify\release\character-forge-desktop.exe`; ProductVersion/FileVersion **1.0.0** observed. The isolated target leaves default and earlier mission binaries untouched; no user process was terminated.
- Final closeout `git diff --check`: **PASS**. All twelve mission-owned source/test/state/report files decode as valid UTF-8 with no replacement characters or trailing whitespace.

Ignored local evidence logs (not release artifacts): `target/b6-final-workspace.log`, `target/b6-complete-verify.log`, `target/b6-frontend-final-reviewed.log`, `target/b6-cargo-metadata.json`, `target/b6-npm-test.log`.

The first explicit no-fail-fast run passed 186 tests before the final future-clock regression was added. An overlapping consolidated attempt hit Windows LNK1104 while replacing the running storage test executable, not an assertion failure. Its log is retained at `target/b6-final-verify.log`. Final suites are rerun sequentially to avoid that local verification-process lock; no user application is terminated. Early frontend fixture failures (ambiguous headings, asynchronous library loading, duplicate history text, strict indexed-access typing) were corrected without weakening product assertions.

`npm test` was attempted per working agreement and cannot find the machine's npm-cli.js at `C:\Users\pontu\AppData\Roaming\npm\node_modules\npm\bin\npm-cli.js`. No npm PASS is claimed; configured pnpm tests are the repository frontend evidence. No dependencies or security tooling were installed.

## Exit-gate review

| Criterion | Status |
|---|---|
| All six saved details expose Connections | PASS — library integration tests |
| Four existing types, both display directions | PASS — definition/storage/eight wording regressions |
| Explicit stale-safe Candidate → Canon | PASS — storage and frontend lifecycle tests |
| Explicit non-destructive stale-safe archive | PASS — Candidate/Canon regressions |
| Immutable revision history inspectable | PASS — storage A.R1 regression and read-only frontend history |
| Real B2 Hometown through new surface | PASS — actual core/storage lifecycle and frontend handoff |
| No inverse duplicates or coupled entity truth | PASS — row-count/entity-row invariants |
| No schema/archive/app-version bump | PASS — schema 2/archive v1/application 1.0.0 unchanged |
| Full verification and optimized build | PASS — complete sequential script, explicit no-fail-fast, metadata, diff check |
| No known B6 P0/P1/P2 | PASS — 0/0/0; existing P3 UI-001 remains open |

## Residual limitations and boundary

No independent native operator smoke is claimed. The mission permits automated PASS when integrity/lifecycle gates pass and no schema/migration/security/product-decision issue or P0/P1/P2 remains. Existing P3 UI-001 remains open pending a current-binary operator check; this mission does not close it. Mission 08's accepted incomplete automated transitive dependency advisory scan remains residual maintenance/release verification risk, not a confirmed vulnerability or a complete dependency-clean claim.

Manual Link Existing defaults, bundled Entity/Relationship promotion, new vocabulary, cardinality policy, symmetry, new Forge Further actions, Visual Forge and later missions remain deferred. Existing creation remains B2 Hometown only in the product UI.

## Changed files

- `crates/app-core/src/relationship.rs`
- `crates/storage/src/lib.rs`, `tests.rs`
- `apps/desktop/src-tauri/src/lib.rs`
- `apps/desktop/src/api.ts`, `App.tsx`, `Connections.tsx`
- `apps/desktop/tests/App.test.tsx`, `Connections.test.tsx`
- This report, `GAZINGA/DISCOVERY_STATE.md`, `GAZINGA/PROJECT_STATE.md`

## Repository state and preservation

Branch `master`; starting/current HEAD `51d32babb3f7a0ff3f3d6067e72af09d25f95a28` (`docs: prepare Relationship lifecycle mission`). B6 changes remain unstaged/uncommitted. No reset, commit, tag, push or publication occurred. The user-specified current HEAD supersedes the mission file's earlier implementation baseline `821dbeb`; `51d32ba` adds only mission preparation.

| Preserved untracked file | Starting SHA-256 |
|---|---|
| The Ashen Claim.json | 32BC3B62555F32A7AB4A8CA936B52F4979305C08027937274E636E6E8033A844 |
| The Ashen Claim.md | A298F7A2DAFDFB4AC4FE354607C67CB565B0CFD38D9D45D9085D1AAA8FEDFC0D |
| mission09-final-verify.log | 958E927350F940EA6075A0E2A4452E922A1C780ADF9EE97052EC59C3DD8FE6AD |
| mission09-rust-test.log | 8E16F7D11EF7432BB341CC22512D59A384321AAD0CFFCF10F9133B315234F0D2 |

All four final SHA-256 values equal the starting values above. Nine tracked files are modified and three mission files are new, all unstaged/uncommitted; the four unrelated files remain untracked.

**B6 exit gate: PASS. STOP. No Link Existing, new vocabulary, new Forge Further action, Visual Forge or later mission was begun.**
