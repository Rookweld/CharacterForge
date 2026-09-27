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

## Authoring Batch B (2026-09-26)

Expanded the six threads in Cyberpunk (`identity-override`, `synthetic-rights`), Nordic Myth (`boundary-oath`, `saga-debt`), and Post-Apocalyptic (`water-allocation`, `seed-vault`). All 14 authored pools per thread moved from 2 to 6 distinct choices (12 per ordinary preset instead of 4); no pool was intentionally left smaller. The `seed-vault` complication pool has 8 choices after the sample review prompted two additional alternatives to reduce a repeated narrow complication. Tags and compatibility rules are unchanged; no generator or ForgeDNA code changed.

Corpus validation: full JSON parsed with Unicode intact; 11 presets and 22 globally unique thread IDs; all pools remained nonempty and contained no exact duplicates, blank strings, or placeholders. The six target threads retain existing nonempty pools and tags. The 64-seed sweeps generated and validated **192/192** Quests. Nine complete samples (seeds 0, 17, and 42 per preset) were inspected. Cyberpunk samples centered on identity access and synthetic labor; Nordic Myth samples centered on oath, kinship, winter debt, and burial; Post-Apocalyptic samples centered on seed stewardship, water, and settlement tradeoffs. One overly specific synthetic-labor twist presumed a fatal route not established by every compatible premise; it was rewritten as a general unsafe-instruction recall test. A seed-vault complication appeared repeatedly in the first sample review, so its pool was broadened with two distinct alternatives. On regeneration, a different existing seed-vault complication still appeared in two of the three spot-check outputs; the larger repetition audit remains open, so this is recorded rather than treated as resolved corpus-wide.

Focused verification: `cargo test -p forge-app-core` passed **52 tests, 0 failed** (0 doc tests), including the controlled Mythic influence regression; 64/64 generation and candidate validation passed for each of the three target presets; `cargo fmt --all -- --check` passed; `git diff --check` passed. This spot check does not replace the final 52-Quest review or full field/concentration audit. Mission 03 remains **PARTIAL**.

## Authoring Batch C (2026-09-27)

Expanded the six threads in Occult (`missing-register`, `borrowed-hour`), Monster Hunter (`brood-migration`, `false-trophy`), and Rogue Assassin (`false-contract`, `missing-informant`). The 14 authored pools in each thread moved from 2 to 6 distinct choices. After the sample review, `brood-migration` moral pressure was expanded from 6 to 8 choices to broaden its habitat-versus-settlement dilemmas. No pool was intentionally left smaller. Tags and compatibility semantics are unchanged; no generator or ForgeDNA code changed.

Corpus validation: the full Unicode JSON parsed; all 22 thread IDs remain globally unique; every authored pool is nonempty, contains no exact duplicates, and has no blank or placeholder strings. Target threads retain their existing tags and candidate pools. The 64-seed sweeps generated and validated **192/192** Quests.

Nine complete samples (seeds 0, 17, and 42 per preset) were read. The outputs showed procedural archive and time debts for Occult, ecology/evidence and habitat tradeoffs for Monster Hunter, and contract/witness leverage for Rogue Assassin. One new Occult moral-pressure choice about an unrelated inheritance was replaced with a register/witness dilemma. A repeated brood-migration habitat dilemma prompted the two additional moral-pressure choices. After regeneration, the brood dilemma still appeared in two of its three samples; the same false-contract premise/stakes pairing appeared in two Rogue Assassin samples. The relevant pools contain multiple alternatives, so three examples alone do not establish a concentration defect; both repetitions remain for the authoritative 256-seed review.

Focused verification: `cargo test -p forge-app-core` passed **52 tests, 0 failed** (0 doc tests), including the Mythic influence regression; all three 64-seed generation sweeps passed; `cargo fmt --all -- --check` passed; `git diff --check` passed. The final 52-Quest matrix and field-concentration audit remain open. Mission 03 remains **PARTIAL**.

## Authoring Batch D (2026-09-27)

Expanded Contemporary Weird's `vanishing-floor` and `route-that-stops` threads, plus the two existing Chaos Blend-compatible threads `tide-memory-archive` and `mythic-salvage`. All 14 authored pools in each thread moved from 2 to 6 distinct choices. No pool was intentionally left smaller. Chaos-specific content remains within the two existing tagged threads; no standalone Chaos pool or fixed templates were added. Chaos still composes independently selected roles from compatible sources.

