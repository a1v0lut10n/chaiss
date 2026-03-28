# Implementation Plan: Chaiss Initial Scaffolding

This document outlines the systematic steps required to physically structure the repository according to the polyglot monorepo design, initializing the Rust workspace, creating the core libraries, and configuring the base dependencies.

## Phase 1: Workspace Initialization
1. Ensure the root directory `chaiss/` contains a workspace-level `Cargo.toml`.
2. Define the workspace members: `["core", "desktop"]`.
3. Set up a comprehensive `.gitignore` in the root to omit `target/`, local databases (e.g., `chaiss.db`), and `.env` files.

## Phase 2: Generating the `core` Library
1. Run `cargo new core --lib` to generate the headless game logic crate.
2. Update `core/Cargo.toml` with fundamental dependencies: 
   - `serde` and `serde_json` (for serialization and LLM data passing).
   - `sqlx` (with `sqlite` and `runtime-tokio` features) for data persistence.
   - The `llm` crate (or relevant HTTP client like `reqwest` if prioritizing external APIs) for frontier model interactions.
3. Scaffold empty module files corresponding to the architecture:
   - `core/src/engine.rs` (Chess logic)
   - `core/src/llm.rs` (Prompt management)
   - `core/src/db.rs` (SQLite operations)
   - `core/src/lib.rs` (Exposing modules)

## Phase 3: Generating the `desktop` Binary
1. Run `cargo new desktop --bin` to generate the native application crate.
2. Update `desktop/Cargo.toml` with presentation and async dependencies:
   - `eframe` / `egui` for the immediate-mode UI.
   - `tokio` (with full features) as the asynchronous runtime.
   - Add a local path dependency to the `core` crate (`core = { path = "../core" }`).
3. Scaffold the initial module skeletons:
   - `desktop/src/app.rs` (State struct and loop)
   - `desktop/src/ui.rs` (Board and chat components)
   - `desktop/src/main.rs` (Tokio initiation and window configuration)

## Phase 4: Foundational Environment & Database Setup
1. Create a `.env` template in the root directory specifying the local `DATABASE_URL=sqlite://chaiss_local.db`.
2. Initialize `sqlx` in the `core` module by running `sqlx db create` (assuming the `sqlx-cli` is installed natively) to generate the physical `.db` file.
3. Stub the initial database migration explicitly documenting the schema formulated in the design docs (`players`, `games`, `moves`).

## Phase 5: Verification
- Run `cargo check` at the workspace root to ensure both `core` and `desktop` compile flawlessly, and that `desktop` can properly consume `core` functionality.
- Execute `cargo run --bin desktop` to ensure an empty `egui` window can be instantiated without panics.
