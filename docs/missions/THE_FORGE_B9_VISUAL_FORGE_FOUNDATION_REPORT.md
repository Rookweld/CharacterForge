# THE FORGE B9 — Visual Forge Foundation report

## Exit status

**PASS — B9 automated exit gates satisfied. STOP.**

Executed on 2026-10-01 using `GAZINGA/THE_FORGE_CODEX_LAUNCHER_PROMPT.txt` and the complete B9 mission contract. Starting/current branch `master`; HEAD `a925d7f6cf1a5048afc2fee49184da2b16c1c369`. Actual baseline contained accepted B8 and only the four unrelated files below. Stale B8 worktree/HEAD wording in state documents was corrected using live Git evidence.

## Implemented scope

- Public provider-neutral core contracts: closed snake_case `VisualRole`, typed semantic `VisualField`, five typed source families, bounded optional `ArtDirection`, versioned recipe, source reference/context snapshot and deterministic generation brief.
- Fifteen app-owned recipes: Character portrait/full_body/character_sheet; Location establishing_shot/environment_concept/interior_concept; Faction emblem/banner/uniform_concept; Item item_concept/inventory_icon/prop_sheet; Creature creature_concept/full_body/anatomy_detail. Stable IDs use `visual-{family}-{snake_case_role}-v1`, version 1. Each defines required/optional semantic facts, composition, semantic aspect, background/transparent intent and forbidden secret categories. Quest has no recipe and is rejected at the backend boundary.
- Pure compiler selects visible fields rather than dumping payloads or ForgeDNA. Character faction_hint is contextual design language, not canonical insignia or clothing. Faction ideology is supporting design context only for uniform concepts. Creature weakness is optional for anatomy detail only when its text contains a visible anatomy cue (eye/limb/bone/scale/skin/wing/fang/shell/joint); no B9 creature role is a scene/encounter role, so signs_of_presence is excluded. Location danger is supporting scene mood. Secret, hidden_truth, notes, story hooks and generation constraints are never selected.
- Bounds: six direction strings ≤512 UTF-8 bytes each, ≤16 entries per fact list, nonblank facts ≤4096 bytes per value, serialized visible facts ≤32 KiB, streaming canonical source/brief hashing ≤1 MiB, standalone brief exports ≤256 KiB. Oversized input rejects rather than silently truncating it. No project-global art-direction state or persistence is introduced.
- Snapshot records exact entity/project IDs, family, loaded persisted write version, source Canon state, optional revision, available generation kind/version/content/preset/axes and exact canonical source body hash. Tauri hashes the exact stored Candidate/Canon wrapper, including DNA/field origins; private values contribute to provenance hashing without being serialized into the brief. B7/B8 private context constraints are explicitly covered by a regression.
- **Hash semantics:** exact source-body and complete brief hashes cover provenance; visual-context hash covers recipe/version/role, selected visible facts, direction and available generation tuple/preset/axes. A secret-only edit changes exact source provenance/write version/complete brief identity, while leaving selected visual facts, visual-context hash and creative prompt unchanged. Claiming the entire provenance-bearing snapshot stayed byte-identical would contradict the required exact source-body hash. Identical complete source/role/direction inputs reproduce the entire snapshot/brief exactly.
- Desktop compiles only the current persisted saved state; it does not infer a Canon revision from a write version. Historical inspectors remain read-only and do not expose Visual Forge. `canon_revision = null` explicitly means the exact current persisted state. The core optional revision reference is available for future safe explicit revision resolution; this mission does not fabricate historical write versions.
- Brief includes concise subject, structured required/supporting facts, frozen direction, composition/aspect/background/transparent intent, avoid/consistency notes, provenance and deterministic readable prompt. Fixed ordering and identifier redaction prevent raw UUID/hash inclusion in creative prompt text. Source facts take precedence over conflicting optional direction; no provider flags, model names, quality-spam boilerplate, hidden reasoning or new fictional facts are generated.
- Provider boundary is an interface only, with opaque associated Settings/Request/Output/Error types and aspect/transparency/reference/seed/variation capabilities. No implementation, configuration or invocation exists. `VisualAssetCandidate` is serializable future metadata with Candidate-only status, no path/bytes/Canon field; no instance or database row is generated.
- Saved current Candidate/Canon detail views for all five supported families expose **Visual Forge**. Family-specific roles, transient direction, compile/recompile, exact facts/state/role/provenance and selectable prompt are shown. Notice: **No image provider is configured in this foundation build.** No Generate Image button, fake image, or generated-asset success state exists. No clipboard plugin/capability is installed, so Select prompt + ordinary keyboard copying and JSON/Markdown export provide the allowed fallback.
- Backend retains one transient authoritative compiled preview. Export binds to its complete brief hash, reads current source version/body hash and verifies brief integrity. Source changes reject export without mutating/deleting the old preview. UI can check source, warns on newer known versions, checks before selecting/copying or export, and provides explicit recompile. Wrong preview identity or another project rejects; late responses after unmount are ignored.
- JSON/Markdown use the existing atomic create-new/no-clobber export writer. Destination comes from the existing operator-entered save-path flow, never from Entity text; default filename is fixed. Markdown includes the same prompt and full structured brief/provenance. No DB writes occur during compile/check/export. Candidate and Canon read-only behavior, stale/duplicate/wrong-format export rejection, exact JSON readback and Markdown output are tested against real temporary SQLite storage.

