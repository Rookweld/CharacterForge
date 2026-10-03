# THE FORGE B10.R3 - Character Subject Dominance + Prompt Budget

## Status / exit gate

**PARTIAL - all automated remediation gates PASS; operator image acceptance PENDING. STOP.**

Authority: `GAZINGA/missions/THE_FORGE_B10_R3_SUBJECT_DOMINANCE_PROMPT_BUDGET.md`, repository launcher and both canon cores. Starting/current branch `master`, HEAD `4fa3a05596dbb7f8134bb2d9c7b5d946c46b4ea1`. B10/R1/R2 are committed baseline work. At entry only the four unrelated operator/export/log files were untracked. No reset/stage/commit/tag/push/publication; no later mission.

## Observed failure and remedy

The mission records the operator's real R2 failure: Mara Crowl / Keeper of the Unlit Mile / Chapel Rat Physician rendered as a symbolic architectural/relic-like object. The saved narrative role is `keeper of parish keys that fit no church`; appearance is `Funeral linen covers one eye and bears a fresh name each dawn.`; gear is `a six-shot chapel pistol wrapped in funeral linen`. This is prompt dominance/semantic-budget evidence, not a provider failure. R3 changes only the pure compiler/brief contract and core tests.

### Mandatory subject anchor and occupation distinction

Every Character prompt now starts with a self-sufficient single-Character anchor: canonical species/personhood, dominant focal subject, strongest supported occupation if present, selected canonical physical and signature-gear clauses, and the selected portrait/full-body/reference framing. Species classification is unchanged from R1, including explicit non-human physical forms.

Occupation is an optional typed field in the derived `character_selection` plan. A finite 29-word vocabulary is searched deterministically in canonical Archetype, Role, then Appearance, with local negation rejection. Bare `keeper`/`warden` are narrative responsibilities and do not become a profession. No LLM inference, default profession or occupation guessed from gear/setting. For the live fixture occupation is **physician**, while the parish-keys text remains labeled **Narrative role**. Unknown/unsupported occupations are omitted.

### Code-level semantic budgets

| Category | Limit / selection |
| --- | --- |
| Subject anchor | Mandatory, first; includes selected physical/gear cues once |
| Canonical appearance | One whole short clause for human; up to two short species/anatomy clauses for explicit non-human forms |
| Canonical gear | One whole short functional clause |
| Occupation evidence | At most 2 single cues from finite profession families; canonical overlap filtered |
| Title/archetype prop accents | At most 2 unique families, fixed priority: unlit, chapel, bell, ash |
| Setting | At most 1 distant roadside/mile-marker cue |
| Symbolic motif | At most 1 deterministic risk-family motif; human metaphor safeguards retained |
| Supporting personality/ability/faction context | 0 extra cues in this conservative R3 pass; exact selected facts remain in the context |
| Style | 1 compact block: medium up to 12 words; palette/materials/atmosphere each up to 8 words |
| Composition | 1 compact role/dominance block; optional camera clause up to 12 words |

Selection is by category, physical/functional relevance and deterministic family priority, not by cutting the final prompt at a character limit. Eligible source clauses are whole clauses (up to 512 UTF-8 bytes, 32 words for physical/narrative cues, 24 for gear). Long unselected clauses remain exact in structured source context. An explicit non-human form may retain two eligible species/anatomy clauses to preserve its literal identity. No clause is partially rewritten into a new canonical assertion.

Priority: species/person + actual occupation + physical cue + gear in anchor; limited occupation evidence; narrative role; limited prop accents; distant setting; one motif; compact style; compact composition. Broad atmosphere stays late.

### Deduplication and provenance

Canonical chapel pistol already represents the chapel family, so the live fixture gets no chapel garment trim/arch/votive niche. Unlit becomes one carried lamp; mile/keeper/road becomes one distant marker, not a marker prop plus waystation/parish-route scene. Rat becomes one small garment motif unless canonical appearance/gear already represents that family, in which case no extra motif is added. Repeated labels cannot multiply category entries. Canonical words also suppress overlapping occupation cues.

