---
date: 2026-10-31
type: incubation
---

# Learning from the opponent: a per-player dossier in the prompt

Captured 2026-10-31 alongside the game 13 post-mortem. In that game the
user told the model in the second message that the opponent "is
typically focused on Italian style games" and chose an asymmetric
defence to unsettle him. That knowledge lived in one chat message of
one game; the next game against the same opponent starts from nothing.

## What the database already holds

`players`, `games` (white and black player ids), `moves` (every ply
with its FEN) and `chat_messages` are all per game, so everything
needed to describe an opponent is there: their openings as White and
as Black, how long their games run, which exchanges they accept,
whether they castle long, which tactical motif decided each game they
won (game 13: a back-rank mate after a rook exchange on an open file).

## Two levels

1. **A dossier distilled by the model.** Before a game against a known
   opponent, hand the model the move lists of the previous games
   against them and ask for a short profile: preferred openings,
   recurring plans, the motifs they punished, the mistakes they made.
   Store it on the player row and inject it into the system prompt as
   "OPPONENT PROFILE". A handful of games is enough for this, and the
   prophylaxis question becomes concrete: "this opponent doubles rooks
   on the open file and looks for the back rank; where is our luft?"
2. **A learned model of the opponent.** The Maia line of work
   (McIlroy-Young et al.; Maia-2, Tang, Jiao and McIlroy-Young 2024)
   predicts human moves by skill level, and the follow-up "Learning
   Models of Individual Behavior in Chess" fine-tunes to a single
   player well enough to identify them from their moves. The project
   site claims a style can be mirrored from about twenty games. That
   needs a trained network and a games corpus chaiss does not have;
   it is the horizon, not the next step.

## Open questions

- Which features generalise across a few games, and which are noise?
  Openings and castling side are stable; a single back-rank mate is an
  event, not yet a pattern.
- The profile is about a named person. Keep it local, in the user's
  own database, and never in a prompt sent for a game against someone
  else.
- The same machinery describes the user: a self-dossier ("you tend to
  skip luft before major-piece exchanges") is the coaching feature the
  first incubation entry's checklist leads to.
