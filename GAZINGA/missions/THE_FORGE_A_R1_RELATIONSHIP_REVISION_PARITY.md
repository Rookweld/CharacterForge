# EXECUTION CONTRACT

You are executing one bounded remediation for THE_FORGE_A_SUBSTRATE.

This is NOT THE_FORGE_B_PLACE.

Before coding:

1. Read GAZINGA/CANON_CORE.md.
2. Read GAZINGA/THE_FORGE_CANON_CORE.md.
3. Read GAZINGA/DISCOVERY_STATE.md.
4. Read GAZINGA/missions/THE_FORGE_A_SUBSTRATE.md.
5. Read docs/missions/THE_FORGE_A_SUBSTRATE_REPORT.md.
6. Read this remediation completely.
7. Inspect the current uncommitted Mission A worktree and the actual Entity/Relationship revision implementations.

Do not discard or reset the current Mission A worktree.

# THE FORGE A.R1 — RELATIONSHIP REVISION PARITY

## Why this remediation exists

Independent post-Codex review found one bounded semantic mismatch.

Existing Entity Canon history behaves like:

    promote A
      revision 1 = Canon A
      current    = Canon A

    revise to B
      revision 1 = Canon A
      revision 2 = Canon B
      current    = Canon B

The new Relationship implementation currently behaves like:

    promote A
      revision 1 = Canon A
      current    = Canon A

    revise to B
      revision 1 = Canon A
      revision 2 = Canon A   <-- duplicate prior state
      current    = Canon B

This was observed in:

- crates/storage/src/lib.rs :: revise_relationship
- crates/storage/src/tests.rs :: relationship_candidate_lifecycle_is_revisioned_and_stale_safe

The current test explicitly expects the second stored Relationship revision to retain the old origin, thereby locking in the mismatch.

This is not accepted as Mission A PASS semantics.

## Required invariant

Relationship Canon revision history must match the proven Entity mental model:

**Each Canon revision number represents the accepted Canon state at that revision.**

Therefore:

1. Candidate promotion to Canon creates Relationship revision 1 containing the promoted Canon state.
2. Explicit Canon revision from A → B creates revision 2 containing B.
3. Revision 1 remains A and remains inspectable.
4. Current Relationship state after the operation is B.
5. A later revision B → C creates revision 3 containing C.
6. No prior revision is overwritten.
7. Stale-write protection remains intact.

Do not create a duplicate snapshot of A merely because A was the previous current state; revision 1 already preserves it.

## Scope

Repair only Relationship Canon revision semantics and directly affected tests/docs/state.

Likely implementation surface:

- crates/storage/src/lib.rs
- crates/storage/src/tests.rs
- docs/missions/THE_FORGE_A_SUBSTRATE_REPORT.md only if needed to correct its claim/evidence
- GAZINGA/DISCOVERY_STATE.md
- optional dedicated remediation report

Do not refactor unrelated storage code.

## Archive semantics

Do NOT broaden this remediation into a redesign of archival history unless a failing invariant proves it necessary.

Current archive/retire behavior may preserve the last Canon assertion as historical evidence.

The required fix is specifically explicit Canon revision A → B → C parity.

If archive semantics are changed for correctness, explain why with tests.

## Tests required

At minimum prove:

### A. Promotion
- Candidate A
- promote
- revision count = 1
- revision 1 == Canon A
- current == Canon A

### B. First explicit Canon revision
- revise A → B
- revision count = 2
- revision 1 == A
- revision 2 == B
- current == B
- revision 1 body unchanged

### C. Second explicit Canon revision
- revise B → C
- revision count = 3
- revision 1 == A
- revision 2 == B
- revision 3 == C
- current == C

### D. Stale write
- stale expected version cannot append a revision
- revision count/current state remain unchanged after the rejected write

### E. Reopen
- close/reopen
- A/B/C revision order and current C survive exactly

Use a meaningful changed field already allowed by Relationship identity rules, such as origin/provenance metadata. Do not mutate immutable relationship ID/project/type/endpoints.

## Regression gates

Run:

- focused Relationship lifecycle tests
- cargo test -p forge-storage
- cargo test --workspace --no-fail-fast
- scripts/verify.ps1
- git diff --check

If the full verify gate fails for an environment-only reason, distinguish it with evidence. Do not call PASS merely from focused tests.

## Report

Create:

docs/missions/THE_FORGE_A_R1_RELATIONSHIP_REVISION_PARITY_REPORT.md

Include:

- exact defect reproduced
- exact behavioral correction
- tests added/changed
- focused/full verification results
- branch/HEAD/worktree
- changed files
- any residual limitation

Update GAZINGA/DISCOVERY_STATE.md:

- Mission A remains PARTIAL while remediation is in progress.
- Mark THE_FORGE_A_SUBSTRATE PASS only after this remediation passes all gates.
- Do not start THE_FORGE_B_PLACE.

## Exit gate

PASS only if Relationship Canon revisions now form:

    R1 = A
    R2 = B
    R3 = C
    current = C

with immutable prior revisions, stale-write safety, reopen persistence, and full repository verification passing.

STOP.
