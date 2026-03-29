# Chaiss Chess Engine & Logic Scaffolding

This document summarizes the strategy detailed locally in `docs/tasks/chaiss_engine.md`. We are shifting focus from the visual scaffolding directly into the pure, headless math of the game engine itself.

## User Feedback Required
Please review the structural strategy mapped out below. Once you approve this, we will write out the raw `struct` and `enum` datatypes and the FEN/ASCII conversion logic inside `chaiss_core/src/engine.rs`.

## Proposed Architecture (Core Engine)

### 1. Fundamental Types
We will formally write out `Color`, `PieceType`, `Piece`, and the geometric structures mapping the `Square` locations on an 8x8 layout.

### 2. State & Rule Enforcement
We will outline the `GameState` struct. This acts as the backbone, recording the active board elements, whose turn it is, castling restrictions, and move numbering (acting as the physical embodiment of a FEN string).

### 3. FEN and ASCII Translators
- We will write a bi-directional `.from_fen()` and `.to_fen()` parser handling universally accepted notation setups.
- **LLM ASCII Grid**: Crucially, we will author a `.to_ascii()` function that sweeps our active board array and emits a multi-line visual string of textual characters representing the board (with rank and file borders) so the LLMs can 'see' the board structurally without heavy JSON noise.

### 4. Heat Map Array Logic Stubbing
While full legal move generation with check-pinning algorithms is massively complex, we will stub out the `generate_heat_map()` calculator that iteratively records attack overlaps onto an uncoupled grid that `desktop` will eventually consume for alpha-blending visuals.

## Verification
I will inject pure Rust `#[test]` macros at the bottom of the engine module testing that our FEN loading logic perfectly constructs an initial game board, and that iterating it back into an ASCII grid renders flawlessly.