Interpretation is explicitly labeled secondary in the prompt and stored in a typed derived selection plan. The exact original facts, narrative role, source/body/context hashes and provenance remain available in the structured brief. Interpreted tools/props/scenery never create possessions, affiliations, locations, relationships or Canon. Validation recomputes the plan and rejects forged self-hashed selections.

### Subject-dominance negatives / family isolation

Character-only negatives cover object-only composition, architecture as sole subject, emblem-only image, still life, empty environment, and shrine/relic/door/wall ornament as sole subject. Positive composition explicitly keeps the Character primary and props/symbols/architecture/environment secondary. These additions do not apply to Item, Location, Faction or Creature. R1 anti-literal-rat/anthropomorphic-rodent/animal-head/mascot/parody safeguards and R2 conditional glamour/clergy negatives are retained.

Non-Character prompt assembly, ComfyUI discovery/HTTP/core-node workflow/queue/history/preview/Save Image/variation/stale-source behavior, settings and frontend are unchanged. No provider-specific weighting syntax, new dependency, credentials, model downloads, assets or archive work.

## Exact regression-fixture positive prompt

Extracted from the passing core test for the operator-provided public fixture (not an actual generated image):

```text
A single human dark fantasy physician Character as the dominant focal subject; Funeral linen covers one eye and bears a fresh name each dawn; carrying a six-shot chapel pistol wrapped in funeral linen; portrait with face and carried gear readable.
Occupation evidence (visual interpretation, secondary): medical satchel; folded bandages
Narrative role: keeper of parish keys that fit no church
Prop accents (visual interpretation, secondary): one extinguished lamp carried as a small accessory
Setting (visual interpretation, secondary): one faint roadside mile-marker in the distant background
Symbolic motif (visual interpretation, secondary): subtle rat symbolism as one small garment motif
Style: dark fantasy clear concept illustration
Composition: portrait with face and carried gear readable; the Character is primary; props, symbols, architecture and environment are secondary supporting elements.
```

This is paired with the existing literalization negatives and new Character subject-dominance negatives. No hidden/private Character fields are emitted.

## Versions / compatibility

Visual brief format is **4**, recording the added derived Character selection and changed compiler semantics. Context format **1**, visual recipe IDs/versions **1**, source/context hashing inputs, Entity IDs/ForgeDNA/generator/content replay, Canon/revisions, database schema **2**, `.forgeproject` **v1**, application **1.0.0** and image-generation settings are unchanged. Complete brief hashes cover the derived selection. Non-Character prompts retain their prior positive/negative content. Compile a fresh transient brief with the R3 executable; old standalone brief exports are not project imports.

## Verification

| Gate | Result |
| --- | --- |
| Focused visual suite | PASS - 20 tests |
| Frontend format/lint/typecheck/production build | PASS |
| Frontend tests | PASS - 123 tests across four files |
| Rust format | PASS |
| Strict Clippy `--workspace --all-targets -- -D warnings` | PASS |
| Explicit workspace `--no-fail-fast --offline` | PASS - 246 tests; one existing fixture writer ignored |
| Rust breakdown | Desktop 38; app-core 120; contracts 17; export 26; storage 45 |
| Offline Cargo metadata | PASS |
| `scripts/verify.ps1` | PASS - final code, pnpm 11.25.0, offline Cargo |
| Optimized no-bundle build, fresh isolated R3 target | PASS - dedicated executable built; first isolated optimized build 2m 43s, final rebuild 52.69s |
| Final `git diff --check`, empty index and four-file hash audit | PASS - four files byte-identical to entry SHA-256; index empty |

