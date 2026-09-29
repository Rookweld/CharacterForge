# THE FORGE A.R1 — Relationship Revision Parity Report

**Status: PASS — Relationship Canon revision history matches the Entity revision mental model**

## Defect reproduced and corrected

Before remediation, `revise_relationship` serialized the prior current Canon body into the next numbered revision, while updating the current row to the submitted new body. Promotion A followed by explicit revision A→B therefore produced R1=A, R2=A, current=B.

The implementation now serializes the validated submitted Canon relationship into the new numbered revision and uses that same serialized state for the current-row update. The immutable relationship identity, project, type, and endpoints remain validated as before. Timestamp assignment, optimistic write-version checks, transaction behavior, and archive behavior are unchanged.

The lifecycle regression proves:

- Promotion: R1 equals Canon A and current is A.
- First explicit revision: R1 remains A; R2 is B; current is B.
- Stale expected version: rejected as a conflict; revision sequence and current B remain unchanged.
- Second explicit revision: R1=A, R2=B, R3=C; current=C.
- Close/reopen: exact A/B/C order and current C persist.
- Archive: existing behavior remains unchanged and retains the last Canon assertion as historical evidence.

The test changes only the already mutable `origin`/provenance field; relationship ID, project, type, and endpoints are unchanged.

## Tests and verification

- Focused `cargo test -p forge-storage relationship_candidate_lifecycle_is_revisioned_and_stale_safe -- --nocapture`: **1 passed**.
- `cargo test -p forge-storage`: **23 passed, 0 failed, 1 ignored** (the ignored test is the one-time frozen-fixture writer).
- `cargo test --workspace --no-fail-fast`: **121 passed, 0 failed, 1 ignored** (12 desktop, 55 app-core, 12 contracts, 19 export, 23 storage).
- `scripts/verify.ps1`: **PASS**. Frontend formatting, lint, TypeScript typecheck, 36 frontend tests, production build, Rust formatting, strict Clippy, workspace tests, and optimized Tauri no-bundle build passed.
- The first `scripts/verify.ps1` attempt stopped on rustfmt differences in the new assertions; `cargo fmt --all` applied standard formatting, and the complete script then passed.
- `git diff --check`: **PASS**.

No dependencies were added. No unrelated implementation or product behavior was changed.

## Changed files for R1

- `crates/storage/src/lib.rs`
- `crates/storage/src/tests.rs`
- `docs/missions/THE_FORGE_A_SUBSTRATE_REPORT.md`
- `docs/missions/THE_FORGE_A_R1_RELATIONSHIP_REVISION_PARITY_REPORT.md`
- `GAZINGA/DISCOVERY_STATE.md`
- `GAZINGA/PROJECT_STATE.md`

The pre-existing Mission A worktree changes remain present and uncommitted. Pre-existing operator export files and Mission 09 logs remain untouched.

## Residual limitations

No R1 security, data-integrity, or verification gate remains outstanding. Mission A remains within its existing bounded substrate scope; no Mission B feature was started.

## Repository state

- Branch: `master`
- HEAD: `31f69ac800c07ffdf5ab5f567f80d5593b9684e` (`docs: canonicalize The Forge entity universe discovery`)
- Worktree: modified and untracked Mission A/R1 changes remain unstaged and uncommitted; unrelated operator exports and Mission 09 logs are preserved.
- No commit, reset, tag, push, publication, or Mission B work was performed.
