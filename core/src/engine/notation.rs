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

pub fn parse_algebraic_move(state: &GameState, san: &str) -> Result<(usize, usize, Option<PieceType>), String> {
    let mut cleaned = san.replace(|c: char| "+#!?".contains(c), "").trim().to_string();
    if cleaned.is_empty() { return Err("Empty move".to_string()); }

    // Check castling
    if cleaned.to_uppercase() == "O-O" || cleaned.to_uppercase() == "0-0" {
        return parse_castling(state, true);
    }
    if cleaned.to_uppercase() == "O-O-O" || cleaned.to_uppercase() == "0-0-0" {
        return parse_castling(state, false);
    }

    // Check promotion
    let mut promotion = None;
    if let Some(eq_idx) = cleaned.find('=') {
        let p_char = cleaned.chars().nth(eq_idx + 1).unwrap_or(' ');
        promotion = super::models::Piece::from_char(p_char).map(|p| p.piece_type);
        if promotion.is_none() || promotion == Some(PieceType::King) {
            return Err("Invalid promotion target".to_string());
        }
        cleaned.truncate(eq_idx);
    } else if let Some(last_char) = cleaned.chars().last() {
         if "NnRrqQbB".contains(last_char) {
             let maybe_rank = cleaned.chars().rev().nth(1).unwrap_or('a');
             if "18".contains(maybe_rank) {
                 promotion = super::models::Piece::from_char(last_char).map(|p| p.piece_type);
                 cleaned.pop();
             }
         }
    }

    let _is_capture = cleaned.contains('x') || cleaned.contains('X');
    cleaned = cleaned.replace(['x', 'X'], ""); 

    if cleaned.len() < 2 {
        return Err("San string too short".to_string());
    }

    let to_rank_char = cleaned.chars().last().unwrap();
    let to_file_char = cleaned.chars().rev().nth(1).unwrap();
    
    if !('a'..='h').contains(&to_file_char) || !('1'..='8').contains(&to_rank_char) {
        return Err("Invalid destination square".to_string());
    }
    
    let to_file = (to_file_char as u8 - b'a') as usize;
    let to_rank = 8 - to_rank_char.to_digit(10).unwrap() as usize;
    let to_sq = to_rank * 8 + to_file;

    let prefix = &cleaned[..cleaned.len() - 2];
    
    let mut target_piece_type = PieceType::Pawn;
    let mut from_file_constraint = None;
    let mut from_rank_constraint = None;

    if !prefix.is_empty() {
        let first_char = prefix.chars().next().unwrap();
        if "NRQBK".contains(first_char) {
            target_piece_type = super::models::Piece::from_char(first_char).unwrap().piece_type;
            for c in prefix.chars().skip(1) {
                if ('a'..='h').contains(&c) {
                    from_file_constraint = Some((c as u8 - b'a') as usize);
                } else if ('1'..='8').contains(&c) {
                    from_rank_constraint = Some(8 - c.to_digit(10).unwrap() as usize);
                }
            }
        } else if ('a'..='h').contains(&first_char) { // Fallback for pure pawn mapping (exd5)
            from_file_constraint = Some((first_char as u8 - b'a') as usize);
        }
    }

    let mut candidates = Vec::new();
    for sq_idx in 0..64 {
        if let Some(piece) = state.board[sq_idx] {
            if piece.color == state.active_color && piece.piece_type == target_piece_type {
                let sq_file = sq_idx % 8;
                let sq_rank = sq_idx / 8;
                
                if let Some(fc) = from_file_constraint {
                    if sq_file != fc { continue; }
                }
                if let Some(rc) = from_rank_constraint {
                    if sq_rank != rc { continue; }
                }

                let moves = movement::get_legal_moves(state, sq_idx, piece);
                if moves.contains(&to_sq) {
                    candidates.push(sq_idx);
                }
            }
        }
    }

    if candidates.is_empty() {
        return Err(format!("No mathematically capable piece for string '{}'", san));
    }
    if candidates.len() > 1 {
        return Err(format!("Ambiguous array natively! More than one piece can organically reach target '{}'", san));
    }

    Ok((candidates[0], to_sq, promotion))
}

