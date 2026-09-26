# Quest Forge Content Quality Audit — Mission 03 Remediation

## Status

**PARTIAL.** The v3 declarative corpus and generation path are implemented, but the similarity gate regressed at the important high-overlap thresholds, most field pools remain shallow, the axis experiment is inconclusive for two axes, and human review/coherence is not complete. Mission 04 remains blocked by Mission 03.

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
| Chaos Blend | **The Chart That Remembered.** A copied-memory chart smuggler asks the party to prove a living district exists before sunrise. A ledger records a tide from a vanished sea; the harbor court's daemon controls the archive gates. Restoring the district makes copied memories public evidence. |

Additional audit samples include 16 seed-42 axis extremes and an Occult `inspiration` constraint sample (“a map written in starlight — 星 🗝️”). For example, the constrained briefing includes the Unicode inspiration verbatim. The v3 Chaos Blend sample is coherent at a glance, but its field pool remains small and its clue does not directly prove the district is inhabited.

## Verification scope and remaining limitations

Focused Quest tests cover determinism, all eleven presets, v3 clue/briefing, JSON Unicode roundtrip, inspiration visibility, structured rejection of unsupported constraints, lock-aware frame reroll, manual edits, and reroll-all preservation of manual edits and original lock flags. The corpus parser validates schema/depth/ids but does not yet validate every semantic relationship or duplicate choice. A full constraint matrix, robust atomic failure injection after a partially planned generation, representative human review, and reliable axes evidence remain outstanding. The final consolidated verification and workspace checks are recorded in the Mission 03 report.
