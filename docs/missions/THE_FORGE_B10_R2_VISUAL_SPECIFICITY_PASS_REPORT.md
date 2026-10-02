# THE FORGE B10.R2 â€” Visual Specificity Pass

## Exit gate

**PARTIAL â€” implementation complete; final verification in progress.**

Mission authority: the operator-pasted **GAZINGA â€” THE FORGE B10.R2 â€” VISUAL SPECIFICITY PASS**, executed with the repository launcher and both canon cores. Starting/current branch `master`, HEAD `3dbb60931be301a11049b5e0ae207a8aad08c22a`. B10 and B10.R1 were already committed; only the four unrelated files were untracked at entry. No reset, staging, commit or later mission.

## Problem and bounded remedy

The operator reports that R1 resolved literal-rat drift but the Keeper of the Unlit Mile / Chapel Rat Physician still becomes a generic attractive gothic/nun-like portrait. That is operator evidence about the previous compiler, not an R2 image acceptance result.

Only the pure core prompt/brief compiler and its tests changed. There are no frontend, provider, HTTP, ComfyUI workflow, discovery, queue/history, polling, preview, Save Image, variation, stale-source, persistence, asset, Canon, archive, dependency or credential changes.

### Identity and occupation precede styling

Character order is now: subject/species and named profession; canonical role; distinctive canonical appearance; named signature gear; concrete working props/garments; title/archetype accents; setting clues; symbolic motifs; style/atmosphere; composition. The composition reinforces readability of occupation tools, garment details and identifying motifs in the selected framing. No provider-specific weight syntax is introduced.

The finite profession vocabulary supplies readable visual evidence: medical instrument roll/bandages/medicine bottles for physician/healer/apothecary; measuring tools and repair kit for engineer; annotated pages and ink-marked cuffs for scholar; evidence notebook for detective; message case for courier; and other bounded occupation families. Unrecognized roles use the actual role and named gear with material/function/wear cues rather than assigning a new occupation.

Canonical appearance and signature gear remain verbatim in the frozen context and positive prompt. Supplemental role evidence is explicitly labeled **visual interpretation** and subordinate to those source facts. It proposes illustrative tools, accents and staging; it does not assert new possessions, clothing, membership, places, Canon facts or relationships.

### Public labels affect concrete details

- `chapel`: faded votive-stitch garment trim and candle-wax wear; secondary weathered chapel arch/votive niche.
- `unlit`: extinguished lamp accessory, candle-soot marks, unlit lamp silhouettes.
- `mile` together with keeper/road/route/way: mile-marker motif on a strap/tool case and roadside marker/waystation/parish-route fragment.
- `bell`: worn bell-shaped accessory/brass clapper detail.
- `ash`: ash-grey garment edging and soot-marked working materials.

These bounded whole-word cues use public Name/Title/Archetype only. Rat and the other R1 risk motifs receive a concrete placement on garment trim or a small tool-case ornament. They never assign physical species. Background details are subordinate to the working subject and equipment; empty direction does not erase the identity clues.

### Conservative negative channel

Default Character negatives add `anonymous glamour headshot` and `fashion editorial without occupational cues`. They are omitted when the operator explicitly requests glamour/fashion/beauty styling. Chapel-associated Characters without canonical clergy cues additionally exclude `nun-only portrait without occupational tools` and `monk-only portrait without occupational tools`; genuine clergy in public labels, role or appearance are exempt. No age, gender, attractiveness or general clergy ban is introduced. R1 anti-literal-animal/mascot/parody guards remain separate and unchanged.

### Visible inspection

The existing R1 UI already displays positive/negative prompts, classification, safeguards and the structured identity/symbolism sections. It now displays the added role evidence, prop/setting interpretation and safeguard text from the compiled brief. JSON/Markdown exports contain the full structured brief. No extra UI or provider change is necessary.

## Example: regression fixture, not a generated image

The test fixture uses the named Keeper/Chapel Rat labels, canonical human appearance and physician role. Its prompt begins:

```text
human dark fantasy physician
Role / profession: physician
Physical identity: A scarred human physician with silver hair and a patched dark coat
```

Before styling, tested cue strings include medical instrument roll, folded bandages, stoppered medicine bottles, practical treatment apron, worn cuffs, medicinal stains, chapel-associated accents, extinguished lamp, candle-soot, roadside mile-marker, waystation and subtle rat symbolism on garment trim or a tool-case ornament. Composition keeps occupational details readable. Negative text retains literal rat/anthropomorphic rodent/animal head/mascot/parody guards plus the conditional specificity guards.

Changing the public labels to Bell Keeper / Street Physician removes chapel/unlit/mile scenery and adds the brass clapper detail while retaining medical role evidence. This proves labels change props and scenery, not merely their quoted text.

