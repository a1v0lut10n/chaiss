# Chaiss - System and Product Requirements

This document captures the initial requirements and architectural guidelines for **Chaiss**, an intelligent, AI-assisted chessboard built in Rust. It serves as the foundation for the project roadmap.

## 1. Core Technology Stack
- **Language:** Rust
- **UI Framework:** `egui` (Immediate mode GUI, chosen for highly customizable, reactive interfaces).
- **Target Platforms:** Native Desktop.
- **LLM Interface:** The `llm` crate, leveraging its multi-backend support to communicate natively with on-prem models as well as remote models like OpenAI, Gemini, and Claude.

## 2. User Interface (UI) Requirements
### 2.1 Chess Board Visualization
- **Standard Layout:** 8x8 grid with traditional rank (1-8) and file (a-h) annotations on the edges.
- **Color Theme:** Must maintain distinguishability of light and dark squares under all highlighting conditions.
- **Perspective Toggle:** The board must be easily transposable between the White player's and the Black player's perspective.
- **Target Heat Map (Unique Feature):**
  - Displays the targeted range of pieces as an intensifying heat map.
  - Uses linear alpha accumulation up to a visual cap to increase the background color intensity of a square as more pieces target it.
- **Captured Pieces Indicators:**
  - Black pieces captured by White are displayed in an inventory on White's side of the board.
  - White pieces captured by Black are displayed in an inventory on Black's side.

### 2.2 Chat & Sidebar Interface
- **Layout:** Positioned adjacent to the chessboard.
- **AI Response Area:** Ample vertical space with rich text/markdown support to display the LLM's analytical output.
- **User Input Field:** A responsive, multi-line text field situated at the bottom of the chat interface for prompt entry.

## 3. Interaction and Game Logic
### 3.1 Mouse and Touch Controls
- **Selection:** First click/tap selects the candidate piece.
- **Move Highlighting:** Once selected, legal moves are highlighted with a transparent color overlay (distinct from the attack heat map).
- **Execution:** Second click/tap on a highlighted square commits the move.

### 3.2 Keyboard Input & Notation (Alternative Interface)
- Located below the chessboard or integrated cohesively.
- **Smart Highlighting:** Keyboard inputs match available pieces dynamically.
  - Example: Pressing `N` highlights all movable Knights.
  - Pressing the disambiguating file character selects the specific piece.
- **Deselection:** `Backspace` drops the current candidate piece selection.
- **Execution:** Following piece selection, entering the destination square triggers the move.

### 3.3 Target Modes
- **Shadow Mode (Default):** Used to "shadow" or mirror a game occurring on a physical board or external interface. Forward progression of moves.
- **Exploration Mode:**
  - Triggered via UI button.
  - Sandboxes the current state.
  - Fully supports `Undo` and `Redo` through a localized move tree.
  - "Exit" action discards the explored branch and snaps the board back to the true live state.

## 4. LLM Integration & Context Management
### 4.1 Communication Loop
- **Configuration:** Model selection and required API keys are loaded securely from a `.env` file.
- **Move Notification:** Upon any executed move, the LLM is pushed a notification string (e.g., `"black played [move]"`) alongside both FEN notation and an ASCII board representation to maximize context understanding.
- **Game State Pacing:** To prevent context fatigue or token limits, the system provides a comprehensive "summary" of the total board state alongside user prompts conditionally every `p` iterations.
- **Context Retention:** The LLM manages a continuous conversational thread matching the lifecycle of a game.

## 5. Persistence & Data Management
- **Database:** SQLite is the planned primary local storage mechanism, likely interfaced using the `sqlx` crate for safe, async queries.
- **Player Profiles:** Ability to assign custom names to the continuous players of White and Black (with suitable defaults).
- **Game Saving & Loading:** Users can name their individual games (with generated defaults like timestamps/metadata) to persist them.
  - Allows pausing a session to return to it later.
  - Offers a library of historical games to review.

## 6. Future Enhancements (Roadmap)
1. **Multi-Platform Ports (Flutter):** Transitioning the presentation layer to Flutter to facilitate deployment across Web, Android, and iOS platforms.
2. **Opening Library:** Built-in named standard openings and responses, integrated natively to aid learning and fast setup.
3. **Networked Sandbox (P2P):** Connect two active instances of Chaiss over the internet representing individual sides of the board.
