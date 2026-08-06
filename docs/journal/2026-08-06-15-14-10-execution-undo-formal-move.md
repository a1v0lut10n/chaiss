# Formal move undo wired to the UI

- **When:** 2026-08-06 15:14:10 local
- **Type:** execution
- **Project:** chaiss
- **Branch:** `feature/CHAI-0034-undo-formal-move`

## Context

Chaiss typically shadows a game played on a real board or chess.com, so a
faulty formal move happens; the only recovery was reloading the game.
`DbClient::undo_last_move` had existed in chaiss-core since the DB layer was
built but was never called from the UI.

## Details

A themed "⟲ Undo Move" button in the board's top bar (shown when the active
game has ≥1 committed move and the LLM is idle) runs the DB undo, then
resynchronizes via the regular resume path. That path was deduplicated into a
new `ChaissApp::spawn_game_resume` helper shared by cold-boot resume, roster
clicks, and post-undo resync. Undoing a logged result marker also reverts a
mistaken resignation. Verified live via the egui MCP with a throwaway game,
including a restart to prove DB persistence.

The button work grew into a top-bar polish pass. All prominent 28px buttons
(Resign, Undo, Create New Game) share one `prominent_button` body that is
painted directly rather than via `egui::Button` — the stock button applies
per-state `ButtonStyle` padding, so its width and label shift by a couple of
pixels on hover, nudging neighboring controls. The painted version allocates a
pinned `max(min_w, text) x 28` rect and centers its galley every frame; hover
changes only the border color. They plus the session cards gained a subtle
hover hint (a slightly brightened border via a paint-only
`paint_hover_border` helper); for the cards this keys on pointer containment
because egui's hover arbitration inside the scroll area never flags the row's
`interact` response — which had also silently disabled the trash icon's red
hover tint since it was introduced. The Analysis Overlay combo matches the
28px height via `interact_size`, sits in the top row as a loose item so it
centers with the buttons, and is sized (132px, with a 15pt label) to keep its
arrow inside the panel at default window width. All hover behavior was
verified by screenshot pixel-diffing through the egui MCP: zero interior
pixels change on hover, only the border ring.

## Links

- Related entry: `2026-08-06-12-56-53-execution-session-roster-cards.md`
- Related entry: `2026-08-06-12-41-09-execution-persist-board-flip.md`
