# CHAI-0047 — refresh the user guide and README against 0.5.0

- **Ticket:** CHAI-0047
- **Branch:** `docs/CHAI-0047-user-guide-refresh`
- **Status:** implemented (PR pending review)

## Premise

`docs/user_guide.md` dates from April and describes an interface that no
longer exists: "+ New Match", "Start Game", "Session Roster Panel",
plain Enter to send, green-coloured squares, an LLM that is "always
analyzing", and no mention of the model dropdown, the overlays, the
sandbox navigation, the predicted arrows, undo, resign, flip or delete.
Its one screenshot shows the old chrome. The README's feature list has
the same drift (heat map as square fills, "undo/redo", "notation
highlighting", "egui 0.35"), and its User Guide section is a paragraph
of prose nobody clicks.

## Action items

1. Inventory the current UI from source (every on-screen string and
   behaviour), then compare both documents against it.
2. Capture new screenshots through the egui MCP inspection session on
   the real app at 1500×1000 logical points: the Deep Blue vs Kasparov
   1996 game 1 session at its final position (37.Rxh7+, on which
   Kasparov resigned; the same position the old screenshot showed),
   the Initialize Match dialog, the Analysis Overlay menu, the
   Predictive Matrix overlay, a selected king with its one legal
   destination, the sandbox strip in exploration mode, and the chat
   pane. Store them under `static/screenshots/`; drop the old
   `static/Deep_Blue_vs_Kasparov-1996-Game-1.png`.
3. Rewrite `docs/user_guide.md` section by section from the inventory:
   window layout, creating and resuming games, PGN loading, mouse and
   typed moves, the toolbar, the overlays, the chat pane (model choice,
   what the model receives, auto-analysis and the silence checkbox,
   the PREDICTIVE MATRIX line and arrows, errors), exploration mode,
   the command line. State the sharp edges plainly: delete has no
   confirmation, PGN applies on top of the current position, typed
   moves always go to the live game.
4. README: rewrite the Key Features bullets to match, correct the
   dropdown statement (hidden with a single provider; Ollama only as
   the configured backend), fix "egui 0.35" to 0.36, and replace the
   User Guide paragraph with the overview screenshot scaled to 800 px,
   linked to the guide. Drop the stale "egui 0.35" code comment in
   `left_panel.rs`.
5. CHANGELOG `[Unreleased]` entry under Changed.

## Verification checklist

- [x] Every image path in the guide and README resolves to a file in
      the tree.
- [x] Every on-screen string quoted in the guide matches the source
      (inventory cross-check).
- [x] fmt / clippy / tests unchanged (one code comment touched).
- [x] The inspection app was stopped and the database left as found:
      no moves, games or chat rows added by the screenshot session
      (dialog cancelled, exploration branch dropped, selections cleared).

## Observations for follow-up (not changed here)

- `left_panel.rs` creates a game with the currently displayed FEN as
  its root even though the UI resets to the start position; harmless
  today because resume replays from the standard root, but misleading.
- Deleting a game from the roster has no confirmation.
- A move typed into the chat box bypasses exploration mode and is
  written to the live game.
- A PGN paste applies on top of the current position rather than from
  a reset.
