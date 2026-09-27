# Interlude 06.5 — WORKBENCH POLISH

## Status

**PASS — all Workbench Polish exit criteria are satisfied, including packaged desktop operator acceptance.** Mission 06 remains PASS and closed. Mission 07 and Mission 08 have not started.

## Scope and result

This pre-GAUNTLET pass improves the project entry path and everyday Workbench readability without changing generation, storage schema, project data, or adding a production dependency.

- Project creation asks for a containing folder and project name. Rust derives a safe `.sqlite3` filename while preserving the human-readable project identity, refuses to overwrite collisions, and returns actionable errors.
- Open Project accepts an existing SQLite project file; `.forgeproject` archives are explicitly directed to Import. Import accepts an archive and destination folder, keeps its embedded project name, and creates a safe filename without overwriting an existing project.
- The welcome screen groups Create, Open, and Import as distinct actions and offers up to eight local Recent Projects. Recent entries contain only display name, canonical path, and last-opened time; stale items are identified and can be forgotten. Project content remains in SQLite.
- Forge axis controls have contextual help and clearer seed/reroll guidance. Character, Quest, Canon, and History empty states explain the next useful action. Canon promotion explains that existing material is retained, and history actions use readable labels with historical snapshots identified as read-only.
- Vault actions are grouped into portability and safety tasks, with separate path inputs and an explicit restore warning. Saved project identity is labeled as the project file.
- Library state and long content are more readable, controls expose pressed/selected state, and focus/keyboard behavior remains native to buttons and inputs.
- Architecture documentation records the path derivation and Recent Projects boundary.

## Exit criteria review

- Create Project uses a normal project name and containing folder, preserves display identity, derives a filesystem-safe filename, and refuses collision overwrite: **PASS** (implementation, backend coverage, and operator review).
- Open and Import are separate, understandable workflows with file/folder-specific inputs and useful errors: **PASS** (UI coverage and operator review).
- Recent Projects is bounded local-only metadata; missing entries can be recognized and forgotten: **PASS** (Rust metadata coverage and operator review of project workflows).
- Character Forge and Quest Forge remain usable; preset, seed, and creative-axis controls are understandable: **PASS** (operator review).
- Lock, reroll, manual-edit, and saved/unsaved state are understandable: **PASS** (operator review).
- Character and Quest libraries are readable; Candidate versus Canon and Canon promotion are clear: **PASS** (operator review).
- History is understandable to an ordinary user: **PASS** (operator review).
- Export, Import, Backup, and Restore are coherently presented; Restore is consequential; user-visible errors and empty/busy/success states are useful and intentional: **PASS** (operator review).
- Laptop-sized desktop layout, keyboard navigation, and focus indication are credible: **PASS** (operator review).
- Packaged desktop product review and no operator-visible P0/P1 regression: **PASS** (operator result PASS).
- Automated checks, repository verification, and optimized Tauri packaging pass: **PASS** (evidence below).

No material Workbench Polish exit criterion remains unsatisfied. The operator considers the application coherent enough to proceed toward GAUNTLET. This closeout does not start that mission.

## Verification evidence

- Rust workspace: **105 passed, 0 failed** (desktop 10, app-core 53, contracts 10, export 16, storage 16).
- Frontend: **30 passed, 0 failed**.
- `scripts/verify.ps1`: PASS, including optimized Tauri build.
- Optimized packaged executable: `C:\dev\CharacterForge\target\release\character-forge-desktop.exe`.
- Frontend formatting: PASS.
- Frontend lint: PASS.
- TypeScript typecheck: PASS.
- Frontend production build: PASS.
- `cargo fmt --all -- --check`: PASS.
- `cargo clippy --workspace --all-targets -- -D warnings`: PASS.
- `cargo metadata --no-deps --format-version 1`: PASS.
- `git diff --check`: PASS.
- Repository `pnpm test`: PASS (30 tests).
- `npm test`: cannot start because the global launcher references missing `C:\Users\pontu\AppData\Roaming\npm\node_modules\npm\bin\npm-cli.js`; repository pnpm verification passes, so this remains an environment issue only.

## Deferred polish

- No native file-picker dependency was added; folder and file paths remain labeled text inputs because the repository does not include an approved dialog integration.
- Entity export continues to use its existing prompt-based destination interaction.
- No Mission 07 GAUNTLET or Mission 08 work has begun.

## Operator product review

- Packaged application reviewed: `C:\dev\CharacterForge\target\release\character-forge-desktop.exe`.
- Operator result: **PASS**. The application was reviewed as a real product, not as a developer harness.
- The no-project landing screen is understandable. Creating a project from a normal name and location no longer requires manually entering a `.sqlite3` path. Open Project and Import Project are clearly distinguishable.
- Character Forge and Quest Forge remain usable. Preset, seed, and creative-axis controls are understandable; lock, reroll, and manual-edit controls are understandable; saved versus unsaved state is clear.
- Character and Quest libraries are readable. Candidate versus Canon and Canon promotion are understandable. History is clear to an ordinary user.
- Export, Import, Backup, and Restore are coherently presented; Restore is appropriately consequential. Errors provide useful guidance, and empty, busy, and success states feel intentional.
- Resized/laptop-sized desktop layout remains usable. Basic keyboard navigation and focus indication are credible.
- No operator-visible P0/P1 regression was found. The operator considers the application coherent enough to proceed toward GAUNTLET.

## Repository state

- Branch: `master`.
- HEAD: `7d59619f6d03897aea994e7537fd236d983ce060` (`feat: implement portable project vault workflows`).
- Worktree: dirty with Interlude 06.5 frontend, Tauri/export, architecture, test, and closeout documentation changes. No commit, staging, or push was performed.
- Mission 06 remains PASS. Interlude 06.5 is PASS. Mission 07 and Mission 08 have not started.

## Stop boundary

Stop at Interlude 06.5. Do not begin Mission 07 or Mission 08.
