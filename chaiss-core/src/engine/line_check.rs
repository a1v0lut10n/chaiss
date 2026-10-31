//! Replays the model's predicted line through the engine.
//!
//! Every analysis ends in a `### PREDICTIVE MATRIX: a, b, c, d` line. The
//! arrows drawn on the board come from replaying it, and so does the
//! verdict sent back to the model when a ply is not a legal move, is
//! written after the game has ended, or the whole line walks the side to
//! move into checkmate. One [`LineReport`] feeds both, so the arrows and
//! the verdict can never disagree.

use super::models::{Color, GameEndStatus, GameState, Piece, PieceType};
use super::{movement, notation};

/// The marker the system prompt binds the model to.
pub const MARKER: &str = "### PREDICTIVE MATRIX:";

/// The plies written after the marker, cleaned: everything but letters,
/// digits, `-` (castling), `=` (promotion) and the `+`/`#` suffixes is
/// dropped. `None` when the message has no marker at all.
pub fn extract_line(message: &str) -> Option<Vec<String>> {
    let idx = message.find(MARKER)?;
    let tail = &message[idx + MARKER.len()..];
    let sequence = tail.trim_start().lines().next().unwrap_or("");
    Some(
        sequence
            .split(',')
            .map(|ply| {
                ply.trim()
                    .replace(|c: char| !c.is_alphanumeric() && !"-=+#".contains(c), "")
            })
            .filter(|ply| !ply.is_empty())
            .collect(),
    )
}

/// What the engine made of one written ply.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlyVerdict {
    /// A legal move; `after` is the game's state once it is played.
    Legal {
        san: String,
        from: usize,
        to: usize,
        color: Color,
        after: Option<GameEndStatus>,
    },
    /// Not a legal move in the position reached; `reason` is written
    /// for the model, naming the piece, the squares it does reach, or
    /// the check it has to answer.
    Illegal { san: String, reason: String },
    /// Written after checkmate or stalemate ended the line.
    AfterEnd { san: String },
}

/// The engine's reading of a whole line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LineReport {
    /// The side to move at the start of the line.
    pub mover: Color,
    /// Set when the game was already over before the line: nothing to
    /// check, the line is accepted as written.
    pub base_terminal: Option<GameEndStatus>,
    pub plies: Vec<PlyVerdict>,
}

impl LineReport {
    /// The arrows for the legal prefix of the line.
    pub fn arrows(&self) -> Vec<(usize, usize, Color)> {
        self.plies
            .iter()
            .map_while(|ply| match ply {
                PlyVerdict::Legal {
                    from, to, color, ..
                } => Some((*from, *to, *color)),
                _ => None,
            })
            .collect()
    }

    /// The side the line checkmates, when its last legal ply does.
    fn mates(&self) -> Option<Color> {
        self.plies.iter().rev().find_map(|ply| match ply {
            PlyVerdict::Legal {
                after: Some(GameEndStatus::Checkmate(winner)),
                ..
            } => Some(winner.opposite()),
            _ => None,
        })
    }

    /// True when every ply is legal, none follows the end of the game,
    /// and the line does not end with the side to move checkmated.
    pub fn is_clean(&self) -> bool {
        if self.base_terminal.is_some() {
            return true;
        }
        let all_legal = self
            .plies
            .iter()
            .all(|ply| matches!(ply, PlyVerdict::Legal { .. }));
        all_legal && self.mates() != Some(self.mover)
    }

    /// The verdict for the model and the chat, `None` when the line is
    /// clean.
    pub fn describe(&self) -> Option<String> {
        if self.is_clean() {
            return None;
        }
        let mut out = String::from("Engine check of your predicted line:\n");
        for (i, ply) in self.plies.iter().enumerate() {
            let n = i + 1;
            match ply {
                PlyVerdict::Legal { san, after, .. } => match after {
                    Some(GameEndStatus::Checkmate(winner)) => out.push_str(&format!(
                        "- ply {n} `{san}`: legal, and it is checkmate — {} is mated.\n",
                        color_name(winner.opposite())
                    )),
                    Some(GameEndStatus::Stalemate) => {
                        out.push_str(&format!("- ply {n} `{san}`: legal, and it is stalemate.\n"))
                    }
                    _ => out.push_str(&format!("- ply {n} `{san}`: legal.\n")),
                },
                PlyVerdict::Illegal { san, reason } => {
                    out.push_str(&format!("- ply {n} `{san}`: NOT A LEGAL MOVE. {reason}\n"))
                }
                PlyVerdict::AfterEnd { san } => out.push_str(&format!(
                    "- ply {n} `{san}`: written after the game has already ended.\n"
                )),
            }
        }
        if self.mates() == Some(self.mover) {
            out.push_str(&format!(
                "Your line walks {}, the side to move, into checkmate.\n",
                color_name(self.mover)
            ));
        }
        out.push_str(
            "These are the rules of chess as the engine applies them; do not argue with them. \
             Revise your analysis from the first rejected ply, and end it with a new \
             `### PREDICTIVE MATRIX:` line of legal moves.",
        );
        Some(out)
    }
}

