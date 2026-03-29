# Implementation Plan: Chess Movement & Raycasting Engine

Before wiring the UI interactive logic, we must construct the core mathematical models that enforce how chess pieces traverse the 8x8 grid. This phase strips out our visualization stubs and injects pure pseudo-legal move generation along with a unified Heat Map aggregator.

## Phase 1: Engine Module Restructuring
Because pathing algorithms are dense, expanding the single `engine.rs` file will result in a 1,500+ line unmanageable monolith. We will split it cleanly:
1. `core/src/engine/mod.rs`: The new module entry exposing everything cleanly.
2. `core/src/engine/models.rs`: To house our existing `Piece`, `Color`, `GameState`, FEN, and ASCII logic.
3. `core/src/engine/movement.rs`: The dedicated movement calculations file.

## Phase 2: Primitive Movement Logic (Pseudo-Legal Rays)
We must define raycasting trajectories. 
A unified model for `generate_pseudo_legal_moves(board, square)` that checks the occupant `PieceType`:
- **Sliders (Queen, Rook, Bishop):** Iterate across respective `(rank_dir, file_dir)` coordinates. `dir_loop`: walk multiplying the offset until hitting board borders or a piece. If the hit piece is friendly, break. If enemy, include capture square and break.
- **Hoppers (Knight):** Check 8 fixed 'L' shape translation offsets.
- **Kings:** Check 8 orthogonally/diagonally adjacent translation offsets.
- **Pawns:** Unique unidirectional rules depending on `Color` (White moves "up" relative to rank numbers). Enforce single pushes, optional double pushes from starting rows, and diagonal captures only when opposing material occupies the square (or En Passant rules apply).

## Phase 3: The Authentic Heat Map (Alpha Generator)
We will rewrite `GameState::generate_heat_map(&self) -> [[u8; 8]; 8]`.
Instead of the vertical line stub, this function will:
- Iterate through every active square (0..64 index).
- For every non-empty square, project the piece's pseudo-legal "Attack Bounds" using the logic from Phase 2.
- Critically: Pawns contribute heat *diagonally forward* (not linearly), King's contribute an adjacency ring, and Sliders cast heat continuously until they hit something.
- We augment the returned 8x8 index `heat_map[rank][file] += 1` for every intersecting attack.

## Phase 4: Verification
- Write a `#[test]` module mapping a complex mid-game FEN position.
- Validate that the Alpha array (Heat Map matrix) maps correct integer concentrations on contested central squares while verifying Pieces are blocked perfectly by friendly pawn walls.
