# LLM model selector in the chat pane

- **When:** 2026-09-14 15:51:34 local
- **Type:** execution
- **Project:** chaiss
- **Ticket:** CHAI-0040

## Context

Chaiss was hard-wired to a single LLM backend chosen by `LLM_BACKEND`,
re-read from the environment on every request; the UI had no say. Goal:
add OpenAI's GPT-6 Astra (`gpt-6-astra`, released September 2026) to
the mix and let the user switch models from a dropdown in the chat pane
whenever more than one backend is configured in `.env`.

## Details

chaiss-core grew `LlmProvider`/`LlmTarget` (displayed as
`model (Provider)`, matching the error-message style) and
`configured_targets()`, which enumerates backends by configured API key
— the `GEMINI_API_KEY` alias is honored, the `.env.example` placeholder
`your_key_here` counts as unset, and keyless Ollama is offered only
when `LLM_BACKEND=ollama`. Model resolution is a pure function over an
env-lookup closure (so tests never mutate process env) with precedence
`LLM_MODEL_<PROVIDER>` → legacy `LLM_MODEL` (only for the
`LLM_BACKEND` provider, keeping existing setups working) → per-provider
default. `LlmPromptPayload` carries a required `target` and
`stream_llm_response` uses it instead of re-reading env per request;
API keys are still read from env at request time and never enter the
payload. The OpenAI default model moved from `gpt-4-turbo` to
`gpt-6-astra`.

The chat pane renders a ComboBox under the "LLM Chat & Analysis"
heading only when more than one target is configured; the selection is
session-scoped, defaulting from `LLM_BACKEND` at launch. Per-game
persistence of the selection is deferred and noted as future work in
the implementation plan.

Seven new chaiss-core unit tests (enumeration, resolution precedence,
alias/placeholder handling, Ollama opt-in, display format); fmt,
clippy `-D warnings`, and all 37 workspace tests pass. Verified live
via the egui MCP loop: the dropdown appears with two keys configured
and switches to `gpt-6-astra (OpenAI)`; a prompt sent with a dummy key
failed with the 401 chat error naming exactly that target — proof the
request follows the dropdown — and a single-backend launch shows no
dropdown.

## Links

- PR: https://github.com/a1v0lut10n/chaiss/pull/53
- Plan: `docs/implementation/CHAI-0040-llm-model-selector.md`
- Related entry: `2026-09-04-10-51-03-execution-predictive-matrix-castling.md`
