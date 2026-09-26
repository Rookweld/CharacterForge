# ADR 0001: v1.0 Foundation Stack

- **Status:** Accepted for the v1.0 FOUNDATION target; implementation/bootstrap verification remains in progress.
- **Date:** 2026-09-26

## Context

The canonical product documents select a Windows-first Tauri 2 desktop application, React + TypeScript + Vite frontend, Rust stable application/core, and SQLite local project store. The application must remain useful offline and keep domain and deterministic generation independent of the visual interface. Mission 00 starts from a repository containing only mission and reference documents.

## Decision

Use the canonical stack and a Cargo workspace with clear desktop, application/domain, persistence, export, and shared-contract boundaries as implementation requires. Use pnpm for JavaScript package management in this workspace because it is available and npm is currently misconfigured. Do not add provider runtimes, marketplace/pack execution, or empty speculative packages. Keep SQLite access behind Rust persistence boundaries, and have the frontend call explicit application commands rather than owning storage.

## Consequences

The stack supports a small offline desktop application while preserving clean seams. The pnpm choice must be encoded and its lockfile committed once the frontend dependency manifest is approved and created. Tauri compilation requires the Windows C++ build toolchain and WebView2 runtime/prerequisites to be present and verified. The current environment does not expose `cl.exe`, Windows CMake, or `tauri-driver`; native build readiness is not established.

## Scope guard

This decision establishes architecture only. It does not implement generation, database schema/migrations, provider integrations, project backup/import, or later-mission features.
