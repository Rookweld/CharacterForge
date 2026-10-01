# THE FORGE B10 — Local ComfyUI Provider report

## Exit status

**PASS — B10 automated exit gates satisfied. STOP.**

Executed 2026-10-01 using `GAZINGA/THE_FORGE_CODEX_LAUNCHER_PROMPT.txt` and the complete B10 contract. Verified baseline: branch `master`, HEAD `2ad9eb56957d29ca51aa3dc8ee45698127301d6d`, accepted B9 implementation `9284c08`, only four unrelated untracked files. No reset/stage/commit/tag/push/publication. State docs had stale B9/preparation HEAD text and a literal backtick-r/backtick-n line separator; live facts and that separator were repaired.

## Implemented provider boundary

- B9 core, recipes, context/brief contracts and image-provider trait are unchanged. Concrete `ComfyAdapter` implements the existing trait in the desktop Rust provider module, using opaque provider-owned Settings/Request/Job/Error DTOs. Trait generation returns an owned queued job; polling is explicit.
- Closed typed settings: checkpoint, seed, width, height, steps, CFG, sampler, scheduler. Conservative suggestions: portrait 512×768, landscape 768×512, square 512×512; 20 steps, CFG 7, Euler/normal if discovered. These are generic workflow settings, not promises of model-specific suitability.
- Backend validates seed 0–9007199254740991 (the exact JavaScript integer range within ComfyUI u64 support), dimensions 256–1024 in multiples of 64 and ≤786432 pixels, steps 1–40, finite CFG 1–15, bounded safe discovered model/sampler/scheduler names. Discovery and settings membership are checked again at submission.
- Fixed seven-node API graph: CheckpointLoaderSimple → two CLIPTextEncode nodes → EmptyLatentImage → KSampler → VAEDecode → SaveImage. Stable node IDs 1–7, batch size 1, denoise 1. Positive text is the exact B9 prompt; negative text joins the B9 avoid list without added boilerplate. Role/direction remain B9 facts, never provider-specific domain fields. All operator/Entity text is input data; no user/imported graph can be executed.
- SaveImage prefix is constant bounded ASCII `TheForge`, with no entity-controlled path or dynamic prefix syntax. ComfyUI uses its ordinary output directory/counter; The Forge never manipulates that directory. Local PNG metadata may contain the submitted visible workflow, as in normal ComfyUI SaveImage behavior; private Entity prose/provenance IDs/hashes are not submitted as creative text.
- Required nodes/choices are read from individual `/object_info/{node}` endpoints, avoiding the entire custom-node registry. Version is bounded from `/system_stats`; checkpoint choices come from CheckpointLoaderSimple, samplers/schedulers from KSampler. Missing nodes, offline/unreachable and reachable/no-compatible-model states disable generation and leave B9 brief use/export available. Checkpoint choices are loader-advertised `.safetensors`/`.ckpt` files, not a guarantee that every discovered architecture supports this generic MODEL/CLIP/VAE workflow; execution failures are reported honestly.

## Local HTTP security

- Only exact `http://127.0.0.1:<1..65535>`, `http://localhost:<port>` and `http://[::1]:<port>` are accepted, with an optional single trailing slash. `localhost` is pinned to 127.0.0.1, avoiding DNS/hosts-file remapping. LAN/WAN, HTTPS, credentials, paths, fragments, queries, alternate numeric hosts and malformed ports are rejected.
- Mature pinned `ureq = 3.3.0`, default features disabled; the operator explicitly approved this new production dependency. No TLS, gzip, cookies, SOCKS or platform proxy features are enabled. Proxy is explicitly None and redirects are disabled, including redirects back to loopback. Non-success statuses are rejected. No frontend fetch/network connection or remote image URL is used.
- Two-second connect and ten-second global per-request deadlines. JSON responses ≤2 MiB, submitted workflow ≤128 KiB, image ≤8 MiB. Discovery uses seven bounded requests; no indefinite background loop. The app polls about every 1.5 seconds, backend throttles to at least one second, and an owned generation has a five-minute overall deadline. Timeouts/errors use fixed safe messages rather than echoing server errors/project prose.
- PNG MIME only; PNG signature/IHDR and exact effective width/height are checked before preview. Browser decoding handles the remaining PNG representation. Output descriptor must be exactly the owned history's single node-7 SaveImage output, type `output`, bounded safe filename ending `.png` and bounded relative subfolder. Traversal, absolute/drive paths, query/control characters, alternate file types and input/temp descriptor types reject. `/view` is compiled from this validated descriptor only.

