# THE FORGE — KNOWN ISSUES

## UI-001 — Location lock visual state can fail to reflect functional lock

**Status:** DEFERRED / non-blocking for B2
**Severity:** P3 UX / visual-state feedback
**First observed:** B1 operator smoke, 2026-09-29

Observed:
- Location Name and/or Location Profile can be functionally locked while the Workbench does not reliably show the expected Locked tag / Unlock label in the operator environment.
- Functional lock behavior itself has been observed to hold across rerolls.
- Automated frontend regressions for Name and Profile lock state pass in source builds.

Known complication:
- Earlier operator checks sometimes launched a stale default executable because Windows had prevented replacement while the old binary was running.
- R1/R2 isolated builds passed their automated lock-indicator regressions.
- A later operator check still reported missing visual lock feedback.

Decision:
- Continue development after explicit operator approval.
- Do not treat this as a data-integrity or deterministic-generation blocker.
- Keep this issue open and require a dedicated Workbench lock-state cleanup before the next release-quality gate.

Acceptance for continued development:
- lock state remains persisted/functionally enforced
- rerolls respect locked values/nonces
- no data loss
- no silent unlock

Do not silently mark this resolved because automated tests pass. Resolution requires a fresh operator check against a clearly identified current binary.
