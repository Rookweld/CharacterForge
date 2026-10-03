# THE FORGE B10.R3 — CHARACTER SUBJECT DOMINANCE + PROMPT BUDGET

## Objective

Fix the next verified Visual Forge failure without changing the working Local ComfyUI provider.

B10 works end-to-end.
B10.R1 fixed literal animal/species drift.
B10.R2 increased specificity, but the first real operator retest over-corrected into object/scene dominance.

Live failure evidence from the saved Character:
- Name: Mara Crowl
- Title: Keeper of the Unlit Mile
- Archetype: Chapel Rat Physician
- Canonical role: keeper of parish keys that fit no church
- Appearance: Funeral linen covers one eye and bears a fresh name each dawn.
- Signature gear: a six-shot chapel pistol wrapped in funeral linen

The exact positive prompt sent to ComfyUI began with:
- human dark fantasy physician
- Role / profession: keeper of parish keys that fit no church
- then many role props, chapel accents, extinguished-lamp accents, mile-marker accents, chapel/waystation scenery, and rat symbolism

The resulting image was effectively a symbolic architectural/relic-like object instead of a human Character.

This is not a provider failure. It is a Character prompt hierarchy / semantic classification / cue-budget failure.

## Scope

Prompt/brief compiler remediation only.

Do NOT:
- redesign B10 provider plumbing
- change ComfyUI HTTP/discovery/queue/history/preview/save/variation behavior
- add cloud providers
- add Visual Asset persistence or Visual Canon
- change database schema
- change .forgeproject format
- change app version
- change Character/Quest generation semantics or ForgeDNA
- change default image-generation settings unless a test proves it is necessary for this specific bug
- add provider-specific weighting syntax

Preserve B10.R1 and B10.R2 protections unless this mission explicitly refines their prompt assembly.

## Required design

### 1. Add an explicit Character subject anchor

Every Character prompt must begin with a short, self-sufficient subject anchor that alone describes what the image is primarily about.

For a human Character it should semantically contain:
- one human Character as the dominant focal subject
- explicit human/person identity
- the strongest supported occupation/profession if one exists
- one or two highest-value canonical physical/gear cues
- selected visual role/composition intent

Example target meaning for this fixture:

"A single human dark-fantasy physician as the dominant focal subject, one eye covered by funeral linen, carrying a medical satchel and a chapel pistol."

Exact prose may differ, but the first prompt block must be sufficient to keep the model centered on a person.

For explicit non-human Characters, preserve the explicit canonical physical form instead of forcing human.

### 2. Separate occupation from narrative role

Do not label arbitrary Character `role` text as "Role / profession".

Introduce a deterministic bounded distinction such as:
- occupation/profession
- narrative/social role

Occupation may be inferred only from explicit supported occupation vocabulary present in canonical/public Character fields such as archetype, role, appearance, or other already-authorized visible facts.

For this exact fixture:
- occupation/profession must resolve to `physician`
- `keeper of parish keys that fit no church` must remain narrative role/context, NOT profession

If no supported occupation is present:
- omit occupation
- do not invent one

No LLM inference.

### 3. Character subject dominance invariant

Character prompts must explicitly state that:
- the Character/person is the primary visual subject
- props, symbols, architecture, and environment are secondary supporting elements
- occupational readability must not replace the person

For portrait/full-body/character-sheet roles, add bounded anti-drift negatives appropriate to Character only, e.g. semantic equivalents of:
- object-only composition
- architectural detail as sole subject
- emblem-only image
- still life
- empty environment
- shrine/relic/door/wall ornament as sole subject

Do not apply these negatives to Item, Location, Faction or Creature families where such compositions may be correct.

### 4. Introduce a deterministic prompt budget

R2 currently emits too many concrete cues at once.

For Character prompts, implement a bounded selection budget. A reasonable v1 target:

- subject anchor: mandatory
- occupation evidence: max 2 strong concrete cues
- canonical appearance/gear: preserve highest-value explicit facts, but avoid redundant paraphrases
- title/archetype prop accents: max 2
- environment/setting cues: max 1 primary + optionally 1 tiny secondary cue
- symbolic motifs: max 1
- supporting context: only if it adds a distinct visible cue not already represented
- style: 1 compact block
- composition: 1 compact block

