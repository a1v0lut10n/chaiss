# CHAI-0042 — cola configuration with .env fallback

- **Ticket:** CHAI-0042
- **Branch:** `feature/CHAI-0042-cola-config`
- **Status:** implemented (PR pending review)

## Premise

LLM configuration currently lives only in `.env`. Adopt the workspace's
own cola configuration language (the published `colap` crate, whose
README example is precisely an LLM provider configuration): a dotted
`.chaiss.cola` in the project directory, gitignored like `.env`, taking
precedence over `.env` per key when present. Because colap's grammar is
markdown-native (```cola fenced blocks are part of the language, plain
cola files parse with the same parser), the committed example
`.chaiss.cola.example` is a literate markdown document that is itself a
valid config: `cp .chaiss.cola.example .chaiss.cola` just works and the
user's real config stays literate.

## Action items

1. Add `colap = "0.2"` to chaiss-core (the workspace's own config
   language; nothing in std or the existing dependency set parses it).
2. New `chaiss-core::cola_config` module:
   - `overlay_from_str(content) -> Result<HashMap<String, String>, String>`
     parses cola (plain or markdown-embedded) via `ColaParser` +
     `ModelBuilder` and probes the known paths — `llm` field `backend`
     → `LLM_BACKEND`; `llm/provider/<p>` fields `api_key` →
     `<P>_API_KEY` and `model` → `LLM_MODEL_<P>` for the fixed provider
     set google/openai/anthropic/ollama (colap stores plural entities
     under their singular name, hence `provider` in the path).
   - Grammar findings from implementation: colap's top rule is
     `Cola: MarkdownItem*`, so fence-less cola parses as ignored prose
     rather than erroring — `overlay_from_str` wraps input lacking a
     ```` ```cola ```` fence in a synthetic one so plain cola truly
     works. Two authoring quirks documented in the example: a comma
     continues a field list (so no trailing comma before a nested
     entity), and a prose line may not begin with a backtick.
   - `init()` reads `.chaiss.cola` from the working directory once at
     startup into a `OnceLock<HashMap<String, String>>` (plain
     `Send + Sync` data, written before the frame loop, read-only
     after — per the egui-threading rules; a small config file read at
     startup is the allowed case). A malformed file never crashes:
     console error, env-only fallback.
3. Layered lookup `config_value(key)` — cola overlay first, then env —
   wired into `configured_targets()`, `env_default_target()`, and the
   API-key resolution inside `stream_llm_response` (keys may now exist
   only in cola). The existing pure `*_from(lookup)` functions and the
   `your_key_here` placeholder rule apply unchanged to cola values.
4. `.chaiss.cola.example`: literate markdown — prose introducing the
   file, per-section explained ```cola blocks (backend default,
   providers with keys and model overrides), closing with the
   precedence rules.
5. `.gitignore` += `.chaiss.cola`; README gains a short Configuration
   pointer at the example.
6. Tests: cola→overlay mapping (plain and markdown-embedded input),
   layering precedence (cola beats env, env fills gaps), malformed
   input, placeholder filtering — plus a test that parses
   `.chaiss.cola.example` itself and asserts the expected overlay, so
   the literate example can never drift from what the code understands.

## Verification checklist

- [x] `cargo fmt --all -- --check`, `cargo clippy --workspace
      --all-targets -- -D warnings`, `cargo test --workspace` clean
      (43 tests, 6 new cola tests).
- [x] Example-file test passes (literate example is a valid config).
- [x] Live egui MCP check: a `.chaiss.cola` with `backend: "openai"`,
      a dummy OpenAI key, and model `gpt-6-astra-cola` — alongside a
      real Google key only in `.env` — produced a two-entry dropdown
      initially selecting `gpt-6-astra-cola (OpenAI)`: cola's backend
      and model overrides won over `.env`, and the placeholder Google
      key in cola fell through to the real env key instead of masking
      it. Startup log dropped from ~6,700 lines to 9 with rustemo's
      debug trace silenced via `RUSTEMO_NOTRACE`.

## Implementation findings

- Placeholder (`your_key_here`) and empty values never enter the
  overlay — strict per-key precedence would let the example's
  placeholders mask real `.env` keys when the example is copied.
- rustemo (colap's parser runtime) traces every parse step to stderr
  in debug builds; chaiss sets `RUSTEMO_NOTRACE=1` at startup. A
  colap-side default worth considering upstream.

## Out of scope

- Per-game persistence of the selected model (CHAI-0043).
- Provider `base_url` overrides via cola (possible later extension).
