# CHAI-0040 — LLM model selector in the chat pane

- **Ticket:** CHAI-0040
- **Branch:** `feature/CHAI-0040-llm-model-selector`
- **Status:** implemented (PR pending review)

## Premise

The LLM backend is picked once via `LLM_BACKEND` and re-read from the
environment inside `stream_llm_response` on every request; the UI has no
say. We want chaiss to enumerate the backends actually configured in
`.env`, default OpenAI to `gpt-6-astra` (GPT-6 Astra, released September
2026), and let the user switch models from a dropdown in the chat pane
whenever more than one backend is configured.

## Action items

### chaiss-core

1. New `LlmProvider` enum (`Google`, `OpenAI`, `Anthropic`, `Ollama`)
   and `LlmTarget { provider, model }`, displayed as
   `model (Provider)` — matching the existing error-message style.
2. `configured_targets()`: one target per provider whose API key is
   present (Google accepts `GOOGLE_API_KEY` or `GEMINI_API_KEY`; Ollama
   is keyless and included only when `LLM_BACKEND=ollama`). Model
   resolution per provider: `LLM_MODEL_<PROVIDER>` override → existing
   `LLM_MODEL` (applies only to the `LLM_BACKEND` provider, for
   back-compat) → per-provider default. Resolution is a pure function
   over an env-lookup closure so tests never mutate process env.
3. `default_target()` honors `LLM_BACKEND` (google by default), falling
   back to the first configured target.
4. `LlmPromptPayload` gains a required `target: LlmTarget` field;
   `stream_llm_response` uses it instead of re-reading
   `LLM_BACKEND`/`LLM_MODEL`. API keys are still read from env at
   request time — never carried in the payload (it derives `Debug`).
5. `DEFAULT_OPENAI_MODEL`: `gpt-4-turbo` → `gpt-6-astra`.

### chaiss

6. App state: `llm_targets: Vec<LlmTarget>` plus a selected index,
   initialized in `ChaissApp::new()`; the four payload construction
   sites (right_panel ×3, board ×1) pass the selection — the new
   required field makes the compiler enforce this.
7. UI: an egui `ComboBox` directly under the "LLM Chat & Analysis"
   heading, rendered only when more than one target is configured.
   Selection is session-scoped, defaulting from env each launch
   (per-game persistence in the DB is future work, not this ticket).
8. `.env.example` documents the per-provider model overrides and that
   setting several keys enables the dropdown.

## Verification checklist

- [x] `cargo fmt --all -- --check`, `cargo clippy --workspace
      --all-targets -- -D warnings`, `cargo test --workspace` clean
      (37 tests, 7 new).
- [x] Unit tests cover target enumeration (which keys → which targets),
      model resolution precedence, the `GEMINI_API_KEY` alias, the
      `your_key_here` placeholder counting as unset, and the Ollama
      opt-in.
- [x] Live egui MCP check: with two keys configured the dropdown
      appears under the chat heading; after selecting
      `gpt-6-astra (OpenAI)` a prompt sent with a dummy key produced
      the 401 chat error naming exactly that target — proof the
      request uses the dropdown selection. With one key configured no
      dropdown is shown.
- [ ] End-to-end GPT-6 Astra call with a real `OPENAI_API_KEY` (user).

## Risk

OpenAI's docs pitch Astra via the Responses API; the `llm` crate
(1.3.7) OpenAI backend speaks chat completions. OpenAI has so far kept
chat-completions compatibility for flagship models — if `gpt-6-astra`
rejects it, fall back to bumping the `llm` crate or routing via
`base_url` the way the Gemini path already does.
