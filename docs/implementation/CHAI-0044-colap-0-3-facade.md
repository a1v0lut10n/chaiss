# CHAI-0044 — colap 0.3: parse `.chaiss.cola` through the facade

- **Ticket:** CHAI-0044
- **Branch:** `chore/CHAI-0044-colap-0-3-facade`
- **Status:** proposed (blocked on the colap 0.3.0 release)
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
