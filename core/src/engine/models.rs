

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn opposite(&self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameEndStatus {
    Checkmate(Color), // Represents the Winner naturally
    Resignation(Color), // Represents the Winner manually
    Stalemate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Piece {
    pub color: Color,
    pub piece_type: PieceType,
}

impl Piece {
    pub fn from_char(c: char) -> Option<Self> {
        let color = if c.is_uppercase() { Color::White } else { Color::Black };
        let piece_type = match c.to_ascii_lowercase() {
            'p' => PieceType::Pawn,
            'n' => PieceType::Knight,
            'b' => PieceType::Bishop,
            'r' => PieceType::Rook,
            'q' => PieceType::Queen,
            'k' => PieceType::King,
            _ => return None,
        };
        Some(Piece { color, piece_type })
    }

    pub fn to_char(&self) -> char {
        let c = match self.piece_type {
            PieceType::Pawn => 'p',
            PieceType::Knight => 'n',
            PieceType::Bishop => 'b',
            PieceType::Rook => 'r',
            PieceType::Queen => 'q',
            PieceType::King => 'k',
        };
        if self.color == Color::White {
            c.to_ascii_uppercase()
        } else {
            c
        }
    }
}

/// A square index on the board, 0 to 63.
/// 0 is a8 (top-left), 63 is h1 (bottom-right).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Square {
    pub index: usize,
}

impl Square {
    pub fn new(index: usize) -> Self {
        Square { index }
    }
    
    pub fn from_file_rank(file: usize, rank: usize) -> Option<Self> {
        if file > 7 || rank > 7 { return None; }
        // 0,0 as top-left (a8). file=x, rank=y (0=8th rank, 7=1st rank).
        Some(Square { index: rank * 8 + file })
    }
}

pub type BoardMatrix = [Option<Piece>; 64];

#[derive(Debug, Clone, PartialEq)]
pub struct GameState {
    pub board: BoardMatrix,
    pub active_color: Color,
    pub castling_rights: String, // e.g. "KQkq"
    pub en_passant_target: Option<Square>,
    pub halfmove_clock: u16,
    pub fullmove_number: u16,
    pub manual_terminal_status: Option<GameEndStatus>,
}

impl GameState {
    /// Create starting position
    pub fn new() -> Self {
        Self::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()
    }

    /// Parses a FEN string into a GameState
    pub fn from_fen(fen: &str) -> Result<Self, String> {
        let parts: Vec<&str> = fen.split_whitespace().collect();
        if parts.len() != 6 {
            return Err("Invalid FEN string: incorrect number of fields".to_string());
        }

        let mut board: BoardMatrix = [None; 64];
        let mut index = 0;

        // 1. Piece placement
        for c in parts[0].chars() {
            if c == '/' {
                continue;
            } else if c.is_digit(10) {
                let empty_squares = c.to_digit(10).unwrap() as usize;
                index += empty_squares;
            } else {
                if index >= 64 { return Err("Invalid FEN string: too many pieces/squares".to_string()); }
                board[index] = Piece::from_char(c);
                index += 1;
            }
        }

        // 2. Active color
        let active_color = match parts[1] {
            "w" => Color::White,
            "b" => Color::Black,
            _ => return Err("Invalid active color in FEN".to_string()),
        };

        // 3. Castling rights
        let castling_rights = parts[2].to_string();

        // 4. En passant target
        let en_passant_target = if parts[3] != "-" {
            let files = "abcdefgh";
            let f_char = parts[3].chars().nth(0).unwrap();
            let r_char = parts[3].chars().nth(1).unwrap();
            let file = files.find(f_char).unwrap();
            let rank = 8 - r_char.to_digit(10).unwrap() as usize; 
            Square::from_file_rank(file, rank)
        } else {
            None
        };

        // 5. Halfmove clock
        let halfmove_clock = parts[4].parse::<u16>().unwrap_or(0);

        // 6. Fullmove number
        let fullmove_number = parts[5].parse::<u16>().unwrap_or(1);

        Ok(GameState {
            board,
            active_color,
            castling_rights,
            en_passant_target,
            halfmove_clock,
            fullmove_number,
            manual_terminal_status: None,
        })
    }

    /// Converts the current state to a FEN string
    pub fn to_fen(&self) -> String {
        let mut fen = String::new();
        
        // 1. Board
        for rank in 0..8 {
            let mut empty_count = 0;
            for file in 0..8 {
                let index = rank * 8 + file;
                if let Some(piece) = self.board[index] {
                    if empty_count > 0 {
                        fen.push_str(&empty_count.to_string());
                        empty_count = 0;
                    }
                    fen.push(piece.to_char());
                } else {
                    empty_count += 1;
                }
            }
            if empty_count > 0 {
                fen.push_str(&empty_count.to_string());
            }
            if rank < 7 {
                fen.push('/');
            }
        }

        // 2. Active Color
        fen.push(' ');
        fen.push(if self.active_color == Color::White { 'w' } else { 'b' });

        // 3. Castling Rights
        fen.push(' ');
        fen.push_str(&self.castling_rights);

        // 4. En Passant
        fen.push(' ');
        if let Some(sq) = self.en_passant_target {
            let file = (sq.index % 8) as u8;
            let rank = 8 - (sq.index / 8) as u8;
            let file_char = (b'a' + file) as char;
            fen.push(file_char);
            fen.push_str(&rank.to_string());
        } else {
            fen.push('-');
        }

        // 5 & 6
        fen.push_str(&format!(" {} {}", self.halfmove_clock, self.fullmove_number));

        fen
    }

    /// Converts the game state into an ASCII representation suited for LLM structural context.
    pub fn to_ascii(&self) -> String {
        let mut ascii = String::from("  +------------------------+\n");
        for rank in 0..8 {
            ascii.push_str(&format!("{} |", 8 - rank));
            for file in 0..8 {
                let index = rank * 8 + file;
                if let Some(piece) = self.board[index] {
                    ascii.push_str(&format!(" {} ", piece.to_char()));
                } else {
                    ascii.push_str(" . ");
                }
            }
            ascii.push_str("|\n");
        }
        ascii.push_str("  +------------------------+\n");
        ascii.push_str("    a  b  c  d  e  f  g  h\n");
        ascii
    }

    /// Generates a heat map of attacked squares
    /// Consumes the raycasting logic to build an authentic alpha-blend array distinguishing White vs Black mathematically!
    pub fn generate_heat_map(&self) -> [[(u8, u8); 8]; 8] {
        let mut heat_map = [[(0u8, 0u8); 8]; 8];
        
        for rank in 0..8 {
            for file in 0..8 {
                let index = rank * 8 + file;
                if let Some(piece) = self.board[index] {
                    // Fetch every square this piece exerts mathematical pressure on
                    let attacks = super::movement::get_pseudo_legal_attacks(self, index, piece);
                    
                    for att_idx in attacks {
                        let att_r = att_idx / 8;
                        let att_f = att_idx % 8;
                        
                        if piece.color == Color::White {
                            heat_map[att_r][att_f].0 += 1;
                        } else {
                            heat_map[att_r][att_f].1 += 1;
                        }
                    }
                }
            }
        }
        heat_map
    }

    /// Synthesizes the Second-Order Predictive Matrix by geometrically forecasting every
    /// legal 1-ply branch mathematically and overlaying bounds!
    pub fn generate_predictive_matrix(&self) -> [[(u8, u8); 8]; 8] {
        let mut aggregate_heat = [[(0u8, 0u8); 8]; 8];
        
        for index in 0..64 {
            if let Some(p) = self.board[index] {
                if p.color == self.active_color {
                    let legal_targets = super::movement::get_legal_moves(self, index, p);
                    for target in legal_targets {
                        // Mathematically fork the evaluation geometry
                        let mut branched_state = self.clone();
                        // Assume Queen promotion implicitly to evaluate maximal geometric consequences
                        branched_state.apply_move(index, target, Some(PieceType::Queen));
                        
                        let branch_heat = branched_state.generate_heat_map();
                        
                        for r in 0..8 {
                            for c in 0..8 {
                                aggregate_heat[r][c].0 = aggregate_heat[r][c].0.saturating_add(branch_heat[r][c].0);
                                aggregate_heat[r][c].1 = aggregate_heat[r][c].1.saturating_add(branch_heat[r][c].1);
                            }
                        }
                    }
                }
            }
        }
        aggregate_heat
    }

    /// Dynamically isolates the Top 4 mathematically contested bounds for the AI Payload formatting!
    pub fn extract_hottest_predictive_squares(&self, matrix: &[[(u8, u8); 8]; 8]) -> Vec<String> {
        let mut heatmap_scores = Vec::new();
        
        for r in 0..8 {
            for c in 0..8 {
                let heat_w = matrix[r][c].0;
                let heat_b = matrix[r][c].1;
                let total_heat = heat_w.saturating_add(heat_b);
                
                if total_heat > 0 {
                    let sq_idx = r * 8 + c;
                    let file_char = (b'a' + (sq_idx % 8) as u8) as char;
                    let rank_char = (b'1' + (7 - (sq_idx / 8)) as u8) as char;
                    let coord = format!("{}{}", file_char, rank_char);
                    
                    heatmap_scores.push((coord, total_heat));
                }
            }
        }
        
        // Sort explicitly by maximum absolute geometric density descending
        heatmap_scores.sort_by(|a, b| b.1.cmp(&a.1));
        
        heatmap_scores.into_iter().take(4).map(|(coord, heat)| format!("{} (Heat: {})", coord, heat)).collect()
    }

    /// Mutates the state structurally, transposing the Piece vector entirely!
    pub fn apply_move(&mut self, from: usize, to: usize, promotion_target: Option<PieceType>) {
        // A direct physical piece translation fundamentally shatters any explicit manual overrides organically!
        self.manual_terminal_status = None;
        
        let is_capture = self.board[to].is_some();
        let mut piece = self.board[from].take();
        let mut reset_halfmove = is_capture;
        
        if let Some(mut p) = piece {
            if p.piece_type == PieceType::Pawn {
                reset_halfmove = true;
            }
            // Handle Castling Geometry Transpositions
            if p.piece_type == PieceType::King {
                // Permanently disable castling rights 
                if p.color == Color::White {
                    self.castling_rights = self.castling_rights.replace("K", "").replace("Q", "");
                } else {
                    self.castling_rights = self.castling_rights.replace("k", "").replace("q", "");
                }
                
                // Physical Jump Execution natively
                if from == 60 && to == 62 { // White Kingside 
                    self.board[61] = self.board[63].take(); 
                } else if from == 60 && to == 58 { // White Queenside
                    self.board[59] = self.board[56].take(); 
                } else if from == 4 && to == 6 { // Black Kingside
                    self.board[5] = self.board[7].take();
                } else if from == 4 && to == 2 { // Black Queenside
                    self.board[3] = self.board[0].take(); 
                }
            }
            
            // Handle Rook explicit movement degradation 
            if p.piece_type == PieceType::Rook {
                if from == 63 { self.castling_rights = self.castling_rights.replace("K", ""); }
                if from == 56 { self.castling_rights = self.castling_rights.replace("Q", ""); }
                if from == 7 { self.castling_rights = self.castling_rights.replace("k", ""); }
                if from == 0 { self.castling_rights = self.castling_rights.replace("q", ""); }
            }

            // Handle physical en passant capture geometry
            if p.piece_type == PieceType::Pawn {
                if let Some(ep_sq) = self.en_passant_target {
                    if to == ep_sq.index {
                        // Wipe mathematically captured pawn behind!
                        let capture_idx = if p.color == Color::White { to + 8 } else { to - 8 };
                        self.board[capture_idx] = None;
                    }
                }
                
                // Implement Auto-Queening Promotion 
                let to_rank = to / 8;
                if to_rank == 0 || to_rank == 7 {
                    p.piece_type = promotion_target.unwrap_or(PieceType::Queen);
                }
            }
            
            piece = Some(p); // Load back the modified Piece structurally
        }
        
        // Execute structural landing
        self.board[to] = piece;

        // Reset en_passant_target dynamically if this was a double pawn push landing!
        self.en_passant_target = None;
        if let Some(p) = piece {
            if p.piece_type == PieceType::Pawn {
                let diff = (to as i32 - from as i32).abs();
                if diff == 16 {
                    let ep_idx = if p.color == Color::White { from - 8 } else { from + 8 };
                    self.en_passant_target = Some(Square::new(ep_idx));
                }
            }
        }
        
        // Castling explicitly ends if Rooks are mathematically captured by an enemy piece!
        if to == 63 { self.castling_rights = self.castling_rights.replace("K", ""); }
        if to == 56 { self.castling_rights = self.castling_rights.replace("Q", ""); }
        if to == 7 { self.castling_rights = self.castling_rights.replace("k", ""); }
        if to == 0 { self.castling_rights = self.castling_rights.replace("q", ""); }
        
        // Keep FEN perfectly stringified
        if self.castling_rights.is_empty() {
            self.castling_rights = "-".to_string(); 
        } else if self.castling_rights != "-" && self.castling_rights.contains('-') {
            self.castling_rights = self.castling_rights.replace("-", "");
        }

        // Toggle native color and turn tracking natively
        self.active_color = self.active_color.opposite();
        if self.active_color == Color::White {
            self.fullmove_number += 1;
        }

        // 50-Move Draw Bounds: physically evaluate resets!
        if reset_halfmove {
            self.halfmove_clock = 0;
        } else {
            self.halfmove_clock += 1;
        }
    }

    /// Evaluates if all geometric paths for the currently active color are violently exhausted natively!
    pub fn evaluate_terminal_state(&self) -> Option<GameEndStatus> {
        if self.manual_terminal_status.is_some() {
            return self.manual_terminal_status;
        }

        let mut has_moves = false;
        
        // Loop purely to test mathematical bounds 
        for sq_idx in 0..64 {
            if let Some(piece) = self.board[sq_idx] {
                if piece.color == self.active_color {
                    let moves = super::movement::get_legal_moves(self, sq_idx, piece);
                    if !moves.is_empty() {
                        has_moves = true;
                        break;
                    }
                }
            }
        }
        
        // Exiting loops natively verifies completely locked boards algebraically! 
        if !has_moves {
            if let Some(king_idx) = super::movement::find_king(self, self.active_color) {
                if super::movement::is_square_attacked(self, king_idx, self.active_color.opposite()) {
                    return Some(GameEndStatus::Checkmate(self.active_color.opposite())); // The attacking hostiles won!
                } else {
                    return Some(GameEndStatus::Stalemate); // Pinned but safe algebraically 
                }
            } else {
                return Some(GameEndStatus::Stalemate); // Failsafe for missing King strings
            }
        }
        
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_fen_parsing() {
        let start_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let state = GameState::from_fen(start_fen).expect("Failed to parse starting FEN");
        
        // Re-serialize and ensure it perfectly matches standard.
        assert_eq!(state.to_fen(), start_fen);
    }

    #[test]
    fn test_ascii_generation() {
        let state = GameState::new();
        let ascii = state.to_ascii();
        assert!(ascii.contains("P  P  P  P  P  P  P  P "));
        assert!(ascii.contains("p  p  p  p  p  p  p  p "));
        assert!(ascii.contains("a  b  c  d  e  f  g  h"));
    }

    #[test]
    fn test_evaluate_fools_mate() {
        // e4 g5, d4 f6, Qh5# 
        // White to move? No, Black to move and is checkmated by White!
        let fen = "rnbqkbnr/ppppp2p/5p2/6pQ/4P3/8/PPPP1PPP/RNB1KBNR b KQkq - 1 3";
        let state = GameState::from_fen(fen).unwrap();
        
        let terminal = state.evaluate_terminal_state();
        assert_eq!(terminal, Some(GameEndStatus::Checkmate(Color::White)), "Mathematically verifies White's victory!");
    }

    #[test]
    fn test_apply_move_fen_output() {
        let mut state = GameState::from_fen("rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq e6 0 2").unwrap();
        // Nc3 = 57 to 42
        state.apply_move(57, 42, None);
        let fen = state.to_fen();
        println!("Test Output FEN: {}", fen);
        
        let state_recovered = GameState::from_fen(&fen).unwrap();
        assert_eq!(state_recovered.board[42].unwrap().piece_type, PieceType::Knight);
    }
}
