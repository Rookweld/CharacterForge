# HANDOFF PROTOCOL

Every mission ends with two artifacts:

1. `docs/missions/MISSION_XX_REPORT.md`
2. updated root `PROJECT_STATE.md`

# Mission report required format

## Mission
Name and number.

## Status
One of:
- PASS
- PARTIAL
- BLOCKED
- FAIL

## Scope delivered
Concrete implemented items only.

## Files materially changed
High-level list.

## Commands executed
Exact commands.

## Verification evidence
Tests/builds/smokes with outcomes.

## Architecture decisions
Only decisions made during this mission that future missions must know.

## Known defects
Severity + reproduction.

## Deferred items
Only items intentionally not done because they belong to later missions or are out of v1 scope.

## Repository state
- branch
- commit hash if committed
- dirty/clean status

## Next mission readiness
YES/NO, with blockers.

# PROJECT_STATE update

Update:
- current mission
- current status
- last verified commit
- schema/generator/content versions if changed
- completed gates
- blockers
- P0/P1 defects
- architecture decisions

Do not paste logs into PROJECT_STATE.

# STOP RULE

After writing the report/state:
**STOP.**

Do not start the next mission.