Corpus validation: full Unicode JSON parsed; 11 presets and all 22 globally unique thread IDs were present; authored pools were nonempty, distinct within each pool, and free of blanks/placeholders. Tags and compatibility references were preserved. Earlier Batch A/B/C content was not edited. The 64-seed Contemporary Weird sweep passed **64/64**; the 128-seed Chaos Blend sweep passed **128/128**. Every generated candidate validated.

Six complete Contemporary Weird samples were inspected, balanced across both threads (three each). They retained concrete housing/permit and transit/reimbursement procedures paired with impossible floors, passengers, stops, and time records. Repetition review found the route thread in 59/64 generated frames and the missing-floor thread in 5/64; the six spot-check samples reused the hospital night-route setup several times. The thread weights and pools were not changed based on this small sample; field concentration and thread distribution should be checked in the final audit.

Ten complete Chaos samples were inspected, five from each thread. The 128-seed frame count was 86 Mythic Salvage and 42 Tide Memory Archive; selection IDs showed secondary-source mixing across titles, locations, complications, twists, rewards, follow-ups, and moral pressure. Outputs combined harbor memory/identity systems with oath law, ration machinery, drowned boundaries, and salvage claims; no separate fixed bundle was observed. Some titles and premises recurred across samples, but the complete title/premise/stakes packages varied. Corpus validation caught exact duplicates among several newly authored Chaos choices; those choices were replaced and the full validation and 128-seed run were repeated. Sample review also found one Mythic moral-pressure choice duplicated its selected consequence, so it was rewritten as a separate convoy-versus-settlement dilemma. No broken joins remained in the final ten samples.

Focused verification: `cargo test -p forge-app-core` passed **52 tests, 0 failed** (0 doc tests), including the existing Chaos mixing and Mythic influence tests; both generation sweeps passed; `cargo fmt --all -- --check` passed; `git diff --check` passed. The 256-seed all-preset field audit, 1,024-seed Chaos audit, similarity analysis, and final 52+ Quest matrix are deferred to the explicitly requested final quality audit. Mission 03 remains **PARTIAL**.

## Final Mission 03 Quality Audit / Closeout (2026-09-27)

### Corpus and deterministic sweep

The committed Batch D baseline was clean at `c61b0701fb8629d049675a8b1e3f41fbc0beea9b` on `master`. The v3 JSON parsed as UTF-8: 11 presets, 22 globally unique thread IDs, 14 authored arrays per thread, no empty strings, local duplicate entries, or placeholder tokens. Tags remained present; all 22 threads generated successfully across the sweep, so all field pools survived selection/compatibility. The 256-seed matrix covers all eleven presets (2,816 candidates): all ten ordinary presets at 256 each, and Chaos seeds 0–255. The extended Chaos run covers seeds 0–1,023 (1,024 total, including that matrix subset). Thus the main v3 audit validated 2,560 ordinary plus 1,024 Chaos candidates (**3,584 unique candidates; 0 invalid**). An additional 1,024 content-version-2 Chaos candidates supported the historical comparison. Exact expanded duplicates in each preset sweep: **0**.

All 14 authored choice pools contain six choices per thread, except the retained expansions: Post-Apocalyptic `seed-vault` complications (8), Monster Hunter `brood-migration` moral pressures (8). These are deliberate broadening additions recorded in their batch notes. Thus source pools usually aggregate to 12 choices per ordinary preset/field across its two threads and 12 across the two Chaos-compatible threads; the two expanded pools aggregate to 14. Quest type is derived from semantic tags, not one of the 14 text arrays. Briefing is composed from giver and premise (36 combinations per thread; 72 across two threads).

### Thread selection distribution

Expected shares below are the selector’s explicit tag-derived weights at neutral axes (50); observed values use the 256-seed run. Differences reflect this bounded deterministic seed set; the observed counts remain consistent with the declared weighting. Contemporary Weird’s `route-that-stops` has 4× `vanishing-floor` weight at neutral axes because the transit/memory tags activate three axis-affinity additions; expected 80%/20%. It selected 215/256 (84.0%) versus 41/256 (16.0%), so the Batch D 59/64 spot check was a small-sample skew, not a 50/50 expectation or a defect.

