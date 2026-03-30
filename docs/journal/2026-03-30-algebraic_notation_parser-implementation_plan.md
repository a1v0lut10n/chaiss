# Algebraic Notation Parser Blueprint

We need to mathematically translate Standard Algebraic Notation strings (`e4`, `exd5`, `Nf3`, `Nbd2`, `O-O`) dynamically entered in the Chat Window into physical `(from_square, to_square)` arrays securely handled by `GameState::apply_move`.

## User Review Required
Before I begin physically constructing the parser loop within `core/src/engine/notation.rs`, verify if you have any explicit notation quirks you want supported natively:
- Should we strictly enforce casing (`e4` vs `E4` / `Nf3` vs `nf3`)?
- Should we support non-standard Long Algebraic Notation inputs gracefully (e.g., users typing `e2e4` instead of `e4`)? I plan to support standard SAN exclusively first.

## Proposed Changes

### [core/src/engine/notation.rs]
I will construct a new robust string parser topologically matching formal FIDE rules explicitly:

#### `pub fn parse_algebraic(&str, &GameState) -> Result<(usize, usize, Option<PieceType>), String>`
1. **Sanitization:** Aggressively strip trailing topological notation (`+`, `#`, `!`, `?`). Handle Pawn Promotions dynamically (e.g., matching `=Q` and stripping it).
2. **Castling Detection:** Match `O-O` and `O-O-O` natively. Mathematically search `GameState` for the active-color King's coordinate and map the explicit +2/-2 castling jump perfectly.
3. **Pawn Arrays (`a-h`):** If the string begins sequentially with a lowercase file letter (e.g., `e4` or `exd5`):
   - Parse the physical destination square natively.
   - If it's a capture (`x`), lock the origin file strictly to the first char (`e`).
   - Iterate through the active color's Pawns, invoke their formal `get_legal_moves()` structurally, and bind the exact match.
4. **Major Piece Arrays (`K, Q, R, B, N`):** 
   - Extract the trailing `[a-h][1-8]` destination structurally.
   - Iterate through every active piece matching the requested type, generate their explicit `get_legal_moves()`.
   - **Disambiguation Mapping:** If multiple identical pieces (`Rooks` or `Knights`) can attack the target (e.g. `Rad1` vs `R1d1`), parse the middle character (`a` or `1`) dynamically to isolate the correct mathematical origin square natively!

### [desktop/src/ui/right_panel.rs]
- We will wire the Egui `Send` button seamlessly into the new `chaiss_core::engine::notation::parse_algebraic` function.
- If it structurally matches, the App dynamically commits `app.game_state.apply_move(...)` natively updating the Egui Board visually, and fires the `[System: I moved X]` Flume payload successfully out to the LLM backend!

## Verification Plan

### Automated Tests (`core/src/engine/notation.rs`)
I will architect explicit Unit Tests inside `notation.rs` geometrically evaluating:
- Disambiguation matrices natively: `Nbd2`, `R1e2`.
- Capture bounds structurally: `exd5`, `Bxc4`.
- Promotion mathematics mapping explicitly: `e8=Q`.
