# THE FORGE — OPERATING MODEL

## Purpose

Define the post-v1 workflow for Rook/ChatGPT, Nullfeather Bridge, Codex, and the repository.

The goal is to stop using chat transcripts as the primary project memory.

## Roles

### Rook / ChatGPT

Primary responsibilities:

- product discovery
- architecture reasoning
- decision framing
- canonical design maintenance
- mission design
- mission review
- diff/report review through Nullfeather Bridge
- acceptance-gate judgment

Rook should not perform large multi-file implementation work through the Bridge merely because file-write access exists.

Small, precise repository maintenance is appropriate.

### Nullfeather Bridge

The Bridge is the local bounded access path into Fury.

Use it for:

- reading real repository files
- inspecting Git state
- writing canonical documentation
- writing bounded mission specs
- reviewing implementation diffs
- running bounded commands/tests
- inspecting screenshots
- verifying actual repository state

The Bridge is not a replacement for disciplined missions.

### Codex

Codex is the primary heavy implementation worker.

Use it for tasks such as:

- multi-file Rust implementation
- SQLite migrations
- larger React/Tauri implementation
- broad refactors
- test suite expansion
- release engineering
- bounded remediation missions

Codex receives a repository mission file instead of a giant chat paste.

### Human operator

The human operator owns:

- product decisions
- explicit approval gates
- manual UI/operator smoke where required
- starting Codex missions
- deciding whether to commit/tag/publish
- resolving product-level OPEN questions

## Canonical State Files

### Foundation baseline

GAZINGA/PROJECT_STATE.md

Historical/current FOUNDATION execution state.

### Post-v1 discovery and implementation

GAZINGA/DISCOVERY_STATE.md

This is the compact live handoff for The Forge expansion.

It should remain short.

### Always-loaded constitutions

For post-v1 work:

1. GAZINGA/CANON_CORE.md
2. GAZINGA/THE_FORGE_CANON_CORE.md

The first protects existing v1 contracts.
The second defines post-v1 expansion rules.

## Canonical Design Files

Located in docs/design.

The design index is:

docs/design/THE_FORGE_DESIGN_INDEX.md

Do not require every agent to read every design file on every mission.

Mission files should name the exact design sections required.

## Workflow

### Discovery

    conversation
      ↓
    architecture/product decision
      ↓
    update canonical design document
      ↓
    update DISCOVERY_STATE

Do not let accepted decisions live only in chat.

### Implementation

    canonical design
      ↓
    bounded mission file in repository
      ↓
    Codex reads current repo + required canon
      ↓
    Codex implements only mission scope
      ↓
    tests/evidence
      ↓
    mission report
      ↓
    Rook reviews actual repo/report through Bridge
      ↓
    PASS / PARTIAL / remediation

### Handoff

A new ChatGPT conversation should be able to resume with:

    Connect to Nullfeather Bridge.
    Read C:\dev\CharacterForge\GAZINGA\DISCOVERY_STATE.md
    and continue the current The Forge work.

Rook then reads only the canonical documents needed for the current Next Action.

## Mission Naming

The FOUNDATION numbered sequence 00–09 is complete.

Do not invent Mission 10 merely because 09 ended.

Post-v1 expansion work uses a separate bounded series until a formal release/version mission plan is approved.

Current working convention:

- THE_FORGE_A_SUBSTRATE
- THE_FORGE_B_PLACE
- THE_FORGE_C_SOCIETY
- THE_FORGE_D_RELICS
- THE_FORGE_E_BEASTS
- THE_FORGE_F_WEB

These names are provisional planning labels, not release promises.

## Commit Discipline

Before starting a new implementation mission:

- verify branch/HEAD
- inspect worktree
- do not mix previous release artifacts/logs/operator exports into the mission commit
- avoid git add -A when unrelated untracked files exist
- stage only intended mission files

Design/docs commits should remain separable from implementation commits where practical.

## Review Discipline

Do not accept a mission report at face value when the Bridge can inspect the repository.

Review should normally include:

- mission report
- git status
- git diff / changed-file list
- relevant code/contracts
- test evidence
- migration/data-safety evidence when applicable

For high-risk storage work, require fixture-backed evidence rather than only unit-level happy paths.

## Tooling Failure Rule

If Nullfeather Bridge temporarily disconnects or times out:

- do not invent repository state
- reconnect/health-check
- resume from actual files
- report any unverified step as unverified

## Scope Rule

Architecture foresight is not permission for scope creep.

A mission implements only what its exit gate requires.

Later capabilities can be documented as DEFERRED without receiving placeholder runtime code.
