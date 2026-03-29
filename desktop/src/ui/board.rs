use eframe::egui;
use chaiss_core::engine::{GameState, Piece, PieceType, Color, GameEndStatus};

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

pub fn draw(ctx: &egui::Context, app: &mut crate::app::ChaissApp) {
    let terminal_state = app.game_state.evaluate_terminal_state();

    #[allow(deprecated)]
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Board Context");
        ui.add_space(10.0);

        let available = ui.available_size();
        
        // Reserve physical vertical space for the Native Navigation Controls at the bottom!
        let controls_height = 90.0; 
        let max_board_height = (available.y - controls_height).max(0.0);
        
        let board_size = available.x.min(max_board_height);
        
        if board_size > 0.0 {
            // Allocate perfectly square area in center
            let (rect, _response) = ui.allocate_exact_size(
                egui::vec2(board_size, board_size),
                egui::Sense::hover(),
            );

            let square_size = board_size / 8.0;
            let heat_map = app.game_state.generate_heat_map();

            // Render checkerboard grid natively over 0-63 indices
            for row in 0..8 {
                for col in 0..8 {
                    let is_light = (row + col) % 2 == 0;
                    
                    let base_color = if is_light {
                        egui::Color32::from_rgb(238, 238, 238) // Off-white square
                    } else {
                        egui::Color32::from_rgb(142, 142, 142) // Greyish square
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
                    if Some(index) == app.selected_square {
                        // Highlight active selected origin tile
                        ui.painter().rect_filled(square_rect, 0.0, egui::Color32::from_rgba_premultiplied(255, 230, 0, 100)); // Yellow haze
                    } else if response.hovered() {
                        ui.painter().rect_filled(square_rect, 0.0, egui::Color32::from_rgba_premultiplied(255, 255, 255, 40)); 
                    }

                    // 3. Draw Dual-Tone Radiance Map pulling structured math tuple overlays dynamically!
                    let (white_heat, black_heat) = heat_map[row][col];
                    if white_heat > 0 || black_heat > 0 {
                        let max_heat = 3.0; // Optimal scaling for intense overlaps
                        let w_norm = (white_heat as f32 / max_heat).min(1.0);
                        let b_norm = (black_heat as f32 / max_heat).min(1.0);
                        
                        let b_val = (w_norm * 255.0) as u8; 
                        let r_val = (b_norm * 255.0) as u8; 
                        
                        let max_norm = w_norm.max(b_norm);
                        
                        let layers = 4;
                        for i in 0..layers {
                            let shrink_px = i as f32 * 3.0;
                            let inset_rect = square_rect.shrink(shrink_px);
                            
                            let alpha_fade = 1.0 - (i as f32 / layers as f32);
                            let final_alpha = (max_norm * 200.0 * alpha_fade) as u8;
                            
                            let heat_color = egui::Color32::from_rgba_premultiplied(r_val, 0, b_val, final_alpha);
                            ui.painter().rect_stroke(inset_rect, 0.0, egui::Stroke::new(3.0, heat_color), egui::StrokeKind::Inside);
                        }
                    }

                    // Native Interaction Math -> Consuming pseudo-math dynamically
                    if terminal_state.is_none() && response.clicked() {
                        // Only allow interactivity natively IF we are NOT in exploration mode, OR we are generating local branches natively!
                        if let Some(sel_idx) = app.selected_square {
                            // Already holding a piece. Where is the user trying to drop it?
                            let active_piece = app.game_state.board[sel_idx].unwrap();
                            let legal_moves = chaiss_core::engine::movement::get_legal_moves(&app.game_state, sel_idx, active_piece);
                            
                            if legal_moves.contains(&index) {
                                // 1. Push FIDE root to history on the very first active move!
                                if app.history_stack.is_empty() {
                                    app.history_stack.push(app.game_state.to_fen());
                                    app.live_db_ply = 1;
                                }
                                
                                // 2. If Sandbox or viewing history, generate branch!
                                if app.is_exploration_mode {
                                    app.sandbox_enabled = true; // Hard-lock into sandbox so Next doesn't mistakenly write!
                                    app.history_stack.truncate(app.view_cursor + 1);
                                    
                                    app.game_state.apply_move(sel_idx, index, None);
                                    app.history_stack.push(app.game_state.to_fen());
                                    app.view_cursor = app.history_stack.len() - 1;
                                } else {
                                    // 3. Live DB Tracked Move
                                    app.game_state.apply_move(sel_idx, index, None);
                                    app.history_stack.push(app.game_state.to_fen());
                                    app.view_cursor = app.history_stack.len() - 1;
                                    app.live_db_ply = app.history_stack.len();
                                    
                                    // TODO: `tokio::spawn` native sqlx `log_move` right here instantly!
                                }
                                
                                app.selected_square = None;
                            } else {
                                // Illegally clicked square. Revert active tracking unless clicking our own piece to swap
                                if let Some(p) = app.game_state.board[index] {
                                    if p.color == app.game_state.active_color {
                                        app.selected_square = Some(index);
                                    } else {
                                        app.selected_square = None;
                                    }
                                } else {
                                    app.selected_square = None;
                                }
                            }
                        } else {
                            // Null. User is picking up a piece to move!
                            if let Some(p) = app.game_state.board[index] {
                                if p.color == app.game_state.active_color { // Can't select enemy pieces to move!
                                    app.selected_square = Some(index);
                                }
                            }
                        }
                    }

                    // Render Native Guidance Dots
                    if let Some(sel_idx) = app.selected_square {
                        let active_piece = app.game_state.board[sel_idx].unwrap();
                        let legal_moves = chaiss_core::engine::movement::get_legal_moves(&app.game_state, sel_idx, active_piece);
                        if legal_moves.contains(&index) {
                            ui.painter().circle_filled(
                                square_rect.center(),
                                square_size * 0.15,
                                egui::Color32::from_rgba_premultiplied(0, 0, 0, 80), // Faint black dot targeting landing zone!
                            );
                        }
                    }

                    // 4. Mount Unicode rendering relative to the generated Engine structure
                    if let Some(piece) = app.game_state.board[index] {
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

            // Draw Checkmate / Stalemate overlay natively across absolute bounds!
            if let Some(status) = terminal_state {
                let overlay_color = egui::Color32::from_black_alpha(150);
                let full_board_rect = egui::Rect::from_min_max(rect.min, rect.max);
                ui.painter().rect_filled(full_board_rect, 0.0, overlay_color);
                
                let text = match status {
                    GameEndStatus::Checkmate(winner) => {
                        format!("Checkmate!\n{:?} Wins", winner)
                    },
                    GameEndStatus::Stalemate => {
                        "Stalemate!\nDraw".to_string()
                    }
                };
                
                ui.painter().text(
                    full_board_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    text,
                    egui::FontId::proportional(board_size * 0.1),
                    egui::Color32::WHITE,
                );
            }
            
            ui.add_space(20.0);

            // Scaffold the non-destructive History Scrubber and Exploration Mode flags!
            ui.horizontal(|ui| {
                ui.heading("Sandbox Navigation:");
                
                // Toggle explicitly manually forces Forward Sandbox mode 
                if ui.checkbox(&mut app.sandbox_enabled, "Enable Forward Sandbox").changed() {
                    if !app.sandbox_enabled {
                        // Instantly restore persisted state when toggling Sandbox Mode OFF natively!
                        app.history_stack.truncate(app.live_db_ply);
                        if app.live_db_ply > 0 {
                            app.view_cursor = app.live_db_ply - 1;
                            if let Ok(gs) = GameState::from_fen(&app.history_stack[app.view_cursor]) { 
                                app.game_state = gs; 
                            }
                        }
                    }
                }
                
                if app.is_exploration_mode {
                    ui.label(egui::RichText::new("EXPLORATION MODE ACTIVE (Not saving to Database)").color(egui::Color32::from_rgb(255, 140, 0))); // Warning Orange
                } else {
                    ui.label(egui::RichText::new("LIVE DB TRACKING").color(egui::Color32::from_rgb(0, 200, 0))); // Safe Green
                }
            });
            
            ui.add_space(5.0);
            ui.horizontal(|ui| {
                if ui.button("<< Start").clicked() && !app.history_stack.is_empty() {
                    app.view_cursor = 0;
                    if let Ok(gs) = GameState::from_fen(&app.history_stack[app.view_cursor]) { app.game_state = gs; }
                }
                if ui.button("< Prev Move").clicked() && app.view_cursor > 0 {
                    app.view_cursor -= 1;
                    if let Ok(gs) = GameState::from_fen(&app.history_stack[app.view_cursor]) { app.game_state = gs; }
                }
                if ui.button("Next Move >").clicked() && app.view_cursor + 1 < app.history_stack.len() {
                    app.view_cursor += 1;
                    if let Ok(gs) = GameState::from_fen(&app.history_stack[app.view_cursor]) { app.game_state = gs; }
                }
                // Live>> Discards the sandbox vector mathematically and resumes from the absolute latest DB layout!
                if ui.button("Live >>").clicked() {
                    app.sandbox_enabled = false;
                    app.history_stack.truncate(app.live_db_ply);
                    if app.live_db_ply > 0 {
                        app.view_cursor = app.live_db_ply - 1;
                        if let Ok(gs) = GameState::from_fen(&app.history_stack[app.view_cursor]) { app.game_state = gs; }
                    }
                }
            });
        }
    });
}
