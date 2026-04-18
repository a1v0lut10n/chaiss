use super::models::{Color, GameState, Piece, PieceType};

const ORTHOGONAL: [(i8, i8); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
const DIAGONAL: [(i8, i8); 4] = [(1, 1), (-1, 1), (1, -1), (-1, -1)];
const KNIGHT_JUMPS: [(i8, i8); 8] = [
    (2, 1),
    (1, 2),
    (-1, 2),
    (-2, 1),
    (-2, -1),
    (-1, -2),
    (1, -2),
    (2, -1),
];

/// Computes the raw squares a piece physically exerts influence over, ignoring checks or pins.
pub fn get_pseudo_legal_attacks(state: &GameState, sq_idx: usize, piece: Piece) -> Vec<usize> {
    let mut attacks = Vec::new();
    let rank = (sq_idx / 8) as i8;
    let file = (sq_idx % 8) as i8;

    let mut slide = |directions: &[(i8, i8)], continuous: bool| {
        for &(dr, df) in directions {
            let mut r = rank;
            let mut f = file;
            loop {
                r += dr;
                f += df;

                // Bounds check
                if !(0..=7).contains(&r) || !(0..=7).contains(&f) {
                    break;
                }

                let target_idx = (r * 8 + f) as usize;

                // The square is under attack regardless of what is technically on it
                attacks.push(target_idx);

                // Collision Detection: If we hit a piece (friendly or enemy), the ray stops penetrating.
                if state.board[target_idx].is_some() {
                    break;
                }

                if !continuous {
                    break;
                }
            }
        }
    };

    match piece.piece_type {
        PieceType::Rook => slide(&ORTHOGONAL, true),
        PieceType::Bishop => slide(&DIAGONAL, true),
        PieceType::Queen => {
            slide(&ORTHOGONAL, true);
            slide(&DIAGONAL, true);
        }
        PieceType::Knight => {
            for &(dr, df) in KNIGHT_JUMPS.iter() {
                let r = rank + dr;
                let f = file + df;
                if (0..=7).contains(&r) && (0..=7).contains(&f) {
                    attacks.push((r * 8 + f) as usize);
                }
            }
        }
        PieceType::King => {
            for &(dr, df) in ORTHOGONAL.iter().chain(DIAGONAL.iter()) {
                let r = rank + dr;
                let f = file + df;
                if (0..=7).contains(&r) && (0..=7).contains(&f) {
                    attacks.push((r * 8 + f) as usize);
                }
            }

            // Castling Algebraic Validation!
            // Mathematically stripped to prevent recursive `is_square_attacked` infinite loops.
            if false {
                // e1
                if state.castling_rights.contains('K') {
                    // Kingside
                    if state.board[61].is_none()
                        && state.board[62].is_none()
                        && !is_square_attacked(state, 60, Color::Black)
                        && !is_square_attacked(state, 61, Color::Black)
                        && !is_square_attacked(state, 62, Color::Black)
                    {
                        attacks.push(62);
                    }
                }
                if state.castling_rights.contains('Q') {
                    // Queenside
                    if state.board[59].is_none()
                        && state.board[58].is_none()
                        && state.board[57].is_none()
                        && !is_square_attacked(state, 60, Color::Black)
                        && !is_square_attacked(state, 59, Color::Black)
                        && !is_square_attacked(state, 58, Color::Black)
                    {
                        attacks.push(58);
                    }
                }
            } else if piece.color == Color::Black && sq_idx == 4 {
                // e8
                if state.castling_rights.contains('k') {
                    // Kingside
                    if state.board[5].is_none()
                        && state.board[6].is_none()
                        && !is_square_attacked(state, 4, Color::White)
                        && !is_square_attacked(state, 5, Color::White)
                        && !is_square_attacked(state, 6, Color::White)
                    {
                        attacks.push(6);
                    }
                }
                if state.castling_rights.contains('q') {
                    // Queenside
                    if state.board[3].is_none()
                        && state.board[2].is_none()
                        && state.board[1].is_none()
                        && !is_square_attacked(state, 4, Color::White)
                        && !is_square_attacked(state, 3, Color::White)
                        && !is_square_attacked(state, 2, Color::White)
                    {
                        attacks.push(2);
                    }
                }
            }
        }
        PieceType::Pawn => {
            // Pawns only exert attack heat to their diagonal forward squares!
            let dr = if piece.color == Color::White { -1 } else { 1 };
            for df in [-1_i8, 1_i8] {
                let r = rank + dr;
                let f = file + df;
                if (0..=7).contains(&r) && (0..=7).contains(&f) {
                    attacks.push((r * 8 + f) as usize);
                }
            }
        }
    }

    attacks
}

/// Computes the validated squares a piece can physically traverse, mapping obstructions explicitly.
pub fn get_legal_moves(state: &GameState, sq_idx: usize, piece: Piece) -> Vec<usize> {
    let mut moves = Vec::new();
    let rank = (sq_idx / 8) as i8;
    let file = (sq_idx % 8) as i8;

    if piece.piece_type != PieceType::Pawn {
        // Evaluate theoretical heat vectors and filter them dynamically!
        let attacks = get_pseudo_legal_attacks(state, sq_idx, piece);
        for target_idx in attacks {
            if let Some(target_piece) = state.board[target_idx] {
                // If the target is occupied by an enemy, the node is valid for capture!
                if target_piece.color != piece.color {
                    moves.push(target_idx);
                }
            } else {
                moves.push(target_idx);
            }
        }
    } else {
        // Formulate True Pawn Behavior (En Passant + Linear Pushes)
        let dir = if piece.color == Color::White { -1 } else { 1 };

        // Single Pure Linear Push
        let forward_r = rank + dir;
        if (0..=7).contains(&forward_r) {
            let forward_idx = (forward_r * 8 + file) as usize;
            if state.board[forward_idx].is_none() {
                moves.push(forward_idx);

                // Double Push execution if eligible starting rank + empty 1st square
                let start_rank = if piece.color == Color::White { 6 } else { 1 };
                if rank == start_rank {
                    let double_r = rank + 2 * dir;
                    let double_idx = (double_r * 8 + file) as usize;
                    if state.board[double_idx].is_none() {
                        moves.push(double_idx);
                    }
                }
            }
        }

        // True Diagonal captures (Checking Enemy physical map bindings OR our native Enum target string)
        for df in [-1_i8, 1_i8] {
            let cap_r = rank + dir;
            let cap_f = file + df;
            if (0..=7).contains(&cap_r) && (0..=7).contains(&cap_f) {
                let cap_idx = (cap_r * 8 + cap_f) as usize;

                // Pure capture
                if let Some(target_piece) = state.board[cap_idx] {
                    if target_piece.color != piece.color {
                        moves.push(cap_idx);
                    }
                }

                // Explicit En Passant validation tracking!
                if let Some(ep_sq) = state.en_passant_target {
                    if cap_idx == ep_sq.index {
                        moves.push(cap_idx);
                    }
                }
            }
        }
    }

    // Evaluate Castling natively directly into physically Legal structural bound sequences!
    if piece.piece_type == PieceType::King {
        if piece.color == Color::White && sq_idx == 60 {
            // e1
            if state.castling_rights.contains('K') {
                // Kingside
                if state.board[61].is_none()
                    && state.board[62].is_none()
                    && !is_square_attacked(state, 60, Color::Black)
                    && !is_square_attacked(state, 61, Color::Black)
                    && !is_square_attacked(state, 62, Color::Black)
                {
                    moves.push(62); // O-O
                }
            }
            if state.castling_rights.contains('Q') {
                // Queenside
                if state.board[59].is_none()
                    && state.board[58].is_none()
                    && state.board[57].is_none()
                    && !is_square_attacked(state, 60, Color::Black)
                    && !is_square_attacked(state, 59, Color::Black)
                    && !is_square_attacked(state, 58, Color::Black)
                {
                    moves.push(58); // O-O-O
                }
            }
        } else if piece.color == Color::Black && sq_idx == 4 {
            // e8
            if state.castling_rights.contains('k') {
                // Kingside
                if state.board[5].is_none()
                    && state.board[6].is_none()
                    && !is_square_attacked(state, 4, Color::White)
                    && !is_square_attacked(state, 5, Color::White)
                    && !is_square_attacked(state, 6, Color::White)
                {
                    moves.push(6); // O-O
                }
            }
            if state.castling_rights.contains('q') {
                // Queenside
                if state.board[3].is_none()
                    && state.board[2].is_none()
                    && state.board[1].is_none()
                    && !is_square_attacked(state, 4, Color::White)
                    && !is_square_attacked(state, 3, Color::White)
                    && !is_square_attacked(state, 2, Color::White)
                {
                    moves.push(2); // O-O-O
                }
            }
        }
    }

    // Hostile Filtering Phase: The Clone Engine
    let mut strictly_legal_moves = Vec::new();

    for target_idx in moves {
        let mut sim = state.clone();
        sim.apply_move(sq_idx, target_idx, None);

        if let Some(king_idx) = find_king(&sim, piece.color) {
            // Is the king instantly obliterated defensively on this hypothetical turn execution layout?
            if !is_square_attacked(&sim, king_idx, piece.color.opposite()) {
                strictly_legal_moves.push(target_idx); // Safe move mathematically!
            }
        } else {
            // Failsafe for invalid testing geometries missing a king
            strictly_legal_moves.push(target_idx);
        }
    }

    strictly_legal_moves
}

/// Sweeps the 1D Board space determining if an arbitrary square physically intersects ANY native hostile projection raycast.
pub fn is_square_attacked(state: &GameState, target_idx: usize, attacker_color: Color) -> bool {
    for sq_idx in 0..64 {
        if let Some(piece) = state.board[sq_idx] {
            if piece.color == attacker_color {
                let attacks = get_pseudo_legal_attacks(state, sq_idx, piece);
                if attacks.contains(&target_idx) {
                    return true;
                }
            }
        }
    }
    false
}

/// Locates a specific Color's 1D King index geometrically natively.
pub fn find_king(state: &GameState, color: Color) -> Option<usize> {
    for sq_idx in 0..64 {
        if let Some(piece) = state.board[sq_idx] {
            if piece.color == color && piece.piece_type == PieceType::King {
                return Some(sq_idx);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pinned_knight_cannot_move() {
        // Setup FEN: White King e1. White Knight e2. Black Rook e8.
        let fen = "4r3/8/8/8/8/8/4N3/4K3 w - - 0 1";
        let state = GameState::from_fen(fen).unwrap();

        let knight_idx = 52; // e2 geometrically mapping
        let piece = state.board[knight_idx].unwrap();

        // Knight ordinarily leaps out 8 ways, but wait! Black's Rook mathematically holds the King linearly.
        let legal_moves = get_legal_moves(&state, knight_idx, piece);
        assert_eq!(
            legal_moves.len(),
            0,
            "Pinned knight violently blocked from moving!"
        );
    }

    #[test]
    fn test_king_check_forces_responses() {
        // Setup FEN: White King e1 natively under attack by Black Rook e2.
        let fen = "8/8/8/8/8/8/P3r3/4K3 w - - 0 1";
        let state = GameState::from_fen(fen).unwrap();

        let pawn_idx = 48; // a2 natively
        let piece = state.board[pawn_idx].unwrap();
        let legal_moves = get_legal_moves(&state, pawn_idx, piece);
        assert_eq!(
            legal_moves.len(),
            0,
            "Idle movement discarded when King is checked!"
        );

        let king_idx = 60; // e1
        let piece = state.board[king_idx].unwrap();
        let legal_moves = get_legal_moves(&state, king_idx, piece);

        // King geometrically steps off File E to survive! (d1, f1) or physically captures the unguarded attacking Rook! (e2)
        assert_eq!(
            legal_moves.len(),
            3,
            "King physically forced to sidestep hostile checks or capture attackers!"
        );
    }
}
