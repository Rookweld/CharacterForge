# Quest Forge Content Quality Audit — Mission 03 Remediation

## Status

**PARTIAL.** The v3 declarative corpus and generation path are implemented, but the similarity gate regressed at the important high-overlap thresholds, most field pools remain shallow, the axis experiment is inconclusive for two axes, and human review/coherence is not complete. Mission 04 remains blocked by Mission 03.

## Remediation II — pre-fix selection-cluster investigation

This section records the Phase 1/2 investigation before changing v3 selection or corpus content. The developer audit now emits stable content fingerprints and local option ordinals for each selected field (`thread/<id>`, `<field>/item-<content-hash>/option-<ordinal>`) plus the briefing composition ID. The IDs are generated only in the audit path and are not added to the Quest contract. The exact selected IDs and seed-indexed records are emitted by `quest_quality_audit`.

### Root cause

All **15,990 / 15,990** pairs at >=10/14 shared the same one selected thread. Cross-thread pairs produced **zero** >=10/14 matches. Same-thread pairs averaged **6.634 /14** equal public fields. The two largest upstream clusters were:

| Selected thread | Seeds in cluster | >=10/14 pairs within it |
|---|---:|---:|
| `tide-memory-archive` | 642 | 11,679 |
| `mythic-salvage` | 382 | 4,311 |

The highest-neighbor seeds in the exact sweep were 129 (55 high-similarity partners), 362 (53), 183 (52), 498 (51), and 93, 94, 97, 400, and 465 (50 each). The complete seed membership and per-seed internal IDs are in the audit's `selection_records` and `thread_clusters` output. Within the high-pair set, each of location, stakes, objective, threat, complication, clue, reward, consequence, moral pressure, title, twist, and quest giver matched in roughly 12.2k pairs; field-local option choices are not aliases of one stream. `quest_type` is a tag sampled from a larger pool and matched in 5,090 high pairs.

**Primary cause:** the v3 generator first picks a single fixed story thread, and every identity-bearing field then selects from a two-choice pool inside that thread. For pairs with the same thread, each binary field has about a 1/2 match chance; for pairs with different threads, many fields cannot match at all. The two-thread mix lowers the global average to 3.528, but conditioning on a shared thread puts the pair in the high-tail regime. This explains why the mean improved while the >=10 tail rose from 2,083 to 15,990. Briefing also uses `thread.givers[0]` rather than the selected giver, coupling it to the thread and premise. There is no accidental identical field stream, no compatibility filter reducing a larger set, and no cross-thread high-similarity cluster. The fixed thread bundle plus binary family depth is the structural cause.

### V3 baseline field concentration

For each cell below, values are `unique / most-common % / top-3 combined %`, across 256 seeds (1,024 for Chaos Blend). Percentages are measured on final rendered values; the top-three measure exposes the heavy concentration even where a field has four distinct strings.

| Preset | Title | Location | Giver | Objective | Stakes | Threat | Complication | Clue | Reward | Consequence | Moral pressure | Briefing |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Dark fantasy | 4/32/82 | 4/32/80 | 4/30/80 | 4/31/84 | 4/34/81 | 4/37/80 | 4/30/80 | 4/30/81 | 4/32/82 | 4/31/81 | 4/30/80 | 4/30/80 |
| Pirate | 4/27/78 | 4/29/78 | 4/28/81 | 4/28/77 | 4/28/80 | 4/29/79 | 4/29/77 | 4/30/77 | 4/29/78 | 4/29/77 | 4/28/79 | 4/27/80 |
| Steampunk | 4/41/91 | 4/42/90 | 4/46/93 | 4/43/91 | 4/43/91 | 4/43/91 | 4/45/92 | 4/41/90 | 4/42/91 | 4/45/91 | 4/41/91 | 4/41/92 |
| Cyberpunk | 4/36/86 | 4/40/88 | 4/39/86 | 4/36/87 | 4/36/88 | 4/39/86 | 4/38/88 | 4/38/88 | 4/36/89 | 4/37/87 | 4/39/86 | 4/37/87 |
| Nordic myth | 4/32/80 | 4/31/77 | 4/28/79 | 4/28/78 | 4/29/81 | 4/29/79 | 4/27/79 | 4/29/79 | 4/31/80 | 4/35/80 | 4/29/78 | 4/28/79 |
| Post-apocalyptic | 4/34/83 | 4/32/84 | 4/32/82 | 4/32/83 | 4/34/83 | 4/33/82 | 4/32/82 | 4/32/84 | 4/32/83 | 4/32/84 | 4/34/82 | 4/36/84 |
| Occult | 4/31/80 | 4/32/79 | 4/29/80 | 4/31/80 | 4/30/80 | 4/31/81 | 4/29/80 | 4/29/83 | 4/29/80 | 4/32/79 | 4/29/81 | 4/33/79 |
| Monster hunter | 4/34/86 | 4/35/86 | 4/38/87 | 4/36/87 | 4/37/86 | 4/37/86 | 4/36/85 | 4/37/85 | 4/43/85 | 4/35/84 | 4/38/85 | 4/39/85 |
| Rogue assassin | 4/28/77 | 4/26/77 | 4/27/78 | 4/26/76 | 4/27/78 | 4/28/79 | 4/27/78 | 4/26/75 | 4/28/79 | 4/27/78 | 4/27/78 | 4/27/77 |
| Contemporary weird | 4/44/93 | 4/43/95 | 4/43/93 | 4/42/95 | 4/43/93 | 4/48/93 | 4/44/93 | 4/44/95 | 4/50/92 | 4/43/92 | 4/48/93 | 4/42/93 |
| Chaos Blend | 4/32/82 | 4/34/82 | 4/32/82 | 4/33/84 | 4/31/82 | 4/34/82 | 4/32/83 | 4/33/82 | 4/32/82 | 4/32/82 | 4/32/82 | 4/31/82 |

