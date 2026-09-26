# GAZINGA SPLIT EDITION
## Character Forge / Quest Forge — v1.0 FOUNDATION

**Purpose:** Build the full v1.0 FOUNDATION release in bounded, auditable Codex missions without feeding the entire project bible and monolithic build prompt into every session.

This pack replaces the "one gigantic autonomous run" execution model. The original World Bible and monolithic GAZINGA prompt remain canonical design references, but implementation is now performed in discrete missions.

---

# THE OPERATING MODEL

Every Codex session receives only:

1. `CANON_CORE.md`
2. `PROJECT_STATE.md`
3. the **current mission file**
4. the existing repository
5. optionally the specific World Bible sections named in `WORLD_BIBLE_LOOKUP_MAP.md`

Do **not** paste the entire World Bible into every session.

Codex must inspect the repository itself, update `PROJECT_STATE.md`, execute the current mission, verify its exit gate, and stop.

The next mission starts only after the previous mission's gate is independently credible.

---

# MISSION ORDER

| Mission | Codename | Purpose |
|---|---|---|
| 00 | IGNITION | Preflight, repo bootstrap, architecture lock |
| 01 | DNA | Domain contracts + deterministic Forge DNA |
| 02 | SOUL | Character Forge vertical slice |
| 03 | CALLING | Quest Forge vertical slice |
| 04 | MEMORY | SQLite persistence, history, canon, revisions |
| 05 | ANVIL | Desktop UX + Tauri integration |
| 06 | VAULT | Export, import, backup, `.forgeproject`, security |
| 07 | GAUNTLET | Property tests, seed sweeps, quality validation |
| 08 | ARMOR | Hardening, diagnostics, accessibility, performance |
| 09 | FOUNDATION | Packaging, smoke test, release certification |

Do not combine missions because one appears "easy." Boundaries exist to prevent hidden scope creep and to make failures diagnosable.

---

# ABSOLUTE RULES

- v1.0 FOUNDATION only.
- No cloud sync.
- No accounts.
- No multiplayer/collaboration.
- No marketplace.
- No World Forge.
- No AI provider dependency.
- No vector database.
- No expansion implementation.
- No "while I'm here" architecture rewrites.
- No claiming PASS without command output or artifact evidence.
- No editing locked/generated fields through hidden side effects.
- No silent data loss.
- No destructive migrations without backup.

The future architecture must remain possible, but future features must remain unimplemented.

---

# HOW TO RUN A MISSION

Start a fresh Codex task/session where practical.

Prompt:

> Execute the attached/current GAZINGA mission exactly.  
> Read `CANON_CORE.md`, `PROJECT_STATE.md`, and the mission file first.  
> Read only the World Bible sections referenced by the mission unless repository evidence requires more.  
> Work autonomously within the mission boundary.  
> Do not start the next mission.  
> Do not report PASS unless every exit criterion is verified.  
> Update `PROJECT_STATE.md` and create/update the required mission report before stopping.

Then let Codex work.

When it stops:

- inspect the mission report;
- inspect changed files;
- inspect test output;
- resolve any `BLOCKED` or `PARTIAL`;
- only then start the next mission.

---

# HANDOFF PHILOSOPHY

The project should never depend on Codex "remembering" prior sessions.

State lives in the repository.

`PROJECT_STATE.md` is the minimal cross-session memory. It must remain concise and factual.

Every mission also produces:

`docs/missions/MISSION_XX_REPORT.md`

These reports preserve evidence without bloating the state file.

---

# SUCCESS CONDITION

At the end of Mission 09, v1.0 FOUNDATION must exist as a real Windows desktop release with:

- Character Forge
- Quest Forge
- deterministic Forge DNA
- selective lock/reroll
- universe presets and creative axes
- project persistence
- history/revisions/canon/favorites
- import/export/backup
- release-grade safety
- mandatory seed/property tests
- packaged Windows artifacts
- final release certification

Anything less is not FOUNDATION.
