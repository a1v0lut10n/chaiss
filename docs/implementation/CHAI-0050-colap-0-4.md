# CHAI-0050 — read `.chaiss.cola` through colap 0.4

- **Ticket:** CHAI-0050
- **Branch:** `chore/CHAI-0050-colap-0-4`
- **Status:** implemented (PR pending review)

## Premise

colap published 0.3.0 (2026-10-30) and 0.4.0 (2026-10-31); chaiss-core
pinned `colap = "0.2"`. Both releases break the API chaiss used:
`ColaParser::parse` now parses one bare cola block (markdown left the
grammar, `colap::literate::scan` finds the fences), `ModelBuilder` became
a struct with a different `add_block`, and `ConfigValue::Integer`/`Float`
were replaced by `ConfigValue::Number` alongside new `Symbol`, `Reference`
and `List` variants.

## Action items

1. `colap = "0.4"`; drop the direct `rustemo` dependency, which existed
   only for the `Parser` trait the old call needed.
2. `overlay_from_str` reads through `colap::parse_str(text, None)`, which
   picks literate or bare cola from the text itself, so the synthetic
   fence chaiss wrapped bare input in is gone. Errors display as
   `line:col: message`.
3. `value_text` unwraps `String` and lets every other variant display as
   its canonical cola text (numbers in their own spelling).
4. The eight existing tests, including the literate example file, pass
   unchanged; CHANGELOG `[Unreleased]` Changed.

## Behaviour that changed for users

- A key set twice in `.chaiss.cola` was silently merged by 0.2 and is a
  parse error in 0.4; the file is then ignored with the error on the
  console and `.env` applies (existing fallback).
- `//` comments are accepted inside cola blocks; string escapes are
  decoded.

## Out of scope

- colap's CLI dependencies (clap, handlebars, env_logger, …) are
  non-optional in 0.4 and now build into chaiss-core; a feature split
  would be a colap change.
