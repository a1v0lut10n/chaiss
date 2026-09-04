# Predictive arrows vanish when the matrix starts with castling

- **When:** 2026-09-04 10:51:03 local
- **Type:** execution
- **Project:** chaiss
- **Ticket:** CHAI-0039

## Context

The LLM's `### PREDICTIVE MATRIX:` line sometimes begins with a castling
move (e.g. `O-O, O-O-O, Nc6, Rhe1`). The arrow parser stripped every
non-alphanumeric character from each ply before handing it to the SAN
parser, mangling `O-O` into `OO` — which the parser rejects, since it
only recognizes castling with its hyphens intact. Arrow-building breaks
on the first unparseable ply, so a castling-first matrix rendered no
arrows at all, and castling mid-sequence silently truncated the
continuation. The digit forms `0-0`/`0-0-0` failed the same way.

## Details

Ply cleanup now preserves `-`, so all four castling spellings reach the
notation parser intact. The duplicated parsing blocks in the
stream-finished and game-resumed handlers were extracted into one
`parse_predictive_arrows` helper, which also confines parsing to the
matrix's own line so trailing prose can no longer swallow the final ply.

Five new unit tests cover kingside castling for both sides (plus a
follow-up rook move), queenside plus `0-0-0` digit notation, truncation
on an unparseable ply, prose after the matrix line, and the no-marker
case. fmt, clippy `-D warnings`, and all 30 workspace tests pass.

Verified live via the egui MCP loop against the affected "Modern" game:
on resume, the previously blank matrix now renders its arrows (red
e8→g8 for Black's O-O, blue e1→c1 for White's O-O-O, red b8→c6) — the
resume path fixes already-affected games with no database change.

## Links

- PR: https://github.com/a1v0lut10n/chaiss/pull/48
- Related entry: `2026-09-03-15-41-43-execution-chat-panel-table-reflow.md`
