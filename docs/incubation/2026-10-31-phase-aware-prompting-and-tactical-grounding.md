---
date: 2026-10-31
type: incubation
---

# Phase-aware prompting and engine-grounded tactical facts

Captured 2026-10-31 from the post-mortem of game 13 ("Asymmetric",
Black, 0-1 by back-rank mate on move 21). Black was a clean pawn up
with the better minor piece and lost to `19.Rhd1 Rfd8?? 20.Rxd8+ Rxd8
21.Rxd8#`: the king on g8 sat behind f7-g7-h7 with no luft while
White's rooks owned the open d-file.

## What the transcript shows

- The model recommended `19...Rfd8!` and called it "the only
  mathematically pure refutation", ending its line in `21.Rxd8+ Bxd8`.
  The bishop was on e5; e5-d8 is not a diagonal. The same impossible
  recapture appears in six consecutive analyses (chat messages 456 to
  468), so the whole plan rested on a move the engine would have
  rejected in a microsecond.
- The injected hotspots for those moves were d2, d5, d6 and g3. They
  are sums of pseudo-legal attacks over every one-ply branch: a
  "where are pieces pointing" statistic. The back rank, the king's
  zero flight squares, and the fact that the only legal reply to
  `Rxd8+` is a recapture that gets mated never register as heat. In
  forced positions the matrix collapses (message 466 cites heat 4 on
  every square) and says nothing.
- The system prompt is one phase-agnostic instruction to "focus purely
  geometrically on pawn structure and piece coordination", plus the
  hotspots. Nothing asks the model to list the opponent's checks and
  captures after its recommended move, or to count the king's escape
  squares. Nothing changes between the opening and a rook ending.

## The shape of a fix

1. **Ground the model's line.** `### PREDICTIVE MATRIX: a, b, c, d`
   is parsed already; replay it through the engine (`notation.rs`,
   `get_legal_moves`). An illegal ply is reported back to the model in
   the next turn ("ply 4 Bxd8 is not a legal move; the position after
   ply 3 is mate") instead of being painted as an arrow.
2. **Inject facts, not only heat.** The engine can compute, cheaply,
   for both sides: king flight squares (luft), enemy heavy pieces with
   access to the back rank, every check available next ply, hanging
   pieces, knight forks and pins. A short "TACTICAL FACTS" block
   carries them. Heat stays for the UI.
3. **Order the analysis the way trainers teach it.** Checks, captures
   and threats for the side to move and then for the opponent (CCT),
   then the prophylaxis question ("what does the opponent want to do
   next move?"), then king safety, then the positional picture. Make
   the final step an explicit blunder check of the recommended move:
   list the opponent's checks and captures after it. Blumenfeld's rule
   (after calculating, look at the position afresh for a mate in one
   or a hanging piece) is this step in classical form.
4. **Switch heuristics by phase.** Engines taper their evaluation
   between middlegame and endgame weights from the non-pawn material
   on the board; the prompt can do the same with three profiles.
   Opening: development, centre, king safety, don't move the same
   piece twice. Middlegame: Aagaard's three questions (where are the
   weaknesses, which piece is worst placed, what is the opponent's
   idea), pawn breaks, outposts. Endgame: king activity, passed pawns
   and the rule of the square, rook behind passed pawns, luft before
   any major-piece exchange, opposition, "do not hurry" (Shereshevsky).
5. **Tone.** The present prompt's register ("thermodynamic density",
   "mathematically bound") is mirrored back as 4 to 6 KB of prose per
   move with no verification in it. A plainer prompt that asks for
   fewer, checked statements is likely to be both shorter and safer.

## Literature to draw on

- Nimzowitsch, *My System*: prophylaxis and overprotection.
- Aagaard, *Grandmaster Preparation: Positional Play*: the three
  questions, the last of which is the opponent's idea.
- Dvoretsky and Yusupov, *Secrets of Positional Play* (School of
  Future Champions 4): prophylactic thinking as a trained habit.
- Kotov, *Think Like a Grandmaster*: candidate moves, the analysis
  tree, Blumenfeld's rule.
- Silman, *How to Reassess Your Chess*: imbalances as the vocabulary
  for the middlegame profile.
- Shereshevsky, *Endgame Strategy*: king activity, do not hurry.
- The "checks, captures, threats" routine from club-level coaching,
  which is the cheapest blunder filter known.
- Classical engine evaluation (Fruit, Stockfish before NNUE): tapered
  evaluation by game phase, king-safety and back-rank terms, as a
  model for which features to compute and when to weight them.