## Job, source and session ownership

- Each submission gets a unique TheForge client identifier and tracks the exact returned bounded `prompt_id`. No frontend-supplied workflow/image descriptor is accepted. Queueing requires the authoritative cached B9 brief identity and actual current saved source version/body hash, and source is rechecked after discovery immediately before submission.
- Explicit queued/running/completed/failed states. Polling reads only the owned `/history/{prompt_id}` result. Read-only `/queue` distinguishes the owned ID in `queue_running` from waiting; unrelated queue data is never returned to UI or retained as result metadata. No queue/history clearing, `/free`, `/interrupt` or Cancel action exists.
- Result metadata is independent of future VisualAssetCandidate persistence: provider/version, brief hash, prompt ID, exact checkpoint/seed/effective settings, validated output descriptor, MIME, byte length, content BLAKE3, completion timestamp and warnings. Hash is computed by The Forge over exact received bytes.
- One transient owned job/result per active project session. Rust retains bounded bytes; UI renders a data URL under existing `img-src data:` CSP. No app-owned temporary image file/directory is needed. The same entity view can restore its cached owned result and resume explicit polling. New generation replaces the single active preview; explicit discard releases it. Project create/open/import/restore/close clears transient ownership. Leaving the view stops UI polling; in-flight HTTP is bounded and there is no detached generation worker. A queued ComfyUI job can still finish in ComfyUI after app/view/session closure or timeout; no global interrupt is used.
- Source changes during/after generation flag the old result as stale without relabeling, mutating or deleting its original brief/result. Current saved-state checks read storage; no Entity/Relationship/history/Canon write occurs. Historic B9 source snapshots remain explicit; no inferred Canon revision number.
- Save Image requires the owned completed bytes and explicit operator-entered `.png` path. Fixed suggested filename `visual-<seed>.png` is independent of Entity prose. Existing atomic create-new/no-clobber writer syncs/finalizes, preserves existing destinations and cleans failed staging files. This is an ordinary external export, never project persistence or archive membership.

## Desktop UX and automated evidence

Local ComfyUI provider section extends B9: address, Check Connection, discovery state, model dropdown, visible seed/resolution/steps/CFG/sampler/scheduler, Generate Image, owned status, PNG preview, exact result metadata, stale warning, Save Image, explicit next-seed Variation and discard. Brief compilation/export remains usable offline. No model is invented or downloaded. Settings validation, source checks, busy guards and unmount/late-response handling are exercised. There is no Canon/Entity mutation action in this flow.

Explicit final workspace `cargo test --workspace --no-fail-fast --offline`: **233 Rust tests PASS**, one existing fixture writer ignored (desktop 38, core 107, contracts 17, export 26, storage 45). The 1,000-sequence / 32,000-operation gauntlet passes. **122 frontend tests PASS**. Strict all-target Clippy, Rust formatting and full offline Cargo metadata pass. Final `scripts/verify.ps1` completed with exit 0, including frontend format/lint/typecheck/tests/production build, Rust format, strict Clippy, complete workspace suite/gauntlet and optimized Tauri `build --no-bundle`. `git diff --check` PASS. Explicit no-fail-fast workspace and metadata evidence above are additional completed gates.

Exact isolated executable: `C:\dev\CharacterForge\target\mission-b10-verify\release\character-forge-desktop.exe`; **15,979,520 bytes**; ProductVersion/FileVersion **1.0.0**; SHA-256 **`6B5655E2CEB30E4CE2DC2A75A9C1E0CD8E5B6BE82353810EE74CBF3DFA5C7B3C`**. This no-bundle build does not replace the earlier portable ZIP or default executable.

