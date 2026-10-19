# Gemini 3.8 Flash default and a billing message that says billing

- **When:** 2026-10-19 17:03:31 local
- **Type:** execution
- **Project:** chaiss

## Context

A game turn reported `gemini-3.7-flash (Google)` as "rate-limiting
requests or your quota is exhausted", and Google had deprecated that
model, so the two were assumed connected.

## Details

Probing the API directly on 2026-10-16 showed both `gemini-3.7-flash` and
`gemini-3.8-flash` still listed, and both returning HTTP 402 "Your
prepayment credits are depleted" with status `RESOURCE_EXHAUSTED`, the
string `classify_backend_error` used for the rate-limit case. The default
moved to `gemini-3.8-flash` (constant, tests, examples, user guide); a
model pinned in `.chaiss.cola` or `LLM_MODEL_GOOGLE` still overrides it,
which is exactly what the local config on this machine did. The classifier
now tests 402 / "credits are depleted" before rate limiting and names the
billing cause, with Google's verbatim body as the fixture. A live 3.8 reply
remains unverified until the balance is topped up.

## Links

- PR: https://github.com/a1v0lut10n/chaiss/pull/68
- Plan: `docs/implementation/CHAI-0048-gemini-3-8-flash.md`
- Related entry: `2026-08-30-10-33-10-execution-llm-outage-messages-gemini-3-7-flash.md`
