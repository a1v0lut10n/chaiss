# Chaiss UX Layout Scaffolding

This formalizes the implementation plan we've drafted locally in `docs/tasks/chaiss_ux_scaffolding.md` to map out our permanent three-pane layout.

## User Review Required
Please review the proposed approach below. Once you approve this UI scaffolding plan, we will execute it, physically drawing the panels and the placeholder chessboard onto the screen!

## Proposed Changes

### [desktop] UI Modularity Refactoring
We will abandon the single `ui.rs` file in favor of a dedicated `desktop/src/ui/` module directory:
#### [NEW] desktop/src/ui/mod.rs
#### [NEW] desktop/src/ui/left_panel.rs
#### [NEW] desktop/src/ui/right_panel.rs
#### [NEW] desktop/src/ui/board.rs

### [desktop] Implementing App Layout
#### [MODIFY] desktop/src/app.rs
- Inject `egui::SidePanel::left` handling the Game Roster.
- Inject `egui::SidePanel::right` handling the LLM Chat UI.
- Inject `egui::CentralPanel::default` rendering the responsive, 1:1 aspect ratio Chessboard layout using `egui::Painter`.

## Verification Plan

### Manual Verification
1. Compilation: Run natively via `cargo run --bin desktop`.
2. UI Resize Testing: Upon launching the desktop application, rapidly resize the window.
- The left and right sidebars should maintain their constraints.
- The central chessboard should grow and shrink dynamically, but ALWAYS remain perfectly square (1:1 aspect ratio constraint required for accurate piece/heat map mapping).
