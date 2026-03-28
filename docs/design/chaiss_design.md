# Chaiss System Design Specification

This document outlines the core architectural components, data models, and interaction flows for the Chaiss engine, exclusive of granular UI layout specifics (which are covered in `chaiss_ui_design.md`).

## 1. High-Level Architecture
The application follows a modular, state-driven architecture, separating the immediate-mode UI from the underlying complex game logic.

* **UI Layer (`egui`):** Reconstructs the visualization every frame based purely on the `AppState`. Handles input events and sends intents to the core engine.
* **Game Engine Core:** Manages chess rules, move validation, FEN/ASCII conversion, and move history (including branching for Exploration mode).
* **LLM Orchestrator:** Asynchronously handles communication with `llm` crate backends or API providers without blocking the UI rendering thread.
* **Persistence Layer:** A local SQLite database interfaced via `sqlx` to store matches, profiles, and API configurations.

## 2. Core Data Models
### 2.1 Game State
```rust
struct GameState {
    board: Board, // 8x8 representation, tracks piece positions
    active_color: Color,
    move_history: Vec<MoveRecord>,
    exploration_branch: Option<ExplorationState>,
    status: GameStatus, // Active, Checkmate, Stalemate, etc.
}
```
### 2.2 Board & Heat Map Data
* The **Board** computes attacked squares dynamically.
* **Heat Map State:** A 2D array or HashMap `[(u8, u8); u8]` tracking the number of times a square is targeted by the current player's pieces (or opponent's). This is recalculated cleanly upon state changes.

### 2.3 LLM Context State
```rust
struct LlmSession {
    selected_model: ModelConfig,
    conversation_history: Vec<Message>, 
    turns_since_full_context: usize, // 'p' iterations tracking
}
```

## 3. Subsystem Flows
### 3.1 LLM Interaction Flow
1. **Move Executed:** The Game Engine finalizes a move.
2. **Context Assembly:** The core generates a `MoveNotification` ("black played Nf6") alongside an ASCII grid and a FEN string.
3. **Pacing Check:** The engine checks if `turns_since_full_context >= p`. If so, a full summary payload is attached to the prompt.
4. **Asynchronous Dispatch:** The prompt is sent to the LLM backend on a separate tokio task. 
5. **Streamed Response:** The UI polls an `mpsc` channel for streamed tokens, dynamically expanding the chat interface.

### 3.2 Database Schema (SQLite)
* `players`: `id`, `name`, `created_at`
* `games`: `id`, `white_player_id`, `black_player_id`, `name`, `start_time`, `fen_snapshot`
* `moves`: `id`, `game_id`, `move_number`, `san` (Standard Algebraic Notation), `timestamp`

## 4. Module Boundaries & Folder Structure
To cleanly support the transition to Flutter (Web, Android, iOS) down the road, the repository will adopt a polyglot monorepo structure. All business logic must be completely decoupled from the UI layer to allow native bindings later via FFI (Foreign Function Interface) or platform channels.

```text
chaiss/
├── core/                # Core Rust library (No UI code)
│   ├── src/
│   │   ├── engine/      # Pure chess logic, FEN validation, heat map calculation
│   │   ├── llm/         # Interaction with LLMs and prompt processing
│   │   ├── db/          # sqlx database queries and migrations
│   │   └── lib.rs       # Unified engine API to be consumed by any frontend
│   └── Cargo.toml
├── desktop/             # Native Desktop application
│   ├── src/
│   │   ├── ui/          # egui components (board rendering, chat window)
│   │   ├── app.rs       # Central state connector fusing `core` and `egui`
│   │   └── main.rs      # Native tokio runtime setup and eframe launch
│   └── Cargo.toml
└── mobile_web/          # Future Flutter Directory (Placeholder)
    └── (Will consume `core` using package like `flutter_rust_bridge` later)
```
