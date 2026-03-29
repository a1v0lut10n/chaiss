# Implementation Plan: Check & Pin Validations

This document handles the final frontier of pure chess movement mechanics: Check resolution, and Pin enforcement. Without this, a player can drag a piece defending their King out of the way, resulting in illegal self-check states.

## Phase 1: Threat Detection (`is_square_attacked`)
To know if a King is in check, we must first mathematically prove if a specific square is under hostile influence.
We will add a helper function inside `src/engine/movement.rs`:
`pub fn is_square_attacked(state: &GameState, target_idx: usize, attacker_color: Color) -> bool`
- We will iterate through all 64 squares.
- If we find a piece matching the `attacker_color`, we call our existing `get_pseudo_legal_attacks()` on it.
- If the returned array contains `target_idx`, the square is hostile!

## Phase 2: Board State Simulation (The "Clone" Filter)
The most robust algorithm for filtering Pins and Checks is a "hypothetical simulation" loop.
In `get_legal_moves()`, we already successfully compute an array of pseudo-legal destinations.
Before returning that array to the GUI, we will:
1. Initialize an empty `Vec<usize>` representing true legal moves.
2. Iterate over every pseudo-legal destination.
3. `clone()` the entire `GameState` memory struct.
4. Structurally apply the target move directly onto the cloned board (bypassing turn-swaps, just physically moving the piece and clearing En Passants/Captures).
5. Locate the active King's `index` on this hypothetical board.
6. Call `is_square_attacked(hypothetical_board, King_index, enemy_color)`.
7. If the King is NOT attacked, we push that target index into the true legal array!

### Why this trivially solves everything:
- **Pins:** If you try to move a pinned Knight, the hypothetical clone simulates the Knight being gone. The `is_attacked` scanner will instantly see the enemy Bishop hitting the King. The move is discarded natively!
- **Checks:** If the King is currently under attack, and you try a random Pawn push... the clone simulates the Pawn moving. The `is_attacked` scanner still flags the King accurately! The move is discarded natively! 

## Phase 3: Verification
We will modify the GUI or write a `#[test]` module wrapping a highly constrained FEN string (e.g. A pinned Queen, a King in Check). We will query `get_legal_moves()` and verify it correctly filters out suicidal slides or mathematically forces blocks/evasions!
