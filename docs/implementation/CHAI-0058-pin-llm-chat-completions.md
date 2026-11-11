# CHAI-0058 — Gemini answered 404 after the llm crate moved to the Responses API

- **Ticket:** CHAI-0058
- **Branch:** `bugfix/CHAI-0058-pin-llm-chat-completions`
- **Status:** implemented (PR pending review)

## Premise

Field report on 0.7.0 (2026-11-11): every Gemini turn failed with

```
[LLM error] Chat Stream err: Response format error: OpenAI responses API
returned error status: 404 Not Found. Raw response:
```

The 0.7.0 dependency refresh moved the `llm` crate from 1.3.7 to 1.3.8
(released 2026-04-19). In 1.3.8 the `OpenAI` backend no longer delegates
to the crate's generic OpenAI-compatible provider, which posts to
`chat/completions`; it builds a Responses API payload and posts to
`<base_url>/responses` unconditionally (`src/backends/openai.rs`,
`responses_url`). chaiss reaches Gemini through that backend with
`base_url` set to Google's OpenAI-compatible endpoint
(`generativelanguage.googleapis.com/v1beta/openai/`), chosen in 0.1.4
so that `reasoning_effort` could be sent. Google serves
`chat/completions` there and nothing at `responses`, hence the 404.

Upstream knows: graniet/llm#121 (April 2026) reports the same break
for llama.cpp and LM Studio. It was closed by the reporter without a
code change, with the workaround of using
`llm::providers::openai_compatible::OpenAICompatibleProvider<T>`
directly, which still has `CHAT_ENDPOINT = "chat/completions"`. Master
still posts to `responses`, so there is no release to wait for.

The requirement `llm = "1.3.7"` also meant that any `cargo install
chaiss` without `--locked` has resolved to 1.3.8 since April, whatever
chaiss version was installed. The lockfile only protected builds from
a checkout.

## Action items

1. `chaiss-core/Cargo.toml`: `llm = "=1.3.7"` with a comment naming the
   reason and this ticket; `cargo update -p llm --precise 1.3.7`. This
   restores the 0.6.0 request path byte for byte and fixes fresh
   installs too.
2. Gates: `cargo fmt --all -- --check`, `cargo clippy --workspace
   --all-targets -- -D warnings`, `cargo test --workspace`.
3. Release 0.7.1 once a live game confirms the fix.

## Follow-up (own ticket)

Leave the pin behind by building the Gemini client on
`OpenAICompatibleProvider<GeminiCompat>` with a `GeminiCompat:
OpenAIProviderConfig` whose `DEFAULT_BASE_URL` is Google's endpoint and
whose `CHAT_ENDPOINT` stays `chat/completions`; the provider's
constructor takes `reasoning_effort`, so nothing is lost. The native
`LLMBackend::Google` backend is the other candidate; in 1.3.8 it
streams and carries the system prompt but has no thinking-effort knob,
and its error strings differ from the ones `classify_backend_error`
was tuned on. Either way the swap deserves a live game before it ships,
which is why it is not part of this fix.

## Verification

- [x] fmt, clippy `-D warnings`, 80 workspace tests.
- [x] `Cargo.lock`: `llm` 1.3.7, 60 crates that only 1.3.8 pulled in
      are gone again.
- [ ] A Gemini turn in a real game completes on the rebuilt binary.
