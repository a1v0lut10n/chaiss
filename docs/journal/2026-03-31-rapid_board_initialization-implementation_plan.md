# Rapid Board Initialization Architecture

The objective is to implement a robust pipeline allowing the user to precisely instantiate custom chess positions (e.g., specific end-game geometries) without triggering verbose LLM analysis or manually dragging pieces 40+ times.

## User Review Required

> [!IMPORTANT]
> Since there are multiple structural ways to achieve this, please review the three distinct implementation vectors below and let me know which approach you prefer before I execute the codebase changes!

---

## Proposed Options

### Option 1: Direct FEN String Import (Fastest & Standardized)
The universal standard for storing instantaneous chess states is the **FEN (Forsyth-Edwards Notation)** string. We can add a "Load from FEN..." button into the left `Game Roster` block. 
- You simply paste an endgame string like `8/k7/3p4/p2P1p2/P2P1P2/8/8/K7 w - - 0 1`.
- The engine bypasses history generation, strictly sets `app.game_state = GameState::from_fen(fen)`, logs a unified new "Start" array in the DB, and immediately renders your endgame geometry!

### Option 2: Sequential PGN Block Paste
We augment the existing `LLM Chat` window logically. If the system detects a text input that contains multiple formal algebraic strings (e.g., `1. e4 e5 2. Nf3 Nc6 3. Bb5...`), it enters an automated "Fast-Forward" mode natively.
- The UI loops over the moves, pushing them sequentially into the `db.rs` persistence layers without dispatching `LlmEvent::InferenceRequested` for each one.
- **Pro:** You get a fully populated history tape natively inside the sandbox.
- **Con:** It requires having the absolute move-list mathematically instead of just the final geometry map.

### Option 3: "Silent/Fast-Play Mode" Toggle
We inject a simple checkbox directly above the `Sandbox Navigation` block reading `[ ] Silence AI Analysis`.
- When toggled mathematically on, it prevents the layout from dropping LLM payload arrays across the Toko bridge gracefully.
- You can physically rapidly drag pieces or type moves into the Chat array to rebuild the board yourself without blowing through token limits or waiting for responses natively.

## Open Questions

> [!NOTE]
> Which of these three workflows aligns best with how you envision setting up your test boards? (Note: Option 1 and Option 2 are not mutually exclusive; we could implement one now and add the other later). 

## Verification Plan

### Automated Tests
- If Option 1 or 2 is selected, verify the `app.rs` Toko event loop successfully loads the matrices cleanly without triggering the background flume AI bindings natively across the application scope securely.
