# ADR 0001: v1.0 Foundation Stack

- **Status:** Accepted for the v1.0 FOUNDATION target; Mission 00 production-baseline verification passed.
- **Date:** 2026-09-26

## Context

The canonical product documents select a Windows-first Tauri 2 desktop application, React + TypeScript + Vite frontend, Rust stable application/core, and SQLite local project store. The application must remain useful offline and keep domain and deterministic generation independent of the visual interface. Mission 00 began from a repository containing only mission and reference documents.

## Decision

Use the canonical stack and a Cargo workspace with clear desktop, application/domain, persistence, export, and shared-contract boundaries as implementation requires. Use pnpm for JavaScript package management in this workspace because it is available and npm is currently misconfigured. Do not add provider runtimes, marketplace/pack execution, or empty speculative packages. Keep SQLite access behind Rust persistence boundaries, and have the frontend call explicit application commands rather than owning storage.

## Consequences

The stack supports a small offline desktop application while preserving clean seams. The pnpm choice is encoded in the frontend manifest and lockfile. Mission 00 verified the optimized Tauri build and the human operator independently launched the built application from the normal interactive Windows desktop, confirming a visible usable Tauri/WebView window and rendered frontend content. This satisfies the v1.0 FOUNDATION native-build and GUI-smoke baseline.

## Scope guard

This decision establishes architecture only. It does not implement generation, database schema/migrations, provider integrations, project backup/import, or later-mission features.
