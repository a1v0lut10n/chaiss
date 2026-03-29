# Hit-Boxes & Legal Traversal implementation

To answer your direct question surrounding **En Passant**:
> *Yes!* The `GameState` struct we modeled natively tracks `pub en_passant_target: Option<Square>`. However, right now the engine purely generates "Attack Influence" (spatial heat mapping) which is distinct from physical movement. We must construct a highly explicit array-builder that factors in empty ranks, double pushes, and en passant capture validation!

This document formally plans the jump from "Visual" checkerboard to "Interactive" chess sandbox.

## User Review Required
Please read through the structural interaction strategy below. Once you approve, I will update the code to let us physically move Unicode pieces with mouse clipping!

## Proposed Approach

### 1. Expanding the Engine Mathematical Bounds
Right now `movement.rs` runs raycasting solely to generate generic heat fields (e.g. projecting pawn lines diagonally regardless of an enemy presence).
We will add `get_legal_moves(&GameState, sq_idx: usize) -> Vec<usize>`. 
Specifically for Pawns: 
- Push `+1` / `-1` rank if empty.
- Push `+2` / `-2` ranks if start position and both empty.
- Capture diagonally if enemy piece physically occupies it, **OR** if the geometric diagonal coordinate perfectly equals `state.en_passant_target`.

### 2. Physical Engine State Manipulation
We will write `src/engine/models.rs -> pub fn apply_move(&mut self, from: usize, to: usize)` that shifts the `Piece` index structurally, clears the origin cell, handles `active_color` inversions, and wipes the `en_passant` trackers.

### 3. Native Egui Hit-Boxing!
`egui` boasts brilliant contextual clipping geometry. As we evaluate `rect_filled` for all 64 spaces natively inside the resizing window, we will simply inject `ui.interact(square_rect, custom_id, Sense::click())`. This binds event listeners directly onto our vector boxes perfectly decoupled from raw pixel math!
In `app.rs`, your `ChaissApp` struct gets `pub selected_square: Option<usize>`. 

**The Play Loop**:
1. You click a square holding your Piece -> It begins glowing (highlighted overlay geometry block).
2. You click a target square -> Core validation confirms the bounds inside `get_legal_moves()`.
3. If valid -> Fire `apply_move()`! `App` naturally redraws the frame natively updated!

## Verification Plan
We will build out the tracking and natively test moving white Pawns up two ranks followed by capturing diagonally dynamically inside `desktop` to ensure the arrays transpose accurately globally.
