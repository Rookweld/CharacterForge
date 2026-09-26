# Architecture

## Mission 00 baseline

The application is a local-first Windows desktop product. The desktop shell hosts a TypeScript user interface and delegates application operations to Rust. Rust owns application services, deterministic domain logic, and persistence boundaries. SQLite is the authoritative project store once persistence is implemented in its mission.

## Dependency direction

```text
React UI (presentation and user input)
    │ invokes explicit application commands
    ▼
Tauri desktop boundary
    │ delegates; does not own domain rules
    ▼
Rust application/core ─────► shared, versioned contracts
    │                              ▲
    ├──► generator/domain          │ content data (versioned)
    ├──► persistence (SQLite)      │
    └──► export                    │
```

The UI must not directly own persistence. Generator/domain logic must not depend on Tauri or visual components. Persistence must not know about visual components. Shared contracts describe data crossing boundaries; they do not become a miscellaneous utility module.

## Boundaries and seams

- **Desktop:** window lifecycle and narrow command transport.
- **Frontend:** rendering, accessible interaction, and transient view state.
- **Application/core:** use cases and orchestration, independent of UI components.
- **Domain/generator:** versioned deterministic rules, independent of Tauri and storage.
- **Persistence:** SQLite access behind application-facing interfaces; no UI dependencies.
- **Content:** declarative, versioned data consumed by domain logic.
- **Export:** explicit portable representations; it does not replace SQLite as the source of truth.

Future provider or expansion support may use these boundaries if later authorized. Mission 00 creates no provider runtime, plugin system, marketplace, or expansion implementation.

## Data and safety

SQLite remains the v1.0 project source of truth. Project/archive handling must validate inputs and preserve recoverability. Detailed schemas, migrations, generation contracts, and import/export behavior belong to their later missions and are intentionally not defined by this bootstrap.