## Compatibility and boundaries

Schema **2**, forgeproject archive **v1**, app **1.0.0**, existing generator/content/protocol versions remain unchanged. No dependencies, capabilities, credentials or global tooling added. Existing storage/contracts, all six generator implementations, Forge Further implementations and package/lock/version files have no diff. Existing Entity/Relationship/Canon/history/export behavior remains covered by passing regressions.

No image provider, cloud/local model call, image-tool shell execution, binary image download/generation, binary Visual Asset persistence, asset-directory layout, forgeproject v2, Visual Canon, Link Existing, multi-source Forge or later mission is implemented. No reset, staging, commit, tag, push, publication or user-process termination occurred. The FOUNDATION portable ZIP and earlier/default executables were not replaced.

## Verification

Final consolidated `scripts/verify.ps1` and optimized Tauri `build --no-bundle` completed successfully (exit 0). Explicit final no-fail-fast workspace run: **216 Rust tests PASS**, one existing fixture writer ignored; desktop 21, core 107, contracts 17, export 26, storage 45. **107 frontend tests PASS**. Strict all-target Clippy, full offline metadata and the 1,000-sequence gauntlet have passed. Frontend format/lint/typecheck/production build, Rust format, strict all-target Clippy, Cargo metadata and `git diff --check` PASS. The existing gauntlet passed 1,000 sequences / 32,000 operations.

Final executable: `C:\dev\CharacterForge\target\mission-b9-verify\release\character-forge-desktop.exe`; 15,199,232 bytes; ProductVersion/FileVersion 1.0.0; SHA-256 `F13196980D00A38113FD81654563D3E37DD6DAA2DD9F91A4B959E8E95DD15B13`. This is the isolated no-bundle build, not a replacement portable release package.

Ignored local evidence under `target/`: `b9-clippy-final.log`, `b9-workspace-tests-final.log`, `b9-final-verify.log`, `b9-frontend-tests-final.log`, `b9-typecheck-final.log`, `b9-cargo-metadata.json`, `b9-npm-test.log`. Isolated Cargo target: `C:\dev\CharacterForge\target\mission-b9-verify`.

`npm test` was attempted after TypeScript changes. Existing machine shim cannot find `C:\Users\pontu\AppData\Roaming\npm\node_modules\npm\bin\npm-cli.js`; no npm PASS is claimed. Configured Corepack/pnpm 11.25.0 suites pass. No global tooling installed.

Initial compile/test setup issues were corrected before final runs: version newtype `.0` access, EntityId UUID formatting, a frontend CreativeAxes type name and the test environment's absent window.prompt. The export function was implemented generically with existing serialization support, avoiding a new production dependency. No unresolved B9 P0/P1/P2 is known.

## Residual limitations