Five new tests cover canonical-motif de-duplication, the exact Mara fixture across all three Character roles, cue-heavy/repeated-family budgets, unknown/negated occupation vs narrative role, and whole-clause selection preserving exact source facts with forged-plan rejection. R1/R2 tests were updated only for mission-authorized anchor/order/budget refinements; literalization, explicit non-human, specificity, conditional negatives, deterministic replay and source immutability remain tested. Existing B10 desktop/provider/UI tests stay in the full suites.

Focused-test first run found one historical family test still requiring the old Character `Physical identity:` heading. It now checks the mandatory Character anchor; non-Character headings and behavior are retained. The hierarchy rerun passed all 19 tests; the final canonical-motif edge-case regression brings the focused suite to 20 passing tests. The first consolidated run caught a collapsible condition and two unnecessary clones in test assertions; these lint issues were corrected without changing prompt behavior. Strict Clippy then passed, and full final-state suites and consolidated verification passed after the final de-duplication change.

Full verification uses `C:\dev\CharacterForge\target\mission-b10-r3-verify`, a fresh isolated target. Default/R1/R2 release executables are not replaced. Evidence logs: ignored `target/b10-r3-*.log`. The normal sandbox process initially failed while applying deny-read ACLs; approved command execution was used for bounded repository work. Available pnpm/Corepack both report 11.25.0; no product configuration or global tooling was changed to repair the environment.

## Dedicated verification executable

- Exact path: `C:\dev\CharacterForge\target\mission-b10-r3-verify\release\character-forge-desktop.exe`
- ProductVersion / FileVersion: **1.0.0 / 1.0.0**
- Length: **16,030,720 bytes**
- SHA-256: **FCCB992FC5EB776625DD6D323CF041975893712F4AA21C52F9B2C0E103D870FC**
- SHA-256 independently recomputed with Python and matched the PowerShell `Get-FileHash` result.

Verification executable only; no new release package, installer, tag, push or publication.

## Operator gate and residual limits

**Actual R3 image acceptance is PENDING.** Automated prompt/provider tests are necessary but do not prove image composition. No live ComfyUI generation or independent native GUI/operator acceptance is claimed.

1. Launch the dedicated R3 executable and open the same saved Mara Crowl Character.
2. Recompile Portrait with the same Local ComfyUI checkpoint/settings used in the failed R2 retest.
3. Confirm the opening anchor says single human physician, funeral-linen eye and chapel pistol; parish-keys text is narrative; lamp/marker/rat cues remain secondary and budgeted.
4. Generate: accept only if it is recognizably a HUMAN CHARACTER first and the physician role is visually readable. Props/chapel/unlit-mile/rat cues must support the person.
5. Check Save Image, variation, preview/provenance and stale-source warning; compare explicit non-human Character/literal Creature when practical.

The finite vocabulary is not a general natural-language occupation/species interpreter. Clause selection may omit complex or overlong visual facts from provider text; they remain exact in context/export. Lexical overlap filtering is conservative, not a universal semantic parser. Model attention/checkpoint behavior can still ignore cues; no token-capacity guarantee is claimed. Existing P3 UI-001 and the human-accepted Mission 08 incomplete automated transitive dependency advisory scan remain residual limitations. No new security finding or resolution of those limitations is claimed.

## Worktree / stop

Owned files: `crates/app-core/src/visual_prompt.rs`, `crates/app-core/src/visual.rs` (brief/compiler/tests), this report, `GAZINGA/DISCOVERY_STATE.md`, `GAZINGA/PROJECT_STATE.md`. No JavaScript/provider/dependency files changed.

Four unrelated files remain untracked and untouched: `The Ashen Claim.json`, `The Ashen Claim.md`, `mission09-final-verify.log`, `mission09-rust-test.log`. SHA-256 values are checked against entry baseline at final closeout. Branch/HEAD unchanged; mission changes unstaged/uncommitted; index empty.

STOP at B10.R3 for operator image verification. No B11, Visual Asset persistence, Visual Canon or later work.
