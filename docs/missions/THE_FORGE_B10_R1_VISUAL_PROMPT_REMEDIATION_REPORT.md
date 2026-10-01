# THE FORGE B10.R1 — Visual Prompt Remediation

## Exit gate

**PASS — B10.R1 automated remediation exit gates satisfied. STOP.**

Closed 2026-10-01. Actual generated-image quality remains an operator retest, not an automated acceptance claim.

Focused prompt/brief remediation from `master` HEAD `88c07d6f80cc14680c71f5934cfdc8e792c78c89`. No later mission was started.

## Changes and rationale

- A pure core compiler classifies the saved source before assembling its prompt. Character defaults to human; only canonical physical appearance supplies explicit non-human species/anatomy. Names, archetypes, epithets and optional art direction cannot assign species. Costume/mask/emblem cues do not override physical identity.
- Classification records subject kind, explicit/inferred-default species mode, literalization risk and the human anatomy safeguard. Whole-word risk detection covers rat, raven, crow, wolf, serpent, spider, beast, ghost, bone, plague, carrion, ash and butcher; it does not mistake `crown` for `crow`.
- Identity, flavor/symbolism and negative guards are separate structured fields. Character positive order is primary subject, physical identity, role, clothing/equipment, setting, symbolic motifs, style and composition. The physician regression begins `human dark fantasy physician`, with subtle rat symbolism. Literal rat, anthropomorphic rodent, animal head, mascot and parody belong to the negative channel.
- Explicit non-human Character appearances remain literal. Creature anatomy remains literal without Character human-only guards. Item uses object/material/function; Location uses environment/architecture/atmosphere; Faction uses symbolic/emblem/representative design language.
- Empty optional art direction uses canonical source facts, all eleven existing preset contexts where present, clear concept illustration, existing role composition, and a neutral Character background. It does not fabricate physical species or new canonical setting facts.
- Visual Forge displays positive/negative prompts, classification, applied safeguards and structured identity/symbolism. Existing copy/export and Local ComfyUI controls remain available. JSON and Markdown exports retain complete structured provenance.
- The existing fixed ComfyUI workflow receives `negative_prompt_text` on its existing negative CLIP node. Discovery, loopback HTTP, job ownership, polling, limits, preview, Save Image, variation and stale-source plumbing are unchanged.

## Compatibility and boundaries

Visual brief format is now **2** because its semantic payload and deterministic compiled prompt changed. Context snapshot format, visual recipe IDs/versions and source/context hashing inputs remain unchanged; complete brief hashes include the new classification, semantics and negative prompt. Validation recomputes these derived fields and rejects forged self-hashed classifications. Previously exported format-1 briefs are not imported into projects; compile a new brief for this compiler. Entity IDs, ForgeDNA, generator/content replay, Canon/revisions, storage schema **2**, `.forgeproject` **v1**, and application **1.0.0** are unchanged.

No dependencies, provider architecture, model downloads, custom nodes, credentials, remote/cloud hosts, Visual Asset DB persistence, Visual Canon or archive-v2 work were added. No commit, staging, tag, push or publication was authorized or performed.

## Verification

| Gate | Result |
| --- | --- |
| Frontend format / lint / typecheck / production build | PASS |
| Frontend tests | PASS — 123 tests across four files |
| Explicit `cargo test --workspace --no-fail-fast --offline` | PASS — 238 tests; one existing fixture writer ignored |
| Rust breakdown | Desktop 38; app-core 112; contracts 17; export 26; storage 45 |
| Rust format check / offline Cargo metadata | PASS |
| Strict Clippy `--workspace --all-targets -- -D warnings` / `scripts/verify.ps1` | PASS |
| Optimized Tauri no-bundle build | PASS — isolated target, optimized release in 3m 08s |
| Final `git diff --check` / preservation audit | PASS — four unrelated files retain baseline SHA-256; index empty |

Evidence logs are under ignored `target/b10-r1-*.log`; builds use isolated `target/mission-b10-r1-verify`. A focused test initially exposed beast-vs-humanoid classification for a rat-headed physician; the corrected implementation passed the complete explicit workspace suite. The final consolidated suite repeated all 238 Rust tests and 123 frontend tests successfully after the last readability-only core initialization cleanup. The existing 1,000-sequence storage gauntlet passed in both runs.

New regressions cover the named physician, all thirteen risky terms, masks and whole-word matching, explicit elf/anthropomorphic rodent/animal-headed/animal/unknown-species/werewolf appearances, all five entity-family structures, literal Creature anatomy, empty direction defaults, negative-channel separation, identifier redaction, exact deterministic replay, source immutability and forged classification rejection. A UI test proves both prompts and classification/safeguards are visible without automatic generation. Existing B10 workflow/lifecycle/UI tests remain part of the full suites.

Exact rebuilt executable: `C:\dev\CharacterForge\target\mission-b10-r1-verify\release\character-forge-desktop.exe`; ProductVersion/FileVersion **1.0.0**, **15,987,200 bytes**. Executable SHA-256: `8CDD6E70E7D695E2C8C6EA1FD865329F85FA94C2A1D003DD5B83492C07925AE4`. This is a verification executable, not a newly staged/published release package.

## Limitations and operator handoff

The physical-appearance classifier uses bounded lexical cues, not a universal natural-language species parser. Unrecognized explicitly declared species remains `unknown` and retains the physical description; unusual prose needs operator inspection. Image models may still disregard prompts; deterministic compiler tests do not establish actual generated-image quality. No live ComfyUI image or independent native GUI acceptance is claimed in this remediation.

Operator retest: launch the rebuilt executable, open the saved Keeper of the Unlit Mile / Chapel Rat Physician, recompile its Visual Forge brief, confirm human classification plus subtle rat symbolism and anti-literalization negatives, then generate with the existing local checkpoint. Inspect that the result is a human physician, then exercise preview, Save Image, variation and stale-source warning. Also check an explicitly non-human Character and literal Creature.

Existing P3 UI-001 remains open; it is not resolved by automated UI tests. Mission 08's accepted incomplete automated transitive dependency advisory scan remains a maintenance/release verification limitation, not a confirmed vulnerability. The global `npm test` launcher still fails because its `npm-cli.js` is missing; configured pnpm verification is used and the failure is recorded separately.

## Repository / ownership

Branch `master`; HEAD `88c07d6f80cc14680c71f5934cfdc8e792c78c89`; mission changes unstaged/uncommitted. The four unrelated files remain untracked and byte-identical: `The Ashen Claim.json`, `The Ashen Claim.md`, `mission09-final-verify.log`, `mission09-rust-test.log`.

Owned files: `crates/app-core/src/visual.rs`, new `crates/app-core/src/visual_prompt.rs`, `apps/desktop/src-tauri/src/comfyui.rs`, `apps/desktop/src/api.ts`, `apps/desktop/src/VisualForge.tsx`, both visual test fixtures/tests, this report, `GAZINGA/DISCOVERY_STATE.md`, and `GAZINGA/PROJECT_STATE.md`.

STOP at B10.R1 remediation exit gate.
