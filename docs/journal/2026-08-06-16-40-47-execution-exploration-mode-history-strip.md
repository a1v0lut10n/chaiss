# Exploration-mode labels and a responsive move-history strip

- **When:** 2026-08-06 16:40:47 local
- **Type:** execution
- **Project:** chaiss
- **Branch:** `feature/CHAI-0036-exploration-mode-history-strip`

## Context

Sandbox navigation exposed raw internals: an "Enable Forward Sandbox"
checkbox, a shouting "EXPLORATION MODE ACTIVE (Not saving to Database)"
banner, and no way to see where in the game you were while stepping through
history.

## Details

Both labels are now simply "Exploration mode". A new horizontal move-history
strip sits above the navigation buttons: the current move is anchored at the
pane center, bright and slightly magnified (16pt `TEXT_BRIGHT`); past and
future moves fan outward, each step smaller and dimmer (`gamma_multiply`
fade), laid out until the pane width is spent so the visible count adapts to
the window. Exploration plies (which record only FENs) render as `···`, the
root as "Start". Predictive-matrix arrows are now drawn only at the live
head — the predicted continuation starts from the latest position, so it
doesn't apply to past or sandbox views. Verified live via the egui MCP,
including width responsiveness by resizing 1200→900 and the arrows
disappearing/reappearing when leaving/returning to live.

## Links

- Related entry: `2026-08-06-15-58-48-execution-predictive-arrow-colors.md`
