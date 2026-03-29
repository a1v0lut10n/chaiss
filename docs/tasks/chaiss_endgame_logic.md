# Implementation Plan: Checkmate & Stalemate State Detection

This document handles evaluating terminal board states. Identifying Checkmate and Stalemate mathematically requires full-board permutations natively. 

## Phase 1: Engine Terminal State Evaluation (`models.rs`)
1. **Status Descriptor:** We will define `pub enum GameEndStatus { Checkmate(Color), Stalemate }` to track explicit conditions.
2. **Terminal Logic `is_game_over()`:**
   - We will implement `pub fn evaluate_terminal_state(&self) -> Option<GameEndStatus>` inside `GameState`.
   - The engine iterates through the board (`0..64`), finding every piece belonging to the `active_color`.
   - It computes `get_legal_moves()` for each piece. If the total count of valid geometric vectors across the entire side is `> 0`, it instantly returns `None` (Game continues).
   - If the total valid responses equal `0`, we fall back on `is_square_attacked()`:
     - If the `active_color` King IS natively attacked `-> Checkmate(Enemy Color)`.
     - If the `active_color` King is NOT attacked `-> Stalemate`.

## Phase 2: GUI Interception & Native Rendering (`board.rs`)
1. **Halting Interactions:** 
   Inside `desktop/src/ui/board.rs`, we will modify the explicit Hit-Box listener:
   `if let Some(end_status) = state.evaluate_terminal_state() { /* Skip click allocations! */ }`
2. **Overlay Visuals:**
   If the game is completely over natively, we will compute a massive transparent black Rect overlaid across the entire geometric board area natively via `ui.painter().rect_filled(bounds, 0.0, Color32::from_black_alpha(150))`.
   We will then inject `ui.painter().text(...)` perfectly dynamically anchored to the center `rect.center()` generating crisp, scalable "Checkmate!" text natively.

## Phase 3: Verification
- `cargo test -p chaiss_core` adding extremely constrained FEN geometries: 
  - Standard Fool's Mate layout -> Assert Checkmate!
  - Minimalistic Pinned corner Stalemate -> Assert Stalemate!
- Launching the UI, loading the stalemate string implicitly, and natively witnessing the board instantly snap the visual end-screen while refusing geometric clicks!
