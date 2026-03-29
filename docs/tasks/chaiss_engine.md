# Implementation Plan: Chaiss Game Engine (Rust Core)

This document maps out the architectural phases for building the pure Rust chess engine inside `chaiss_core/src/engine.rs`. The goal is to enforce chess logic entirely stripped of GUI dependencies, providing robust state representations (FEN and ASCII arrays) suitable for ingestion by frontier LLMs.

## Phase 1: Core Datatypes Definition
We need to model the fundamental elements of the board using Rust's powerful type system to enforce valid states natively.
- Enum `Color`: `White`, `Black`
- Enum `PieceType`: `Pawn`, `Knight`, `Bishop`, `Rook`, `Queen`, `King`
- Struct `Piece`: Composing `Color` and `PieceType`.
- Struct `Square`: Representing algebraic coordinates mathematically (typically as an `u8` alias for a 0-63 1D array, or a `(u8, u8)` Tuple for direct Rank/File addressing). We'll lean toward a 1D internal array for performance, with translating functions mapable to `egui`.
- Type alias `BoardMatrix`: `[Option<Piece>; 64]` or `[[Option<Piece>; 8]; 8]`.

## Phase 2: Game State Implementation
We will build a `GameState` struct that acts as the single source of truth for the active chess match, encompassing everything needed for serialization.
- `board`: Our `BoardMatrix`.
- `active_color`: Tracks whose turn it is.
- `castling_rights`: Tracks Kingside/Queenside availability for both colors.
- `en_passant_target`: The `Square` representing a legally capturable pawn.
- `halfmove_clock` & `fullmove_number`: Required for the 50-move rule and classic notation tracking.

## Phase 3: Serialization & LLM Integration (FEN & ASCII)
Since LLM context relies heavily on text, we must serialize our `GameState` efficiently.
- **FEN Parsers:**
  - `impl GameState { pub fn from_fen(fen: &str) -> Result<Self, EngineError> }`
  - `impl GameState { pub fn to_fen(&self) -> String }`
- **ASCII Renders:**
  - `impl GameState { pub fn to_ascii(&self) -> String }` 
  - This fulfills the requirement to inject structural "sight" into LLMs by drawing a raw 8x8 text map composed of lowercase (black) and uppercase (white) standard algebraic letters.

## Phase 4: Movement Validation & The Heat Map algorithm
Separating pseudo-legal logic (how pieces physically move based on board walls) from true legal logic (pinning constraints protecting the king).
- **Psuedo-legal calculations:** Generating physical raycasts for sliding pieces (Rooks, Bishops, Queens) and jump patterns (Knights, Kings).
- **The Attack Map (Heat logic):**
  - Implement a specialized function `fn generate_heat_map(&self) -> [[u8; 8]; 8]` that sweeps the entire board. For every piece capable of landing an attack on a target square, we increment that target index by `1`. This output feeds directly into the `egui::Rect` alpha blender drafted in our UI phase. 

## Phase 5: Verification
- Write localized `#[test]` modules testing standard FEN parsing conversions (`Starting Position FEN -> Engine -> to_fen -> match == true`).
- Ensure ASCII output is legible using CLI debugging (`cargo test -- --nocapture`).
