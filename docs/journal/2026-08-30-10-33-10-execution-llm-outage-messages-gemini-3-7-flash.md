# Intelligible LLM outage messages and Gemini 3.7 Flash default

- **When:** 2026-08-30 10:33:10 local
- **Type:** execution
- **Project:** chaiss

## Context

Gemini began returning `503 UNAVAILABLE — This model is currently
experiencing high demand`, which surfaced verbatim in the chat panel wrapped
in `[System Error: …]`. The default `gemini-3.5-flash` had also been
superseded.

## Details

`stream_llm_response` now returns a typed `LlmError` splitting a
plain-language `user_message` from the raw backend `detail`;
`classify_backend_error` maps HTTP 503/429/401/403/404/5xx and transport
failures to distinct messages naming the model and provider. The app logs
`detail` to stderr and pushes only `user_message` into the chat.

Default Google model moved to `gemini-3.7-flash` (GA, August 2026), chosen
over `gemini-3.1-pro-preview` for higher aggregate reasoning scores
(Artificial Analysis Intelligence Index 56 vs 48, GPQA within 1.5 pts) and GA
availability, which matters directly for the high-demand 503s. `LLM_MODEL`
overrides the per-backend default without a rebuild, and `GEMINI_API_KEY` is
accepted alongside `GOOGLE_API_KEY` (`.env.example` had named the former
while the code only read the latter).

Verified live via the egui MCP: with `LLM_MODEL=gemini-does-not-exist` the
chat showed the friendly "model was not found" message while the console
carried the raw 404 body; with the default, `gemini-3.7-flash` answered a
prompt first try with a full predictive matrix. Unit tests cover the exact
Gemini 503 text and the other status classes.

## Links

- PR: https://github.com/a1v0lut10n/chaiss/pull/40
- Related entry: `2026-08-06-17-59-19-execution-release-0-2-0.md`
