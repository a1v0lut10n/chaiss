# CHAI-0043 — persist the last-used LLM target per game

- **Ticket:** CHAI-0043
- **Branch:** `feature/CHAI-0043-per-game-model`
- **Status:** implemented (PR pending review)

## Premise

The chat pane's model selection (CHAI-0040) is session-scoped: every
launch starts from the configured default. The last *used* model should
be remembered per game — resuming a game selects the model that served
its previous request, provided that model is still configured.

## Action items

1. Migration `..._add_last_llm_target.sql`:
   `ALTER TABLE games ADD COLUMN last_llm_target TEXT;` (nullable,
   additive — existing databases migrate automatically on launch, like
   `flip_board` did).
2. `DbClient::set_last_llm_target(game_id, &str)` and
   `get_last_llm_target(game_id) -> Option<String>`, following the
   `flip_board` pattern; regenerate the sqlx offline metadata
   (`cargo sqlx prepare` against a scratch database with migrations
   applied — never the real `chaiss.db`, whose migration bookkeeping
   must stay owned by the app).
3. Storage format `provider/model` (e.g. `openai/gpt-6-astra`).
   chaiss-core grows `LlmTarget::storage_key()`, `LlmProvider`
   token parsing, and `select_stored_target(&[LlmTarget], &str) ->
   Option<usize>` — exact provider+model match first, else the same
   provider with a different model (covers a changed model override),
   else `None` (stored target no longer configured; the default
   stands). Unit-tested in chaiss-core.
4. Write path: when an LLM request is dispatched for the active game
   (the `InferenceRequested` handler that already logs the chat
   message), persist `payload.target` — "last used", not merely
   clicked.
5. Read path: `spawn_game_resume` also loads the stored target; the
   `GameResumed` event carries it and the handler applies
   `select_stored_target` to the configured `llm_targets`.

## Verification checklist

- [x] fmt / clippy `-D warnings` / `cargo test --workspace` clean
      (45 tests; 3 new for storage round-trip, same-provider fallback,
      unconfigured-provider and garbage input). Offline sqlx metadata
      regenerated against a scratch database.
- [x] Live egui MCP check against a throwaway game: selected
      `gpt-6-astra (OpenAI)`, sent a prompt (the 401 chat error named
      that target, proving it served the request), switched the
      dropdown back to Google, resumed the game — the dropdown
      restored `gpt-6-astra (OpenAI)`, and the database row read
      `openai/gpt-6-astra`. The unconfigured-provider fallback is
      covered by unit test (`select_stored_target` returns `None` and
      the handler leaves the selection unchanged). Test game deleted
      afterwards; no residual rows.

## Out of scope

- Persisting a selection that was never used for a request.
- Global (non-per-game) persistence of the selection.
