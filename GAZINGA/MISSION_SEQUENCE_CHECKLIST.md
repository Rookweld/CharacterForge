# FOUNDATION MISSION SEQUENCE

Use this as the human operator checklist.

- [ ] Mission 00 — IGNITION
- [ ] Review report + repository
- [ ] Mission 01 — DNA
- [ ] Review deterministic/lock tests
- [ ] Mission 02 — SOUL
- [ ] Review Character output quality
- [ ] Mission 03 — CALLING
- [ ] Review Quest output quality
- [ ] Mission 04 — MEMORY
- [ ] Review persistence/canon/migration failure paths
- [ ] Mission 05 — ANVIL
- [ ] Manually exercise desktop core loop
- [ ] Mission 06 — VAULT
- [ ] Test project roundtrip + malicious archives
- [ ] Mission 07 — GAUNTLET
- [ ] Inspect seed/property/content quality report
- [ ] Mission 08 — ARMOR
- [ ] Review P0/P1 list; must be empty before release
- [ ] Mission 09 — FOUNDATION
- [ ] Independently inspect packaged artifacts/certification

# If a mission fails

Do not advance the mission number.

Rerun the same mission with:
- its previous report;
- current PROJECT_STATE;
- concrete failure evidence;
- an instruction to repair only blockers/defects needed for that mission's exit gate.

# If Codex starts scope-creeping

Stop it and restate:

> Later-version compatibility is an architecture constraint, not permission to implement later-version features.
