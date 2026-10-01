# EXECUTION CONTRACT

Execute exactly one bounded post-v1 The Forge mission.

Before coding:
1. Read GAZINGA/CANON_CORE.md.
2. Read GAZINGA/THE_FORGE_CANON_CORE.md.
3. Read GAZINGA/DISCOVERY_STATE.md.
4. Read GAZINGA/KNOWN_ISSUES.md.
5. Read this mission completely.
6. Read docs/design/VISUALS_AND_ENGINE_BOUNDARY.md completely.
7. Read the B9 Visual Forge foundation report and inspect crates/app-core/src/visual.rs.
8. Inspect the installed local ComfyUI source/API examples at:
   C:\Users\pontu\AppData\Local\Programs\ComfyUI\resources\ComfyUI
9. Inspect the actual repository before modifying provider boundaries.

Observed operator environment at mission preparation time:
- Comfy Desktop installed under C:\Users\pontu\AppData\Local\Programs\ComfyUI
- ComfyUI user/data root under C:\Users\pontu\Documents\ComfyUI
- last recorded ComfyUI core version: 0.22.2
- last recorded local server URL: http://127.0.0.1:8000
- GPU: NVIDIA RTX 3500 Ada Laptop GPU, 12 GB VRAM
- standard local model directories currently contain no discovered checkpoint/model files
- ComfyUI was not listening on port 8000 at preparation time
- local ComfyUI source exposes /system_stats, /models, /object_info, /prompt, /history/{prompt_id}, /view, /queue, /interrupt, and /ws

Treat those as environment observations, not hardcoded product assumptions.

During work:
- preserve all B9 provider-neutral Visual Forge contracts
- preserve all entity/relationship/Forge Further/Canon behavior
- preserve database schema 2
- preserve forgeproject v1
- preserve app version 1.0.0
- do not add OpenAI/fal/cloud provider code
- do not add API keys or credentials
- do not auto-download models
- do not install custom ComfyUI nodes
- do not persist generated images into Entity payloads
- do not implement Visual Canon
- do not change archive format
- do not silently start/kill the operator's ComfyUI process
- do not begin B11 or later work

At the end:
- run full verification
- create docs/missions/THE_FORGE_B10_COMFYUI_LOCAL_PROVIDER_REPORT.md
- update GAZINGA/DISCOVERY_STATE.md
- update GAZINGA/PROJECT_STATE.md only where material facts changed
- report PASS only if every automated exit gate is satisfied
- STOP

# THE FORGE B10 — LOCAL COMFYUI PROVIDER
## First real image-provider adapter

# Objective

Connect B9 Visual Generation Briefs to a local ComfyUI server through a narrow, provider-specific adapter.

The product flow becomes:

    saved Entity
      → B9 deterministic Visual Brief
      → ComfyUI Local adapter
      → discover local capabilities/models
      → compile bounded ComfyUI API workflow
      → queue generation
      → wait/poll for completion
      → fetch generated image bytes
      → show transient preview/result metadata

B10 is the first real image-generation path, but it deliberately stops before permanent Visual Asset storage or Visual Canon.

# Product behavior

On Visual Forge for a supported Entity:

1. Compile B9 brief.
2. Choose Provider = Local ComfyUI.
3. App checks ComfyUI health/capabilities.
4. App discovers compatible local checkpoint models.
5. User chooses a model and generation settings.
6. Generate Image.
7. The Forge sends a bounded workflow to local ComfyUI.
8. Progress/status is shown.
9. The resulting image is fetched from ComfyUI and shown in The Forge.
10. User may:
   - keep the transient preview during the current session
   - save/export the image explicitly to a user-chosen location
   - generate another variation
   - discard the transient result

No generated image becomes narrative Canon or Visual Canon automatically.

# Local-only network boundary

B10 is specifically the LOCAL ComfyUI adapter.

Accepted base URLs:
- http://127.0.0.1:<port>
- http://localhost:<port>
- http://[::1]:<port> where the HTTP library safely supports it

Default:
- http://127.0.0.1:8000

