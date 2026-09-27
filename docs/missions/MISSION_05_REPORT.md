# Mission 05 — ANVIL

## Status

**PASS — all Mission 05 exit criteria are satisfied.** The Character Name lock defect was corrected at the Tauri boundary, automated verification passed, and the human operator re-verified the updated packaged application including lock preservation across rerolls and save/reopen, plus Name eligibility after unlock.

## Scope delivered

- Replaced the static desktop shell with a React/Tauri application for project creation/open/close, Character and Quest generation, all canonical presets and creative axes, seed control, constraints, structured candidate presentation, field locks, single-field and unlocked-field rerolls, manual edits, candidate saves, favorites, canon promotion, history snapshots, and canon revision inspection.
- Added a typed frontend invoke API and narrow Tauri commands. The frontend does not issue SQL or perform arbitrary filesystem operations. Project databases are opened through the Rust storage boundary.
- Added project-aware Quest generation so the generated Quest envelope carries the active project's ID. ForgeDNA field selection and deterministic stream behavior are otherwise unchanged.
- Added loading, empty, validation, database-error and persistence-status UI states, keyboard-visible focus, accessible field action names, non-color-only lock/canon indicators, and reduced-motion styling.
- Added 17 frontend workflow tests and eight desktop-boundary unit tests. Existing on-disk storage tests continue to cover persistence and reopen behavior.
- Documented the new command/UI/storage boundary in `docs/ARCHITECTURE.md`.

## Final lock-preservation remediation

### Reproduced defect and root cause

The Name control key was correct end to end: React displays `03-name`, sends the field enum as `name`, and the Tauri boundary resolves it to `CharacterField::Name`. Character Forge core protects the locked Name value and nonce, including when an Archetype reroll invalidates dependent Title.

The defect was in the Tauri application adapter. It forcibly assigned `character.envelope.name = character.title` after initial generation, single-field rerolls, reroll-all, and manual Title edits. Thus a dependent Title update could overwrite a locked Name in the full-candidate response after core had preserved it. The UI then correctly replaced its candidate with that incorrect response. No ForgeDNA dependency or core lock behavior caused the loss.

The regression was reproduced at the command-operation boundary with Name locked and an Archetype reroll. Before the fix, the test observed Name changing from `The Operator's Locked Name` to `The Bell Beneath the Hill` while its lock flag and nonce remained intact.

### Corrective change

- Removed the adapter's implicit Name/Title synchronization from generation, reroll, reroll-all, and Title-edit operations. Name and Title now remain the distinct core fields they already are.
- Kept reroll and lock execution in the existing Character/Quest core operations. ForgeDNA, dependency declarations, nonce behavior, and generator architecture are unchanged.
- Extracted the shared command-side candidate operations so tests exercise the same logic called by Tauri commands.

### UI path trace

- Lock Name then reroll Title or Archetype: the core preserves locked Name; the adapter previously copied recomposed Title over it. The overwrite is removed and Archetype dependency reroll is covered.
- Lock Name then reroll all: core excludes the locked field; the adapter no longer rewrites the response. Covered with nonce, lock, value, origin, and serialization assertions.
- Change axes then choose Forge: this creates a new candidate with a new recipe and fresh locks; it is a full generation replacement, not a reroll of the locked candidate.
- Lock, save, then reroll: the session retains the current candidate; the reroll command mutates through the shared helper, and storage tests validate locked-state persistence on disk.
- Close/reopen then load/reroll: `get_entity` installs the loaded record as the working candidate and returns its saved controls. The real on-disk regression confirms the Name lock survives reopen and an Archetype reroll.
- Edit another field, including Title: the old adapter synchronized Title into Name after Title edits. That coupling is removed and Title editing under a Name lock is tested.
- Load a saved entity: the library detail path calls `get_entity`, which updates the backend working entity and write version before later actions.
- Frontend refresh/reconciliation: React accepts the full candidate returned by Tauri. Command-side tests verify that payload preserves Name; the frontend regression checks the locked Name remains rendered after full-candidate replacement.

The Archetype-to-Name dependency is intentional generation context. The domain lock firewall already protects locked Name from that dependency's reroll; the adapter overwrite was the cause.

### Regression coverage

- Desktop command-boundary tests cover a locked Name through a Title-dependent Archetype reroll, reroll-all, full-candidate JSON serialization, Title manual edit, lock/unlock mapping, and post-unlock Name reroll eligibility. They assert Name value, lock, nonce, and field origin.
- A generation-boundary test confirms the command returns the core's independent Name and Title values.
- A real on-disk close/reopen test confirms locked Name value, lock, nonce, and origin survive reload and a subsequent Archetype reroll.
- A Quest lock test exercises the shared command-side reroll path.
- A frontend test verifies the Name control sends `name` and displays the preserved Name/lock after replacing state with the full-candidate reroll response.

The command-boundary regression failed before the adapter fix and passed afterward.

## Verification

