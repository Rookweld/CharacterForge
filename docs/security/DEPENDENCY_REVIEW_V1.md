# Dependency Security Review v1

**Lockfile baseline:** `Cargo.lock`, `pnpm-lock.yaml`; reviewed 2026-09-28 at Mission 08 baseline `d42eb3c`.

## Direct production dependencies

### Rust

| Dependency | Locked version | Production purpose / security-relevant features |
|---|---:|---|
| `tauri` | 2.11.6 | Native desktop shell and command bridge; no optional shell/dialog/opener/updater feature enabled in the app manifest. |
| `serde` | 1.0.229 | Typed serialization/deserialization. |
| `serde_json` | 1.0.151 | Versioned settings, contracts, and project payloads. |
| `rusqlite` | 0.40.2 | SQLite persistence; workspace disables defaults and enables `bundled`. No extension-loading code found. |
| `zip` | 8.6.0 | Closed `.forgeproject` reader/writer; defaults disabled, only `deflate-flate2-zlib-rs` enabled. No extract-all use. |
| `blake3` | 1.8.5 | Deterministic generation streams and archive member integrity, not publisher authentication. |

Internal production crates: `forge-app-core`, `forge-contracts`, `forge-storage`, and `forge-export` are local workspace crates. The desktop uses `serde` and `serde_json` directly; export uses `blake3`, storage, serde, and the constrained ZIP feature set; storage uses SQLite and typed serialization; core/contracts use deterministic/data-contract dependencies. `cargo tree -p character-forge-desktop --edges normal --depth 1` confirmed the shell's direct normal dependency graph.

### Frontend

| Dependency | Locked version | Classification / purpose |
|---|---:|---|
| `@tauri-apps/api` | 2.11.1 | Runtime command bridge. |
| `react` | 19.3.0 | Runtime UI rendering; user prose remains React text. |
| `react-dom` | 19.3.0 | Runtime UI rendering. |

Build/test-only direct packages are documented by `apps/desktop/package.json` and root `package.json`: Tauri CLI 2.11.5, Vite 8.3.1, React Vite plugin 6.1.1, TypeScript 6.0.3, Vitest 5.0.2, Biome 2.5.14, Node/React types, Testing Library, and happy-dom. They are not production runtime dependencies.

## Advisory and audit-tool result

- `cargo-audit` and `cargo-deny` are not installed in this environment.
- `pnpm audit --offline` could not reach the npm advisory endpoint (`EACCES`); it did not produce a complete advisory result.
- Current RustSec advisory index was checked on 2026-09-28. RustSec lists RUSTSEC-2025-0168 for `zip` versions `>=1.3.0,<2.3.0`, caused by extraction path canonicalization. The locked `zip` 8.6.0 is outside that affected range, and this application does not call archive extraction; it reads only fixed members into isolated staging. [RustSec advisory index](https://rustsec.org/advisories/) and [GitHub advisory](https://github.com/advisories/GHSA-94vh-gphv-8pm8).
- No complete current advisory disposition is claimed for all transitive Rust/npm packages. That is an unresolved verification limitation, not evidence of an advisory.
- The human operator accepts this unavailable full automated scan as a **non-blocking documented residual maintenance/release risk** for Mission 08 closeout. Do not describe the transitive dependency graph as automatically verified clean. Re-run the full scan when registry access and approved tooling are available, before release packaging.
- During Mission 08, no dependencies were added, removed, or upgraded. At that baseline, no production network client, telemetry client, updater, opener, shell, or dialog plugin is a direct app dependency.

## Reproducibility and limitations

Both lockfiles are committed. Versions above are lock-resolved direct production dependencies, not a full transitive inventory. A full `cargo audit` / online `pnpm audit` remains recommended maintenance before release packaging when advisory database access is available. No new global security tooling was installed during closeout.

## B10 local-provider addendum — 2026-10-01

The operator approved one new direct desktop production HTTP dependency: pinned `ureq` 3.3.0 with default features disabled. Intentional lock additions are `ureq`, `ureq-proto` 0.6.0 and `utf8-zero` 0.8.1; existing locked base64/log/percent-encoding/http/httparse support is reused. `cargo tree -p ureq -e features --offline` confirms the narrow client/std graph; no TLS, compression, cookies, SOCKS or platform proxy feature is enabled. Code disables all proxies and redirects, pins localhost to numeric loopback and bounds deadlines/body sizes. The core/provider-neutral contracts have no HTTP dependency.

Installed source/configuration and the official [ureq documentation](https://docs.rs/ureq/3.3.0/ureq/) were reviewed. Available RustSec index/search did not identify a confirmed applicable advisory for this addition; this is not a full automated transitive advisory scan. The accepted Mission 08 verification limitation remains. No security tooling was installed. See [B10 report](../missions/THE_FORGE_B10_COMFYUI_LOCAL_PROVIDER_REPORT.md).
