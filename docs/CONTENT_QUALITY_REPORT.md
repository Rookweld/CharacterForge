# SOUL Content Quality Audit

**Scope:** pre-Mission-03 audit of Mission 02 Character content. No Quest Forge, UI, persistence, or ForgeDNA stream/reroll protocol changes.

## Method

- Deterministic samples use sequential root seeds starting at 0. Each canonical preset uses 256 seeds; Chaos Blend uses 1,024 in its dedicated run.
- Multi-field similarity counts exact equality across 17 identity fields. Personality, skills, ability, and the three story hooks are each compared as complete ordered bundles. Pair counts cover every unordered pair and are descriptive, not a quality threshold.
- For each field, the distribution table reports distinct observed values and the mode count among 256 candidates. For list-valued fields, these are complete bundles rather than list-item frequencies.
- Names have 36 combinations from 6 given names and 6 family names.

## Per-preset field distributions

Each cell is `distinct values / modal count` for 256 seeds.

| Preset | Archetype | Role | Title | Appearance | Faction | Gear | Desire | Fear | Flaw | Secret | Relationship | Backstory | Personality bundle | Skills bundle | Ability | Story-hooks bundle | Name |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| dark-fantasy | 4/122 | 6/53 | 6/53 | 6/52 | 6/53 | 6/73 | 6/56 | 6/49 | 6/45 | 6/51 | 6/50 | 6/49 | 42/13 | 30/15 | 6/70 | 224/3 | 36/14 |
| pirate | 4/102 | 6/55 | 6/48 | 6/50 | 6/49 | 6/78 | 6/52 | 6/54 | 6/56 | 6/55 | 6/55 | 6/47 | 42/12 | 30/14 | 6/49 | 231/3 | 36/11 |
| steampunk | 4/95 | 6/48 | 6/48 | 6/49 | 6/55 | 6/67 | 6/45 | 6/46 | 6/46 | 6/54 | 6/54 | 6/49 | 42/11 | 30/14 | 6/63 | 225/3 | 36/13 |
| cyberpunk | 4/114 | 6/50 | 6/50 | 6/49 | 6/49 | 6/88 | 6/49 | 6/53 | 6/52 | 6/57 | 6/51 | 6/50 | 42/13 | 30/20 | 6/98 | 229/3 | 36/12 |
| nordic-myth | 4/109 | 6/49 | 6/55 | 6/55 | 6/49 | 6/56 | 6/51 | 6/50 | 6/53 | 6/55 | 6/54 | 6/58 | 42/12 | 30/19 | 6/83 | 225/3 | 36/12 |
| post-apocalyptic | 4/95 | 6/51 | 6/46 | 6/55 | 6/56 | 6/64 | 6/52 | 6/52 | 6/48 | 6/47 | 6/50 | 6/48 | 41/13 | 30/13 | 6/76 | 227/3 | 36/13 |
| occult | 4/100 | 6/51 | 6/58 | 6/45 | 6/56 | 6/57 | 6/54 | 6/50 | 6/53 | 6/58 | 6/46 | 6/47 | 42/13 | 30/15 | 6/79 | 224/3 | 36/13 |
| monster-hunter | 4/122 | 6/48 | 6/47 | 6/57 | 6/55 | 6/75 | 6/49 | 6/48 | 6/51 | 6/50 | 6/48 | 6/50 | 42/12 | 30/14 | 6/49 | 230/2 | 36/12 |
| rogue-assassin | 4/103 | 6/54 | 6/50 | 6/51 | 6/50 | 6/47 | 6/48 | 6/45 | 6/48 | 6/52 | 6/48 | 6/51 | 42/19 | 30/14 | 6/52 | 218/3 | 36/15 |
| contemporary-weird | 4/105 | 6/52 | 6/49 | 6/53 | 6/56 | 6/51 | 6/54 | 6/49 | 6/52 | 6/49 | 6/47 | 6/47 | 42/13 | 30/15 | 6/50 | 221/4 | 36/12 |
| chaos-blend | 4/93 | 11/34 | 11/42 | 11/30 | 11/33 | 11/34 | 11/27 | 11/35 | 11/32 | 11/30 | 11/29 | 11/30 | 110/6 | 99/6 | 11/45 | 247/2 | 36/11 |

