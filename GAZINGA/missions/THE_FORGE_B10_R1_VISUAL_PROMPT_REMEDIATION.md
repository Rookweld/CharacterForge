# THE FORGE B10.R1 — VISUAL PROMPT REMEDIATION

## Objective

Fix Visual Forge prompt compilation quality without changing the working B10 ComfyUI provider architecture.

Current failure example:
A Character with archetype "Chapel Rat Physician" generated a literal rat instead of a human dark-fantasy physician with subtle rat symbolism.

## Scope

Prompt/brief remediation only.

Do NOT:
- redesign B10 provider plumbing
- add cloud providers
- add Visual Asset persistence
- add Visual Canon
- change forgeproject format
- add unrelated features

## Required behavior

1. Add explicit subject classification before prompt assembly.

For Character:
- default to human unless canonical data explicitly says non-human
- animal words in title/archetype/epithet are symbolic by default, not literal anatomy
- literal animal/non-human rendering only when canonical species/appearance explicitly requires it

Recommended internal fields:
- subject_kind: human | humanoid | creature | beast | object | location | faction_symbolic | unknown
- species_mode: explicit | inferred-default
- literalization_risk: low | medium | high
- humanoid_guardrail: bool

2. Separate prompt semantics into:
- identity
- flavor/symbolism
- negative guardrails

3. Character prompt order:
- primary subject/type
- physical identity
- role/profession
- clothing/equipment
- setting
- symbolic motifs
- style
- composition

4. Add literalization safeguards for risky metaphor terms such as:
rat, raven, crow, wolf, serpent, spider, beast, ghost, bone, plague, carrion, ash, butcher

Example:
"Chapel Rat Physician"
must compile toward:
"human dark fantasy physician, subtle rat symbolism"

and negatives such as:
"literal rat, anthropomorphic rodent, animal head, mascot, parody"

5. Preserve literal creature behavior for Creature entities.

6. Make entity-family prompting distinct:
- Character = person/identity
- Item = object/material/function
- Location = environment/architecture/atmosphere
- Creature = literal non-human allowed
- Faction = symbol/emblem/representative visual language

7. Expose in Visual Forge UI:
- compiled positive prompt
- compiled negative prompt
- subject classification
- safeguards applied

8. Empty optional art-direction fields must not create a weak prompt.
Use canonical entity data and existing universe/preset context to provide sensible defaults.

9. Keep Local ComfyUI generation, model discovery, preview, Save Image, variation and stale-source handling green.

## Tests

At minimum prove:
- "Chapel Rat Physician" => human subject + anti-rat-literalization guards
- animal metaphor Character => symbolic only unless explicitly non-human
- explicit non-human Character => literal form preserved
- Creature remains literal
- Character / Item / Location prompt structures differ correctly
- existing B10 provider tests remain green

## Verification

Run:
- frontend format
- frontend lint
- frontend typecheck
- frontend tests
- frontend production build
- cargo fmt --all -- --check
- strict clippy -D warnings
- cargo test --workspace --no-fail-fast
- scripts/verify.ps1
- optimized Tauri no-bundle build
- git diff --check

Preserve these unrelated files and never stage them:
- The Ashen Claim.json
- The Ashen Claim.md
- mission09-final-verify.log
- mission09-rust-test.log

Write docs/missions/THE_FORGE_B10_R1_VISUAL_PROMPT_REMEDIATION_REPORT.md.
Update discovery/project state only where materially required.
STOP at the remediation exit gate.

Target result:
The same Keeper of the Unlit Mile / Chapel Rat Physician test should produce a HUMAN dark-fantasy physician with subtle rat symbolism, not His Holiness Pope Rattus I.