Requirements:
- reject non-loopback hosts
- reject https/http URLs pointing at arbitrary LAN/WAN hosts
- reject embedded credentials
- bounded port parsing
- no redirects to non-loopback destinations
- short connect/read timeouts
- bounded response sizes
- clear offline/unreachable state

This is not a general HTTP proxy.

# Provider contract

Implement a concrete ComfyUI adapter behind the B9 provider-neutral boundary.

The adapter owns:
- ComfyUI HTTP request/response DTOs
- health/capability discovery
- checkpoint discovery
- workflow JSON compilation
- prompt queueing
- job polling/history parsing
- output image retrieval
- provider error translation

The core VisualGenerationBrief must remain provider-neutral.

Do not leak ComfyUI node JSON into Entity contracts or B9 recipe contracts.

# ComfyUI capability discovery

Use local API endpoints already present in the installed source.

At minimum discover:
- server reachable
- system stats where available
- object/node availability needed by the workflow
- compatible checkpoint choices
- relevant sampler/scheduler choices if available through object_info

Prefer /object_info to validate required core nodes.

Required nodes for the v1 generic checkpoint workflow should use only built-in/core ComfyUI nodes where possible:
- CheckpointLoaderSimple
- CLIPTextEncode
- EmptyLatentImage
- KSampler
- VAEDecode
- SaveImage

If installed version uses different canonical names, inspect object_info and adapt narrowly.

No custom nodes are required for PASS.

# No-model state

The operator currently appears to have no model files in standard ComfyUI model directories.

This is a NORMAL supported state.

If no compatible checkpoint is discovered:
- provider status = reachable but no compatible model
- explain clearly: "ComfyUI is available, but no compatible local checkpoint was found."
- Generate is disabled
- do not download anything automatically
- do not fabricate a model
- Visual Brief preview/export remains fully usable

B10 may pass repository tests without a locally installed model, provided:
- adapter/workflow tests are complete
- no-model behavior is correct
- local health/discovery smoke is attempted when safely possible
- no false claim of successful native image generation is made

If a compatible model is present during execution, perform an opt-in-safe bounded local smoke without changing operator data unexpectedly. Do not download one solely to satisfy the gate.

# Workflow v1

Implement a minimal generic checkpoint text-to-image workflow.

Inputs:
- selected checkpoint
- positive prompt derived from B9 prompt_text
- negative/avoid prompt derived from B9 avoid list
- seed
- width
- height
- steps
- cfg
- sampler
- scheduler

Pipeline:
CheckpointLoaderSimple
→ positive CLIPTextEncode
→ negative CLIPTextEncode
→ EmptyLatentImage
→ KSampler
→ VAEDecode
→ SaveImage

Requirements:
- no custom nodes
- no arbitrary workflow execution supplied by user text
- no direct execution of imported workflow JSON in B10
- node IDs deterministic/stable inside adapter tests
- prompt text inserted only as data
- user/entity text cannot alter graph structure
- filename prefix sanitized and bounded
- output remains under ComfyUI's normal output mechanism until explicitly saved/exported by user

# Generation settings

Expose a deliberately small typed settings surface:

- model/checkpoint
- seed: u64 / bounded integer supported by ComfyUI
- width
- height
- steps
- cfg
- sampler
- scheduler

Defaults should be conservative and compatible with 12 GB VRAM, but do not hardcode model-specific claims.

Resolution:
- derive a suggested width/height from B9 aspect intent
- allow a bounded set/range
- protect against absurd pixel counts
- default should favor reliability over maximum resolution

Steps/CFG:
- bounded safe ranges
- no provider-specific "magic" hidden behind UI
- user-visible values

Variation:
- "Generate variation" may reuse same brief/model/settings with a new explicit seed
- no hidden randomness if determinism can be preserved; if a random seed button exists, the chosen seed must be surfaced before queueing and retained in result metadata

# Prompt mapping

Positive prompt:
- B9 deterministic prompt_text is source of truth
- adapter may add minimal structural syntax required by ComfyUI/model workflow, but may not invent fictional facts

