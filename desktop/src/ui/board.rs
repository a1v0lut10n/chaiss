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

pub fn draw(ctx: &egui::Context, state: &mut GameState, selected_square: &mut Option<usize>) {
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
                    let index = row * 8 + col;

                    // 1. Assign native geometric Hit-Box listener 
                    let response = ui.interact(square_rect, ui.id().with(index), egui::Sense::click());
                    
                    // 2. Draw base square natively mapped to vector rect bounds
                    ui.painter().rect_filled(square_rect, 0.0, base_color);

                    // Optional overlays based on Hit-Box tracking / Hover
                    if Some(index) == *selected_square {
                        // Highlight active selected origin tile
                        ui.painter().rect_filled(square_rect, 0.0, egui::Color32::from_rgba_premultiplied(255, 230, 0, 100)); // Yellow haze
                    } else if response.hovered() {
                        ui.painter().rect_filled(square_rect, 0.0, egui::Color32::from_rgba_premultiplied(255, 255, 255, 40)); 
                    }

                    // 3. Draw Heat Map Transparent Modifier directly over the active vectors if heat exists
                    let heat = heat_map[row][col];
                    if heat > 0 {
                        let max_heat_alpha = 4.0; 
                        let alpha = ((heat as f32 / max_heat_alpha) * 150.0).clamp(0.0, 200.0) as u8;
                        let heat_color = egui::Color32::from_rgba_premultiplied(220, 20, 60, alpha); // Crimson
                        ui.painter().rect_filled(square_rect, 0.0, heat_color);
                    }

                    // Native Interaction Math -> Consuming pseudo-math dynamically
                    if response.clicked() {
                        if let Some(sel_idx) = *selected_square {
                            // Already holding a piece. Where is the user trying to drop it?
                            let active_piece = state.board[sel_idx].unwrap();
                            let legal_moves = chaiss_core::engine::movement::get_legal_moves(state, sel_idx, active_piece);
                            
                            if legal_moves.contains(&index) {
                                state.apply_move(sel_idx, index);
                                *selected_square = None;
                            } else {
                                // Illegally clicked square. Revert active tracking unless clicking our own piece to swap
                                if let Some(p) = state.board[index] {
                                    if p.color == state.active_color {
                                        *selected_square = Some(index);
                                    } else {
                                        *selected_square = None;
                                    }
                                } else {
                                    *selected_square = None;
                                }
                            }
                        } else {
                            // Null. User is picking up a piece to move!
                            if let Some(p) = state.board[index] {
                                if p.color == state.active_color { // Can't select enemy pieces to move!
                                    *selected_square = Some(index);
                                }
                            }
                        }
                    }

                    // Render Native Guidance Dots
                    if let Some(sel_idx) = *selected_square {
                        let active_piece = state.board[sel_idx].unwrap();
                        let legal_moves = chaiss_core::engine::movement::get_legal_moves(state, sel_idx, active_piece);
                        if legal_moves.contains(&index) {
                            ui.painter().circle_filled(
                                square_rect.center(),
                                square_size * 0.15,
                                egui::Color32::from_rgba_premultiplied(0, 0, 0, 80), // Faint black dot targeting landing zone!
                            );
                        }
                    }

                    // 4. Mount Unicode rendering relative to the generated Engine structure
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
