# Release Checklist

This checklist is a durable release-gate placeholder. No release is being claimed by Mission 00.

## Build and runtime

- [ ] Clean checkout can install declared dependencies reproducibly.
- [ ] Frontend typecheck and production build pass.
- [ ] Rust workspace tests and lint pass.
- [ ] Tauri desktop shell compiles and is launched for a native smoke check.
- [ ] Supported Windows installation and project open/save behavior are verified.

## Data safety and product gates

- [ ] Versioned SQLite migration and recovery behavior are tested.
- [ ] Project backup/import roundtrip preserves supported data.
- [ ] Deterministic Character and Quest seeded runs meet the v1.0 release gate.
- [ ] Locked-field invariants pass.
- [ ] No known P1/P2 data-loss defects.

## Packaging and evidence

- [ ] Release artifact is reproducible and inspected for secrets/unintended files.
- [ ] Checks, launch evidence, artifact identity, and known limitations are recorded.
- [ ] Documentation matches the verified product.