Negative prompt:
- compile from B9 avoid list + minimal quality/safety exclusions if they are provider/workflow-level and documented
- do not silently add huge style-negative boilerplate

Provider adapter must never include B9 provenance IDs/hashes in creative prompt text.

# Job lifecycle

Represent explicit states:
- idle
- checking_provider
- unavailable
- no_model
- ready
- queued
- running
- completed
- failed
- canceled where supported

Queue generation via /prompt.

Completion:
- poll /history/{prompt_id} or use another bounded official local API path
- WebSocket support is optional in B10; polling is acceptable
- bounded polling interval and overall timeout
- cancellation may call /interrupt only if it is safe not to interrupt unrelated operator jobs; otherwise omit cancellation in v1 rather than globally interrupting another queue

IMPORTANT:
ComfyUI /interrupt can affect global execution. Do not expose a Cancel button unless the adapter can prove it is canceling only The Forge-owned active job or the product clearly warns about the global effect. Prefer no cancel in B10 if isolation is uncertain.

# Queue safety

ComfyUI may already be in use by the operator.

Requirements:
- never clear the global queue
- never delete history globally
- never call /free automatically
- never interrupt an unrelated job
- attach a unique client/request identifier where the API permits
- track The Forge's own prompt_id
- only fetch/result-handle the exact prompt_id submitted by The Forge

# Result contract

Create a provider result DTO separate from VisualAssetCandidate persistence.

At minimum:
- provider = comfyui_local
- provider version/server info if safely available
- source brief hash
- prompt_id
- selected checkpoint
- seed
- effective width/height/steps/cfg/sampler/scheduler
- generated output descriptor: filename/subfolder/type
- image MIME type
- image byte length
- content SHA-256 or BLAKE3 computed by The Forge
- completed timestamp
- warnings

The image bytes may live transiently in memory/session state for preview.

Do NOT create a database Visual Asset row in B10.

# Image fetch and preview

Fetch only the exact output descriptor returned by the owned prompt history through /view.

Requirements:
- bounded maximum image response size
- accept only expected image MIME types
- reject path traversal/manipulated descriptors
- do not allow arbitrary filesystem reads
- render image from trusted in-memory/temporary app representation
- do not treat image metadata as authoritative Entity truth

If a temporary file is required by Tauri/browser limitations:
- use an application-owned temp directory
- unique bounded filenames
- clean up best-effort
- never write into project source tree
- document exact behavior

# Explicit Save Image

Allow user to save/export the completed image through a user-chosen safe path using existing desktop save patterns.

This is an ordinary exported file, not authoritative project persistence.

Suggested filename:
<entity-name>-<visual-role>-<seed>.<ext>

Sanitize safely.

Do not modify .forgeproject archive.

# Stale source behavior

B9 brief already records exact source state.

Before queueing:
- compare current saved Entity write_version to the brief source write_version
- if stale, block generation by default and ask user to recompile the Visual Brief
- do not silently generate from an old brief while labeling it current

After generation:
- if Entity changes while the image is being generated, result remains truthful to the original brief
- display "Generated from an older source state" or equivalent when detected
- do not discard or mutate the result automatically

# Desktop UX

Extend B9 Visual Forge rather than redesigning it.

Provider section:
- Local ComfyUI
- server address (default loopback:8000; loopback only)
- Check Connection
- provider state
- discovered model dropdown
- basic generation settings
- Generate Image when ready

Result:
- status/progress text
- image preview
- exact seed/model/settings
- source stale/current indicator
- Save Image
- Generate Variation

When ComfyUI is offline:
- clear local instructions, e.g. "Start ComfyUI, then Check Connection."
- no crash
- brief compile/export remains available

When no model:
- clear no-compatible-model state
- no fake Generate action

# Tauri/backend architecture

Provider HTTP/network work belongs in Rust/Tauri/provider adapter layer, not React fetch calls.

Frontend receives typed commands/results only.

Suggested commands (names may follow repo conventions):
- comfyui_status
- comfyui_models
- comfyui_generate
- comfyui_generation_status
- comfyui_result
- save_visual_image