/// Replays `plies` from `base`, stopping the replay at the first ply the
/// engine rejects (the rest is reported as written, unverified).
pub fn check_line(base: &GameState, plies: &[String]) -> LineReport {
    let mover = base.active_color;
    let base_terminal = base.evaluate_terminal_state();
    let mut report = LineReport {
        mover,
        base_terminal,
        plies: Vec::new(),
    };
    if base_terminal.is_some() {
        return report;
    }

    let mut state = base.clone();
    let mut ended = false;
    let mut stopped = false;
    for san in plies {
        if stopped {
            // The position is unknown from here on; report the ply as
            // illegal in context without a reason the engine can stand by.
            report.plies.push(PlyVerdict::Illegal {
                san: san.clone(),
                reason:
                    "It follows a rejected ply, so the position it was meant for does not exist."
                        .to_string(),
            });
            continue;
        }
        if ended {
            report.plies.push(PlyVerdict::AfterEnd { san: san.clone() });
            continue;
        }
        match notation::parse_algebraic_move(&state, san) {
            Ok((from, to, promo)) => {
                let color = state.active_color;
                state.apply_move(from, to, promo);
                let after = state.evaluate_terminal_state();
                ended = after.is_some();
                report.plies.push(PlyVerdict::Legal {
                    san: san.clone(),
                    from,
                    to,
                    color,
                    after,
                });
            }
            Err(_) => {
                let reason = explain_illegal(&state, san);
                report.plies.push(PlyVerdict::Illegal {
                    san: san.clone(),
                    reason,
                });
                stopped = true;
            }
        }
    }
    report
}

fn color_name(color: Color) -> &'static str {
    match color {
        Color::White => "White",
        Color::Black => "Black",
    }
}

fn piece_name(piece_type: PieceType) -> &'static str {
    match piece_type {
        PieceType::Pawn => "pawn",
        PieceType::Knight => "knight",
        PieceType::Bishop => "bishop",
        PieceType::Rook => "rook",
        PieceType::Queen => "queen",
        PieceType::King => "king",
    }
}

/// `a8` is index 0, `h1` is 63.
pub fn square_name(idx: usize) -> String {
    let file = (b'a' + (idx % 8) as u8) as char;
    let rank = (b'1' + (7 - idx / 8) as u8) as char;
    format!("{file}{rank}")
}

/// Every legal move of the side to move, as SAN.
fn legal_moves_san(state: &GameState) -> Vec<String> {
    let mut moves = Vec::new();
    for idx in 0..64 {
        if let Some(piece) = state.board[idx] {
            if piece.color != state.active_color {
                continue;
            }
            for to in movement::get_legal_moves(state, idx, piece) {
                let promo = (piece.piece_type == PieceType::Pawn && !(8..56).contains(&to))
                    .then_some(PieceType::Queen);
                moves.push(notation::get_algebraic_notation(state, idx, to, promo));
            }
        }
    }
    moves
}

/// The piece letter and destination square a SAN string names, read
/// leniently: the engine's parser has already rejected the move, this
/// only serves the explanation.
fn san_piece_and_target(san: &str) -> (PieceType, Option<usize>) {
    let cleaned: String = san.chars().filter(|c| !"+#!?x".contains(*c)).collect();
    let piece_type = match cleaned.chars().next() {
        Some('K') => PieceType::King,
        Some('Q') => PieceType::Queen,
        Some('R') => PieceType::Rook,
        Some('B') => PieceType::Bishop,
        Some('N') => PieceType::Knight,
        _ => PieceType::Pawn,
    };
    let core: String = cleaned
        .split('=')
        .next()
        .unwrap_or("")
        .trim_end_matches(|c: char| "QRBNqrbn".contains(c) && cleaned.len() > 2)
        .to_string();
    let mut chars = core.chars().rev();
    let (rank, file) = (chars.next(), chars.next());
    let target = match (file, rank) {
        (Some(f), Some(r)) if ('a'..='h').contains(&f) && ('1'..='8').contains(&r) => {
            Some((8 - r.to_digit(10).unwrap_or(8) as usize) * 8 + (f as u8 - b'a') as usize)
        }
        _ => None,
    };
    (piece_type, target)
}

