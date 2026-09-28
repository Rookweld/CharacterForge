# Security and privacy notes

Character Forge stores projects locally in SQLite and `.forgeproject` archives. Imported project files and user-entered paths are treated as untrusted input and validated by Rust before use. Canonical project prose is structured data; the frontend renders it as ordinary React text. Markdown export intentionally preserves user-authored Markdown and may be interpreted by the viewer chosen by the user.

The application does not include a production shell, process, updater, external opener, file-dialog, network, analytics, crash-upload, or telemetry plugin. No application-controlled HTTP/WebSocket/EventSource path, remote runtime asset, or frontend `fetch` was found. The Tauri development URL is loopback-only and is not the production frontend source; packaged builds use the local Vite output. The operator verified the packaged app's core workflows with network disconnected. Operating-system/WebView networking is not disabled or covered by this application-level statement.

No persistent production log file is currently configured, so there is no application log-folder location. Recent-project settings retain up to eight project display names, paths, and last-opened timestamps in the per-user app-config directory. They do not retain entity prose. Structured errors are returned to the local UI; the application does not upload them.

The complete trust-boundary review, accepted assumptions, and remaining offline/advisory gates are in [THREAT_MODEL_V1.md](security/THREAT_MODEL_V1.md), [ARMOR_FINDINGS.md](security/ARMOR_FINDINGS.md), [DEPENDENCY_REVIEW_V1.md](security/DEPENDENCY_REVIEW_V1.md), and [MISSION_08_REPORT.md](missions/MISSION_08_REPORT.md).