*The table is generated from the final deterministic audit output. The full per-field top values and complete pair metrics are summarized below; the audit example emits the machine-readable field-by-field JSON when run.*

## Multi-field similarity

| Preset | Seeds | Candidate pairs | Mean equal fields / 17 | Maximum | Pairs ≥8 | Pairs ≥10 | Exact clones |
|---|---:|---:|---:|---:|---:|---:|---:|
| dark-fantasy | 256 | 32,640 | 2.442 | 9 | 42 | 0 | 0 |
| pirate | 256 | 32,640 | 2.401 | 10 | 28 | 1 | 0 |
| steampunk | 256 | 32,640 | 2.384 | 9 | 27 | 0 | 0 |
| cyberpunk | 256 | 32,640 | 2.487 | 9 | 38 | 0 | 0 |
| nordic-myth | 256 | 32,640 | 2.431 | 11 | 35 | 1 | 0 |
| post-apocalyptic | 256 | 32,640 | 2.399 | 9 | 27 | 0 | 0 |
| occult | 256 | 32,640 | 2.405 | 10 | 29 | 1 | 0 |
| monster-hunter | 256 | 32,640 | 2.430 | 10 | 25 | 2 | 0 |
| rogue-assassin | 256 | 32,640 | 2.365 | 9 | 28 | 0 | 0 |
| contemporary-weird | 256 | 32,640 | 2.368 | 9 | 21 | 0 | 0 |
| chaos-blend | 256 | 32,640 | 1.424 | 7 | 0 | 0 | 0 |
| Chaos Blend extended | 1,024 | 523,776 | 1.425 | 8 | 7 | 0 | 0 |

## Archetype coupling

The generator selects semantic values from preset-level pools using separate deterministic field streams; it does not choose an archetype-specific fixed bundle. The conditional counts from archetype clusters confirm this. In the 1,024-character Chaos Blend run, each of the four archetypes had all 11 observed values for ability, backstory, fear, flaw, relationship, and secret. Personality bundle variants ranged from 84 to 125 per archetype; skill bundles from 85 to 107; story-hook bundles from 142 to 385. This preserves the shared thematic family without turning archetype into a template selector.

## Chaos Blend

Across 1,024 seeds, Chaos Blend observed 11 values for each scalar semantic field, 4 archetypes, and 36 names. It produced 132 full personality bundles, 110 skill bundles, and 890 three-hook bundles. Mean exact field matches were 1.425/17; maximum was 8, with 7 pairs at 8 or more and none at 10 or more or fully identical. The most frequent scalar was an ability at 168/1,024; the next listed ability occurred 102 times. No semantic family had duplicate source choices after correction.

## Human sample review

The 15 supplied sample JSON files were read without modification. The reported pairs were confirmed: Pirate shared 4/16 exact fields and 3/8 list items; Cyberpunk shared 5/16 exact fields and 7/8 list items; Occult shared 3/16 exact fields and 6/8 list items; Chaos Blend shared 6/16 exact fields and 5/8 list items. The Cyberpunk list overlap is high and corroborates the observed issue. The v1 corpus had only three options in most scalar semantic families, which constrained identity-level variation.

## Unicode investigation

- **Source:** the canonical source contains correct UTF-8 text `séance` and an em dash. No source mojibake was found.
- **Rust and JSON:** an automated canonical JSON roundtrip test covers `séance`, `refusing—and`, Japanese text, and emoji; it passes and preserves the serialized characters.
- **Capture path:** PowerShell 7.6.6 with UTF-8 output encoding successfully captured occult seed 1414 and rogue-assassin seed 1010 using both direct redirection and `Tee-Object`. All four outputs parsed as JSON and retained the Unicode text. This does not identify the exact historical shell/configuration used for the supplied corrupted samples. Those persisted samples are mojibaked, so corruption occurred after canonical source text and within the historical output/capture path.
- Example PowerShell 7 capture: `cargo run -q -p forge-app-core --example character_forge -- --seed 1414 --preset occult > $direct`; pipeline capture: `cargo run -q -p forge-app-core --example character_forge -- --seed 1414 --preset occult | Tee-Object -FilePath $tee | Out-Null`. Parse with `Get-Content -Raw $path | ConvertFrom-Json` and keep PowerShell output encoding UTF-8.

## Controlled creative-axis comparisons

