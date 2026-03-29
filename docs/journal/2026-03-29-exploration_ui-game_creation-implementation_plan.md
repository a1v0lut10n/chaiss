# Exploration UI Controls & Game Creation Mode

You are perfectly right! Destructive DB rollbacks (`DELETE FROM moves`) ruin the learning capacity of the application. A chess app designed for "reflection" needs a decoupled sandbox.

Instead of writing a destructive "Undo" macro, we will introduce a non-destructive **History Scrubber** (`[<<]` `[<]` `[>]` `[>>]`) beneath the board, coupled with explicit `Exploration Mode` sandbox blocks!

## User Feedback Required
Please review the exact proposed structure for transitioning back and forth between "Live DB Plays" and "Exploration Mode" in the Control Bar phase! 

## Refined UI Pipeline

### 1. Database Schema Patch (Naming Games)
Right now, `games` tracks White, Black, FEN, and Status. We must inject:
`name TEXT NOT NULL DEFAULT 'Untitled Game'` into the `games` table array natively. 

### 2. Sandbox Memory Scaffold (`app.rs`)
`ChaissApp` must be expanded to track these string inputs continuously AND buffer the exploration timeline:
```rust
pub show_new_game_modal: bool,
pub new_game_name: String,   // Added!
pub white_player_name: String,
pub black_player_name: String,
/* History & Exploration Management */
pub history_stack: Vec<String>, // Stores every DB FEN sequentially
pub view_cursor: usize,        // Tracks what FEN the Board UI is physically rendering
pub is_exploration_mode: bool, // Dynamically true if view_cursor < history_stack.len()
```

### 3. The New Game Modal (`left_panel.rs`)
When you click `"Create New Game"` in the left drawer:
- We toggle `show_new_game_modal = true;`
- The Main loop natively launches an `egui::Window` drawing 3 clean string text-entry boxes.

### 4. Board Context Controls (History Scrubbing)
Instead of a destructive undo button, we will draw a gorgeous FIDE-standard `ui.horizontal()` control strip sitting precisely beneath the geometric board:
- **`[<<] Start`** / **`[<] Prev`** / **`Next [>]`** / **`Live [>>]`**
- Clicking `Prev` decreases the `view_cursor` index. The UI fetches the FEN from `history_stack[view_cursor]` and instantly replaces the `GameState`!
- **Implicit Sandboxing**: If `view_cursor` is *not* at the end of the stack, the app sets `is_exploration_mode = true`.
- **The Sandbox Block**: While `is_exploration_mode == true`, you can violently drag pieces around the `board.rs` grid generating hypotheticals, but our code mechanically blocks `DbClient::log_move()` and stops triggers from firing into `llm.rs`! You can experiment endlessly, then click `Live [>>]` to snap the `view_cursor` back to the active persistent DB database instantly! 

## Verification
- Booting the application and asserting the floating Modal dynamically captures the 3 named fields.
- Visual validation that the History Scrubber bar sits elegantly pinned beneath the layouts naturally enforcing the exploration loop geometry!
