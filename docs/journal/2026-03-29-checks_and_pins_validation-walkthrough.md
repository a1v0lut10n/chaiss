# True Legal Validations (Checks & Pins)

The engine now enforces true chess logic! A player can no longer visually drag a pinned piece away from their King via the `desktop` UI, nor can they make idle moves while forced to respond to an active check natively!

## Architectural Successes

1. **Hostile Array Sweep (`is_square_attacked`)**
    Instead of writing horrifyingly complex analytic line-of-sight checks that easily break with En Passants or double Pawn pushes, we simply wrote an inverted scanner:
    - We loop `0..64`. If the square contains hostile logic, we ask what it naturally attacks.
    - If the target index (e.g. your King's coordinate) equals an active hostile mathematical bounding vector, the square is violently flagged!

2. **The "Hypothetical Clone" Implementation**
    We added the ultimate filter into `get_legal_moves()`!
    - When you click a Square on the checkerboard, `desktop` asks what its legal moves are.
    - `movement.rs` computes pseudo-bounds (e.g. your Knight can theoretically jump 8 ways).
    - For all 8 coordinates, it `clone()`s the physical 64-byte `GameState`.
    - It immediately pushes the Knight to that new coordinate structurally on the clone.
    - It then asks: `is_square_attacked(&clone, my_king_idx, hostile_color)`?
    - If the answer is `true` (because you just exposed a Pin, or you failed to step in front of a Check), the coordinate is violently purged from the acceptable array!

## Validation via Local Benchmarks
If you execute `cargo test -p chaiss_core`, two highly specialized scenarios compile identically via FEN strings:
- **`test_pinned_knight_cannot_move`**: We load a string where a White Knight blocks a Black Rook's direct check on the White King locally! The mathematical loop processes the Knight natively and evaluates `0` legal moves!
- **`test_king_check_forces_responses`**: We load a string where a Black Rook natively holds a King in check directly beside a White Pawn. The engine correctly validates the Pawn `0` legal moves (idle moves aren't permitted), whilst isolating the King's 3 legal moves (stepping away sideways off the E-file organically, or directly capturing the unprotected attacking Rook)!

## Next Step
Our physics and bounding geometries are nearly perfectly aligned!
Would you like to shift to configuring `chaiss_core/src/llm.rs` so we can finally start asking LLMs to evaluate our string setups, or stick mechanically here to wire up `Pawn Promotions` and `Kingside/Queenside Castling` implementations?
