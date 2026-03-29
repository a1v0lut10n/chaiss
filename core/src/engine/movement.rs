use super::models::{Color, GameState, Piece, PieceType};

const ORTHOGONAL: [(i8, i8); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
const DIAGONAL: [(i8, i8); 4] = [(1, 1), (-1, 1), (1, -1), (-1, -1)];
const KNIGHT_JUMPS: [(i8, i8); 8] = [
    (2, 1), (1, 2), (-1, 2), (-2, 1),
    (-2, -1), (-1, -2), (1, -2), (2, -1)
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
                if r < 0 || r > 7 || f < 0 || f > 7 { 
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
        },
        PieceType::Knight => {
            for &(dr, df) in KNIGHT_JUMPS.iter() {
                let r = rank + dr;
                let f = file + df;
                if r >= 0 && r <= 7 && f >= 0 && f <= 7 {
                    attacks.push((r * 8 + f) as usize);
                }
            }
        },
        PieceType::King => {
            for &(dr, df) in ORTHOGONAL.iter().chain(DIAGONAL.iter()) {
                let r = rank + dr;
                let f = file + df;
                if r >= 0 && r <= 7 && f >= 0 && f <= 7 {
                    attacks.push((r * 8 + f) as usize);
                }
            }
        },
        PieceType::Pawn => {
            // Pawns only exert attack heat to their diagonal forward squares!
            let dr = if piece.color == Color::White { -1 } else { 1 };
            for df in [-1_i8, 1_i8] {
                let r = rank + dr;
                let f = file + df;
                if r >= 0 && r <= 7 && f >= 0 && f <= 7 {
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
        if forward_r >= 0 && forward_r <= 7 {
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
            if cap_r >= 0 && cap_r <= 7 && cap_f >= 0 && cap_f <= 7 {
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

    // TODO: Verify true validation against Kings moving into self checks natively. Filter `moves`.
    moves
}
