# Deep PGN Sequence Loading Architecture

Rebuilding a board move-by-move is definitely tedious. Integrating a **PGN (Portable Game Notation)** parser allows you to simply copy-paste a full game sequence (e.g., `1. e4 e5 2. Nf3 Nc6 3. Bb5...`) directly from applications like exact Chess.com or Lichess into the Chat Input Box!

Because we already built incredibly robust validation logic natively inside the `parse_algebraic_move` FIDE engine mathematically, implementing this is surprisingly straightforward. 

## Proposed Architecture

### 1. Unified Parser Pipeline (Inside `right_panel.rs`)
When you submit a multiline payload to the Chat Box, we will natively intercept it and execute a **Bulk Analysis Pipeline**:
- First, we explicitly strip away standard PGN metadata blocks (e.g., `[Event "FIDE..."]`) usually wrapped in square brackets.
- We then split the remaining text by whitespaces mechanically.
- We mathematically ignore formal move numbering artifacts (like `1.`, `35...`) and game termination markers (`1-0`, `1/2-1/2`).
- What remains internally is a pure, perfectly sequential array of geometrical Algebraic constraints (e.g., `["e4", "e5", "Nf3", "Nc6", ...]`).

### 2. Physical Batch Execution
For every parsed valid algebraic move geometrically matched:
- The system recursively invokes our existing `app.game_state.apply_move()`.
- The system mathematically injects the subsequent FEN array straight into the persistent `app.history_stack`.
- The system commits the historical chain securely into your SQLite bindings identically without dropping any data context!
- By utilizing the `[x] Silence AI Auto-Analysis` toggle during this loop, you can flawlessly inject a 60-move game natively without the LLM pipeline organically locking or requesting 60 separate analyses!

## User Review Required
> [!NOTE]
> Are you strictly looking to paste the plain algebraic moves themselves (`e4 e5 Nf3 Nc6`), or do you want the parser robust enough to accept the entire raw 'Export to PGN' text format directly from external applications (which frequently includes the `[Site "Lichess"]` and `[Date "2026..."]` headers)? 
>
> Once confirmed, I can dynamically inject this regex formatting safely straight into the Chat Text Engine!
