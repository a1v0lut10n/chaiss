# Plan: engine-checked predicted lines (CHAI-0051)

- **When:** 2026-10-31 17:32:33 local
- **Type:** plan
- **Project:** chaiss

## Context

Game 13 was lost from a winning position after the model recommended
`19...Rfd8` on a line ending in an impossible `Bxd8`, repeated across
six analyses. The engine replays the line for arrows and stopped
silently at the ply it could not parse.

## Details

Agreed: a pure `line_check` in chaiss-core replays the model's line and
reports illegal plies (with the piece's actual reach, or the legal
replies when in check), plies written after mate, and legal lines that
end with the side to move checkmated. On stream completion a flawed line
triggers an immediate bounded correction round trip (two at most): the
flawed reply stays in the chat, an `Engine` message carries the verdict,
and the same request is re-sent with that verdict as the prompt, so a
bad suggestion is corrected before it is acted on. Arrows and verdict
come from one report. Tactical facts in the prompt, phase profiles and
deeper look-ahead stay with the incubation entry.

## Links

- Plan: `docs/implementation/CHAI-0051-engine-checked-lines.md`
- PR: https://github.com/a1v0lut10n/chaiss/pull/75
- Incubation: `docs/incubation/2026-10-31-phase-aware-prompting-and-tactical-grounding.md`
