# Pre-GAUNTLET FOUNDATION Gap Fix

**Status: PASS.** The four audited P2 gaps are implemented, automated verification passes, and the human operator accepted the focused packaged-app workflows. Mission 07 remains NOT STARTED.

## Baseline and scope

The gap-fix began from clean committed `master` at `d01222c49ff908048c95e6d63d2d3b49d0f205da`. It addressed only the four P2 findings in `docs/FOUNDATION_COMPLETENESS_AUDIT.md`: session undo/redo, revision inspection/revision flow, non-destructive historical working copies, and application-local user settings. No generator/ForgeDNA behavior, Vault format, production dependency, Mission 07 scope, or unrelated audit finding was changed.

## Remediation

1. **Session undo/redo:** the desktop session now uses the existing `SessionHistory<StoredEntity>`. Forge replacement starts a fresh session; lock, field reroll, reroll-unlocked, and manual edits push complete candidate snapshots. Undo/redo controls expose availability and Ctrl+Z/Ctrl+Y/Ctrl+Shift+Z shortcuts; text editing retains native undo. A new mutation clears redo. These operations only change in-memory working state and never write SQLite history/canon.
2. **Revision inspection and canon revision:** prior canon revisions can be inspected as structured, read-only Character/Quest detail, including existing lock and manual provenance. A deliberate “Use as working copy” path and a reasoned “Save Canon revision” action use the existing revision transaction, which preserves the prior current canon. The current canon stays unchanged until that explicit save.
3. **Historical working copy:** History snapshots can be opened as editable working copies. Canon revision snapshots have the same path. Replacing unsaved work asks first; applying a snapshot changes only session state. It does not remove or rewrite history/revisions, silently change canon, or persist until explicit Save/Canon revision.
4. **Minimal settings:** added versioned, application-local reduced-motion preference using the existing Tauri app-config directory. Missing, malformed, oversized, or unsupported-version files fall back to the safe default. The setting is reachable from Project, persists across restart, and layers on top of the existing operating-system reduced-motion preference. Theme controls are intentionally excluded by this pass's explicit scope freeze.

## Verification

- Rust workspace: **107 tests passed** (12 desktop, 53 app-core, 10 contracts, 16 export, 16 storage; doc-test targets had zero tests).
- Frontend: **34 tests passed**.
- Formatting, frontend lint, TypeScript typecheck, frontend production build, strict Clippy, Cargo metadata, and `git diff --check`: PASS.
- `scripts/verify.ps1`: PASS, including optimized no-bundle Tauri build at `target/release/character-forge-desktop.exe`.
- Repository `pnpm test`: PASS.
- `npm test`: cannot start because the global npm launcher points to a missing `C:\Users\pontu\AppData\Roaming\npm\node_modules\npm\bin\npm-cli.js`; this remains an environment issue only.

## Operator verification status

**PASS.** The operator completed the focused review in the packaged desktop application and confirmed:

- Undo and Redo restored exact working-session state; a new mutation after Undo invalidated the prior Redo branch, and manual-edit/session state behaved correctly.
- Saved candidate/history and canon revisions were inspectable as read-only structured data, and inspection left current state unchanged.
- An older snapshot became the editable working copy without removing newer history or silently changing canon. Explicitly saving produced normal new history rather than overwriting older revisions.
- Reduced Motion persisted after fully closing and relaunching the application.
- No operator-visible P0/P1 defect was found in the remediated areas.

The following is retained as a reusable focused regression checklist, not a pending acceptance gate.

### Focused operator checklist

1. Forge a Character; reroll a field; Undo; Redo. Confirm the exact values and DNA-controlled lock/nonces return.
2. Manually edit a field; Undo and Redo. Confirm manual-edit provenance returns. Type in an editor and confirm Ctrl+Z edits its text instead of undoing the Forge session.
3. Save multiple Character revisions and inspect an older snapshot. Confirm it is read-only and structured.
4. Use the older history snapshot as a working copy. Confirm newer history entries remain. Edit and explicitly Save; confirm a new normal history event exists.
5. Repeat the history working-copy path for a Quest.
6. Inspect current Canon and prior Canon revisions. Use an old Canon revision as working material; confirm current Canon remains unchanged until explicit “Save Canon revision.” Supply a reason and confirm the old revision remains inspectable.
7. Open Project → Settings, enable Reduce motion, close and relaunch the app, and confirm the preference remains enabled. Turn it off and confirm the system preference still applies.

## Scope and limitations

- The implementation does not add theme choice, generation defaults, sound, default-folder customization, or unrelated preference controls; theme work is explicitly excluded in the gap-fix brief.
- User settings are application-local JSON; project settings remain separate and unchanged.
- No P0/P1 defect was found in automated or operator verification of these four remediated areas. Later mission gates remain separate.

## Git state

- Branch: `master`.
- HEAD: `d01222c49ff908048c95e6d63d2d3b49d0f205da` (baseline commit; no commit created during this pass).
- Worktree: modified implementation, tests, and closeout documentation; nothing staged or committed. Exact paths are recorded in `GAZINGA/PROJECT_STATE.md` at closeout.
- Mission 07: **NOT STARTED**. Stop after this closeout; the PASS makes FOUNDATION ready for GAUNTLET but does not start it.