B10 adds sixteen adapter tests plus one real saved-source/export command-helper test, and fifteen frontend tests. Adapter tests use bounded local TCP HTTP servers and a small authored valid 512×512 PNG fixture, not a real image model. Coverage includes strict authority/redirect rejection, timeouts/offline, discovery/no-model/missing nodes, deterministic fixed workflow, five-family B9 mapping, prompt-as-data injection resistance, settings bounds, owned queue/history/error parsing, descriptor/MIME/image/JSON limits, exact image hash, no destructive/global endpoints, stale-before-submission rejection, stale-result retention and no-clobber export with unchanged project rows/history.

Initial test setup issues were repaired: Windows accepted mock sockets inherited nonblocking behavior, an added read-only queue probe changed a request index, B9 assertions needed to expect disabled generation in B10, duplicate alert semantics and effect dependencies were tightened, and Clippy required `is_multiple_of`. Final passing reruns supersede those preliminary failures; no known B10 P0/P1/P2 remains.

`npm test` was attempted after TypeScript changes, but the existing machine launcher cannot find `C:\Users\pontu\AppData\Roaming\npm\node_modules\npm\bin\npm-cli.js`. No npm PASS is claimed; configured Corepack/pnpm 11.25.0 suites pass. No global tooling installed.

Ignored local evidence: `target/b10-adapter-tests.log`, `b10-workspace-tests-final.log`, `b10-frontend-tests-final.log`, `b10-clippy.log`, `b10-cargo-metadata.json`, `b10-ureq-features.txt`, `b10-final-verify.log`, `b10-npm-test.log`, `b10-local-smoke.log`. Isolated target `C:\dev\CharacterForge\target\mission-b10-verify`.

## Real local smoke — separate from mocks

Installed ComfyUI `server.py` and built-in `nodes.py` were inspected read-only at `C:\Users\pontu\AppData\Local\Programs\ComfyUI\resources\ComfyUI`. Their current route/node definitions support the bounded API graph and queue/history/output handling. Initial ordinary reads were denied; approved read-only sandbox escalation allowed the mission-required source inspection.

An explicit non-destructive `GET http://127.0.0.1:8000/system_stats` with a three-second deadline, no proxy and no redirects was attempted. Result: **offline/unreachable — connection actively refused**. No server was started/killed; no queue/image submission was made. Current live node/model availability could not be observed. Reachable/no-model and complete image paths are verified with local mocks only. **No native image-generation smoke PASS or independent operator GUI acceptance is claimed.** No model/custom node was downloaded/installed.

## Dependency review and compatibility

