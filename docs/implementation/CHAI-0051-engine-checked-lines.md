# CHAI-0051 — engine-checked predicted lines, corrected before they are shown

- **Ticket:** CHAI-0051
- **Branch:** `feature/CHAI-0051-engine-checked-lines`
- **Status:** planned
- **Origin:** `docs/incubation/2026-10-31-phase-aware-prompting-and-tactical-grounding.md`,
  item 1, and the game 13 post-mortem.

## Premise

Every analysis ends in a `### PREDICTIVE MATRIX: a, b, c, d` line. Today
`ChaissApp::parse_predictive_arrows` replays it through the engine only
to draw arrows, and stops silently at the first ply the notation parser
rejects. In game 13 the model's line ended in `Bxd8` from e5, which is
not a bishop move, six analyses in a row; the engine knew, the arrows
simply stopped one ply short, and the user played the recommended
`19...Rfd8??` into a back-rank mate.

The verdict must reach the model *before* its analysis reaches the
user, not as a remark on the next turn. The model's final line is the
last thing it writes, so the earliest moment to check it is when the
stream completes; the check then triggers an immediate, bounded
correction round trip, and the user sees the engine's verdict and the
revised analysis together.

## Design

### 1. `chaiss-core::engine::line_check` (new module)

Pure function over `GameState` and the move list, no I/O:

```rust
pub struct LineReport {
    pub plies: Vec<PlyVerdict>,   // one per written ply, in order
    pub stopped_at: Option<usize> // index of the first ply not replayed
}
pub enum PlyVerdict {
    Legal { san: String, from: usize, to: usize, color: Color,
            after: Option<GameEndStatus> },
    Illegal { san: String, reason: String },
    AfterEnd { san: String },     // written after checkmate/stalemate
}
pub fn check_line(base: &GameState, plies: &[&str]) -> LineReport;
```

- Replays with `notation::parse_algebraic_move` and `apply_move`,
  evaluating `evaluate_terminal_state` after each ply, so a line that
  "continues" past a mate is reported as such, and a line whose last
  legal ply mates the side the user plays is flagged even when every
  ply is legal.
- `reason` is plain language for the model ("the bishop on e5 does not
  reach d8: its diagonals are f6-h8, d6-b8, f4-h2, d4-a1"; "the king on
  g8 is in check from d8 and Rxd8 is the only legal move"). The
  notation parser's current error strings ("Not eligible physically")
  are not good enough to send; `line_check` builds its own from the
  piece, the squares and, for the king, the attacker.
- `LineReport::is_clean()` and `LineReport::describe()` (the text the
  model and the chat receive). `parse_predictive_arrows` becomes a thin
  wrapper over the report's legal plies, so arrows and verdict can never
  disagree.
- The line is extracted by one shared function (marker, first line,
  split on commas, the existing cleanup that keeps `-` for castling).

### 2. The correction round trip (`chaiss/src/app.rs`, `InferenceFinished`)

The existing retry (`retry_count < 3` when the marker is missing)
already models a bounded re-request that does not re-log the user's
prompt. It grows into a small state machine:

1. Stream completes → sanitise → extract the line → `check_line`
   against the position the payload was built for (`payload.current_fen`,
   not `self.game_state`: the user may have moved on or be browsing
   history).
2. Clean → finish as today.
3. Not clean and `correction_count < MAX_CORRECTIONS` (2) → push the
   flawed reply to the chat as an `Agent` message marked unverified,
   push an `Engine` message with `describe()`, then re-request with the
   same payload and a chat history that now ends in that engine
   message, so the model sees exactly what the engine saw. The chat
   pane shows a "revising…" state in place of the usual "thinking".
4. Still not clean after the last correction → finish, keep the engine
   verdict visible, draw arrows only for the legal prefix, and add one
   line: "the engine could not confirm this line; treat the suggestion
   as unverified."

No automatic correction is attempted when the response has no marker
at all (that path stays the existing retry) or when the user has
silenced analysis.

### 3. Roles

- Chat tuples gain a third role, `Engine`, rendered distinctly in
  `right_panel.rs` (monospace verdict, no Markdown sanitising) and
  persisted through `log_chat_message` with role `Engine`, so a resumed
  game replays the record faithfully.
- `llm.rs` maps roles to the API as: `Agent` → assistant, everything
  else → user. Today `System` entries (backend errors) are sent as
  assistant turns, which is wrong and becomes visible once `Engine`
  exists; the fix is one line.
- The system prompt gains one sentence: the engine replays the final
  line and will send back any ply it rejects, and a revised analysis
  must then replace the line, not argue with the engine.

### 4. Tests

- `line_check`: the game 13 position after `19.Rhd1` with the model's
  `Rfd8, Rxd8+, Rxd8, Rxd8+` (clean plies, ends in mate for Black →
  flagged) and with `…, Bxd8` (illegal ply with the diagonal reason);
  castling and promotion plies; a line continuing after mate; the
  "only legal move" wording when in check.
- `app.rs`: the correction state machine with a fake stream (the
  existing `LlmEvent` channel can be driven directly): flawed →
  engine message → re-request carries the engine message last; clean
  → no re-request; two failures → unverified finish; no marker → the
  old retry, no engine message.
- Role mapping in `llm.rs`.

## Not in this ticket

- Tactical facts in the prompt (luft, back-rank access, checks, forks):
  incubation item 2, its own ticket.
- Phase profiles and the CCT/prophylaxis ordering: items 3 and 4.
- A deeper search than replaying the written line. One or two plies of
  "what can the opponent do after your move" belongs with item 2.

## Verification before the proposal

- The three gates.
- Live, through the egui inspection session: load game 13 to ply 38
  (after `19.Rhd1`) in a scratch copy of the database, request analysis,
  and watch the round trip: the engine verdict appears, the revised
  analysis does not recommend `Rfd8`, the arrows match the verdict.
- The user's `chaiss.db` is not modified; a scratch copy is used.
