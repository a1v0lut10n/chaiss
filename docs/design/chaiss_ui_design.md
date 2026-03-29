# Chaiss UI/UX Design Specification

This document details the visual layout, interactions, component logic, and styling for the Chaiss user interface built with `egui`.

## 1. Top-Level Layout Strategy
The primary interface will be split into a resizable three-pane layout:
* **Left Pane (Game Roster Sidebar):** A permanent vertical panel listing active and recorded games, providing quick navigation, creation, and deletion of matches.
* **Center Pane (Game Area):** Centers the chessboard, scaling responsively while maintaining a 1:1 aspect ratio. Contains the keyboard input overlay or algebraic input field anchored at the bottom.
* **Right Pane (Chat & Context Sidebar):** A fixed-minimum-width vertical panel hosting the LLM chat interface, captured pieces inventories, and game controls (Undo/Redo, Exploration toggle).

## 2. Rendering the Chessboard
* **Coordinate Annotations:** Ranks (1-8) and Files (a-h) rendered outside the 8x8 grid bounds.
* **Responsive Scaling:** Each square's size (`cell_size`) is calculated continuously during the `egui` recalculation frame to ensure the board fits dynamically within the Game Area pane.
* **Vector Pieces:** Uses scale-independent SVGs or a specialized chess font to ensure pieces remain crisp at any window size.

### 2.1 The Heat Map Overlay Calculation
* For each square, an integer `attack_count` is provided by the game engine.
* **Color Blending:** 
  - `Base Color`: The light or dark square color.
  - `Heat Color`: A distinct target color (e.g., deep amber or crimson).
  - `Alpha Calculation`: `alpha = min(max_alpha, attack_count * step_alpha)`.
  - The `Heat Color` is drawn over the `Base Color` using the computed `alpha`, ensuring the checker pattern remains visible beneath the heat.

## 3. Interaction Mechanics

### 3.1 Mouse / Touch Flow
1. **Hover:** Slight highlight or lifted drop-shadow effect on pieces.
2. **First Click (Selection):** Selects the piece. Legal destination squares receive a transparent circular overlay (distinct from the heat map).
3. **Second Click (Commit):** Triggering the move logic. If the click lands on an illegal square or another friendly piece, selection is dropped or swapped.

### 3.2 Keyboard Input Flow (Algebraic Notation)
* Always active if the user begins typing, or focused via a dedicated input field below the board.
* **Smart Filtering:** 
  - Typing `N` instantly highlights all knights that have legal moves.
  - Adding `f` (so it reads `Nf`) dims knights not on the 'f' file.
  - Adding `3` commits the move if it logically resolves to a single valid piece and destination.
* **Visual Feedback:** Keystrokes are visually queued next to the board. `Backspace` clears the buffer and drops highlights smoothly.

## 4. Specific Panels & States

### 4.1 Exploration Mode UX
* When enabled, a prominent visual indicator (e.g., an amber banner) appears across the top of the board: *"Exploration Mode Active"*.
* The Chat sidebar temporarily pauses LLM live game responses, perhaps offering an isolated "variations chat" thread.
* Two buttons appear: **[Commit Variation]** (Rare, if altering the real game) and **[Discard & Return]** (Standard, snaps back to real game state).

### 4.2 LLM Chat Interface
* **Message Bubbles:** Alternating layout (Left-aligned for AI, Right-aligned for user prompts).
* **Rich Output:** The `egui` Markdown parser will be leveraged to format the LLM's analytical output (bolding piece names, creating code blocks for lists or variations).
* **Prompt Field:** A pinned multi-line input box at the bottom of the right pane functioning similarly to standard chat apps (Enter to send, Shift+Enter for newline).

## 5. Game & Profile Management UX

### 5.1 Permanent Game Navigation (Left Sidebar)
* **Game Roster:** A permanent, scrollable list housed in the Left Pane showing active and recorded games. Displays metadata such as Game Name, Date/Time, and Player Matchups.
* **Actions:** The sidebar offers dedicated buttons to **Create New Game**, **Select** an existing game, and **Delete** a game.
* **Context Switching:** Selecting any game from the roster instantly syncs the Center Pane (board state) and Right Pane (chat context) to reflect the chosen game's current reality.

### 5.2 Game Creation & Profile Naming
* **New Game Modal:** Clicking "Create New Game" summons a focused modal window prompting for the 'Game Name' and both 'White/Black Player Names' (with sensible default titles). 
* **Editing Details:** Users retain the ability to edit these profile and game names at any point during the game's lifecycle via an inline edit option.
* **In-Game Display:** The active player names are persistently overlaid on the top and bottom visual boundaries of the Center Pane, ensuring clear contextual awareness of who is playing whom.
