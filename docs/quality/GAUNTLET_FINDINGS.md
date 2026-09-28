# GAUNTLET Findings

Baseline: `master` at `c60eef04cfae14928cdda8c7d8f6d4909b13033b`.

## Product findings

### G07-F001 — RESOLVED — History event snapshot identity (P2)

Distinct history rows can share a forge `run_id`; using that provenance identifier to fetch a snapshot could show the wrong payload. History records now expose and query the unique existing SQLite `event_id`. Tauri preview and working-copy paths use that event identity. No schema change. Regression and close/reopen tests pass.

### G07-F002 — RESOLVED — Out-of-order History preview response (P2)

Rapid selection could allow an older asynchronous response to replace the latest preview. Request sequencing and navigation invalidation now suppress stale success/error/finally handlers. The UI regression test deliberately resolves responses in reverse order and passes.

### G07-F003 — RESOLVED — Canon revision working-copy save semantics (P2)

The product decision defines same-entity transient working copies and an explicit Canon-safe save. The save flow now shows a structured old/new diff and warns that the existing current Canon will be preserved as revision history. Cancel performs no persistence call; confirmation requests a reason and uses the existing atomic Canon revision transaction. Character and Quest regression coverage proves A/B/C stay immutable, D becomes current, identity is retained, no clone is created, Undo/Redo does not persist, and the result survives reopen.

## Residual verification limits (non-blocking observations)

- No dedicated memory sampling or large-history open/list timing distribution was captured; the bounded persistent stress run completed without errors, and no hard performance SLA was invented.
- Malformed archive, database corruption, recovery, restore rollback, Unicode, and retry cases are covered; not every conceivable malformed-input permutation was enumerated.

Finding totals: P0 0; P1 0; P2 3 (all resolved); P3 0; unresolved findings 0.

Automated Mission 07 evidence: 45,056 Character + 45,056 Quest deterministic generations PASS; 1,000 deterministic state sequences / 32,000 operations PASS; 114 Rust tests; 36 frontend tests; `scripts/verify.ps1`, optimized Tauri build, format, lint, typecheck, strict Clippy, metadata and diff hygiene pass. The global npm launcher issue is environment-only; repository pnpm tests pass.

Packaged operator result: **PASS**. The operator verified the Canon A working-copy cancellation and confirmation flows for Character and Quest, preservation of A/B/C and identity/locks/provenance, D as current Canon after restart, History/navigation coherence, export/import, backup/restore, resize/keyboard use, and recovery from an expected error. No operator-visible P0/P1/P2 defect remains.

All three P2 findings are resolved; no Mission 07 finding or exit gate remains open. Mission 07 — GAUNTLET: **PASS**. Mission 08 has not started.