Every listed identity-bearing field has only four values in each preset sample. Many top-three concentrations exceed 80%; this confirms that the problem is the per-thread binary pool design, not merely the number of full-candidate combinations.

### Content-version decision

The committed checkpoint labels Mission 03 **PARTIAL** and contains no v3 golden compatibility promise or v3 freeze decision. The explicit compatibility decision freezes Quest v2 replay; v3 was the unfinished remediation path. Therefore this run refines v3 in place. Quest v2 remains unchanged; no v4 is introduced.

## Original PARTIAL findings and causes

The initial v2 audit used 1,024 Chaos Blend seeds and measured 2,083/523,776 pairs sharing at least 10 of 14 public Quest fields, 30,149 pairs sharing at least 8, mean 4.668 shared fields, maximum 13, and no exact clones. Each of six measured fields had three distinct values. The Rust `theme()` switch selected one fixed preset tuple; field prose drew from three shared variants. This is shallow reusable template content, not a failure of ForgeDNA stream separation.

The v3 corpus fixes the source-code content table and adds semantically tagged, compatible story threads, but it currently has only two threads per preset and two options in most fields per thread. That yields four observed values for most fields (two for each of two threads). Independently streamed choices improve average diversity, while their short pools still cause repeat bundles. Chaos Blend uses only two threads, so it is a narrow two-family mixture rather than broad source mixing. The audit also found four exact expanded clones and one exact clone in the Monster Hunter 256-seed sweep.

## Declarative corpus and compatibility

`content/quests/v3/core.json` stores eleven presets. Each preset has two authored threads with semantic tags and separate title, location, giver, premise, objective, stakes, threat, complication, clue, twist, reward, consequence, moral-pressure, and follow-up choices. The Rust loader validates the schema version, preset count, unique thread ids, nonempty tags, and nonempty choice pools. Selection is deterministic and uses the existing per-field streams. A selected thread supplies the compatibility boundary; field choices within it remain independent. The internal Frame field records the selected thread and is the explicit dependency for generated fields.

Version 2 remains available for replay. New Quest recipes use content version 3. ForgeDNA serialization, field-stream context, nonce format, lock operations, and dependency semantics were not changed. The shared contract's supported content-version range now includes 3.

## Chaos Blend, same 14-field comparison

Comparison uses the original public field basis and exact-equality rule: title, quest type, premise, patron, objective, stakes, opposition, complications, twist, moral pressure, locations, rewards, failure consequences, and follow-up hooks. The sweep compares all 523,776 pairs from 1,024 sequential seeds.

| Metric | v2 baseline | v3 remediation |
|---|---:|---:|
| Exact expanded clones | 0 | 4 |
| Pairs with >=10/14 equal fields | 2,083 | 15,990 |
| Pairs with >=8/14 equal fields | 30,149 | 88,856 |
| Mean shared public fields | 4.668 | 3.528 |
| Maximum shared fields | 13 | 14 |
| Unique expanded candidates | 1,024 | 1,020 |