| Preset | Thread | Observed | Observed % | Expected % |
|---|---|---:|---:|---:|
| dark-fantasy | `claim-of-the-dead` | 105/256 | 41.0% | 44.4% |
| dark-fantasy | `winter-lanterns` | 151/256 | 59.0% | 55.6% |
| pirate | `drowned-roll` | 139/256 | 54.3% | 57.1% |
| pirate | `mutiny-debt` | 117/256 | 45.7% | 42.9% |
| steampunk | `foundry-pressure` | 51/256 | 19.9% | 25.0% |
| steampunk | `clockwork-inheritance` | 205/256 | 80.1% | 75.0% |
| cyberpunk | `identity-override` | 184/256 | 71.9% | 75.0% |
| cyberpunk | `synthetic-rights` | 72/256 | 28.1% | 25.0% |
| nordic-myth | `boundary-oath` | 116/256 | 45.3% | 40.0% |
| nordic-myth | `saga-debt` | 140/256 | 54.7% | 60.0% |
| post-apocalyptic | `water-allocation` | 97/256 | 37.9% | 40.0% |
| post-apocalyptic | `seed-vault` | 159/256 | 62.1% | 60.0% |
| occult | `missing-register` | 144/256 | 56.2% | 50.0% |
| occult | `borrowed-hour` | 112/256 | 43.8% | 50.0% |
| monster-hunter | `brood-migration` | 175/256 | 68.4% | 66.7% |
| monster-hunter | `false-trophy` | 81/256 | 31.6% | 33.3% |
| rogue-assassin | `false-contract` | 126/256 | 49.2% | 50.0% |
| rogue-assassin | `missing-informant` | 130/256 | 50.8% | 50.0% |
| contemporary-weird | `vanishing-floor` | 41/256 | 16.0% | 20.0% |
| contemporary-weird | `route-that-stops` | 215/256 | 84.0% | 80.0% |
| chaos-blend | `tide-memory-archive` | 389/1,024 | 38.0% | 38.5% |
| chaos-blend | `mythic-salvage` | 635/1,024 | 62.0% | 61.5% |

Largest absolute observed deviation from the explicit neutral-axis weights was 2.00 binomial standard deviations (computed using 256 seeds for ordinary presets and 1,024 for Chaos). No weighting or selection change was warranted.

### Rendered field depth and concentration

For each direct authored field, “source choices” is the total number of entries across the preset’s two threads; each thread contributes six unless the expanded exceptions above apply. `Briefing` is a composed field; `Quest type` comes from tags. Every row reports unique rendered values, mode count and share, and the combined top-three share across the 256 seeds (1,024 for Chaos).