The approved HTTP addition changes only the desktop dependency and intentional Cargo lock entries: ureq 3.3.0, ureq-proto 0.6.0 and utf8-zero 0.8.1; base64/log/percent-encoding/http/httparse were already locked. Enabled features and pinned source/configuration were reviewed. The official [ureq documentation](https://docs.rs/ureq/3.3.0/ureq/) and installed registry source describe bounded body limits, timeouts, redirect and proxy settings. Available [RustSec advisory search](https://rustsec.org/advisories/) did not identify a confirmed applicable new advisory; this is a bounded review, not an automated clean transitive-graph scan.

Mission 08's human-accepted incomplete automated transitive advisory scan remains residual maintenance/release verification risk, not a confirmed vulnerability. No cargo-audit/cargo-deny/global tooling installed. Existing P3 UI-001 remains open pending native operator check. Schema **2**, forgeproject **v1**, application **1.0.0**, existing generators/content/replay, storage/Canon/relationships and B2/B6/B7/B8 implementations are unchanged. FOUNDATION remains unsigned/WebView2-required, with no new MSI/NSIS or release-package claim.

Security notes and the dependency review receive factual B10 addenda; Mission 08 baseline evidence/limitations are preserved.

No real cloud/remote provider, credentials, auto-download, custom node, binary Visual Asset DB persistence, Visual Canon, forgeproject v2, Link Existing, multi-source Forge, reference image/ControlNet/IP-Adapter/LoRA management or later mission is implemented.

## Git ownership and preservation

Branch `master`, HEAD `2ad9eb56957d29ca51aa3dc8ee45698127301d6d` unchanged, index empty. Seventeen mission-owned files remain unstaged/uncommitted:

- `Cargo.lock`
- `GAZINGA/DISCOVERY_STATE.md`, `GAZINGA/PROJECT_STATE.md`
- `apps/desktop/src-tauri/Cargo.toml`, `apps/desktop/src-tauri/src/lib.rs`, new `apps/desktop/src-tauri/src/comfyui.rs`
- new test-only `apps/desktop/src-tauri/tests/fixtures/b10.png` (1,881 bytes, not a product Visual Asset)
- `apps/desktop/src/VisualForge.tsx`, `apps/desktop/src/api.ts`, new `apps/desktop/src/ComfyProvider.tsx`
- `apps/desktop/tests/VisualForge.test.tsx`, new `apps/desktop/tests/ComfyProvider.test.tsx`, new `apps/desktop/tests/visualFixture.ts`
- `crates/export/src/lib.rs`
- `docs/SECURITY.md`, `docs/security/DEPENDENCY_REVIEW_V1.md`, this report

Final `git diff --check` PASS; all four preserved SHA-256 values match baseline. Four unrelated files remain untracked and byte-identical:

| File | SHA-256 |
| --- | --- |
| `The Ashen Claim.json` | `32BC3B62555F32A7AB4A8CA936B52F4979305C08027937274E636E6E8033A844` |
| `The Ashen Claim.md` | `A298F7A2DAFDFB4AC4FE354607C67CB565B0CFD38D9D45D9085D1AAA8FEDFC0D` |
| `mission09-final-verify.log` | `958E927350F940EA6075A0E2A4452E922A1C780ADF9EE97052EC59C3DD8FE6AD` |
| `mission09-rust-test.log` | `8E16F7D11EF7432BB341CC22512D59A384321AAD0CFFCF10F9133B315234F0D2` |

## Exit-gate re-evaluation

| Mission criterion | Result / evidence |
| --- | --- |
| B9 provider-neutral | PASS — no core/recipe contract diff; adapter implements existing interface |
| Local ComfyUI adapter | PASS — typed Rust HTTP/discovery/workflow/queue/poll/image adapter |
| Loopback-only connection security | PASS — authority validation, numeric pinning, no proxy/redirects, timeout/size tests |
| Model/node discovery | PASS — installed source/API reviewed; mocked six-node choices and version parsing |
| No-model/offline first-class states | PASS — backend/UI regressions; actual local connection refusal recorded |
| Deterministic safe core-node workflow | PASS — fixed seven-node graph, input-only text, five-family mapping, bounded settings |
| Exact owned prompt lifecycle | PASS — client ID, returned prompt ID, owned running queue/history/output handling |
| Safe transient image fetch/preview | PASS — descriptor/MIME/header/size checks, exact-byte hash and memory data URL; mock evidence only |
| Explicit Save Image | PASS — owned bytes, operator path and atomic no-clobber readback regression |
| Stale-source semantics | PASS — actual store checks before submission and truthful retained stale result afterward |
| No Entity/Canon mutation | PASS — source/store/history/relationship readback plus existing full lifecycle regressions |
| No image DB/schema/archive bump | PASS — session only; schema 2/archive v1/app 1.0.0 unchanged |
| No model/custom node automatically installed | PASS — no download/install/start/kill code or execution |
| Full verification green | PASS — 233 Rust / 122 frontend; all required gates above |
| No known B10 P0/P1/P2 | PASS — none identified within reviewed/automated scope; native image smoke is unverified |

**STOP at B10.** No B11+ work, provider/model install, permanent visual persistence or cloud/remote support begins.

## Optional native operator follow-up

1. Launch the exact isolated B10 executable recorded below, open a disposable project and compile a supported saved Entity's Visual Brief.
2. Start the already-installed ComfyUI manually if desired; Check Connection. Verify offline/no-model behavior first. A compatible checkpoint must already be available; this app will not install one.
3. With a compatible core checkpoint and a safe existing queue, use one conservative generation. Verify exact seed/model/settings, preview, explicit PNG export/no-clobber, variation seed, stale-source warning and no Entity/Canon mutation. Native GPU/model reproducibility and operator GUI acceptance remain unverified until actually performed.
