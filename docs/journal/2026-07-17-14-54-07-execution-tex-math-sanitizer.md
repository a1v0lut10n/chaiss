# Sanitize Gemini TeX math spans in LLM chat rendering

- **When:** 2026-07-17 14:54:07 local
- **Type:** execution
- **Project:** chaiss

## Context

Newer Gemini models wrap chess lines in LaTeX math
(`$$\text{7. d4 \quad exd4 \quad 8. Nxd4!}$$`). egui_commonmark 0.24 only
parses math when a `render_math_fn` is registered (`ENABLE_MATH` is off
otherwise), so these spans rendered verbatim — dollar signs, backslashes and
braces — in the chat panel (CHAI-30).

## Details

Extended `ChaissApp::sanitize_markdown` (both render paths — live stream and
finalized history — already flow through it) with a de-TeX pass: complete
`$$...$$`/`$...$` spans become bold plain text, unwrapping `\text{}`-style
groups, mapping spacing macros and common symbols to Unicode. Incomplete
spans, fenced code, and dollar amounts are deliberately left untouched. Added
a `FORMATTING CONSTRAINT` line to the Gemini system prompt forbidding LaTeX
output at the source. Verified with 8 new unit tests (including the exact
reported sample and mid-stream truncation) plus a clean workspace clippy run.

Alternatives weighed: real math rendering via `render_math_fn` + MiTeX/Typst
(deferred to a future feature branch — heavy dependency, and images of chess
moves render worse than native text); ReX/katex-rs/mathjax_svg rejected
(unmaintained / wrong output format / embeds V8).

## Links

- Branch: `bugfix/CHAI-30-llm-response-rendering`
- `chaiss/src/app.rs` — sanitizer + tests; `chaiss-core/src/llm.rs` — prompt constraint
- Prior analysis in this session: egui_commonmark 0.24 `parser_options()` omits `ENABLE_MATH` unless a math fn is set
