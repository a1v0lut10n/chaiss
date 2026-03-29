# Implementation Plan: Context Scaffolding & Exploration Mode

This document maps out the required GUI scaffolding to capture user intent for Game Creation, Player Assignment, and the critical historical "Exploration Mode" sandbox.

## Phase 1: Database Schema Expansion (`init_db.sh` & `db.rs`)
The user requested the ability to "name a game". Currently, the SQLite `games` table lacks a `name` column.
1. We will modify `tools/scripts/init_db.sh` to inject `name TEXT NOT NULL DEFAULT 'Untitled Game'` into the `games` table.
2. We will update `core/src/db.rs` `create_game()` to accept and `INSERT` the `game_name` parameter natively.

## Phase 2: App State Scaffold (`desktop/src/app.rs`)
To manage Exploration Mode without corrupting the active database session, we must bind state fields directly to `ChaissApp`:
- `show_new_game_modal: bool`
- `active_game_id: Option<i64>` (Database Session)
- `history_stack: Vec<String>` (In-Memory array of all FENs from the DB)
- `exploration_cursor: usize` (Which move in the history are we currently looking at?)
- `is_exploration_mode: bool` (If true, block all DB writes and LLM interactions!)

## Phase 3: The New Game Modal (`left_panel.rs`)
When `Create New Game` is clicked, we toggle `show_new_game_modal = true`.
We will instruct `left_panel` (or `app.rs` directly) to render a pristine native `egui::Window` featuring:
- 3 explicit `TextEdit` fields for naming the Game and both Players.
- A "Start Game" confirmation button.

## Phase 4: History Scrubbing & Exploration Sandbox (`board.rs`)
Inside the `CentralPanel` rendering the physical chessboard, we will allocate a `horizontal` control bar sitting tightly beneath the 64-tile grid. Instead of a destructive DB "Undo", we follow FIDE standard History controls!
1. **Nav Toggles:** Render `[<<]` `[<]` `[>]` `[>>]` buttons. Clicking `<` lowers the `exploration_cursor`, fetches the FEN from `history_stack`, and overwrites the board UI instantly.
2. **Exploration Toggle:** If `exploration_cursor < history_stack.len() - 1`, we automatically enter `Exploration Mode`! If they move a piece now, we branch off into a sandbox.
3. **Sandbox Guard:** When `is_exploration_mode` is true, all `DbClient` write calls and `llm.rs` triggers are securely blocked, allowing infinite native "what-if" testing physically!

## Phase 5: Verification
Launch `cargo run --bin desktop`. Verify the Left pane triggers a dynamically floating "Create Game" dialog capturing string contexts, and the Board pane renders an elegant History Scrubbing navigation bar!
