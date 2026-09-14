# Per-game persistence of the last-used LLM target

- **When:** 2026-09-14 17:28:51 local
- **Type:** execution
- **Project:** chaiss
- **Ticket:** CHAI-0043

## Context

The chat pane's model dropdown (CHAI-0040) was session-scoped: every
launch started from the configured default. Each game now remembers
the model that served its most recent LLM request, and resuming the
game restores that selection — provided the model is still configured.

## Details

An additive migration gives the games table a nullable
`last_llm_target` column, written as a `provider/model` storage key
(e.g. `openai/gpt-6-astra`) whenever a request is dispatched for the
active game — "last used", not merely clicked. On resume,
`select_stored_target` matches the stored key against the configured
targets: the exact provider+model pair wins, a target on the same
provider with a different model is next (covering a changed model
override), and an unconfigured provider leaves the current selection
unchanged. `LlmTarget` grew `storage_key()`; the `GameResumed` event
carries the stored value; the sqlx offline metadata was regenerated
against a scratch database so the real `chaiss.db`'s migration
bookkeeping stays owned by the app.

Three new unit tests (storage round-trip, same-provider fallback,
unconfigured provider and garbage input); fmt, clippy `-D warnings`,
and all 45 workspace tests pass. Verified live via the egui MCP loop
against a throwaway game: after a request served by
`gpt-6-astra (OpenAI)` and a manual switch back to Google, resuming
the game restored the OpenAI selection, with `openai/gpt-6-astra` in
the database row. The test game was deleted afterwards, leaving no
residual rows.

## Links

- PR: https://github.com/a1v0lut10n/chaiss/pull/56
- Plan: `docs/implementation/CHAI-0043-per-game-llm-target.md`
- Related entry: `2026-09-14-15-51-34-execution-llm-model-selector.md`
- Related entry: `2026-09-14-17-20-20-execution-cola-configuration.md`