The mean improved, but both high-overlap counts and exact-clone count worsened. The v3 audit therefore does **not** demonstrate reduced routine renamed-template behavior. Similarity is still concentrated within the two selected threads; at neutral axes the 1,024 seeds split 382/642 between `mythic-salvage` and `tide-memory-archive`.

## Per-preset diversity

Each of the ten ordinary presets used 256 seeds; Chaos Blend used 1,024. Pair totals are 32,640 and 523,776 respectively. Per-field counts are observed exact distinct values; most public narrative fields have four values per preset. `quest_type` is derived from thread tags and is not yet a well-formed authored quest taxonomy.

| Preset | Unique expanded | Exact clones | Mean shared /14 | Max | Pairs >=8 | Pairs >=10 | Distinct per usual field |
|---|---:|---:|---:|---:|---:|---:|---:|
| Dark fantasy | 256 | 0 | 3.52 | 14 | 6,093 | 1,274 | 4 |
| Pirate | 256 | 0 | 3.46 | 13 | 5,734 | 1,216 | 4 |
| Steampunk | 256 | 0 | 4.65 | 13 | 7,920 | 1,685 | 4 |
| Cyberpunk | 256 | 0 | 4.05 | 14 | 6,902 | 1,455 | 4 |
| Nordic myth | 256 | 0 | 3.44 | 13 | 5,925 | 1,201 | 4 |
| Post-apocalyptic | 256 | 0 | 3.60 | 13 | 6,110 | 1,277 | 4 |
| Occult | 256 | 0 | 3.46 | 13 | 5,970 | 1,225 | 4 |
| Monster hunter | 255 | 1 | 3.87 | 14 | 6,675 | 1,391 | 4 |
| Rogue assassin | 256 | 0 | 3.40 | 13 | 5,802 | 1,177 | 4 |
| Contemporary weird | 256 | 0 | 5.00 | 14 | 8,647 | 1,806 | 4 |
| Chaos Blend | 1,020 | 4 | 3.53 | 14 | 88,856 | 15,990 | 4 (14 for tag-derived type) |

The v3 implementation increased the number of measured field families and includes clue and briefing. The four-value depth is still not credible field-level variety for a 256-seed sample; this is an explicit remaining gate, not something uniqueness of complete records can hide.

## Coherence and wording review

The two-thread compatibility boundary ties each generated field to the same authored narrative family. Seed examples generally connect the request, action, pressure, opposition, clue, and consequence. The new Clue field is explicit and the Briefing gives the user the giver and immediate pressure. Some compositions still have gaps: a clue may be atmospheric rather than necessary to the objective; an objective may not explain how it defeats the opposition; and a success consequence may not follow directly from the chosen objective. `quest_type` currently picks a semantic tag such as `scarcity`, `identity`, or `cyberpunk`, so it can read as a label rather than a contract-quality category.

Wording improved by using complete authored alternatives rather than joining arbitrary sentence fragments. Remaining systemic issues are repeated reuse of the same two choices in each thread, duplicated giver/premise text in Briefing, and inconsistent capitalization because corpus fields are sentence fragments mixed with full sentences. A human copy edit and broader variant pool review are outstanding. Samples are intentionally shown below so the remaining defects are visible.

## Creative-axis evidence

The selector weights compatible threads by semantic tags for all eight canonical axes: Bleak, Mythic, Chaos, Epic, Ambiguity, Surreal, Technology, and Mysticism. The audit compares the same 256 seeds, same Chaos Blend preset, and neutral values for all other axes, changing only the tested axis from 0 to 100. Thread counts were:

| Axis | Low: mythic-salvage / tide-memory | High: mythic-salvage / tide-memory | Assessment |
|---|---:|---:|---|
| Bleak | 67 / 189 | 111 / 145 | Visible shift |
| Mythic | 99 / 157 | 97 / 159 | No measurable shift |
| Chaos | 100 / 156 | 98 / 158 | No measurable shift |
| Epic | 119 / 137 | 95 / 161 | Visible shift |
| Ambiguity | 119 / 137 | 90 / 166 | Visible shift |
| Surreal | 107 / 149 | 71 / 185 | Visible shift |
| Technology | 109 / 147 | 75 / 181 | Visible shift |
| Mysticism | 56 / 200 | 130 / 126 | Strong shift |

