# CHAI-0044 — colap 0.3: parse `.chaiss.cola` through the facade

- **Ticket:** CHAI-0044
- **Branch:** `chore/CHAI-0044-colap-0-3-facade`
- **Status:** proposed (colap 0.3.0 was published 2026-10-30; see the
  update below before executing)
- **Origin:** colap COLA-0001 (a1v0lut10n/colap PR #18), which moved
  markdown handling out of colap's grammar into a fence scanner and
  added a parsing facade. It is the fix for the adoption pains this
  repository recorded against colap 0.2: the direct `rustemo`
  dependency, fence-less input silently yielding an empty model, and
  the debug trace.

## Premise

`chaiss-core/src/cola_config.rs` reads `.chaiss.cola` with
`ColaParser::new().parse(content)` and
`ModelBuilder::build_config_model(&ast)`, wrapping fence-less content in
a synthetic ```` ```cola ```` fence first because colap 0.2's grammar
only saw fenced blocks. `chaiss-core/Cargo.toml` therefore carries
`rustemo = "0.7.1"` for the `Parser` trait.

In colap 0.3:

- `ColaParser::parse` parses one cola block, not a markdown document, so
  the current call fails on a literate `.chaiss.cola`.
- `colap::parse_str(text, path)` does what `overlay_from_str` does by
  hand: literate when the text contains a cola fence, bare cola
  otherwise. `colap::parse_file(path)` adds the read.
- Errors are `colap::Error`, printing `path:line:col: message`, and the
  result carries `warnings` (an unterminated fence names its opening
  line).
- Foreign fenced blocks and any prose are accepted in literate files,
  so `.chaiss.cola.example` may carry a `toml` comparison block or a
  diagram without breaking.

## Action items

1. `chaiss-core/Cargo.toml`: `colap = "0.3"`; drop `rustemo` and the
   comment explaining it.
2. `overlay_from_str`: replace the synthetic fence, `ColaParser` and
   `ModelBuilder` with `colap::parse_str(content, None)`; keep the
   `Result<_, String>` surface by mapping `colap::Error` through
   `to_string()` (the message now carries line and column). Delete the
   module-doc paragraph about wrapping plain cola.
3. `init`: pass the file path so errors name `.chaiss.cola`
   (`parse_str(&content, Some(path))` or `parse_file`), and log
   warnings under the config subsystem's `tracing` target.
4. Tests: `maps_plain_cola_to_logical_keys`,
   `parses_markdown_embedded_cola` and
   `example_file_is_a_valid_literate_config` cover both dialects
   already; add one asserting that a malformed file's error names the
   line.

## Verification

- [ ] `cargo build --workspace`, `cargo test -p chaiss-core` pass on
      colap 0.3 with no `rustemo` in `chaiss-core/Cargo.toml`.
- [ ] A deliberately broken `.chaiss.cola` logs `path:line:col`.

## Update 2026-10-31: against the published API

colap 0.3.0 is on crates.io, and colap PR #26 (COLA-0009, comments)
follows it on `main` as the unreleased 0.4.0. Reread against both, the
plan above holds with two additions:

- **`value_text` no longer compiles as written.** `ConfigValue` gained
  `Symbol(String)`, `Reference(String)` and `List(Vec<ConfigValue>)` in
  0.3.0, so the four-arm match in `cola_config.rs` needs arms for them.
  In 0.4.0 (colap COLA-0010) `Integer(i64)` and `Float(f64)` are one
  `Number(Number)` kept exactly as written: the overlay wants
  `n.text()`, which is the literal without separators, so `port: 5432`
  stays `5432` and nothing is rounded.
  `Symbol` and `Reference` carry the text the overlay wants (a bare
  `backend: ollama` reads as a symbol now, which is the natural way to
  write it); a `List` has no place in a `.env`-style overlay, so join
  its scalars with a space or ignore it, and say which in the code.
  Strings now arrive decoded, so `\"` in a value is one character.
- **Stay on the facade, not `build_config_model`.** Item 2 already
  says so; the reason grew: in 0.4.0 the builder needs the block text
  to attach `//` comments, and `build_config_model(&ast)` builds a
  model without them. chaiss only reads values, so it would not notice,
  but `colap::parse_str` is the call that stays correct either way.
  `//` comments are then legal in `.chaiss.cola`, worth one line in
  `.chaiss.cola.example`.

Pin `colap = "0.3"` now; nothing here depends on 0.4.0, and the facade
signatures are unchanged by it.