| Preset | Field | Source choices | Rendered unique | Mode count (%) | Top 3 (%) |
|---|---|---:|---:|---:|---:|
| dark-fantasy | briefing | 72 pairings max | 69 | 8 (3.1%) | 9.4% |
| dark-fantasy | clue | 12 | 12 | 28 (10.9%) | 32.4% |
| dark-fantasy | complication | 12 | 12 | 27 (10.5%) | 30.9% |
| dark-fantasy | consequence | 12 | 12 | 32 (12.5%) | 34.4% |
| dark-fantasy | follow_up | 12 | 12 | 30 (11.7%) | 34.4% |
| dark-fantasy | location | 12 | 12 | 34 (13.3%) | 34.8% |
| dark-fantasy | moral_pressure | 12 | 12 | 32 (12.5%) | 34.4% |
| dark-fantasy | objective | 12 | 12 | 31 (12.1%) | 33.6% |
| dark-fantasy | premise | 12 | 12 | 30 (11.7%) | 34.0% |
| dark-fantasy | quest_giver | 12 | 12 | 29 (11.3%) | 32.8% |
| dark-fantasy | quest_type | semantic tags | 6 | 55 (21.5%) | 59.0% |
| dark-fantasy | reward | 12 | 12 | 34 (13.3%) | 33.6% |
| dark-fantasy | stakes | 12 | 12 | 33 (12.9%) | 34.4% |
| dark-fantasy | threat | 12 | 12 | 36 (14.1%) | 37.1% |
| dark-fantasy | title | 12 | 12 | 32 (12.5%) | 34.0% |
| dark-fantasy | twist | 12 | 12 | 28 (10.9%) | 32.4% |
| pirate | briefing | 72 pairings max | 68 | 9 (3.5%) | 10.2% |
| pirate | clue | 12 | 12 | 26 (10.2%) | 29.7% |
| pirate | complication | 12 | 12 | 34 (13.3%) | 34.0% |
| pirate | consequence | 12 | 12 | 30 (11.7%) | 34.0% |
| pirate | follow_up | 12 | 12 | 29 (11.3%) | 30.5% |
| pirate | location | 12 | 12 | 28 (10.9%) | 32.0% |
| pirate | moral_pressure | 12 | 12 | 27 (10.5%) | 28.9% |
| pirate | objective | 12 | 12 | 27 (10.5%) | 30.1% |
| pirate | premise | 12 | 12 | 28 (10.9%) | 32.8% |
| pirate | quest_giver | 12 | 12 | 31 (12.1%) | 34.8% |
| pirate | quest_type | semantic tags | 5 | 78 (30.5%) | 68.0% |
| pirate | reward | 12 | 12 | 28 (10.9%) | 30.1% |
| pirate | stakes | 12 | 12 | 29 (11.3%) | 32.4% |
| pirate | threat | 12 | 12 | 28 (10.9%) | 31.6% |
| pirate | title | 12 | 12 | 28 (10.9%) | 30.1% |
| pirate | twist | 12 | 12 | 32 (12.5%) | 30.9% |
| steampunk | briefing | 72 pairings max | 62 | 12 (4.7%) | 11.7% |
| steampunk | clue | 12 | 12 | 40 (15.6%) | 44.9% |
| steampunk | complication | 12 | 12 | 44 (17.2%) | 45.7% |
| steampunk | consequence | 12 | 12 | 41 (16.0%) | 44.9% |
| steampunk | follow_up | 12 | 12 | 42 (16.4%) | 43.8% |
| steampunk | location | 12 | 12 | 45 (17.6%) | 44.9% |
| steampunk | moral_pressure | 12 | 12 | 40 (15.6%) | 43.4% |
| steampunk | objective | 12 | 12 | 45 (17.6%) | 43.4% |
| steampunk | premise | 12 | 12 | 41 (16.0%) | 43.8% |
| steampunk | quest_giver | 12 | 12 | 49 (19.1%) | 48.4% |
| steampunk | quest_type | semantic tags | 6 | 70 (27.3%) | 80.1% |
| steampunk | reward | 12 | 12 | 43 (16.8%) | 44.5% |
| steampunk | stakes | 12 | 12 | 41 (16.0%) | 44.9% |
| steampunk | threat | 12 | 12 | 45 (17.6%) | 46.1% |
| steampunk | title | 12 | 12 | 38 (14.8%) | 43.0% |
| steampunk | twist | 12 | 12 | 38 (14.8%) | 43.0% |
| cyberpunk | briefing | 72 pairings max | 68 | 10 (3.9%) | 11.3% |
| cyberpunk | clue | 12 | 12 | 35 (13.7%) | 38.3% |
| cyberpunk | complication | 12 | 12 | 41 (16.0%) | 42.2% |
| cyberpunk | consequence | 12 | 12 | 34 (13.3%) | 38.7% |
| cyberpunk | follow_up | 12 | 12 | 35 (13.7%) | 38.7% |
| cyberpunk | location | 12 | 12 | 38 (14.8%) | 42.2% |
| cyberpunk | moral_pressure | 12 | 12 | 39 (15.2%) | 43.0% |
| cyberpunk | objective | 12 | 12 | 37 (14.5%) | 39.8% |
| cyberpunk | premise | 12 | 12 | 35 (13.7%) | 39.5% |
| cyberpunk | quest_giver | 12 | 12 | 34 (13.3%) | 39.1% |
| cyberpunk | quest_type | semantic tags | 6 | 62 (24.2%) | 71.9% |
| cyberpunk | reward | 12 | 12 | 33 (12.9%) | 38.3% |
| cyberpunk | stakes | 12 | 12 | 35 (13.7%) | 39.5% |
| cyberpunk | threat | 12 | 12 | 45 (17.6%) | 41.4% |
| cyberpunk | title | 12 | 12 | 34 (13.3%) | 38.7% |
| cyberpunk | twist | 12 | 12 | 39 (15.2%) | 42.2% |
| nordic-myth | briefing | 72 pairings max | 71 | 8 (3.1%) | 8.6% |
| nordic-myth | clue | 12 | 12 | 30 (11.7%) | 32.4% |
| nordic-myth | complication | 12 | 12 | 28 (10.9%) | 30.5% |
| nordic-myth | consequence | 12 | 12 | 32 (12.5%) | 34.8% |
| nordic-myth | follow_up | 12 | 12 | 29 (11.3%) | 32.0% |
| nordic-myth | location | 12 | 12 | 30 (11.7%) | 32.8% |
| nordic-myth | moral_pressure | 12 | 12 | 32 (12.5%) | 31.6% |
| nordic-myth | objective | 12 | 12 | 27 (10.5%) | 30.5% |
| nordic-myth | premise | 12 | 12 | 29 (11.3%) | 31.2% |
| nordic-myth | quest_giver | 12 | 12 | 30 (11.7%) | 31.2% |
| nordic-myth | quest_type | semantic tags | 6 | 52 (20.3%) | 55.9% |
| nordic-myth | reward | 12 | 12 | 31 (12.1%) | 34.0% |
| nordic-myth | stakes | 12 | 12 | 32 (12.5%) | 31.2% |
| nordic-myth | threat | 12 | 12 | 29 (11.3%) | 31.6% |
| nordic-myth | title | 12 | 12 | 32 (12.5%) | 32.4% |
| nordic-myth | twist | 12 | 12 | 33 (12.9%) | 33.6% |
| post-apocalyptic | briefing | 72 pairings max | 67 | 10 (3.9%) | 10.5% |
| post-apocalyptic | clue | 12 | 12 | 32 (12.5%) | 35.9% |
| post-apocalyptic | complication | 14 | 14 | 24 (9.4%) | 25.8% |
| post-apocalyptic | consequence | 12 | 12 | 33 (12.9%) | 36.3% |
| post-apocalyptic | follow_up | 12 | 12 | 32 (12.5%) | 37.5% |
| post-apocalyptic | location | 12 | 12 | 32 (12.5%) | 34.8% |
| post-apocalyptic | moral_pressure | 12 | 12 | 35 (13.7%) | 34.8% |
| post-apocalyptic | objective | 12 | 12 | 31 (12.1%) | 34.0% |
| post-apocalyptic | premise | 12 | 12 | 40 (15.6%) | 38.3% |
| post-apocalyptic | quest_giver | 12 | 12 | 33 (12.9%) | 36.3% |
| post-apocalyptic | quest_type | semantic tags | 6 | 56 (21.9%) | 62.1% |
| post-apocalyptic | reward | 12 | 12 | 32 (12.5%) | 34.0% |
| post-apocalyptic | stakes | 12 | 12 | 38 (14.8%) | 36.3% |
| post-apocalyptic | threat | 12 | 12 | 38 (14.8%) | 35.9% |
| post-apocalyptic | title | 12 | 12 | 31 (12.1%) | 34.4% |
| post-apocalyptic | twist | 12 | 12 | 32 (12.5%) | 35.9% |
| occult | briefing | 72 pairings max | 70 | 7 (2.7%) | 7.8% |
| occult | clue | 12 | 12 | 29 (11.3%) | 32.8% |
| occult | complication | 12 | 12 | 32 (12.5%) | 32.8% |
| occult | consequence | 12 | 12 | 32 (12.5%) | 31.6% |
| occult | follow_up | 12 | 12 | 30 (11.7%) | 32.4% |
| occult | location | 12 | 12 | 32 (12.5%) | 33.6% |
| occult | moral_pressure | 12 | 12 | 31 (12.1%) | 32.4% |
| occult | objective | 12 | 12 | 31 (12.1%) | 31.2% |
| occult | premise | 12 | 12 | 29 (11.3%) | 31.6% |
| occult | quest_giver | 12 | 12 | 30 (11.7%) | 32.4% |
| occult | quest_type | semantic tags | 6 | 53 (20.7%) | 57.0% |
| occult | reward | 12 | 12 | 33 (12.9%) | 33.2% |
| occult | stakes | 12 | 12 | 29 (11.3%) | 32.4% |
| occult | threat | 12 | 12 | 32 (12.5%) | 34.8% |
| occult | title | 12 | 12 | 32 (12.5%) | 34.0% |
| occult | twist | 12 | 12 | 31 (12.1%) | 32.0% |
| monster-hunter | briefing | 72 pairings max | 69 | 11 (4.3%) | 10.9% |
| monster-hunter | clue | 12 | 12 | 36 (14.1%) | 39.1% |
| monster-hunter | complication | 12 | 12 | 34 (13.3%) | 37.1% |
| monster-hunter | consequence | 12 | 12 | 34 (13.3%) | 38.7% |
| monster-hunter | follow_up | 12 | 12 | 36 (14.1%) | 37.9% |
| monster-hunter | location | 12 | 12 | 35 (13.7%) | 39.1% |
| monster-hunter | moral_pressure | 14 | 14 | 27 (10.5%) | 28.9% |
| monster-hunter | objective | 12 | 12 | 38 (14.8%) | 39.5% |
| monster-hunter | premise | 12 | 12 | 39 (15.2%) | 40.6% |
| monster-hunter | quest_giver | 12 | 12 | 35 (13.7%) | 38.3% |
| monster-hunter | quest_type | semantic tags | 6 | 63 (24.6%) | 68.4% |
| monster-hunter | reward | 12 | 12 | 44 (17.2%) | 42.6% |
| monster-hunter | stakes | 12 | 12 | 34 (13.3%) | 37.1% |
| monster-hunter | threat | 12 | 12 | 33 (12.9%) | 37.5% |
| monster-hunter | title | 12 | 12 | 40 (15.6%) | 40.2% |
| monster-hunter | twist | 12 | 12 | 36 (14.1%) | 37.9% |
| rogue-assassin | briefing | 72 pairings max | 69 | 9 (3.5%) | 9.4% |
| rogue-assassin | clue | 12 | 12 | 29 (11.3%) | 30.9% |
| rogue-assassin | complication | 12 | 12 | 27 (10.5%) | 30.5% |
| rogue-assassin | consequence | 12 | 12 | 29 (11.3%) | 31.6% |
| rogue-assassin | follow_up | 12 | 12 | 29 (11.3%) | 29.3% |
| rogue-assassin | location | 12 | 12 | 29 (11.3%) | 29.3% |
| rogue-assassin | moral_pressure | 12 | 12 | 26 (10.2%) | 28.5% |
| rogue-assassin | objective | 12 | 12 | 27 (10.5%) | 29.7% |
| rogue-assassin | premise | 12 | 12 | 29 (11.3%) | 32.4% |
| rogue-assassin | quest_giver | 12 | 12 | 33 (12.9%) | 33.2% |
| rogue-assassin | quest_type | semantic tags | 6 | 48 (18.8%) | 55.1% |
| rogue-assassin | reward | 12 | 12 | 31 (12.1%) | 31.6% |
| rogue-assassin | stakes | 12 | 12 | 25 (9.8%) | 28.9% |
| rogue-assassin | threat | 12 | 12 | 27 (10.5%) | 30.5% |
| rogue-assassin | title | 12 | 12 | 26 (10.2%) | 28.5% |
| rogue-assassin | twist | 12 | 12 | 30 (11.7%) | 32.0% |
| contemporary-weird | briefing | 72 pairings max | 60 | 10 (3.9%) | 10.9% |
| contemporary-weird | clue | 12 | 12 | 43 (16.8%) | 44.9% |
| contemporary-weird | complication | 12 | 12 | 44 (17.2%) | 48.0% |
| contemporary-weird | consequence | 12 | 12 | 46 (18.0%) | 45.7% |
| contemporary-weird | follow_up | 12 | 12 | 46 (18.0%) | 46.9% |
| contemporary-weird | location | 12 | 12 | 43 (16.8%) | 48.8% |
| contemporary-weird | moral_pressure | 12 | 12 | 44 (17.2%) | 48.4% |
| contemporary-weird | objective | 12 | 12 | 45 (17.6%) | 46.9% |
| contemporary-weird | premise | 12 | 12 | 38 (14.8%) | 43.8% |
| contemporary-weird | quest_giver | 12 | 12 | 47 (18.4%) | 47.7% |
| contemporary-weird | quest_type | semantic tags | 6 | 74 (28.9%) | 84.0% |
| contemporary-weird | reward | 12 | 12 | 47 (18.4%) | 49.6% |
| contemporary-weird | stakes | 12 | 12 | 45 (17.6%) | 48.4% |
| contemporary-weird | threat | 12 | 12 | 44 (17.2%) | 48.8% |
| contemporary-weird | title | 12 | 12 | 41 (16.0%) | 46.1% |
| contemporary-weird | twist | 12 | 12 | 43 (16.8%) | 46.9% |
| chaos-blend | briefing | 72 pairings max | 72 | 30 (2.9%) | 8.0% |
| chaos-blend | clue | 12 | 12 | 121 (11.8%) | 32.6% |
| chaos-blend | complication | 12 | 12 | 99 (9.7%) | 27.7% |
| chaos-blend | consequence | 12 | 12 | 118 (11.5%) | 32.4% |
| chaos-blend | follow_up | 12 | 12 | 105 (10.3%) | 28.2% |
| chaos-blend | location | 12 | 12 | 95 (9.3%) | 27.5% |
| chaos-blend | moral_pressure | 12 | 12 | 102 (10.0%) | 27.4% |
| chaos-blend | objective | 12 | 12 | 118 (11.5%) | 33.2% |
| chaos-blend | premise | 12 | 12 | 135 (13.2%) | 34.4% |
| chaos-blend | quest_giver | 12 | 12 | 132 (12.9%) | 33.7% |
| chaos-blend | quest_type | semantic tags | 15 | 109 (10.6%) | 30.2% |
| chaos-blend | reward | 12 | 12 | 97 (9.5%) | 27.6% |
| chaos-blend | stakes | 12 | 12 | 112 (10.9%) | 32.3% |
| chaos-blend | threat | 12 | 12 | 118 (11.5%) | 32.0% |
| chaos-blend | title | 12 | 12 | 102 (10.0%) | 29.0% |
| chaos-blend | twist | 12 | 12 | 104 (10.2%) | 29.4% |

