# Implementation Plan: Wiring GameState to Egui

This document outlines how we will pull the newly defined `GameState` engine from the `chaiss_core` library and mount its data visually onto our `desktop` checkerboard.

## Phase 1: State Initialization
- We will update `ChaissApp` in `desktop/src/app.rs` to persist a `pub game_state: chaiss_core::engine::GameState`.
- We will initialize it with `GameState::new()` when the native application launches, supplying the standard universal starting layout.
- The `ui::board::draw` function signature will be updated to accept `&chaiss_core::engine::GameState` as an immutable reference.

## Phase 2: Piece Rendering via Unicode
To keep the application natively blisteringly fast and resolution-independent without the headache of bundling SVG decoders, we will rely on native unicode mapping:
- We will write a helper `fn piece_to_unicode(piece: Piece) -> &'static str` that translates our core Engine models into standard chess symbols (♚, ♛, ♜, ♝, ♞, ♟ for Black; ♔, ♕, ♖, ♗, ♘, ♙ for White).
- Inside our existing 8x8 nested rendering loop in `board.rs`, after we paint the geometric square rectangle, we will query `game_state.board[rank * 8 + file]`.
- If a piece exists, we will use `ui.painter().text()` to perfectly center the unicode glyph inside the geometric tile, scaling the font size proportionally to `square_size * 0.75` so it breathes beautifully regardless of how the user stretches the window.

## Phase 3: Activating the Heat Map Stub
- We will call `game_state.generate_heat_map()` inside the drawing loop.
- Before we draw the unicode piece, we will draw an overlay transparent red rectangle directly over the wooden square if the heat map `[rank][file] > 0`.
- The `alpha` transparency ratio will be calculated incrementally based on the heat map value (so 3 pieces attacking a square looks intensely red, 1 looks faint).

## Phase 4: Verification
- Execute `cargo run --bin desktop`.
- You should instantly see the universal starting formation perfectly mounted on the checkerboard.
- You should see our artificial "stubbed" heat map visually warming up the empty squares directly vertically adjacent to the starting pieces (as written in the engine stub).