fn parse_castling(state: &GameState, kingside: bool) -> Result<(usize, usize, Option<PieceType>), String> {
    let king_sq = movement::find_king(state, state.active_color).ok_or("King physically missing natively!")?;
    
    if state.active_color == super::models::Color::White && king_sq != 60 { return Err("Not eligible physically".to_string()); }
    if state.active_color == super::models::Color::Black && king_sq != 4 { return Err("Not eligible physically".to_string()); }
    
    let target_sq = if kingside { king_sq + 2 } else { king_sq - 2 };
    
    let piece = state.board[king_sq].unwrap();
    let moves = movement::get_legal_moves(state, king_sq, piece);
    if !moves.contains(&target_sq) {
        return Err("Castling array blocked algebraically!".to_string());
    }
    
    Ok((king_sq, target_sq, None))
}

pub fn parse_pgn_moves(pgn: &str) -> Vec<String> {
    let mut moves = Vec::new();
    for line in pgn.lines() {
        let line = line.trim();
        if Default::default() || line.starts_with('[') {
            continue; // Safely strip rigid external application metadata natively!
        }
        
        for token in line.split_whitespace() {
            if token == "1-0" || token == "0-1" || token == "1/2-1/2" || token == "*" {
                continue; // Ignore final mathematical terminal block structures!
            }
            if token.contains('.') {
                let parts: Vec<&str> = token.split('.').collect();
                if let Some(mv) = parts.last() {
                    let clean = mv.trim();
                    if !clean.is_empty() {
                        moves.push(clean.to_string());
                    }
                }
                continue; // Processed numbered format logically
            }
            
            // Mathematically strip external annotations conditionally!
            let clean = token.replace("!", "").replace("?", "");
            if !clean.is_empty() {
                moves.push(clean.to_string());
            }
        }
    }
    moves
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pgn_sequence() {
        let pgn = "1. c4 e5 2. Nc3 Bb4 3. Nd5 Nc6 4. Nxb4 Nxb4 5. a3 Nc6 6. g3 d6 7. Bg2 Bd7 8. d3 Nf6 9. Nf3 O-O 10. O-O e4";
        let moves = super::parse_pgn_moves(pgn);
        let mut state = super::super::GameState::new();
        for m in moves {
            if let Ok((from, to, promo)) = super::parse_algebraic_move(&state, &m) {
                state.apply_move(from, to, promo);
                println!("Success: {}", m);
            } else {
                panic!("Fail: {}", m);
            }
        }
    }

    #[test]
    fn test_parse_basic_pawn_moves() {
        let state = GameState::new();
        // Test e4 mathematically
        let (from, to, promo) = parse_algebraic_move(&state, "e4").unwrap();
        assert_eq!(from, 52); // e2
        assert_eq!(to, 36);   // e4
        assert_eq!(promo, None);
    }

    #[test]
    fn test_parse_basic_knight_moves() {
        let state = GameState::new();
        let (from, to, promo) = parse_algebraic_move(&state, "Nf3").unwrap();
        assert_eq!(from, 62); // g1
        assert_eq!(to, 45);   // f3 
        assert_eq!(promo, None);
    }

    #[test]
    fn test_parse_algebraic_disambiguation() {
        // Construct a state where two knights can reach d2 mathematically (b1 and f3)
        // We must clear the friendly pawn at d2 so the Knights can physically jump natively!
        let fen = "rnbqkbnr/pppppppp/8/8/8/5N2/PPP1PPPP/RN1QKB1R w KQkq - 0 2";
        let state = GameState::from_fen(fen).unwrap();
        
        let (from, to, _) = parse_algebraic_move(&state, "Nbd2").unwrap();
        assert_eq!(from, 57); // b1
        assert_eq!(to, 51);   // d2
    }
}