The major directly authored fields render 12 or 14 distinct values per ordinary preset in the sweep, compared with four before expansion. Chaos Blend renders 12 values for the direct pools, with 72 possible giver/premise briefing combinations (72 observed). Across ordinary presets, modal source-choice concentration is generally 9.4–19.1%; briefing concentration is lower because it composes two independent selections. The audit does not require every authored choice to appear in every 256-seed range.

### Prior spot-check observations

- **Batch B — Post-Apocalyptic complication:** the repeated choice is now 24/256 (9.4%); 14 complication values rendered, top three combined 25.8%. This is an ordinary small-sample recurrence, not a concentration defect.
- **Batch C — Brood Migration moral dilemma:** the leading moral-pressure value is 27/256 (10.5%); 14 values rendered, top three combined 28.9%. Two of three in the earlier spot check is not corpus concentration.
- **Batch C — Rogue-Assassin premise/stakes pairing:** 71 distinct pairs occur in 256 seeds; the most frequent pair occurs 9 times (3.5%). The earlier repeated pair is a small-sample coincidence.
- **Batch D — Contemporary Weird 59/5 thread split:** the 256-seed split is 215/41 and follows expected 80/20 tag weighting (84/16 observed). No weight change was made.
- **Batch D — repeated bus-route setup:** it is the intentionally favored transit/memory thread, not a generic random city threat. The audit renders 12 values for each direct ordinary pool, and the human matrix includes both route and vanishing-floor examples. The repeated setup is acceptable thematic recurrence under the documented weight.