The Mythic and Chaos comparisons do not support a meaningful distribution claim. Output text can still change because axis values participate in existing stream inputs; that change alone is not proof of semantic influence. The axis gate remains open until a controlled test produces reliable distribution shifts for every claimed relevant axis.

## Quest constraint coverage

Version 3 accepts `inspiration` (1–200 non-whitespace characters) and places it verbatim in the briefing. Other keys fail with a structured `InvalidConstraint` error; they are not silently ignored. The canonical Character-specific keys `must_be_non_human`, `no_magic`, `no_firearms`, and `urban_only` are not Quest constraints and are explicitly rejected by Quest Forge. The generic contract does not define a Quest constraint vocabulary beyond carrying normalized key/value pairs. Broader Quest-world filters need a reviewed vocabulary and content-level tags; those filters are not implemented and this gate remains partial.

## Representative samples

Samples below are seed 0, neutral axes, no constraint. Each line includes the central request/action, clue, opposition, and consequence; full candidate structures are produced by `quest_quality_audit`.

| Preset | Title and story chain |
|---|---|
| Dark fantasy | **A Hearth Owed to the Dark.** A hospice keeper seeks help as a curse draws warmth from homes; break it before nightfall. Warm footprints lead to a sealed tomb; the village warden denies making the bargain. The valley survives, but the watchtower claims abandoned homes. |
| Pirate | **At the Next Tide.** A harbor clerk asks the party to stop a salvage claim transferring a sunken neighborhood; ring the brass tide-bell before the census. A dry logbook records an unknown tide; the harbor magistrate profits from unclaimed wrecks. The families regain their claim and the magistrate loses office. |
| Steampunk | **The Heir in the Clock.** A registrar needs the maker's final instruction before the registry erases a living heir. A gear tooth bears the heir's nickname; the executor wound the automaton to reject human claims. The will is voided, but workers still need housing. |
| Cyberpunk | **Four Hundred Ghost Accounts.** An auditor asks the party to restore residents' source memories before a corporate identity overwrite at midnight. A checksum changes when a resident speaks their chosen name; a privacy lawyer holds the only unaltered record. The overwrite is delayed, while clinic tracking begins. |
| Nordic myth | **A Verse Missing from the Fire.** A skald's apprentice seeks the debtor's name before a dead ruler's debt takes the village's winter stores. A goat-herd remembers the missing meter; the rune-keeper refuses to alter the birth-stone. The children are spared, while a rival skald claims the saga's honor. |
| Post-apocalyptic | **The Seed Vault's Claim.** A grower wants proof of who can open the vault before the council burns the last viable seed. Tending notes contain a vanished settlement's water code; a marshal is ordered to destroy the vault. The vault is saved, but the convoy abandons one car. |
| Occult | **A Name Under Saint Orra's.** A night archivist asks the party to restore missing patients before dawn. A carbon copy records one pulse under two names; the hospital director sealed the chapel ward. The patients return, and the hospital must admit the ward existed. |
| Monster hunter | **A Trail of Warm Scales.** A healer asks the party to redirect a brood before market day. Venom appears upstream from the trail; a trophy hunter hides the only safe route. The wells are cured, but the healer closes the clinic to stop spread. |
| Rogue assassin | **The Informant Who Wasn't There.** A bodyguard wants the informant moved before a purge. A route ledger shows a detour added after the target left; a rival escort knows the fallback routes. The informant escapes, but the lookout faces the network. |
| Contemporary weird | **A Stop No One Remembers.** A neighborhood organizer asks the party to document a route serving a block missing from the city map. The driver's paper log lists passengers the camera missed; the route returns whenever someone names the stop. The stop returns to the map, but one passenger remains missing. |
| Chaos Blend (Remediation II seed 0) | **A Harbor with Two Histories.** An oath-speaker asks the party to reach the signal tower and guide a convoy past drowned boundary stones before settlements lose their water. A carved boundary stone carries the pump's serial mark; a sworn guardian mistakes the salvage crew for oath-breakers. The settlements share the water while the guardian keeps the pump key. The mixed location is the tide-powered signal archive; the reward is a claim ticket honored by harbor law and machine code. |

Additional audit samples include 16 seed-42 axis extremes and an Occult `inspiration` constraint sample (“a map written in starlight — 星 🗝️”). For example, the constrained briefing includes the Unicode inspiration verbatim. The v3 Chaos Blend sample is coherent at a glance, but its field pool remains small and its clue does not directly prove the district is inhabited.

