# colap 0.4 through the facade

- **When:** 2026-10-31 14:39:18 local
- **Type:** execution
- **Project:** chaiss

## Context

`.chaiss.cola` was read with colap 0.2's markdown-first grammar: a
direct `rustemo` dependency for the parser trait, a synthetic fence
around bare cola, and errors without a line. CHAI-0044 planned the
move to colap 0.3's facade and waited on its release; 0.3.0 and
0.4.0 shipped a day apart, so the migration took both at once.

## Details

`overlay_from_str` is now `colap::parse_str`, which detects literate
or bare input itself; `init` reads through `colap::check_file` and
prints every finding as `path:line:col: severity: message`, using the
file unless it has errors. `value_text` gained the 0.3 and 0.4 value
kinds: symbols and references as text, numbers as their exact
literal, lists joined by spaces. `rustemo` left the manifest. The
example config lost a quirk the new scanner made obsolete and shows
a `//` comment. 58 workspace tests pass; a new one pins that a
malformed file names its line.

## Links

- Plan: `docs/implementation/CHAI-0044-colap-0-3-facade.md`
- PR: https://github.com/a1v0lut10n/chaiss/pull/70
- colap 0.4.0: https://github.com/a1v0lut10n/colap/releases/tag/v0.4.0
