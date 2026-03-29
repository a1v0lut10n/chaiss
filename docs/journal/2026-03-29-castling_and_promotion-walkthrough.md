# Physical Manipulations (Castling & Promoting)

We've conquered the final massive edge-cases required to build a mechanically compliant chess physics loop! `apply_move` is no longer a simple struct-swapping function, but an intelligent rule-parser.

## Achievements 

1. **Castling Threat & Path Vectors**
   `chaiss_core` now naturally registers Castling (Kingside & Queenside) internally inside `get_legal_moves()`!
   The Engine rigorously confirms:
   - Your `"KQkq"` string contains the explicit Algebraic character natively representing the requested side.
   - Every square strictly between your King and the chosen Rook is geometrically empty!
   - Your King is NOT uniquely threatened.
   - And critically: the square your King passes *through* naturally (`f1` or `d1`) is also verified instantly via `!is_square_attacked()` ensuring you inherently cannot jump through hostile "heat"!

2. **Simultaneous Multi-Piece Transpositions**
   If you naturally click the 2-space target ring to confirm the castle on the UI, `apply_move()` detects the exact `1D` coordinate diff! It physically obliterates the chosen Rook from the corner square and seamlessly repaints it right alongside your new King position natively without triggering separate turns!

3. **Auto-Queening Dynamics**
   FIDE rules dictate immediate substitutions when Pawns naturally hit back ranks (`0` or `7`). 
   To preserve UI velocity without halting the application natively, any pawn satisfying the rank criteria has its `PieceType` explicitly overwritten to `PieceType::Queen` before rendering! We updated the `apply_move` signature natively `(..., promotion_target: Option<PieceType>)` meaning you natively control standard Queening or explicit UI hooks down the road.

4. **String Degradation Tracking**
   Every time an interaction involves a Rook natively leaving its square or a King physically moving, the mathematical Engine dynamically strips the corresponding `"K"`, `"Q"`, `"k"`, or `"q"` flags from the FEN tracking string so future attempts are natively blocked immediately algebra-side!

## Sandbox Verification
Run the `desktop` application! You can now freely clear pieces blocking your Rooks and castling will dynamically appear as a target ring cleanly on the board geometry natively, executing exactly as mapped!
