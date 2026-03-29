

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
    /// Consumes the raycasting logic to build an authentic alpha-blend array.
    pub fn generate_heat_map(&self) -> [[u8; 8]; 8] {
        let mut heat_map = [[0u8; 8]; 8];
        
        for rank in 0..8 {
            for file in 0..8 {
                let index = rank * 8 + file;
                if let Some(piece) = self.board[index] {
                    // Fetch every square this piece exerts mathematical pressure on
                    let attacks = super::movement::get_pseudo_legal_attacks(self, index, piece);
                    
                    for att_idx in attacks {
                        let att_r = att_idx / 8;
                        let att_f = att_idx % 8;
                        heat_map[att_r][att_f] += 1;
                    }
                }
            }
        }
        
        heat_map
    }

    /// Mutates the state structurally, transposing the Piece vector entirely!
    pub fn apply_move(&mut self, from: usize, to: usize) {
        let piece = self.board[from].take();
        
        // Handle physical en passant capture execution geometrically!
        if let Some(p) = piece {
            if p.piece_type == PieceType::Pawn {
                if let Some(ep_sq) = self.en_passant_target {
                    if to == ep_sq.index {
                        // The user moved into the EP square. We must physically wipe the pawn orthogonal to it!
                        let capture_idx = if p.color == Color::White { to + 8 } else { to - 8 };
                        self.board[capture_idx] = None;
                    }
                }
            }
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

        // Toggle native color and turn tracking natively
        self.active_color = self.active_color.opposite();
        if self.active_color == Color::White {
            self.fullmove_number += 1;
        }

        // (Pending: Castling executions & halfmove clocks)
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
        assert!(ascii.contains("r  n  b  q  k  b  n  r "));
        assert!(ascii.contains("R  N  B  Q  K  B  N  R "));
        assert!(ascii.contains("P  P  P  P  P  P  P  P "));
        assert!(ascii.contains("p  p  p  p  p  p  p  p "));
        assert!(ascii.contains("a  b  c  d  e  f  g  h"));
    }
}
