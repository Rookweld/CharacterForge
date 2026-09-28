# Character Forge / Quest Forge

**Version 1.0.0 FOUNDATION** is a local-first creative desktop application for Windows 11 x64. Forge structured Characters and Quests, shape individual fields with locks, rerolls, and edits, then save candidates into project history and promote chosen work to canon.

The v1.0 application works offline after installation and needs no account, AI provider, API key, or telemetry service. Projects are local SQLite files; `.forgeproject` archives support export, import, backup, and restore. See the [User Guide](docs/USER_GUIDE.md), [Release Notes](docs/RELEASE_NOTES_v1.0.0.md), and [Security Notes](docs/SECURITY.md).

## Windows release

The locally prepared release supports Windows 11 x64. See [the release checklist](docs/RELEASE_CHECKLIST.md) for the exact artifact paths, verification state, signing status, and remaining native acceptance gate. Unsigned artifacts may show Windows SmartScreen or publisher warnings because they have no verified publisher reputation.

## Development prerequisites

- Rust stable with the `x86_64-pc-windows-msvc` toolchain and Visual Studio C++ Build Tools
- Node.js and pnpm 11.25.0
- Tauri 2 Windows prerequisites, including Microsoft Edge WebView2 Runtime

## Development commands

```powershell
pnpm install --frozen-lockfile
pnpm run tauri:dev
pnpm run format:check
pnpm run lint
pnpm run typecheck
pnpm test
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
pnpm run check:version
pnpm run release:verify
```

`pnpm run release:verify` runs the repository verification script, which also builds the optimized Tauri application. The release build can use a dedicated Cargo target directory with `$env:CARGO_TARGET_DIR` when the existing executable is open in Windows.

## Repository structure

- `apps/desktop` — React, TypeScript, Vite, and Tauri shell
- `crates/app-core` — deterministic Character and Quest generation
- `crates/contracts` — versioned domain and ForgeDNA contracts
- `crates/storage` — SQLite persistence and history
- `crates/export` — `.forgeproject`, backup/restore, and entity exports
- `content` — built-in versioned generation content
- `docs` — user, architecture, security, and release documentation

## Product boundaries

FOUNDATION includes the eleven built-in presets, deterministic seeded generation, creative axes, Character constraints, field locks/rerolls/manual edits, session undo/redo, favorites, persistent history, canon and revisions, JSON/Markdown entity exports, local project storage, `.forgeproject` import/export, backups, restore, and reduced-motion settings. It does not include AI/provider integration, cloud sync, marketplace/plugin execution, or later roadmap features.

The SQLite schema remains version 1, generator version 1, recipe version 1, Character content versions 1 and 2, Quest content versions 2 and 3, and `.forgeproject` format version 1. These compatibility versions are independent of the application release version.

## License

No project license has been selected. Cargo metadata marks the workspace `UNLICENSED`; review licensing before public redistribution. No license is implied by this repository.
