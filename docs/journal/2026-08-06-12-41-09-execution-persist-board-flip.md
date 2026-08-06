# Persist board flip per game session

- **When:** 2026-08-06 12:41:09 local
- **Type:** execution
- **Project:** chaiss
- **Branch:** `feature/CHAI-0032-persist-board-flip`

## Context

The "Flip Board (Play as Black)" toggle was ephemeral UI state: resuming a
game where the user played black always reopened white-side-down. The
orientation belongs to the game, so it now lives in the `games` table.

## Details

Migration `20260806000001_add_flip_board.sql` adds a `flip_board` column
(`INTEGER NOT NULL DEFAULT 0`), applied automatically at startup so databases
created under the previous schema upgrade in place with existing sessions
defaulting to unflipped. `DbClient::set_flip_board`/`get_flip_board` persist
and read it; the checkbox writes on change, and both resume paths (cold-boot
auto-resume and roster click) restore it via a new `flip_board` field on
`DbEvent::GameResumed`. New games created while flipped are stamped at
creation. Regenerated the `.sqlx` offline cache for the two new queries.
Verified live via the egui MCP: flip → restart → resumes flipped; sibling
sessions unaffected.

## Links

- Related entry: `2026-07-13-17-07-21-execution-enable-egui-mcp-inspection.md`
  (the tooling used to verify)
