# Character Forge / Quest Forge v1.0.0 FOUNDATION

## What it is

Character Forge / Quest Forge is a local-first Windows desktop application for creating structured Characters and Quests, revising individual fields, and keeping selected work in a local project canon.

## Included

- Character Forge and Quest Forge with eleven built-in universe presets, deterministic seeds, and creative axes.
- Character constraints and optional inspiration direction.
- Field locks, single-field rerolls, reroll-unlocked, manual edits, and session Undo/Redo.
- Local projects, favorites, persistent generation history, candidate/canon workflow, and canon revisions.
- Structured JSON and readable Markdown entity exports.
- `.forgeproject` project export/import, backup, and recoverable restore.
- Reduced Motion preference.

No AI model, account, API key, cloud service, or network call is required by the core workflows after installation. The packaged offline workflow was verified by the Mission 08 operator smoke.

## Supported platform and installation

This release targets **Windows 11 x64** only. The portable executable depends on Microsoft Edge WebView2 Runtime being present. The local release artifacts are unsigned; Windows may show a publisher or SmartScreen warning because the binaries have no verified publisher reputation. No signing certificate was available or fabricated.

Use the instructions in [USER_GUIDE.md](USER_GUIDE.md). The staged release inventory and artifact hashes are in `artifacts/releases/v1.0.0/`.

## Data and compatibility

Project databases stay at the location selected by the user. Keep backups of important projects. The application release version is 1.0.0; SQLite schema 1, generator 1, recipe 1, Character content 1/2, Quest content 2/3, and `.forgeproject` format 1 remain independent compatibility versions.

## Security and known limitations

The ARMOR review and packaged offline/security smoke found no P0/P1/P2 security defect. Direct/runtime dependencies and enabled features were reviewed. The available ZIP advisory was investigated and does not apply to locked ZIP 8.6.0. A complete automated transitive dependency advisory scan was unavailable because the advisory endpoint could not be reached and Rust audit tools were not installed. The dependency graph is not claimed to be automatically verified vulnerability-free; see [DEPENDENCY_REVIEW_V1.md](security/DEPENDENCY_REVIEW_V1.md).

The application does not currently provide an About/version screen or persistent application log folder. Recent-project metadata is stored under the operating-system application config directory. No project license has been selected; public redistribution requires a separate licensing decision.

## Release status

These notes describe a locally staged portable ZIP only; no NSIS or MSI installer is included. The artifact is unsigned. The exact staged ZIP passed the fresh-install-like operator smoke and all automated gates; Mission 09 is PASS. It has not been published, uploaded to a store, or distributed publicly. See `docs/RELEASE_CHECKLIST.md` and `artifacts/releases/v1.0.0/RELEASE_MANIFEST.md`.
