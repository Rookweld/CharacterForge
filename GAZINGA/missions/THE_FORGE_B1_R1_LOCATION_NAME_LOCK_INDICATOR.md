# EXECUTION CONTRACT

You are executing one bounded B1 remediation on the CURRENT uncommitted B1 worktree.

Before coding:

1. Read GAZINGA/CANON_CORE.md.
2. Read GAZINGA/THE_FORGE_CANON_CORE.md.
3. Read GAZINGA/DISCOVERY_STATE.md.
4. Read GAZINGA/missions/THE_FORGE_B1_PLACE_CORE.md.
5. Read docs/missions/THE_FORGE_B1_PLACE_CORE_REPORT.md.
6. Read this remediation completely.
7. Inspect the actual current App.tsx lock-rendering code and Location field keys before editing.

Do not reset/discard the current B1 worktree.
Do not start B2.

# THE FORGE B1.R1 — LOCATION NAME LOCK INDICATOR

## Operator finding

Human B1 smoke otherwise passed.

Observed defect:

- Location Name lock FUNCTIONALLY held during reroll.
- Other locked Location fields visibly showed the locked state.
- Location Name did NOT visually show that it was locked.

Treat B1 as PARTIAL until this visual-state defect is fixed and independently reviewed.

## Reproduced root cause from independent review

Current Workbench rendering special-cases Name:

    field.key === 'name'
      ? '03-name'
      : suffix lookup...

That key is Character-specific.

Character Name key:
- 03-name

Location Name key:
- 02-name

The lock mutation path already resolves by suffix and therefore functional locking works.
The visual rendering path uses the hardcoded Character key and therefore fails to display Location Name as locked.

Re-verify this in the actual code before editing.

## Required fix

Remove the Character-specific Name-key assumption from visual lock-state resolution.

Preferred direction:
- use the existing generic field-control resolver, or
- use the same suffix-based lookup used by lock mutation

The rendered lock state must resolve the correct field control for Character, Quest, and Location without hardcoding per-entity Name numbering.

Do not rename existing stable field keys.

## Regression test

Add a frontend regression test using a REAL LocationCandidate-shaped payload whose Location Name control is:

    02-name: { locked: true, ... }

The test must prove that after locking Location Name:

- api.setLock receives ('name', true)
- Name renders with the locked visual state
- the button becomes accessible as Unlock Name
- aria-pressed = true
- the Locked tag is present for the Name field

Do not satisfy this test with a Character payload.

Keep existing Character Name-lock regression green.

## Scope

Expected files:
- apps/desktop/src/App.tsx
- apps/desktop/tests/App.test.tsx
- docs/missions/THE_FORGE_B1_PLACE_CORE_REPORT.md
- GAZINGA/DISCOVERY_STATE.md
- optional dedicated R1 report

Do not refactor unrelated Workbench code.

## Verification

Run:

- npm test as required by repository AGENTS; if the known global npm launcher failure remains, record it exactly
- repository-managed frontend tests using corepack/pnpm
- frontend format
- frontend lint
- frontend typecheck
- frontend production build
- scripts/verify.ps1
- git diff --check

Do not weaken tests.

## Documentation

Create:

docs/missions/THE_FORGE_B1_R1_LOCATION_NAME_LOCK_INDICATOR_REPORT.md

Update:

- docs/missions/THE_FORGE_B1_PLACE_CORE_REPORT.md
- GAZINGA/DISCOVERY_STATE.md

Record that operator smoke passed all other requested B1 behavior and the only observed issue was this lock indicator.

## Exit gate

Automated R1 PASS requires:

- Location Name uses the correct 02-name control through generic resolution
- visual locked state is proven by a real LocationCandidate frontend test
- Character Name lock test remains green
- required frontend/full verification passes
- no unrelated behavior changed

After automated R1 PASS, B1 requires only a minimal operator re-check:

1. launch the rebuilt app
2. Forge/open a Location
3. lock Name
4. confirm Name visibly shows Locked / Unlock Name

Do NOT require the operator to repeat the entire B1 smoke unless another defect is found.

STOP.
