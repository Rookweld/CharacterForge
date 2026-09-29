# EXECUTION CONTRACT

You are executing one bounded B1 remediation on the CURRENT uncommitted B1 worktree.

Before coding:

1. Read GAZINGA/CANON_CORE.md.
2. Read GAZINGA/THE_FORGE_CANON_CORE.md.
3. Read GAZINGA/DISCOVERY_STATE.md.
4. Read GAZINGA/missions/THE_FORGE_B1_PLACE_CORE.md.
5. Read docs/missions/THE_FORGE_B1_PLACE_CORE_REPORT.md.
6. Read docs/missions/THE_FORGE_B1_R1_LOCATION_NAME_LOCK_INDICATOR_REPORT.md.
7. Read this remediation completely.
8. Inspect the actual Location UI field descriptors and Tauri LocationField parsing before editing.

Do not reset/discard the current B1 worktree.
Do not start B2.

# THE FORGE B1.R2 — LOCATION PROFILE ACTION KEY PARITY

## Operator finding

During the human B1 recheck, Location Profile actions fail.

Observed UI error:

    Unknown Location field: location_profile

The operator also re-reported the Name lock indicator, but independent process inspection proved the running app was:

    C:\dev\CharacterForge\target\release\character-forge-desktop.exe

R1 did NOT replace that executable because Windows denied replacement while it was running. The actual R1 verified binary is in the isolated target directory.

Therefore:
- do not reopen/re-fix the already-verified R1 Name renderer unless current source evidence contradicts it;
- fix the independently confirmed Location Profile action-key defect;
- ensure the next operator recheck uses a newly rebuilt/current binary, not the stale default executable.

## Reproduced root cause from independent review

Current Location UI descriptor exposes:

    key: 'location_profile'

The typed Rust/Tauri field enum serializes:

    LocationField::Profile => 'profile'

The stable ForgeDNA control key is:

    01-location-profile

This means:
- display payload property = location_profile
- action/domain field identifier = profile
- ForgeDNA control key = 01-location-profile

The UI currently conflates the display payload property name with the action/domain field identifier.

Re-verify before editing.

## Required behavioral fix

Location Profile must behave as a normal Location Forge field.

At minimum:

- Lock Location Profile succeeds.
- Locked visual state appears.
- Button becomes Unlock Location profile.
- Reroll Location Profile uses the valid domain action key.
- Edit Location Profile, if the existing UI exposes Edit for enum/profile fields, must either:
  a) work with valid profile values, or
  b) be intentionally disabled/specialized if free-text editing would violate the typed enum.
- Reroll All Unlocked respects a locked profile.
- No stable Rust FieldKey is renamed.
- Character/Quest field action behavior is unchanged.

Preferred design:
- distinguish UI/display property name from domain action field key cleanly;
- do not add one-off string replacement inside Tauri to accept an incorrect frontend contract unless there is a strong compatibility reason.

A simple Location descriptor with domain key 'profile' and value l.location_profile is acceptable if it preserves all intended UI behavior.

## Regression tests

Add frontend tests using a REAL LocationCandidate-shaped payload.

Required proof:

### A. Profile lock
- render Location Candidate
- click Lock Location profile
- api.setLock receives ('profile', true)
- returned payload has 01-location-profile locked=true
- Location profile shows Locked
- button reads Unlock Location profile
- aria-pressed=true

### B. Profile reroll
- click Reroll Location profile while unlocked
- api.reroll receives ('profile')
- no Unknown Location field error path is possible through the UI contract

### C. Locked profile / reroll-all
Where practical in frontend or existing Rust tests:
- locked 01-location-profile remains unchanged during reroll-all-unlocked

Keep existing Location Name R1 regression green.
Keep existing Character Name regression green.

## Current-binary / operator-recheck requirement

The prior R1 report documented:

- default target replacement failed with Access denied
- isolated current binary:
  apps/desktop/src-tauri/target/mission-b1-r1-verify/release/character-forge-desktop.exe

The operator accidentally ran the stale default binary.

For R2 closeout:

1. Produce a clearly identified current verification binary.
2. Record its exact full path and timestamp in the R2 report.
3. If the default target executable is locked, do NOT claim it is current.
4. Do not require overwriting/killing a user-running process automatically.
5. Operator instructions must explicitly tell which executable to launch.

## Scope

Expected files:
- apps/desktop/src/App.tsx
- apps/desktop/tests/App.test.tsx
- possibly minimal type metadata if needed
- docs/missions/THE_FORGE_B1_PLACE_CORE_REPORT.md
- GAZINGA/DISCOVERY_STATE.md
- dedicated R2 report

Avoid unrelated refactors.

## Verification

Run:

- npm test as repository instructions require; if known global npm launcher remains broken, record exact failure
- repository-managed frontend tests using corepack/pnpm
- frontend format
- frontend lint
- frontend typecheck
- frontend production build
- focused relevant Rust/Tauri tests if any contract changes
- scripts/verify.ps1 using isolated CARGO_TARGET_DIR if default executable remains locked
- git diff --check

## Documentation

Create:

docs/missions/THE_FORGE_B1_R2_LOCATION_PROFILE_ACTION_KEY_PARITY_REPORT.md

Update:

- docs/missions/THE_FORGE_B1_PLACE_CORE_REPORT.md
- GAZINGA/DISCOVERY_STATE.md

Record:
- operator's earlier full smoke passed except Name visual indicator and Location Profile action;
- Name visual source fix remains R1 automated PASS but had been tested against stale default binary;
- R2 fixes the profile action identity mismatch;
- a final minimal operator check must use the exact current binary path.

## Exit gate

Automated R2 PASS requires:

- Location Profile UI sends domain field key 'profile' (or equivalent correct typed contract)
- lock visibly works
- reroll targets the correct field
- locked profile survives reroll-all
- Location Name R1 regression remains green
- Character/Quest regressions remain green
- full required verification passes
- exact current executable path is reported

Then require ONE final minimal operator recheck on the exact current executable:

1. Forge/open a Location.
2. Lock Name → confirm Locked + Unlock Name.
3. Lock Location profile → confirm Locked + Unlock Location profile.
4. Unlock profile and reroll it → confirm no error.

Do not repeat the entire B1 smoke.

STOP.
Do not begin B2.