- `scripts/verify.ps1` — passed: frontend format, lint, typecheck, tests and production build; Rust format, workspace Clippy with warnings denied, workspace tests, and optimized Tauri no-bundle build. Frontend lint reported 8 non-failing CSS descending-specificity warnings.
- Frontend tests: **17 passed, 0 failed**.
- `cargo test --workspace`: **87 passed, 0 failed** (desktop 8, app-core 53, contracts 10, storage 16; other/doc-test targets 0).
- `cargo fmt --all -- --check` — passed.
- Workspace Clippy with `-D warnings` — passed.
- `cargo metadata --no-deps --format-version 1` — passed; all five workspace packages resolved.
- `git diff --check` — passed.
- `scripts/verify.ps1` — passed, including frontend checks and tests, Rust formatting, strict Clippy, workspace tests, and optimized Tauri build.
- Tauri optimized no-bundle build — passed using an isolated Cargo output directory because the earlier executable was open and Windows denied replacing it. Updated executable: `C:\dev\CharacterForge\target\mission05-lock-remediation\release\character-forge-desktop.exe`.
- The first default-target rebuild attempt was blocked with Windows `Access denied`; process inspection confirmed `target\release\character-forge-desktop.exe` was still running. No process was terminated. The isolated-target `scripts/verify.ps1` run completed successfully.
- Real database close/reopen cases passed within the storage suite, including the new locked-Name value/lock/nonce/origin case and existing Character/Quest metadata, generation history, and canon revision snapshots.
- `git diff --check` — passed after removing two extra trailing blank lines found during final review.
- Required `npm test` — could not start because the machine's global npm launcher references missing `C:\Users\pontu\AppData\Roaming\npm\node_modules\npm\bin\npm-cli.js`. The repository's pnpm test command succeeded.

## Operator GUI verification — 2026-09-27

The human operator completed the initial smoke test in a normal interactive Windows desktop using the packaged executable:

- Application launch and real project create/open — **PASS**.
- Character generation, preset/creative-axis controls, single-field reroll, manual edit, save, and Characters library — **PASS**.
- Character lock behavior — **PARTIAL / FAIL:** the operator observed that the name changed while locked; another locked field remained unchanged.
- Quest generation, Quest reroll/lock/edit, save, and Quests library — **PASS**.
- Canon promotion and history/revision UI — **PASS**.
- Full application close, relaunch, project reopen, and persistence of Character, Quest, manual edit, lock state, canon state, and history/revisions — **PASS**.
- Operator's overall smoke result — **PASS**, with the Character name lock exception recorded above.

The lock exception was an application behavior defect, not an observation-environment limitation. The source fix and automated regression tests are complete. The operator then repeated the focused lock check using `C:\dev\CharacterForge\target\mission05-lock-remediation\release\character-forge-desktop.exe` and reported **PASS**:

- Forged a Character, recorded its Name, and locked Name.
- Rerolled an unrelated field and rerolled all unlocked fields; Name remained unchanged both times.
- Edited Title; Name remained independent and unchanged.
- Saved the Character, fully closed the application, relaunched the updated packaged executable, and reopened the same project; Name and its lock remained intact.
- Unlocked Name and rerolled it; Name became eligible to change as expected.

This operator result verifies the packaged fix and closes the previously outstanding acceptance gate.

## Mission 05 exit-criterion review

- Welcome/project create and open through the packaged UI — **PASS**.
- Character/Quest workspace, preset and creative-axis controls, seed controls, generation, structured results, editing, saves, favorites, canon promotion/revisions, and history access — **PASS** by operator observation.
- Per-field lock visibility and lock preservation — **PASS** by updated packaged-app operator retest; locked Name survived unrelated-field reroll, reroll-all, Title edit, save, close/relaunch, and project reopen. After unlock, Name reroll eligibility was restored.
- Project close/reopen and durable Character, Quest, edit, lock, canon, and history state — **PASS** by operator observation.
- Typed Tauri/storage boundary, automated frontend/Rust verification, and optimized packaged executable — **PASS** as recorded above.
- Automated verification, optimized packaged build, and operator acceptance — **PASS**.
- Overall Mission 05 exit gate — **SATISFIED**. **MISSION 05 — PASS.**

## Known limits

- The user-facing path entry is a manual project path field; no native file-dialog dependency was added.
- No automated test drove a live Tauri window through the complete cross-screen workflow. Frontend interaction tests use mocked command responses; persistence close/reopen is covered by real on-disk Rust tests.
- Export entry points remain unavailable as allowed for Mission 06.
- The updated executable was operator-verified at the path above; the old default-path executable had been open during the isolated build.
- No commit was created.

## Repository state

- Branch: `master`.
- HEAD: `75f736c5fdbd8456f13896d87ab7005a7d42fffd` (`feat: complete ANVIL desktop workflow`). The worktree was clean before this remediation.
- Current worktree changes are limited to the Character command boundary, frontend regression test, storage reopen regression test, and Mission 05 documentation; no commit was created.

## Closeout

**MISSION 05 — PASS.** Every exit criterion is satisfied by the recorded packaged-app operator verification and automated evidence above. Mission 06 was not started.
