# User guide refreshed through the inspection session

- **When:** 2026-10-13 12:29:41 local
- **Type:** execution
- **Project:** chaiss

## Context

The user guide dated from April and described an interface that no
longer existed ("+ New Match", "Start Game", plain Enter to send, green
squares, an always-analyzing LLM), missing the model dropdown, the
overlays, the sandbox navigation, the predicted arrows, undo, resign,
flip and delete. The README feature list had drifted the same way, and
its one screenshot showed the old chrome.

## Details

CHAI-0047. A source inventory of every on-screen string and behaviour
drove a section-by-section rewrite of `docs/user_guide.md` and the
README feature list, stating the sharp edges plainly (delete has no
confirmation, PGN applies on top of the current position, typed moves
always go to the live game). Screenshots retaken through the egui MCP
inspection session on the real app at 1500×1000 logical points,
resuming the stored Deep Blue vs Kasparov 1996 game 1 at 37.Rxh7+ (the
position the old screenshot showed) and walking the Initialize Match
dialog, the overlay menu, the Predictive Matrix overlay, a selected
king, exploration mode and the chat pane, with every step cancelled or
reverted and the database row counts identical before and after. The
README's User Guide section now carries the overview screenshot linked
to the guide; "egui 0.35" corrected to 0.36 in the README and a code
comment. Four behaviours noticed along the way are recorded in the
plan as ticket candidates: the new-game root FEN, delete without
confirmation, typed moves bypassing exploration mode, and PGN not
resetting the board. PR #67.

## Lesson

The inspection session is the right tool for documentation
screenshots: the same position, the same window size and the same
state every time, and the tree queries confirm what the picture shows.
The egui MCP binary built against the previous egui minor still worked
against 0.36 here, but the workspace rule to reinstall it on an egui
bump stands.

## Links

- Plan: `docs/implementation/CHAI-0047-user-guide-refresh.md`
