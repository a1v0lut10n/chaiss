# Predictive arrows colored by side to match the overlay

- **When:** 2026-08-06 15:58:48 local
- **Type:** execution
- **Project:** chaiss
- **Branch:** `feature/CHAI-0035-predictive-arrow-colors`

## Context

The AI's predicted continuation rendered as uniform transparent cyan arrows,
disconnected from the analysis overlay's color language (blue = White's power
sphere, red = Black's, purple where they blend).

## Details

`ai_predictive_arrows` tuples gained the moving side, captured from the
simulation state's `active_color` before each predicted ply is applied — in
both parse sites (post-inference and chat-history resume). The renderer uses
the overlay's hues: transparent blue for White's predicted moves, transparent
red for Black's. Transparency increases with depth — alpha ramps linearly
from 230 for the next ply to 55 for the deepest — and the color is built
unmultiplied, fixing the old premultiplied misuse that rendered the arrows
over-bright and muted the fade. Verified live via the egui MCP on a resumed
game whose stored matrix alternates blue/red per mover with a clearly visible
depth gradient.

## Links

- Related entry: `2026-08-06-15-14-10-execution-undo-formal-move.md`
