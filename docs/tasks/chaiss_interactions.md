# Implementation Plan: GUI Interaction & Formal Movement Mechanics

This document bridges the gap between pure visualization and playable logic. We will establish proper Hit-Box tracking using `egui`'s interaction responses, and formally upgrade our engine's pathing to distinguish between "Heat Projection" and "Legal Moves" (handling unique anomalies like *En Passant* and Castling).

## Phase 1: Pure Engine Move Generation & Structuring
Currently, `movement.rs` calculates `get_pseudo_legal_attacks` purely to drive the alpha heat map (spatial influence). We must now implement true physical displacement pathways.

1. **Move Definitions (`models.rs`)**:
   We will define `struct Move { from: usize, to: usize }`. (This will later expand to handle promotion indicators).
   
2. **Valid Move Logic (`movement.rs`)**:
   We will build `get_legal_moves(state: &GameState, sq_idx: usize) -> Vec<usize>`.
   - **Pawns:** Unique logic. Unidirectional pushes into empty squares. Initial double pushes. And crucially: *En Passant*. 
   - *Addressing the En Passant Requirement:* Our `GameState` natively tracks `pub en_passant_target: Option<Square>`. During Pawn move bounding queries, we will directly check if the target diagonal offset maps perfectly to the `en_passant_target.index`.
   
3. **State Mutation (`models.rs`)**:
   We will introduce `impl GameState { pub fn apply_move(&mut self, source: usize, destination: usize) }` which will transpose the Piece structurally, clear the old index, update `active_color`, and wipe/assign `en_passant_target` trackers dynamically.

## Phase 2: Egui Spatial Hit-Box Interactions
Instead of messy mathematical mouse-interrogation bounds checks globally, we will lean into fundamental `egui` GUI idioms.
In `desktop/src/ui/board.rs`:
1. Because we map an explicit geometrical `Rect` for each algebraic tile during the 8x8 loop, we simply bind an `egui::Sense::click()` interaction directly onto that specific `square_rect`!
2. `let response = ui.interact(square_rect, id, egui::Sense::click());`
3. We will modify `ChaissApp` to track `pub selected_square: Option<usize>`.
4. **Interaction Flow**:
   - If `selected_square` is `None` AND the clicked `square_rect` holds an active piece belonging to the human player, assign `selected_square = Some(index)`.
   - The selected square's background will illuminate dynamically (e.g. glowing yellow overlay) to visualize activation.
   - If `selected_square` is `Some(Active_Piece)` AND the user clicks an unoccupied or enemy-occupied tile, we invoke the `engine` to verify if that destination index resides within `get_legal_moves(..., active_piece)`.
   - If legal: Send `game_state.apply_move(...)`, reset `selected_square`, and naturally swap the colors!

## Phase 3: Verification
1. We will natively click a starting Pawn (e.g. `e2`). It should glow. 
2. We will click `e4` (valid double push). The unicode character will instantly transpose over the grid geometry and the underlying `GameState` matrix will be successfully overwritten, unlocking black's turn seamlessly!
