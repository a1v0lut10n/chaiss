# Literate cola configuration for chaiss

- **When:** 2026-09-14 17:20:20 local
- **Type:** execution
- **Project:** chaiss
- **Ticket:** CHAI-0042

## Context

LLM configuration lived only in `.env`. Chaiss now also reads
`.chaiss.cola` — dotted and gitignored like `.env` — parsed with the
workspace's own colap crate, taking precedence over `.env` per key.
The committed `.chaiss.cola.example` is a literate markdown document
that is itself a valid configuration: colap's grammar is
markdown-native, so the fenced cola block is the config and the prose
documents it. Copying it to `.chaiss.cola` just works, and the copy
may stay literate.

## Details

A new `chaiss-core::cola_config` module reads the file once at startup
into a `OnceLock` overlay of the same logical keys the `.env` lookup
uses (`LLM_BACKEND`, `<P>_API_KEY`, `LLM_MODEL_<P>`); target
enumeration, the default target, and request-time API-key resolution
all go through the layered `config_value` lookup (cola first, env
fallback). Placeholder (`your_key_here`) and empty values never enter
the overlay — strict precedence would let the example's placeholders
mask real `.env` keys the moment the example is copied, observed live
before the rule was added. A malformed file is reported on the console
and ignored. Fence-less cola input is wrapped in a synthetic
```` ```cola ```` fence before parsing, since colap's top grammar rule
treats it as ignorable prose otherwise.

Six new unit tests, including one that parses `.chaiss.cola.example`
itself so the literate example can never drift from what the code
understands; fmt, clippy `-D warnings`, and all 43 workspace tests
pass. Verified live via the egui MCP loop: a `.chaiss.cola` with
`backend: "openai"`, a dummy OpenAI key, and model `gpt-6-astra-cola`,
against a `.env` holding only a real Google key, produced a two-entry
dropdown initially selecting `gpt-6-astra-cola (OpenAI)` — cola's
backend and model overrides won, and the placeholder Google key fell
through to the real env key.

## Meta-impact (colap)

Candidates for colap-side tickets, found while integrating:

- Fence-less cola parses as ignored prose (top rule is markdown
  items) — silently yielding an empty model instead of an error.
- A comma continues a field list, so a nested entity may not follow a
  trailing comma — an easy authoring trap worth a clearer diagnostic.
- A prose line in a literate document may not begin with a backtick
  (`ParagraphLine` excludes it), which constrains inline-code layout.
- rustemo's debug-build parse trace (~6,700 lines per parse to
  stderr) is on by default; chaiss silences it via `RUSTEMO_NOTRACE=1`
  at startup — a quieter default in colap would serve every consumer.

## Links

- PR: https://github.com/a1v0lut10n/chaiss/pull/55
- Plan: `docs/implementation/CHAI-0042-cola-configuration.md`
- Related entry: `2026-09-14-15-51-34-execution-llm-model-selector.md`
