# EXECUTION CONTRACT

You are executing exactly one bounded GAZINGA mission.

Before coding:
1. Read `CANON_CORE.md`.
2. Read `PROJECT_STATE.md`.
3. Read this mission completely.
4. Read only the World Bible/spec sections mapped to this mission.
5. Inspect the existing repository and current git status.

During work:
- stay inside mission scope;
- preserve existing passing behavior;
- add tests with implementation;
- prefer explicit contracts to implicit coupling;
- document architecture decisions that materially affect later work;
- do not implement future-version features;
- do not hide failures;
- do not weaken tests to make them pass.

At the end:
- run every required verification command;
- create `docs/missions/MISSION_XX_REPORT.md`;
- update `PROJECT_STATE.md`;
- report PASS only if every exit gate is satisfied;
- STOP without starting the next mission.

# MISSION 07 — GAUNTLET
## Property tests, seed sweeps, quality validation

# Objective

Try to break the product before users do.

This mission is verification-led. Fix defects discovered here, but do not add unrelated features.

# Mandatory deterministic seed sweeps

Run at least:
- **1,000 Character seeds**
- **1,000 Quest seeds**

Across presets/axes sufficiently to detect structural and content pathologies.

Record:
- required-field failures
- panics/errors
- invalid enum/state
- obvious template corruption
- severe repetition indicators
- performance distribution

# Lock invariant gauntlet

Property/stress test:
1. generate candidate;
2. lock random subset;
3. repeatedly reroll random unlocked fields;
4. assert locked fields remain unchanged;
5. serialize/reload;
6. repeat;
7. assert provenance/nonces obey rules.

Test Character and Quest.

Any hidden mutation of a locked unrelated field is P1.

# Reproducibility

Golden seeds must reproduce for the same compatible:
- generator version
- content version
- preset
- axes
- DNA

If a deliberate algorithm/content change alters goldens, versioning must reflect it. Do not simply overwrite expectations without explanation.

# Content quality matrix

Inspect representative outputs for every preset and:
- low/high structure
- low/high weirdness
- low/high darkness/tone axes as defined
- Chaos Blend

Check:
- concrete nouns
- grammatical composition
- semantic coherence
- variety
- universe identity
- axis influence
- repeated clichés
- contradictions that break usability

Create `docs/CONTENT_QUALITY_REPORT.md`.

# Persistence/failure regression

Re-run critical:
- migration
- restart
- backup
- import
- corrupt input
- transaction rollback
- canon preservation

# EXIT GATE

PASS only if mandatory seed/property gates pass and content quality is credible enough that FOUNDATION is not merely technically correct.