### Chaos Blend similarity evolution

| Audit | Exact clones | ≥12/14 | ≥10/14 | ≥8/14 | Mean | Median | Max |
|---|---:|---:|---:|---:|---:|---:|---:|
| Initial baseline | 0 | — | 2,083 | 30,149 | 4.668 | — | 13 |
| Early v3 | 4 | — | 15,990 | 88,856 | 3.528 | — | 14 |
| Structural remediation | 0 | 23 | 1,162 | 18,071 | 3.409 | 3 | 13 |
| Current expanded v3, 1,024 seeds / 523,776 pairs | 0 | 0 | 0 | 17 | 1.186 | 1 | 9 |

Current similarity is below the structural-remediation tail. At the ≥8/14 edge threshold, the largest connected component has 3 seeds (90, 349, 393); at ≥10/14 there are no edges and no clusters. This indicates no severe remaining similarity cluster in this bounded run.

### Human-readable matrix and full read-through

Saved 52 complete outputs in `docs/quality/MISSION_03_FINAL_QUEST_SAMPLES.md`: 30 ordinary (three for each of ten non-Chaos presets), 10 neutral Chaos, six paired Mythic/Chaos/Epic low/high examples at identical seed 42, and six constrained examples. Contemporary Weird’s three include two `route-that-stops` outputs and one `vanishing-floor` output (seed 6), so both institutional setups were inspected. The 52 full Quest objects, clue and briefing were read against the rubric; results: **45 usable as-is, 7 usable with minor editing, 0 weak/substantial rewrite**. The seven minor cases are individually identified in the artifact.