## Verification scope and remaining limitations

Focused Quest tests cover determinism, all eleven presets, v3 clue/briefing, JSON Unicode roundtrip, inspiration visibility, structured rejection of unsupported constraints, lock-aware frame reroll, manual edits, and reroll-all preservation of manual edits and original lock flags. The corpus parser validates schema/depth/ids but does not yet validate every semantic relationship or duplicate choice. A full constraint matrix, robust atomic failure injection after a partially planned generation, representative human review, and reliable axes evidence remain outstanding. The final consolidated verification and workspace checks are recorded in the Mission 03 report.

## Remediation II — cluster collapse and axis proof (2026-09-26)

The pre-content investigation above is the instrument-first baseline. It found that every pre-fix pair sharing at least 10 of 14 public fields belonged to one selected thread: 11,679 `tide-memory-archive` pairs and 4,311 `mythic-salvage` pairs. The selected Frame fixed one shallow two-choice pool for almost every field. Within those high pairs, 12 identity fields each matched in about 12,200 pairs. Distinct per-field streams were not aliases; their pools were coupled by the one-thread bundle. This explains the paradox: v3 lowered average overlap by adding a second thread, while the thread-local fields formed a larger high-similarity tail than v2.

### Structural correction

Quest v3 now treats Chaos Blend as a coherent story spine plus deterministic secondary-role source mixing. The selected Frame remains the spine for giver, premise, objective, stakes, threat, clue, consequence, and quest type. Explicitly `mix-compatible` source threads can supply title, location, complication, twist, moral pressure, reward, and follow-up according to each field's existing stream and the Chaos axis. Chaos 0 keeps those roles on the spine; Chaos 100 selects the other compatible source. No global random source, ForgeDNA protocol change, persistence, or v4 was introduced. The compatibility marker is excluded from user-facing quest-type selection.

Briefing is now composed from the selected giver and premise. Its dependency list includes Frame, Patron, and Premise, and a regression test confirms that rerolling either input recomposes the briefing. Quest v2 is unchanged. The repository's committed PARTIAL checkpoint freezes v2 replay, but makes no v3 replay promise; v3 is therefore refined in place, with no v4.

### Same 1,024-seed Chaos Blend audit

All 523,776 pairs use the same original 14-field equality basis. The final audit found:

| Metric | Initial v2 | Previous v3 | Remediation II v3 |
|---|---:|---:|---:|
| Exact expanded clones | 0 | 4 | 0 |
| Pairs >=12/14 | not recorded | not recorded | 23 |
| Pairs >=10/14 | 2,083 | 15,990 | 1,162 |
| Pairs >=8/14 | 30,149 | 88,856 | 18,071 |
| Mean equal fields | 4.668 | 3.528 | 3.409 |
| Median equal fields | not recorded | not recorded | 3 |
| Maximum equal fields | 13 | 14 | 13 |
| Unique expanded candidates | 1,024 | 1,020 | 1,024 |

The >=10 count is down 92.7% from the previous v3 run and 44.2% below v2. All 1,162 remaining >=10 pairs still share their primary thread (856 mythic-spine pairs; 306 tide-spine pairs); no cross-spine pair entered that tail. This remaining correlation is visible in the field-level data and is not declared solved. The largest connected component of the >=10 pair graph contains 584 seeds, beginning `0, 1, 3, 5, 7, 8, 9, 11, 12, 14, 15, 18, 22, 25, 27`; the next contains 176 seeds, beginning `13, 19, 24, 47, 57, 69, 72, 77, 80, 81, 91, 95, 99, 102, 108`. These are connected components via pair edges, not claims that every pair inside directly matches >=10. The largest per-seed high-pair neighborhood is seed 508 and seed 703 with 8 partners each. The full membership and selection IDs are emitted by `quest_quality_audit`.

### Concentration and controlled axes

The final Chaos Blend sample still has four rendered choices in each ordinary identity-bearing corpus field (title, location, giver, objective, stakes, threat, complication, clue, reward, consequence, moral pressure, follow-up, and twist); Briefing has eight giver-premise combinations. Top-one / top-three concentration remains high: roughly 25.9% / 77.3% for location, 26.0% / 76.0% for title, 32.5% / 82.6% for giver, 31.3% / 81.6% for objective, 31.2% / 81.6% for stakes, 31.4% / 83.1% for threat, 26.5% / 76.1% for complication, 32.2% / 81.2% for clue, 26.9% / 75.9% for reward, 32.4% / 82.2% for consequence, and 26.2% / 75.8% for moral pressure. The earlier per-preset table reports all 12 required fields' unique/top-one/top-three measurements across every preset; non-Chaos pools are unchanged by this source-selection correction. The low unique counts remain a material gate.

