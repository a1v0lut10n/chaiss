use super::models::{GameEndStatus, GameState, PieceType};
use super::movement;

/// Translates active board geometries natively into formal FIDE Standard Algebraic Notation directly!
pub fn get_algebraic_notation(state: &GameState, from: usize, to: usize, promotion: Option<PieceType>) -> String {
    let piece = state.board[from].expect("No piece structurally present at explicit algebraic origin square!");
    
    // 1. Castling Transpositions
    if piece.piece_type == PieceType::King && (from as i32 - to as i32).abs() == 2 {
        if to > from {
            return "O-O".to_string(); // Kingside
        } else {
            return "O-O-O".to_string(); // Queenside
        }
    }
    
    // 2. Identify Hostile Space Overlaps (Captures)
    let mut is_capture = state.board[to].is_some();
    if piece.piece_type == PieceType::Pawn {
        if let Some(ep) = state.en_passant_target {
            if to == ep.index {
                is_capture = true;
            }
        }
    }

    // 3. Resolve Structural Ambiguity (If 2 overlapping identical pieces mathematically reach the target)
    let mut disambiguation = String::new();
    if piece.piece_type != PieceType::Pawn && piece.piece_type != PieceType::King {
        let mut identical_attackers = Vec::new();
        for sq in 0..64 {
            if sq != from {
                if let Some(other) = state.board[sq] {
                    if other.color == piece.color && other.piece_type == piece.piece_type {
                        let moves = movement::get_legal_moves(state, sq, other);
                        if moves.contains(&to) {
                            identical_attackers.push(sq);
                        }
                    }
                }
            }
        }

        if !identical_attackers.is_empty() {
            let from_file = from % 8;
            let from_rank = from / 8;
            
            let mut file_unique = true;
            let mut rank_unique = true;
            
            for &sq in &identical_attackers {
                if sq % 8 == from_file { file_unique = false; }
                if sq / 8 == from_rank { rank_unique = false; }
            }
            
            if file_unique {
                disambiguation.push((b'a' + from_file as u8) as char);
            } else if rank_unique {
                disambiguation.push_str(&(8 - from_rank).to_string());
            } else {
                disambiguation.push((b'a' + from_file as u8) as char);
                disambiguation.push_str(&(8 - from_rank).to_string());
            }
        }
    } else if piece.piece_type == PieceType::Pawn && is_capture {
        // Pawns capturing ALWAYS structurally state departing file regardless!
        let from_file = from % 8;
        disambiguation.push((b'a' + from_file as u8) as char);
    }

    // 4. Construct Explicit Prefix (Piece Mapping)
    let mut notation = String::new();
    if piece.piece_type != PieceType::Pawn {
        notation.push(match piece.piece_type {
            PieceType::Knight => 'N',
            PieceType::Bishop => 'B',
            PieceType::Rook => 'R',
            PieceType::Queen => 'Q',
            PieceType::King => 'K',
            _ => unreachable!(),
        });
    }

    notation.push_str(&disambiguation);

    if is_capture {
        notation.push('x');
    }

    // Explicit Targeting
    let to_file = to % 8;
    let to_rank = to / 8;
    notation.push((b'a' + to_file as u8) as char);
    notation.push_str(&(8 - to_rank).to_string());

    // 5. Pawn Promotion
    if let Some(target_type) = promotion {
        notation.push('=');
        notation.push(match target_type {
            PieceType::Knight => 'N',
            PieceType::Bishop => 'B',
            PieceType::Rook => 'R',
            PieceType::Queen => 'Q',
            _ => 'Q',
        });
    }

    // 6. Check / Checkmate Target Overlays
    let mut sim = state.clone();
    sim.apply_move(from, to, promotion);
    
    // Evaluate geometry termination exactly mathematically evaluating the freshly toggled state natively!
    if let Some(GameEndStatus::Checkmate(_)) = sim.evaluate_terminal_state() {
        notation.push('#');
    } else {
        // The color has ALREADY flipped sequentially natively globally inside `sim`!
        if let Some(enemy_king_idx) = movement::find_king(&sim, sim.active_color) {
            // Is that hostile king currently caught natively by our newly updated geometric pressure line?
            if movement::is_square_attacked(&sim, enemy_king_idx, sim.active_color.opposite()) {
                notation.push('+');
            }
        }
    }

    notation
}
