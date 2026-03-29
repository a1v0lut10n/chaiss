# Implementation Plan: Castling & Promotions

This phase handles the complex edge-case mechanics of traditional chess: moving two pieces simultaneously (Castling), and structurally transforming pieces upon reaching opposite boards edges (Pawn Promotion).

## Phase 1: Castling Mechanics (`movement.rs` & `models.rs`)
1. **Castling Rights Tracking:** Our FEN parser natively tracks `"KQkq"` via `pub castling_rights: String`.
2. **Move Generation:** Inside `get_legal_moves()` for `PieceType::King`, we will interrogate the `castling_rights` state.
   - For White Kingside (`K`): King is on `e1` (60), check if `f1` (61) and `g1` (62) are empty.
   - We must also mathematically guarantee that `e1`, `f1`, and `g1` are **NOT** attacked. (A King cannot castle out of, through, or into check).
   - If valid, we push `g1` into the legal moves array!
3. **Move Execution (`apply_move`)**: If we identify the King has moved exactly `2` ranks horizontally, it's a castle! 
   We must natively teleport the respective Rook `board[target +/- 1] = Rook` and delete it from its native corner algebraically.
4. **State Degradation**: If the King or bounding Rooks move (or are captured), we dynamically strip the `"KQkq"` string down to remove future rights algebraically.

## Phase 2: Pawn Promotion (Auto-Queening Engine)
1. **State Mutation:** We will overhaul `apply_move` to intercept pawns landing on `Rank 0` (White) or `Rank 7` (Black).
2. **Auto-Queening Implementation:** For massive initial GUI velocity, if a pawn lands on the promotion rank without an explicitly requested promotion target, we will automatically transmute the `board[to]` reference into `PieceType::Queen`. 
3. **Future-proofing:** We will modify `apply_move(from, to, promotion: Option<PieceType>)` so the GUI can eventually feed explicit Bishop/Knight requests via a radio-modal.

## Phase 3: Verification
- `cargo test -p chaiss_core` injecting a FEN layout of an empty back-rank with pristine `"KQkq"` rights.
- Ensuring `get_legal_moves()` successfully evaluates `O-O` geometries while blocking them under threat geometries natively!
