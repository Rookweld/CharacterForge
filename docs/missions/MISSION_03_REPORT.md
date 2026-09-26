# Mission 03 — CALLING Remediation II report

## Status

**PARTIAL.** The structural cluster remediation and controlled Mythic/Chaos axis proofs pass, but Mission 03 exit gates remain open. Mission 04 — MEMORY has not started. No persistence, ForgeDNA protocol change, or Character Forge redesign was made. No commit was created.

The work began from clean committed checkpoint `8395f07647d823681f1fcd4c9fec628a7f324942` (`wip: checkpoint Mission 03 declarative Quest Forge`) on `master`. Quest v2 replay is frozen. The checkpoint does not promise v3 compatibility, so the unfinished v3 path was refined in place; no v4 was introduced.

## Remediation

- Added developer-only selection IDs and per-field concentration, high-pair, and connected-component audit evidence without adding debug fields to the Quest wire entity.
- Found the primary cause: v3 selected one Frame/thread and nearly every identity-bearing field drew from that thread's shallow two-option pools. All 15,990 pre-fix >=10/14 pairs were same-thread pairs; field streams were distinct, but source pools were coupled by the bundle.
- Changed Chaos Blend v3 to retain a spine for the core story chain while deterministic Chaos-weighted secondary-role source mixing draws only from explicit `mix-compatible` threads. The compatibility marker is excluded from visible quest types.
- Re-composed Briefing from the chosen Patron and Premise. Briefing now declares dependencies on Frame, Patron, and Premise; targeted reroll regression covers both inputs.
- Added controlled Mythic selection and Chaos source-mixing regression tests, plus the supported inspiration constraint matrix across all eleven presets.
- Preserved v2 behavior and ForgeDNA serialization/stream semantics.

## Similarity evidence

| Chaos Blend, 1,024 seeds / 523,776 pairs | Initial v2 | Previous v3 | Remediation II v3 |
|---|---:|---:|---:|
| Exact expanded clones | 0 | 4 | 0 |
| >=12/14 | not recorded | not recorded | 23 |
| >=10/14 | 2,083 | 15,990 | 1,162 |
| >=8/14 | 30,149 | 88,856 | 18,071 |
| Mean matches | 4.668 | 3.528 | 3.409 |
| Median matches | not recorded | not recorded | 3 |
| Maximum matches | 13 | 14 | 13 |

The final >=10 set contains only same-primary-thread pairs: 856 mythic-spine and 306 tide-spine. Its largest connected component has 584 seed nodes (first members: 0, 1, 3, 5, 7, 8, 9, 11, 12, 14, 15, 18, 22, 25, 27); the next has 176 (13, 19, 24, 47, 57, 69, 72, 77, 80, 81, 91, 95, 99, 102, 108). Components are linked by >=10 pair edges; members are not necessarily all mutually high-similar. The largest individual seed neighborhoods have 8 partners (seeds 508 and 703). Full membership and internal IDs are in the quality audit output.

## Axis, content, and constraint results

- Mythic, same 256 Chaos Blend seeds and all other axes fixed: `mythic-salvage` selected 99 times at level 0 and 194 times at level 100.
- Chaos, same 256 seeds: primary-thread counts were 150/106 at low and 145/111 at high, while secondary location source was spine in 256/256 low cases and alternate compatible source in 256/256 high cases.
- Chaos identity-bearing fields remain four rendered values per field; Briefing has eight combinations. Top-one concentration ranges from 25.9% (location) to 33.5% (premise); top-three ranges from 75.8% (moral pressure) to 83.1% (threat). Ordinary preset pools remain at four values per field. See the quality report for the full per-preset concentration matrix.
- `inspiration` is the only supported Quest constraint. Valid Unicode values were generated and surfaced for every preset; whitespace-only and >200-character values fail with `InvalidConstraint`. Unsupported/Character-only keys remain structured errors. Broader semantic Quest constraints are not implemented.
- Representative scripted outputs preserve a connected story chain in ordinary presets. Chaos cross-source combinations still need human review for thematic seams. Human acceptance and broad wording review remain open.

## Verification

- Targeted Quest tests: 13 passed, 0 failed (including reroll dependencies, source mixing, Mythic weighting, and constraint matrix).
- Required quality audit: 256 seeds for each ordinary preset, 1,024 Chaos Blend seeds, 523,776 pair comparisons, internal selection IDs, concentration, connected high-similarity components, controlled axis samples, and representative outputs. Passed.
- `cargo fmt --all -- --check`: PASS.
- `cargo clippy --workspace --all-targets -- -D warnings`: PASS.
- `cargo test --workspace`: 62 passed (52 app-core, 10 contracts), 0 failed.
- `cargo metadata --no-deps --format-version 1`: PASS.
- `./scripts/verify.ps1`: PASS; frontend format/lint/typecheck, 1 frontend test, frontend production build, Rust workspace checks/tests, and optimized Tauri no-bundle build passed.
- `git diff --check`: PASS.

## Exit decision

Mission 03 remains **PARTIAL**. Structural high-overlap counts fell 92.7% from previous v3 and below the original v2 count, and both Mythic and Chaos axes now have controlled measurable effects. The shallow four-choice field pools, remaining same-spine high-similarity pairs, Chaos seam review, broader constraint policy, and human acceptance keep the mission open. Stop here; do not begin Mission 04.
