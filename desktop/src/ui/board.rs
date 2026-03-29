use eframe::egui;
use chaiss_core::engine::{GameState, Piece, PieceType, Color};

fn get_unicode_for_piece(piece: &Piece) -> &'static str {
    match piece.color {
        Color::White => match piece.piece_type {
            PieceType::King => "♔",
            PieceType::Queen => "♕",
            PieceType::Rook => "♖",
            PieceType::Bishop => "♗",
            PieceType::Knight => "♘",
            PieceType::Pawn => "♙",
        },
        Color::Black => match piece.piece_type {
            PieceType::King => "♚",
            PieceType::Queen => "♛",
            PieceType::Rook => "♜",
            PieceType::Bishop => "♝",
            PieceType::Knight => "♞",
            PieceType::Pawn => "♟",
        },
    }
}

pub fn draw(ctx: &egui::Context, state: &GameState) {
    #[allow(deprecated)]
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Board Context");
        ui.add_space(10.0);

        let available = ui.available_size();
        let board_size = available.x.min(available.y);
        
        if board_size > 0.0 {
            // Allocate perfectly square area in center
            let (rect, _response) = ui.allocate_exact_size(
                egui::vec2(board_size, board_size),
                egui::Sense::hover(),
            );

            let square_size = board_size / 8.0;
            let heat_map = state.generate_heat_map();

            // Render checkerboard grid natively over 0-63 indices
            for row in 0..8 {
                for col in 0..8 {
                    let is_light = (row + col) % 2 == 0;
                    
                    let base_color = if is_light {
                        egui::Color32::from_rgb(240, 217, 181) // Light square
                    } else {
                        egui::Color32::from_rgb(181, 136, 99) // Dark square
                    };

                    let min = rect.min + egui::vec2(col as f32 * square_size, row as f32 * square_size);
                    let max = min + egui::vec2(square_size, square_size);
                    let square_rect = egui::Rect::from_min_max(min, max);
                    
                    // 1. Draw base square natively mapped to vector rect bounds
                    ui.painter().rect_filled(square_rect, 0.0, base_color);

                    // 2. Draw Heat Map Transparent Modifier directly over the active vectors if heat exists
                    let heat = heat_map[row][col];
                    if heat > 0 {
                        let max_heat_alpha = 4.0; 
                        let alpha = ((heat as f32 / max_heat_alpha) * 150.0).clamp(0.0, 200.0) as u8;
                        let heat_color = egui::Color32::from_rgba_premultiplied(220, 20, 60, alpha); // Crimson
                        ui.painter().rect_filled(square_rect, 0.0, heat_color);
                    }

                    // 3. Mount Unicode rendering relative to the generated Engine structure 
                    let index = row * 8 + col;
                    if let Some(piece) = state.board[index] {
                        let text = get_unicode_for_piece(&piece);
                        // Make font fill 75% of arbitrary resizing cell vectors!
                        let font_id = egui::FontId::proportional(square_size * 0.75);
                        
                        ui.painter().text(
                            square_rect.center(),
                            egui::Align2::CENTER_CENTER,
                            text,
                            font_id,
                            egui::Color32::BLACK, 
                        );
                    }
                }
            }
        }
    });
}
