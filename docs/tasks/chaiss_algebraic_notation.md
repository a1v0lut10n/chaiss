# Implementation Plan: FIDE Algebraic Notation Generator

This document outlines the architectural implementation for automatically parsing geometric layout moves into FIDE Standard Algebraic Notation strings natively before passing them to the Database tracker!

## Phase 1: Context Mounting (`notation.rs`)
We will create `core/src/engine/notation.rs` and expose it internally via `pub mod notation;` inside `core/src/engine/mod.rs`.
The signature of the generator mathematically requires knowledge of the pre-move state explicitly:
`pub fn get_algebraic_notation(state: &GameState, from: usize, to: usize, promotion: Option<PieceType>) -> String`

## Phase 2: Algebraic String Formatting
We intercept the data layout exactly BEFORE the geometry updates:
1. **Target Identification**: We determine the `Piece` executing the jump and inherently verify if the destination square physically houses an enemy piece (or if this is structurally an `en passant` trigger!).
2. **Piece Identifiers**: We map `PieceType::Knight` to `N`, `Bishop` to `B`, `Rook` to `R`, `Queen` to `Q`, and `King` to `K`. Pawns inherently emit no prefix letters mathematically.
3. **Capture Syntaxes (`x`)**: If an enemy occupies the target space, we inject `x`. For pawn captures specifically, FIDE rules mathematically demand the departure file (`e.g. exd5`).
4. **Castling Triggers**: If the `Piece` is a King moving precisely 2 spaces horizontally geometrically, we natively return `O-O` or `O-O-O` strictly.
5. **Pawn Promotions**: If a Pawn crosses `to_rank == 0 || to_rank == 7`, we dynamically append the `=` sign alongside the `PieceType` character natively (e.g. `=Q`).

## Phase 3: Disambiguation Sweeping
If you have two `Knights` identically able to hit target square `f3`, FIDE requires the engine to specify the origin constraint natively (`Ndf3` or `N1f3`):
1. The parser structurally loops the entire `board` matrix isolating identical pieces belonging to `active_color`.
2. It passes those pieces into `movement::get_legal_moves()` dynamically. If overlapping targets hit `to_idx`, we structurally append the departing **File** string to the prefix! (If the departing files are identically matching, we inject the departing **Rank** string instead).

## Phase 4: Checks & Mates (`+` & `#`)
After calculating the text natively, we mathematically synthesize a future timeline (`let mut sim = state.clone(); sim.apply_move(...)`) purely to evaluate checks!
1. We run `evaluate_terminal_state()` over the `sim`. If Checkmate structurally returned, we append `#`.
2. Else, we evaluate `is_square_attacked(&sim, enemy_king_idx)` dynamically. If `true`, we append `+`.

## Phase 5: GUI Binding
Inside `desktop/src/ui/board.rs`, we replace the placeholder MVP string!
```rust
let notation = chaiss_core::engine::notation::get_algebraic_notation(&app.game_state, sel_idx, index, None);
// ... toksio span DB logging ...
app.game_state.apply_move(sel_idx, index, None);
```