- No independent native operator smoke or real provider acceptance is claimed. Automated React tests use mocked IPC; command-boundary tests use actual core/storage/export helpers.
- Historical revision brief compilation is not exposed; exact current persisted state is explicit, with no fabricated revision number. Adding that workflow requires safe historical source-version handling.
- Weakness anatomy selection is a bounded text cue, not a semantic model. Recipe v1 keeps other weaknesses out and never adds secret fields.
- A brief remains a truthful historical snapshot if a source changes later. Export checks current source immediately before writing; it cannot prevent another process from editing after that check. Complete source/version/body provenance is retained, never silently relabelled.
- Existing P3 UI-001 remains open and needs a fresh operator check. Mission 08's accepted incomplete automated transitive advisory scan remains maintenance/release verification risk, not a confirmed vulnerability; direct/runtime/feature review found no confirmed applicable vulnerability, and ZIP 8.6.0 is outside the reviewed range below 2.3.0. The entire dependency graph is not claimed automatically verified clean.
- FOUNDATION package remains unsigned and requires WebView2 Runtime; no MSI/NSIS installer is claimed. B9 builds an isolated no-bundle executable only.

## Worktree and preserved files

Branch `master`, HEAD `a925d7f6cf1a5048afc2fee49184da2b16c1c369` unchanged. Index empty. Twelve mission-owned files remain unstaged/uncommitted: `GAZINGA/DISCOVERY_STATE.md`, `GAZINGA/PROJECT_STATE.md`, `apps/desktop/src-tauri/src/lib.rs`, `apps/desktop/src/App.tsx`, `apps/desktop/src/api.ts`, `apps/desktop/src/VisualForge.tsx`, `apps/desktop/tests/App.test.tsx`, `apps/desktop/tests/VisualForge.test.tsx`, `crates/app-core/src/lib.rs`, `crates/app-core/src/visual.rs`, `crates/export/src/lib.rs`, and this report. Four unrelated files remain untracked and byte-identical:

| File | SHA-256 |
| --- | --- |
| `The Ashen Claim.json` | `32BC3B62555F32A7AB4A8CA936B52F4979305C08027937274E636E6E8033A844` |
| `The Ashen Claim.md` | `A298F7A2DAFDFB4AC4FE354607C67CB565B0CFD38D9D45D9085D1AAA8FEDFC0D` |
| `mission09-final-verify.log` | `958E927350F940EA6075A0E2A4452E922A1C780ADF9EE97052EC59C3DD8FE6AD` |
| `mission09-rust-test.log` | `8E16F7D11EF7432BB341CC22512D59A384321AAD0CFFCF10F9133B315234F0D2` |

## Exit-gate re-evaluation

| Mission criterion | Result / evidence |
| --- | --- |
| Provider-neutral roles and recipes | PASS — typed closed roles and fifteen versioned recipes |
| Five deterministic bounded source snapshots | PASS — all families/roles, replay and bounds regressions |
| Provider-neutral brief and readable prompt | PASS — frozen structured brief and fixed-order prompt |
| No default secret leakage | PASS — all-family sentinel and B7/B8 frozen-private-context regressions |
| No raw identity/hash in creative prompt | PASS — identifier redaction regression |
| Exact source state/provenance | PASS — actual persisted wrapper hash/version/state; optional revision explicitly absent for current state |
| Safe stale-source behavior | PASS — frozen preview retained; current version/hash rechecked; stale export rejected |
| Desktop preview/export | PASS — five family actions, direction, selectable prompt, JSON/Markdown no-clobber export |
| Quest deliberately unsupported | PASS — UI absent and backend rejects |
| No network/provider/binary persistence | PASS — interface only; no provider calls or asset rows |
| No schema/archive/app-version bump | PASS — schema 2, archive v1, app 1.0.0 unchanged |
| B2/B6/B7/B8 and standalone generators green | PASS — unchanged implementations and full workspace regressions |
| Full verification green | PASS — all required automated gates above; npm shim limitation disclosed separately |
| No known B9 P0/P1/P2 | PASS — none identified within reviewed/automated scope |

**STOP at B9.** Native operator smoke has not been performed by Codex. Actual provider/persistence choices require a new operator decision; no B10+ work begins.