Controlled same-seed Mythic comparisons on Chaos Blend (256 seeds, only Mythic changed) selected `mythic-salvage` 99 times at level 0 and 194 times at level 100. Mythic now has a direct semantic target: the thread explicitly tagged `mythic`, with stronger affinity in the selector. Controlled Chaos comparisons keep primary-thread counts nearly unchanged (150/106 vs 145/111) while changing secondary location source: low Chaos uses the spine for 256/256 seeds; high Chaos uses the compatible alternate source for 256/256 seeds. These prove intentional effects on story-family weighting and source mixing, respectively.

### Regression review and remaining gates

The final generated samples cover all eleven presets, axis extremes, and an inspiration-constrained output. They show the narrative chain remains connected for ordinary presets. Chaos's deliberately mixed secondary roles can still produce thematic seams; the cross-source alternatives need a human quality pass. The corpus still offers only two authored alternatives per source thread, and the four-value field result makes credible broad field-level diversity unproven. Quest's supported constraint surface remains only `inspiration`; its valid/empty/overlength matrix was exercised across all eleven presets, while unsupported and Character-only constraints remain structured errors. No broader semantic Quest filters are claimed. Human acceptance is not automated.

Mission 03 therefore remains **PARTIAL**. The cluster regression and both axis proofs pass, but field depth, Chaos seam review, complete semantic constraint policy, and human coherence/wording acceptance remain open.

### Final verification record