The initial matrix review found a systemic punctuation join in constrained briefings: inspiration text followed the premise without a sentence boundary. The v3 composer now inserts a period before `Inspiration:`; the existing Unicode constraint test asserts the punctuation and a UTF-8 JSON round-trip. The six constrained samples were regenerated and re-read after the correction. No further systemic grammar, compatibility, or content defect was found.

Representative strengths include procedural occult record restoration (sample 19), evidence-led migration and bait preparation (samples 23–24), witness leverage and contract extraction (samples 25–27), institutional weirdness across both transit and housing (samples 28–30), and coherent oath-law / machine-memory collision (Chaos sample 42). Seven cases would benefit from minor editorial tightening; none requires substantial rewrite. This is a full sample-by-sample reviewer read, not a player playtest. The saved packet is ready for independent external review.

### Axis, constraints, state-safety, replay, and Unicode

- **Mythic:** same 256 Chaos seeds, all other inputs fixed: `mythic-salvage` 99/256 at Mythic 0 → 194/256 at Mythic 100.
- **Chaos:** same 256 seeds: secondary location-source mix 0/256 at Chaos 0 → 256/256 at Chaos 100; primary thread counts remain comparable (150/106 → 145/111).
- **Constraints:** Quest recognizes only `inspiration` (1–200 non-whitespace characters). Existing tests verify valid generation and visible inclusion for all eleven presets; six varied constrained examples are in the review packet. Whitespace-only, >200-character, unknown, and Character-only keys return structured `InvalidConstraint` errors. Constrained samples remain diverse across presets and threads.
- **Reroll/lock/manual edits:** Quest tests cover single-field dependency scope, locked descendant firewall, reroll-all lock preservation, manual edits across unrelated rerolls, and briefing recomposition after giver/premise changes. Shared reroll tests cover atomic failure and nonce overflow. The generator receives DNA immutably; the audit confirmed unchanged caller DNA during v2 replay and the v3 extreme-axis test does the same.
- **Quest v2 replay:** a version-2 Pirate candidate at seed 42 was validated twice from identical DNA; candidates matched, caller DNA was unchanged, and v2 clue/briefing remain absent. Batch A–D touched declarative v3 data only; the narrowly fixed v3 inspiration briefing path does not alter v2.
- **Unicode:** full corpus decoded and generated as UTF-8; the Quest constraint test verifies `séance`, em dash, emoji, serialization, and deserialization preserve the candidate. Existing corpus Unicode remains intact.

### Verification

- Targeted Quest suite: `cargo test -p forge-app-core quest::tests::` — **13 passed, 0 failed**.
- Full workspace suite: `cargo test --workspace` — **62 unit tests passed (52 app-core, 10 contracts), 0 failed; all doc-test targets passed (0 tests)**.
- `cargo fmt --all -- --check` — PASS.
- `cargo clippy --workspace --all-targets -- -D warnings` — PASS.
- `cargo metadata --no-deps --format-version 1` — PASS.
- `scripts/verify.ps1` — PASS; Biome format (11 files), lint (12 files), TypeScript, Vitest (1/1), production frontend build, Rust checks/tests, and optimized no-bundle Tauri build.
- `git diff --check` — PASS after the closeout documentation was written.

### Closeout decision

**Mission 03: PASS.** All requested quality and verification gates completed. Mission 04 has not started.
