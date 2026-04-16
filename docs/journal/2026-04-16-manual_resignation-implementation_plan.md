# Resignation Feature Implementation Plan

Adding a "Resignation" option involves interacting with the game's termination state natively and ensuring that this explicitly asserted status is persisted algebraically into out native SQLite database structure when games are resumed.

## Proposed Changes

### 1. `core/src/engine/models.rs`
- **[MODIFY]** `GameEndStatus` enumeration enum:
  - Inject new variant: `Resignation(Color)` (Representing the *Winner*).
- **[MODIFY]** `GameState` struct:
  - Add `pub manual_terminal_status: Option<GameEndStatus>` dynamically.
  - Modify `evaluate_terminal_state()` to instantly short-circuit and return `manual_terminal_status` if it isn't `None`, locking all mathematical interactivity gracefully.
  - Hardcode it to naturally initialize as `None` in `new()` and `from_fen()` natively.

### 2. `desktop/src/ui/right_panel.rs`
- **[MODIFY]** The text parsing loop.
- Right before attempting the `parse_algebraic_move` fallback, mathematically intercept explicit case-insensitive terminators:
  - `"1-0"`, `"white wins"`, `"black resigned"` -> Triggers White Victory.
  - `"0-1"`, `"black wins"`, `"white resigned"` -> Triggers Black Victory.
- If hit mathematically, formulate a pseudo-move using the existing DB tracking infrastructure:
  - Overwrite `app.game_state.manual_terminal_status`.
  - PGN-append the resulting mathematical string `"1-0"` or `"0-1"` to `algebraic_history`.
  - Duplicate the final FEN string structurally into `history_stack` so that `live_db_ply` perfectly synchronizes.
  - Log it via `db_client.log_move(..., "1-0")` dynamically.

### 3. `desktop/src/ui/board.rs`
- **[MODIFY]** Active Turn Display Header.
  - Render a new conditionally mapped `Resign` button directly next to the "Active Turn: White" visualizer.
  - If triggered explicitly, execute identically to the textual input (Set manual status, inject pseudo-move, dispatch to DB natively).
- **[MODIFY]** Terminal State Visual Array.
  - Update the overlay matcher organically to extract and render `GameEndStatus::Resignation(winner)` cleanly as `Resignation!\nWhite Wins`.

### 4. `desktop/src/app.rs`
- **[MODIFY]** Desktop Startup / Frame Resume `DbEvent::GameResumed`.
  - After natively pulling `history` and `algebraic` queues locally, explicitly poll `self.algebraic_history.last()`. 
  - If the very final recorded topological event across the chronos is algebraically `"1-0"` or `"0-1"`, forcefully re-hydrate `self.game_state.manual_terminal_status`, ensuring games natively resurrect directly into locked termination matrices!

## User Review Required
Does this pure-algebraic approach mapping "1-0" standard strings structurally into the DB move-tree logically align with what you envisioned? 
