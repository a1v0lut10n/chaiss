# Scaffold Cargo Workspace and Foundational Crates

- `[x]` Phase 1: Workspace Initialization
  - `[x]` Create root `Cargo.toml`
  - `[x]` Create `.gitignore`
  - `[x]` Create `.env` file
- `[x]` Phase 2: Generating `core` Library
  - `[x]` Initialize new cargo `core` lib crate (renamed to chaiss_core to avoid shadowing)
  - `[x]` Add dependencies (`tokio`, `sqlx`, `serde`, `reqwest`)
  - `[x]` Scaffold module stubs (`engine.rs`, `llm.rs`, `db.rs`, `lib.rs`)
- `[x]` Phase 3: Generating `desktop` Binary
  - `[x]` Initialize new cargo `desktop` bin crate
  - `[x]` Add dependencies (`eframe`, `tokio`, path to `core`)
  - `[x]` Scaffold module stubs (`app.rs`, `ui.rs`, `main.rs`)
- `[x]` Phase 4: Foundational Verification
  - `[x]` Run `cargo check` workspace-wide
  - `[x]` Run `cargo run --bin desktop` to verify blank egui frame opens