Exact constants may differ if justified, but there must be code-level bounded limits and tests.

Do not simply truncate text by characters. Select semantically by category and priority.

### 5. Priority order

Character prompt priority must be:

1. subject/species/personhood
2. actual occupation/profession if supported
3. canonical appearance
4. canonical signature gear
5. limited occupation evidence
6. narrative role
7. limited title/archetype accents
8. limited setting cue(s)
9. one symbolic motif
10. compact style
11. compact composition

Broad atmosphere must never outrank subject identity.

### 6. De-duplicate semantically overlapping cues

Avoid repeating the same concept across multiple blocks.

Examples:
- chapel should not independently cause garment trim + candle wax + chapel arch + votive niche + chapel scenery unless budget permits and each adds unique value
- unlit should not independently create multiple lamp references
- mile/keeper/road should collapse into one bounded roadside/marker cue
- rat symbolism should remain one subtle motif

### 7. Keep canonical facts distinct from visual interpretation

Retain the R2 provenance rule:
- canonical facts remain exact/source-derived
- visual interpretation remains explicitly marked as interpretation
- no interpreted prop becomes Canon, owned gear, faction membership, place, relationship or persisted truth

### 8. Preserve R1 literalization safety

Keep:
- human default for Character unless canonical data explicitly says non-human
- animal epithets/archetypes symbolic by default
- explicit non-human Character literal forms preserved
- Creature literal non-human behavior preserved

### 9. Preserve family separation

Do not regress Item / Location / Faction / Creature prompt behavior.
Character-only subject-dominance negatives must not leak into other families.

## Required tests

Add/update tests that prove all of the following:

1. Exact Keeper / Chapel Rat Physician fixture:
   - begins with a human Character subject anchor
   - occupation resolves to physician
   - "keeper of parish keys that fit no church" is NOT emitted as profession
   - canonical funeral-linen eye cue and signature chapel pistol remain represented
   - occupation evidence is bounded
   - chapel/unlit/mile accents are bounded
   - rat symbolism appears at most once as a subtle motif
   - prompt explicitly makes the person the dominant focal subject
   - Character anti-object/anti-architecture negatives are present

2. Prompt-budget invariants:
   - deliberately cue-heavy title/archetype input cannot exceed category budgets
   - repeated semantic cue families are collapsed/de-duplicated
   - output remains deterministic and bounded

3. Unknown/no-occupation Character:
   - no occupation is fabricated
   - narrative role is preserved as narrative role

4. Explicit non-human Character:
   - literal form remains correct
   - no forced-human regression

5. Creature:
   - literal creature anatomy remains valid
   - Character anti-object negatives do not leak

6. Item / Location / Faction:
   - existing family behavior remains distinct and valid

7. Existing B10/B10.R1/B10.R2 tests remain green.

## Verification

Run and report:
- focused visual tests
- frontend format
- frontend lint
- frontend typecheck
- frontend tests
- frontend production build
- cargo fmt --all -- --check
- strict clippy --workspace --all-targets -- -D warnings
- cargo test --workspace --no-fail-fast
- scripts/verify.ps1
- optimized Tauri no-bundle build in a fresh isolated target directory
- git diff --check

The known global pnpm launcher issue may be worked around with Corepack / a temporary PATH shim, but do not modify product behavior to solve the developer environment.

## Operator acceptance

Automated prompt tests are necessary but not sufficient.

At exit gate provide a dedicated R3 verification executable and exact path/hash.

Operator will re-run the same Mara Crowl / Keeper of the Unlit Mile / Chapel Rat Physician image against Local ComfyUI.

Success means the output is recognizably a HUMAN CHARACTER first, with physician identity visually readable, while chapel/unlit-mile/rat cues support rather than replace the subject.

## Repository safety

Preserve these unrelated files and never stage them:
- The Ashen Claim.json
- The Ashen Claim.md
- mission09-final-verify.log
- mission09-rust-test.log

Write:
docs/missions/THE_FORGE_B10_R3_SUBJECT_DOMINANCE_PROMPT_BUDGET_REPORT.md

Update GAZINGA/DISCOVERY_STATE.md and GAZINGA/PROJECT_STATE.md only where materially required.

STOP at the B10.R3 exit gate.
Do not begin B11 or any later Visual Asset persistence / Visual Canon work.