Avoid one giant untyped JSON command.

No background task should outlive application/session ownership without explicit lifecycle handling.

# Dependencies

Prefer existing HTTP/runtime capabilities if available.

If a new Rust HTTP dependency is truly required:
- choose a narrow mature dependency
- document why
- use minimal features
- no TLS needed for loopback-only HTTP if implementation can remain HTTP
- update lockfile intentionally
- run dependency/security review appropriate to the repo

Do not install global tools or Python packages.

# Tests — adapter

At minimum:
- loopback URL validation
- rejects WAN/LAN/credential URLs
- connection timeout/error mapping
- health/system_stats parsing
- required node discovery
- checkpoint model discovery
- no-model state
- deterministic workflow compilation
- user prompt cannot alter graph
- exact model/settings mapping
- resolution/steps/cfg bounds
- positive/negative prompt mapping
- queue response parsing
- exact owned prompt_id tracking
- history success parsing
- ComfyUI error response parsing
- output descriptor validation
- /view request generation safe
- response size/MIME bounds
- content hash deterministic
- no global queue clear/free/history delete
- no accidental /interrupt usage unless explicitly justified/tested

Use a local mock HTTP server in tests; no real ComfyUI/model required for automated suite.

# Tests — integration with B9

At minimum:
- Character Portrait brief → Comfy workflow
- Location Establishing Shot → workflow
- Faction Emblem → workflow
- Item Item Concept → workflow
- Creature Creature Concept → workflow
- B9 prompt text preserved as positive semantic content
- avoid list mapped to negative prompt
- stale brief blocked before queue
- entity change during generation marks result stale rather than corrupting it
- Quest remains unsupported

# Tests — frontend

At minimum:
- ComfyUI offline state
- reachable/no-model state
- ready/model dropdown state
- Generate disabled until brief + provider + model ready
- settings validation
- queued/running/completed/failed rendering
- preview shown from completed owned result
- Save Image path
- Generate Variation changes surfaced seed
- stale-before-generation blocked
- stale-after-generation warning
- no provider result mutates Canon or Entity fields

# Real local smoke

Because the operator already has ComfyUI installed, attempt a non-destructive local smoke if possible:

1. Do NOT start ComfyUI automatically.
2. If server is already listening, query /system_stats and /object_info.
3. Discover models.
4. If no model is installed, record provider reachable/no-model and STOP there.
5. If a compatible model is installed, ask no additional product question; generate one bounded low-resolution test from a disposable Visual Brief only if doing so will not overwrite operator content or disrupt an existing queue.
6. Never claim image-generation smoke PASS when no actual image was produced.

Automated mission PASS does not require an installed model.

# Full verification

Run:
- frontend format
- frontend lint
- frontend typecheck
- frontend tests
- frontend production build
- cargo fmt --all -- --check
- strict Clippy -D warnings
- cargo test --workspace --no-fail-fast
- 1,000-sequence gauntlet
- Cargo metadata
- scripts/verify.ps1
- optimized Tauri build --no-bundle
- git diff --check

# Human/product decision boundary after B10

B10 must STOP before:
- auto-downloading a model
- choosing a permanent default model for all users
- installing custom nodes
- persisted Visual Asset library/database
- Visual Canon/selected visual
- forgeproject v2/assets
- reference-image/identity workflows
- ControlNet/IP-Adapter
- LoRA management
- cloud fallback
- remote ComfyUI hosts

Those are later bounded choices.

# Exit gate

PASS only if:
- B9 remains provider-neutral
- Local ComfyUI adapter exists
- loopback-only connection security exists
- model/node discovery exists
- no-model/offline states are first-class
- deterministic core-node workflow compiles safely
- exact The Forge prompt_id lifecycle is tracked
- completed images can be fetched safely and previewed transiently
- user can explicitly Save Image
- stale-source semantics are preserved
- no Entity/Canon mutation occurs
- no image DB persistence/schema/archive bump occurs
- no model/custom node automatically installed
- full verification green
- no known B10 P0/P1/P2 remains

STOP.