/// Why `san` is not a legal move in `state`, in words the model can act on.
fn explain_illegal(state: &GameState, san: &str) -> String {
    let mover = state.active_color;
    let upper = san.to_ascii_uppercase();
    if upper == "O-O" || upper == "0-0" || upper == "O-O-O" || upper == "0-0-0" {
        return format!(
            "{} cannot castle here. Legal moves: {}.",
            color_name(mover),
            legal_moves_san(state).join(", ")
        );
    }

    // In check, the answer is the short list of legal replies.
    if let Some(king) = movement::find_king(state, mover) {
        if movement::is_square_attacked(state, king, mover.opposite()) {
            return format!(
                "The {} king on {} is in check; the only legal moves are: {}.",
                color_name(mover),
                square_name(king),
                legal_moves_san(state).join(", ")
            );
        }
    }

    let (piece_type, target) = san_piece_and_target(san);
    let Some(target) = target else {
        return format!("`{san}` does not name a square on the board.");
    };
    let target_name = square_name(target);

    let own: Vec<(usize, Piece)> = (0..64)
        .filter_map(|idx| {
            state.board[idx]
                .filter(|p| p.color == mover && p.piece_type == piece_type)
                .map(|p| (idx, p))
        })
        .collect();
    if own.is_empty() {
        return format!(
            "{} has no {} on the board.",
            color_name(mover),
            piece_name(piece_type)
        );
    }
    let reach: Vec<String> = own
        .iter()
        .map(|(idx, piece)| {
            let mut targets: Vec<String> = movement::get_legal_moves(state, *idx, *piece)
                .into_iter()
                .map(square_name)
                .collect();
            targets.sort();
            if targets.is_empty() {
                format!(
                    "the {} on {} has no legal move",
                    piece_name(piece_type),
                    square_name(*idx)
                )
            } else {
                format!(
                    "the {} on {} reaches {}",
                    piece_name(piece_type),
                    square_name(*idx),
                    targets.join(", ")
                )
            }
        })
        .collect();
    format!(
        "No {} {} can reach {}: {}.",
        color_name(mover),
        piece_name(piece_type),
        target_name,
        reach.join("; ")
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Game 13 after 19.Rhd1: Black to move, rooks doubled on the d-file,
    /// the g8 king behind f7-g7-h7 without luft.
    const GAME_13_AFTER_RHD1: &str = "r4rk1/3R1ppp/p1p1p3/1p2b3/4N3/P7/1PP2PPP/2KR4 b - - 5 19";

    fn plies(line: &str) -> Vec<String> {
        extract_line(&format!("prose\n\n{MARKER} {line}\n")).unwrap()
    }

    fn state(fen: &str) -> GameState {
        GameState::from_fen(fen).unwrap()
    }

    #[test]
    fn extracts_the_first_line_after_the_marker() {
        assert_eq!(
            plies("Rfd8, Rxd8+, Rxd8, Bxd8\nmore prose"),
            vec!["Rfd8", "Rxd8+", "Rxd8", "Bxd8"]
        );
        assert_eq!(plies("O-O, 0-0-0"), vec!["O-O", "0-0-0"]);
        assert!(extract_line("no marker").is_none());
    }

    #[test]
    fn the_impossible_bishop_recapture_is_rejected_with_its_reach() {
        let report = check_line(
            &state(GAME_13_AFTER_RHD1),
            &plies("Rfd8, Rxd8+, Rxd8, Rxd8+, Bxd8"),
        );
        assert!(!report.is_clean());
        // 21.Rxd8 is mate, so Bxd8 never gets a position to be played in.
        assert!(matches!(
            &report.plies[3],
            PlyVerdict::Legal {
                after: Some(GameEndStatus::Checkmate(Color::White)),
                ..
            }
        ));
        assert!(matches!(&report.plies[4], PlyVerdict::AfterEnd { .. }));
        let text = report.describe().unwrap();
        assert!(
            text.contains("ply 4 `Rxd8+`: legal, and it is checkmate — Black is mated"),
            "{text}"
        );
        assert!(
            text.contains("walks Black, the side to move, into checkmate"),
            "{text}"
        );
    }

    #[test]
    fn a_bishop_asked_to_leave_its_diagonals_is_explained() {
        // Same position, but the rook pair is still on the board: ask the
        // e5 bishop to go to d8 directly.
        let report = check_line(&state(GAME_13_AFTER_RHD1), &plies("Bxd8"));
        let PlyVerdict::Illegal { reason, .. } = &report.plies[0] else {
            panic!("{:?}", report.plies[0]);
        };
        assert!(
            reason.starts_with("No Black bishop can reach d8: the bishop on e5 reaches "),
            "{reason}"
        );
        assert!(reason.contains("b8"), "{reason}");
        assert!(reason.contains("h2"), "{reason}");
        assert!(!reason.contains("d8,"), "{reason}");
        assert_eq!(report.arrows(), vec![]);
    }

    #[test]
    fn the_model_s_four_ply_line_mates_the_mover() {
        // The exact line the model gave in game 13 (message 462).
        let report = check_line(
            &state(GAME_13_AFTER_RHD1),
            &plies("Rfd8, Rxd8+, Rxd8, Rxd8+"),
        );
        assert!(report
            .plies
            .iter()
            .all(|p| matches!(p, PlyVerdict::Legal { .. })));
        assert!(!report.is_clean());
        assert_eq!(report.arrows().len(), 4);
        assert!(report.describe().unwrap().contains("into checkmate"));
    }

    #[test]
    fn a_line_that_mates_the_opponent_is_clean() {
        // Scholar's mate: White to move delivers Qxf7#.
        let report = check_line(
            &state("r1bqkbnr/pppp1ppp/2n5/4p2Q/2B1P3/8/PPPP1PPP/RNB1K1NR w KQkq - 4 4"),
            &plies("Qxf7#"),
        );
        assert!(report.is_clean());
        assert!(report.describe().is_none());
        assert_eq!(report.arrows().len(), 1);
    }

    #[test]
    fn in_check_the_reply_lists_the_legal_moves() {
        // After 20.Rxd8+ in game 13: Black must recapture.
        let report = check_line(
            &state("r2R2k1/5ppp/p1p1p3/1p2b3/4N3/P7/1PP2PPP/2KR4 b - - 0 20"),
            &plies("Kf8"),
        );
        let PlyVerdict::Illegal { reason, .. } = &report.plies[0] else {
            panic!("{:?}", report.plies[0]);
        };
        assert_eq!(
            reason,
            "The Black king on g8 is in check; the only legal moves are: Rxd8."
        );
    }

    #[test]
    fn castling_and_a_finished_game_are_handled() {
        let mut s = GameState::new();
        for san in ["e4", "e5", "Nf3", "Nf6", "Bc4", "Bc5"] {
            let (f, t, p) = notation::parse_algebraic_move(&s, san).unwrap();
            s.apply_move(f, t, p);
        }
        let report = check_line(&s, &plies("O-O, O-O"));
        assert!(report.is_clean());
        assert_eq!(
            report.arrows(),
            vec![(60, 62, Color::White), (4, 6, Color::Black)]
        );

        let over = state("3R2k1/5ppp/p1p1p3/1p2b3/4N3/P7/1PP2PPP/2K5 b - - 0 21");
        let report = check_line(&over, &plies("1-0"));
        assert!(report.base_terminal.is_some());
        assert!(report.is_clean());
        assert!(report.plies.is_empty());
    }

    #[test]
    fn plies_after_a_rejected_one_are_not_judged() {
        let report = check_line(&state(GAME_13_AFTER_RHD1), &plies("Bxd8, g3, Kf8"));
        assert!(matches!(&report.plies[0], PlyVerdict::Illegal { .. }));
        assert!(
            matches!(&report.plies[1], PlyVerdict::Illegal { reason, .. } if reason.contains("rejected ply"))
        );
        assert_eq!(report.plies.len(), 3);
    }

    #[test]
    fn nonsense_and_missing_pieces_get_plain_reasons() {
        let report = check_line(&state(GAME_13_AFTER_RHD1), &plies("xyzzy, Qd7"));
        let PlyVerdict::Illegal { reason, .. } = &report.plies[0] else {
            panic!();
        };
        assert_eq!(reason, "`xyzzy` does not name a square on the board.");
        let report = check_line(&state(GAME_13_AFTER_RHD1), &plies("Qd7"));
        let PlyVerdict::Illegal { reason, .. } = &report.plies[0] else {
            panic!();
        };
        assert_eq!(reason, "Black has no queen on the board.");
    }
}