## Compatibility / provenance

Visual brief format is **3**, explicitly distinguishing the revised compiled prompt semantics. Context snapshot format **1**, visual recipe IDs/versions **1**, source body hash inputs and context hash inputs are unchanged. Full brief hashes include the resulting semantic content; validation recomputes interpretation/guards and rejects altered self-hashed derived fields. Recompile a new transient brief rather than reuse an old format-2 export. Brief exports are not project imports.

The physical species classifier is unchanged from R1. Explicit non-human Characters remain literal; all five family prompt structures remain distinct and Creature anatomy remains literal. Non-Character positive/negative assembly does not receive Character specificity additions. Database schema **2**, `.forgeproject` **v1**, application **1.0.0**, Entity IDs/ForgeDNA/generator/content replay, locks, manual provenance, Canon/history and storage remain unchanged.

## Automated verification

| Gate | Result |
| --- | --- |
| Focused visual tests | PASS â€” 15 tests |
| Frontend format / lint / typecheck / production build | PASS |
| Frontend tests | PASS â€” 123 tests across four files |
| `cargo fmt --all -- --check` | PASS |
| Strict Clippy `--workspace --all-targets -- -D warnings` | PASS |
| `cargo test --workspace --no-fail-fast --offline` | PASS â€” 241 tests; one existing fixture writer ignored |
| Rust breakdown | Desktop 38; app-core 115; contracts 17; export 26; storage 45 |
| Offline Cargo metadata | PASS |
| `scripts/verify.ps1` | PASS — executed with a temporary Corepack-backed `pnpm.cmd` PATH shim because global `pnpm` is unavailable |
| Optimized Tauri no-bundle build | PASS — `target/mission-b10-r2-verify/release/character-forge-desktop.exe` |
| `git diff --check` / four-file preservation / empty index | PASS — line-ending warnings only; four unrelated files remain untracked |

`r`nFinal verification executable: `C:\dev\CharacterForge\target\mission-b10-r2-verify\release\character-forge-desktop.exe`; **16,012,288 bytes**; SHA-256 **0F4BC3272543781ADF1555126A80BE4C7A37AB0957843F13D29B3ECDF14E55EE**. This is a verification executable, not a newly published release package.`r`n`r`nThree new core tests cover public-label prop/scene changes; profession-specific evidence, immutable canonical facts/replay and forged-derived-field rejection; and conditional negatives preserving actual clergy/explicit styling. Expanded R1 tests cover the complete physician specificity cue set and order, all thirteen risky words, explicit non-human forms, literal Creature and family separation. Existing B10 desktop/provider/UI tests remain in the full suites.

Focused tests used the existing isolated R1 debug cache. Full verification and the final optimized executable use a fresh isolated `target/mission-b10-r2-verify`; the R1/default release executable is not replaced. Evidence logs: ignored `target/b10-r2-*.log`.

## Operator acceptance and limitations

Automated compiler/provider evidence cannot establish that a checkpoint renders these cues correctly. **Actual R2 image acceptance remains unverified; operator retest required.** No live server/model operation or independent native GUI/image-generation acceptance is claimed.

Operator: launch the R2 executable; open the saved Keeper / Chapel Rat Physician; recompile with empty optional direction; inspect human classification, physician tools/workwear, chapel/unlit-mile details and rat motif plus separate negatives. Use the existing Local ComfyUI checkpoint to generate. Confirm an obviously working human physician with the specific title/archetype cues, then check Save Image, variation and source-change warning. Compare an explicit non-human Character and literal Creature. If still generic, report checkpoint/settings and which compiled cues were visually absent.

This is a bounded lexical projection, not a complete semantic interpreter of arbitrary titles/professions. Suggested tools/clothing/backgrounds are illustrative staging and may need operator art direction to fit unusual source facts, era, or equipment. Long prompts and model attention limits may still omit details. Existing P3 UI-001 and the human-accepted Mission 08 incomplete automated transitive dependency advisory scan remain open residual limitations. No new security vulnerability or resolution of UI-001 is claimed.

## Repository ownership / stop

Owned files: `crates/app-core/src/visual_prompt.rs`, `crates/app-core/src/visual.rs` (compiler and tests), this report, `GAZINGA/DISCOVERY_STATE.md`, `GAZINGA/PROJECT_STATE.md`. No JavaScript files or production dependencies changed.

Four unrelated files remain untouched/untracked and are never staged: `The Ashen Claim.json`, `The Ashen Claim.md`, `mission09-final-verify.log`, `mission09-rust-test.log`. SHA-256 baseline values are checked again at closeout. At automated closeout the mission-owned changes are ready for selective staging/commit; no tag, push, or publication is performed.

STOP at the B10.R2 remediation exit gate. No later work begun.

