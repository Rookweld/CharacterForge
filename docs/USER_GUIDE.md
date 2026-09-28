# Character Forge / Quest Forge — User Guide

## Start and create a project

Launch the Windows application. On the welcome screen, choose a folder and a project name, then create the project. The app creates a local SQLite project file in that folder. To continue work, enter the path to an existing project file and open it, or choose a recent project. A `.forgeproject` archive is imported from the Import Project action rather than opened as a SQLite database.

Project data is kept in the project file at the location you chose. The app keeps a short Recent Projects list in its per-user application settings; it stores display names, paths, and last-opened times, not Character or Quest prose. The default new-project location is under Documents, but you can choose another folder.

## Forge a Character or Quest

Open Forge and choose Character or Quest. Pick one of the built-in universe presets, enter a seed, and adjust the eight creative axes from 0 to 100. Reusing a seed with compatible settings reproduces the same generated starting result. Randomize chooses a new seed.

Character Forge also offers constraints such as Non-human, No magic, No firearms, Urban only, and an optional Inspiration direction. Quest Forge has an optional Inspiration direction. Press Forge Character or Forge Quest to create a candidate in the current session.

## Shape the result

- **Lock / Unlock:** preserve a field while rerolling other fields. Locked fields are labeled and cannot be individually rerolled until unlocked.
- **Reroll:** regenerate one unlocked field. Explicit field dependencies may also update dependent unlocked fields.
- **Reroll all unlocked fields:** regenerate eligible fields while preserving locked and manually edited values.
- **Edit:** enter your own field text and save the edit.
- **Undo / Redo:** move through the current in-memory editing session. These controls do not undo or delete already saved history or canon. Text editing retains the text editor's normal undo behavior.

The seed and preset are shown with the current result. Generated and manually edited content remains local to the project.

## Save, favorite, history, and canon

Choose **Save candidate** to save a new result. Later edits to a saved candidate use **Save changes**. Use the library to favorite an entity or explicitly promote a candidate to Canon. Canon is separate from a working candidate; generation does not silently overwrite it.

History shows saved generation and edit snapshots. You can inspect prior snapshots and use one as a working copy without changing the saved record. Canon revisions are also inspectable. Saving a working copy over Canon requires reviewing the displayed old/new changes, confirming, and entering a revision reason; the previous Canon is retained as a revision.

## Export, import, backup, and restore

- Use the library's JSON or Markdown actions to export an entity to a new file. JSON preserves the structured entity. Markdown is a readable representation that includes its structured data; external Markdown viewers apply their own rendering rules.
- **Export Project** creates a `.forgeproject` portable archive at a path you choose. **Import Project** validates an archive and imports it into a selected folder. Import creates a new project and refuses to overwrite an existing destination.
- **Create Backup** writes a `.forgeproject` backup to the path you choose.
- **Restore Backup** replaces the currently open project's state after an explicit confirmation. Keep a separate copy of important project files before restoring.

Failed or invalid imports should be rejected without activating a partial project. If an operation reports a destination collision, choose a different file or folder; do not delete a file unless you have verified it is yours to remove.

## Settings, offline use, and recovery

Project settings include a Reduced Motion preference. The setting is stored in the per-user application config directory. Character Forge / Quest Forge does not require an account, AI provider, API key, or network connection for its core workflows.

If a project cannot be opened, verify its path and that it is a Character Forge SQLite project. Bring `.forgeproject` files in through Import Project. If a destination already exists, choose a different path. For corrupt or unsupported data, stop and preserve the original file before seeking help. There is no application log folder configured; error details appear in the app when an operation fails.

## Platform and release notes

The release target is Windows 11 x64. The staged portable package is unsigned; no NSIS or MSI installer was produced for this local release candidate. The portable executable requires a compatible Microsoft Edge WebView2 Runtime on the machine. Read [Release Notes](RELEASE_NOTES_v1.0.0.md) for the automated dependency-review limitation and exact release status.