- Targeted Quest tests: 13 passed, 0 failed.
- `cargo fmt --all -- --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `cargo test --workspace`: 62 passed (52 app-core and 10 contracts), 0 failed.
- `cargo metadata --no-deps --format-version 1`: passed.
- `./scripts/verify.ps1`: passed; frontend format/lint/typecheck, 1 frontend test, frontend build, Rust workspace checks/tests, and optimized Tauri no-bundle build passed.
- `git diff --check`: passed.

## Final Remediation — pre-expansion content-depth inventory (2026-09-26)

This inventory was completed against the clean committed `004973e649ffd4babe4b7b2840730057635f32c1` baseline before any corpus edits. The existing deterministic audit ran 256 seeds for each of the ten ordinary presets and 1,024 for Chaos Blend. In every one of the 22 corpus threads, each of the 14 authored prose arrays (title, location, giver, premise, objective, stakes, threat, complication, clue, twist, reward, consequence, moral pressure, follow-up) contained exactly two nonempty entries.

### Source and compatible counts

For every ordinary preset and every listed prose field, source count was **4 per preset** (2 choices in each of its 2 threads); after the selected Frame/thread, the compatible pool was **2**. Chaos Blend likewise has 4 source items per prose field: core-spine roles had 2 Frame-compatible values; the secondary roles had 4 compatible values across the two explicitly mix-compatible threads (2 from the spine and 2 from the alternate source). No content filter reduced a larger pool. Giver and premise each had four source items per preset and two per selected Frame; Briefing composed those components and had eight observed combinations per preset (four per thread).

The rendered counts and most-common frequencies below are the observed 256-seed baseline for all 12 required fields. Cells are `unique / top-one frequency`:

| Preset | Title | Location | Giver | Objective | Stakes | Threat | Complication | Clue | Reward | Consequence | Moral pressure | Briefing |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| Dark fantasy | 4/32.0% | 4/31.6% | 4/29.7% | 4/30.9% | 4/34.4% | 4/37.1% | 4/30.1% | 4/30.1% | 4/32.4% | 4/30.9% | 4/29.7% | 8/16.0% |
| Pirate | 4/27.3% | 4/28.9% | 4/27.7% | 4/28.1% | 4/28.1% | 4/28.9% | 4/28.5% | 4/29.7% | 4/28.5% | 4/28.5% | 4/27.7% | 8/14.8% |
| Steampunk | 4/41.4% | 4/41.8% | 4/46.1% | 4/43.4% | 4/43.0% | 4/42.6% | 4/44.9% | 4/41.0% | 4/41.8% | 4/44.9% | 4/41.0% | 8/23.8% |
| Cyberpunk | 4/36.3% | 4/40.2% | 4/38.7% | 4/35.9% | 4/36.3% | 4/39.5% | 4/37.9% | 4/37.5% | 4/35.9% | 4/36.7% | 4/38.7% | 8/20.3% |
| Nordic myth | 4/32.4% | 4/30.9% | 4/27.7% | 4/27.7% | 4/28.5% | 4/28.5% | 4/27.3% | 4/29.3% | 4/30.9% | 4/34.8% | 4/29.3% | 8/14.5% |
| Post-apocalyptic | 4/34.4% | 4/31.6% | 4/32.4% | 4/32.4% | 4/33.6% | 4/33.2% | 4/32.0% | 4/32.4% | 4/32.0% | 4/32.0% | 4/33.6% | 8/18.0% |
| Occult | 4/30.9% | 4/32.4% | 4/29.3% | 4/31.2% | 4/30.1% | 4/31.2% | 4/28.9% | 4/28.5% | 4/28.5% | 4/31.6% | 4/28.5% | 8/16.0% |
| Monster hunter | 4/34.4% | 4/34.8% | 4/37.9% | 4/36.3% | 4/37.1% | 4/36.7% | 4/35.9% | 4/36.7% | 4/42.6% | 4/35.2% | 4/37.5% | 8/19.1% |
| Rogue assassin | 4/27.7% | 4/26.2% | 4/27.0% | 4/25.8% | 4/27.3% | 4/27.7% | 4/27.3% | 4/25.8% | 4/27.7% | 4/27.0% | 4/27.0% | 8/16.0% |
| Contemporary weird | 4/44.1% | 4/42.6% | 4/43.0% | 4/42.2% | 4/43.4% | 4/48.0% | 4/44.1% | 4/43.8% | 4/49.6% | 4/43.4% | 4/48.4% | 8/22.3% |
| Chaos Blend | 4/26.0% | 4/25.9% | 4/32.5% | 4/31.3% | 4/31.2% | 4/31.4% | 4/26.5% | 4/32.2% | 4/26.9% | 4/32.4% | 4/26.2% | 8/17.2% |

**Cause classification:** every ordinary identity field is case A: only four source entries exist, two per thread. The Chaos secondary fields also have only four source entries; the compatible-source rule does not further collapse their pool. Cases B (a larger pool filtered down), C (weighting suppresses existing alternatives), and D (distinct items rendered identically) were not supported by the evidence. Both alternatives from each source thread appear in the measured sample. Thus the first correction belongs in the corpus; the existing Frame/source architecture is not an obstacle to deeper authored families.

## Authoring Batch A (2026-09-26)

Expanded the six threads in Dark Fantasy (`claim-of-the-dead`, `winter-lanterns`), Pirate (`drowned-roll`, `mutiny-debt`), and Steampunk (`foundry-pressure`, `clockwork-inheritance`) in the existing v3 corpus. All 14 authored pools per thread now contain **6 distinct choices**, up from 2: title, location, giver, premise, objective, stakes, threat, complication, clue, twist, reward, consequence, moral pressure, and follow-up. That is 12 source choices per field per preset instead of 4. Briefing remains composed from giver and premise; each selected thread now supports 36 pairings instead of 4. No field was intentionally left smaller. No generator or protocol code changed.

Validation: the JSON parsed; all 22 thread IDs remain globally unique; the authored pools have no exact duplicates, empty values, or placeholder strings. The six expanded threads retain their existing tags and compatibility rules. The temporary deterministic smoke runner generated and validated 64/64 candidates for each target preset (192/192 total). Nine complete samples (seeds 0, 17, and 42 per preset) were inspected for joins and thematic fit; no broken template sentence or invalid Quest was observed. The samples retain recognizable grave/winter obligation, harbor/crew law, and civic machinery/inheritance motifs. This was a focused spot check, not the final 52-Quest human review; broader coherence, repetition, rendered-field concentration, and cross-preset quality remain open for later batches and the final audit.

Focused verification: `cargo test -p forge-app-core` passed **52 tests, 0 failed** (0 doc tests); the relevant Quest core subset passed 13/13; all three 64-seed generation sweeps passed; `cargo fmt --all -- --check` passed; `git diff --check` passed. Mission 03 remains **PARTIAL**.
