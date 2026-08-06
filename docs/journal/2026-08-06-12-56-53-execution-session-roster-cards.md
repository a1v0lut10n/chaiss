# Brainforge-style theme: session cards and Resign button

- **When:** 2026-08-06 12:56:53 local
- **Type:** execution
- **Project:** chaiss
- **Branch:** `feature/CHAI-0033-session-roster-cards`

## Context

The roster rendered sessions as wrapped selectable labels with a trailing
trash button, so the icons never aligned, and Resign was bright-red heading
text on a default button. Adopted the brainforge-app "Feed + Desk" visual
language (its skills-view cards and button styles).

## Details

New `ui/theme.rs` carries a hand-synced subset of aicogito-ui's tokens and
widgets (card/chip surfaces, text tiers, accent, primary/standard buttons,
section labels) plus its `frame_with_corner_click` pattern — the card senses
clicks across its full width while the corner trash icon, registered last,
wins hit-testing. Sessions render as two-line cards (name + players) with
right-aligned delete icons; the active card uses the chip-blue surface with
the accent name. Resign became a `danger_button`: standard button body with a
faint red border and soft red text. Verified live via the egui MCP.

## Links

- Origin: aicogito `crates/aicogito-ui/src/tokens.rs`, `widgets.rs`
- Related entry: `2026-08-06-12-41-09-execution-persist-board-flip.md`
