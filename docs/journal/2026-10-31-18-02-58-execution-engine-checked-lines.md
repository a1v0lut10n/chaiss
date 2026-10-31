# Engine-checked predicted lines (CHAI-0051)

- **When:** 2026-10-31 18:02:58 local
- **Type:** execution
- **Project:** chaiss

## Context

The agreed plan (same day): replay the model's `PREDICTIVE MATRIX` line
through the engine and send any verdict straight back, so a bad
suggestion is corrected before it is acted on.

## Details

`line_check` (chaiss-core) replays the line and names what is wrong in
the model's terms: the piece's real reach, the legal replies in check,
a ply after the game's end, a line that mates the side to move. The
arrows are its legal prefix. `app.rs` pushes the verdict as an `Engine`
message and re-sends the payload with it as the prompt, twice at most;
only `Agent` replies return to the model as assistant turns (error
notices had been going back as the model's own words); an empty final
reply is a `System` line. Live on a scratch database at game 13's ply 38
the model repeated its `Rfd8` line twice and the engine rejected it both
times, exactly as in the lost game. Revisions then came back truncated
near 1000 characters under the 8000 cap, so `max_tokens` is 32000; the
rerun was cut short by the session locking, which leaves the revised
analysis itself unobserved. `CHAISS_LLM_TRACE=1` echoes the raw stream.

## Links

- PR: https://github.com/a1v0lut10n/chaiss/pull/75
- Plan: `docs/implementation/CHAI-0051-engine-checked-lines.md`
- Plan entry: `2026-10-31-17-32-33-plan-engine-checked-lines.md`