Each comparison used 256 seeds, the same preset/root seed on each side, and changed only the named axis. Repeated generation with identical inputs is deterministic.

| Preset / axis | Low→high | Archetype counts at low | Archetype counts at high | Changed candidate fields |
|---|---:|---|---|---:|
| Cyberpunk / Technology | 0→100 | Civic Algorithm Auditor 85; Memory-Bank Salvager 36; Roofline Clinic Runner 97; The Unregistered Fork 38 | Civic Algorithm Auditor 38; Memory-Bank Salvager 163; Roofline Clinic Runner 40; The Unregistered Fork 15 | 17/17 had at least one changed output |
| Chaos Blend / Chaos | 0→100 | Neon-Mask Lighthouse Keeper 29; Saga-Engine Salvager 90; Tide-Court Signal Witch 65; Wasteland Oath Cartographer 72 | Neon-Mask Lighthouse Keeper 47; Saga-Engine Salvager 103; Tide-Court Signal Witch 52; Wasteland Oath Cartographer 54 | 17/17 had at least one changed output |

Memory-Bank Salvager changed from 36/256 at Technology 0 to 163/256 at Technology 100. The Chaos comparison also shifted archetype frequencies. Individual outputs are not expected to change every time; the controlled distributions demonstrate deterministic axis influence.

## Changes and compatibility

- Added an expanded original content corpus as content version 2 for all eleven presets; Chaos Blend received eight added choices per scalar family. The v1 corpus remains unchanged and generatable for saved v1 DNA.
- New DNA defaults to content version 2. ForgeDNA canonical encoding, hash/stream derivation context, dependency topology, and lock/reroll behavior remain unchanged. Version validation accepts v1 and v2 for deterministic replay.
- Content loading now rejects duplicate text within a semantic family. Three duplicate Chaos Blend additions were replaced before final measurement.
- Added a deterministic audit example, separate v1/v2 golden tests, pool expansion/uniqueness checks, controlled-axis comparison, and UTF-8 JSON roundtrip test.

## Limits and conclusion

These measurements cover this corpus and bounded seed ranges; they do not prove subjective creativity or future player satisfaction. Ordered list bundles are compared exactly, so list-item overlap is a separate measure. No repeated identity-level cloning defect remains in the audited runs: every preset had zero full clones across its 256-seed pair comparison, and Chaos Blend had zero across 1,024 seeds. Mission 02 remains PASS after the verification below completed successfully.

## Verification

All required checks completed successfully:

- `cargo test -p forge-app-core --lib` — **38 passed, 0 failed**.
- `cargo run -q -p forge-app-core --example character_quality_audit -- $auditOut` — PASS; wrote `character-quality-audit-final.json` in the current user's temporary directory. Covers 256 seeds for each preset, 1,024 Chaos Blend seeds, archetype clusters, candidate similarity, and 256-seed controlled comparisons.
- `cargo test -p forge-app-core --lib canonical_json_roundtrip_preserves_utf8_unicode` — **1 passed**.
- `cargo test -p forge-app-core --lib controlled_creative_axis_comparison_keeps_seed_and_other_axes_fixed` — **1 passed**.
- PowerShell 7 direct redirection and `Tee-Object` capture for occult seed 1414 — both files parsed as JSON, contained `séance`, and contained neither tested mojibake string.
- `cargo fmt --all -- --check` — PASS.
- `cargo clippy --workspace --all-targets -- -D warnings` — PASS.
- `cargo test --workspace` — **48 unit tests passed** (38 app-core, 10 contracts), 0 failed; doc-test targets passed with 0 tests.
- `& .\scripts\verify.ps1` — PASS; frontend formatting, lint, typecheck, React test (1/1), production build, Rust formatting, clippy, workspace tests, and optimized Tauri no-bundle build all succeeded. Output executable: `target/release/character-forge-desktop.exe`.
- `cargo metadata --no-deps --format-version 1` — PASS.
- `git diff --check` — PASS (exit code 0).

## Repository state

- Branch: `master`.
- HEAD: `0593a01de54be9857d5689c57abcd338db3904cc`.
- Worktree: dirty from this audit's code/content/documentation changes and the pre-existing supplied user data `character-samples.zip` and `character-samples/`; those sample inputs were not modified.

**Mission 02 status after this audit: PASS.**
