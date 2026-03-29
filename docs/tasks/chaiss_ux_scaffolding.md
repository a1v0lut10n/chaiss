# Implementation Plan: Chaiss UX Scaffolding

This document details the step-by-step strategy for translating our three-pane UI design into the `egui` framework. The goal is to establish the layout containers and stub the core visual elements (including a blank responsive chessboard) so that game logic and heat maps can be cleanly integrated in the next phase.

## Phase 1: Structuring UI Modules
To keep `app.rs` clean, we will modularize the UI components. We will create a new directory structure inside the `desktop` crate:
- `desktop/src/ui/mod.rs`
- `desktop/src/ui/left_panel.rs` (Game Roster & Navigation)
- `desktop/src/ui/right_panel.rs` (LLM Chat & Controls)
- `desktop/src/ui/board.rs` (Chessboard Canvas & Heat Map rendering)

## Phase 2: Building the Layout Containers
We will update `desktop/src/app.rs` to invoke the separated UI modules within their distinct `egui` panels, ensuring they are called in the correct order:
1. `egui::SidePanel::left("roster_panel")` calls `left_panel::draw(...)`
2. `egui::SidePanel::right("chat_panel")` calls `right_panel::draw(...)`
3. `egui::CentralPanel::default()` calls `board::draw(...)`

## Phase 3: Implementing Component Stubs
### 1. Left Panel (Game Roster)
- Implement a basic heading ("Games").
- Stub out standard `egui::Button` elements for "Create New Game", "Select Game", and "Delete Game".
- Implement a temporary `egui::ScrollArea` placeholder showing mock historical matches.

### 2. Right Panel (Chat Interface)
- Implement a basic layout with a flexible `ScrollArea` taking up the majority of vertical space for the chat history.
- Pin a `TextEdit::multiline` input box at the bottom for typing prompts.
- Establish visual boundaries for AI vs Human text bubbles.

### 3. Center Panel (The Board)
- Use `egui::Painter` to allocate a perfectly square rendering area within the flexible central bounds.
- Calculate the `cell_size` dynamically based on the window's current dimensions.
- Use explicit coordinates to draw the alternating 8x8 squares (`Rect::from_min_max`) to serve as the foundation for both the physical pieces and the future Heat Map alpha accumulation.

## Phase 4: Verification
- Execute `cargo run --bin desktop`.
- Verify the main window correctly partitions into 3 functional zones.
- Continuously resize the window to confirm the central chessboard area scales responsively while retaining its 1:1 format constraint.
