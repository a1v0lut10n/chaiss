use eframe::egui;
use chaiss_core::engine::{GameState, Piece, PieceType, Color, GameEndStatus};

fn get_image_source_for_piece(piece: &Piece) -> egui::ImageSource<'static> {
    match piece.color {
        Color::White => match piece.piece_type {
            PieceType::King => egui::include_image!("../../assets/wK.svg"),
            PieceType::Queen => egui::include_image!("../../assets/wQ.svg"),
            PieceType::Rook => egui::include_image!("../../assets/wR.svg"),
            PieceType::Bishop => egui::include_image!("../../assets/wB.svg"),
            PieceType::Knight => egui::include_image!("../../assets/wN.svg"),
            PieceType::Pawn => egui::include_image!("../../assets/wP.svg"),
        },
        Color::Black => match piece.piece_type {
            PieceType::King => egui::include_image!("../../assets/bK.svg"),
            PieceType::Queen => egui::include_image!("../../assets/bQ.svg"),
            PieceType::Rook => egui::include_image!("../../assets/bR.svg"),
            PieceType::Bishop => egui::include_image!("../../assets/bB.svg"),
            PieceType::Knight => egui::include_image!("../../assets/bN.svg"),
            PieceType::Pawn => egui::include_image!("../../assets/bP.svg"),
        },
    }
}

pub fn draw(ctx: &egui::Context, app: &mut crate::app::ChaissApp) {
    let terminal_state = app.game_state.evaluate_terminal_state();

    #[allow(deprecated)]
    egui::CentralPanel::default().show(ctx, |ui| {
        egui::TopBottomPanel::bottom("sandbox_nav_panel")
            .frame(egui::Frame::NONE.inner_margin(egui::Margin::symmetric(10, 10)))
            .show_inside(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
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
            ui.horizontal_wrapped(|ui| {
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
        });

        ui.horizontal_wrapped(|ui| {
            
            // Draw visually striking Active Turn graphic explicitly mathematically instead of relying on unpredictable glyphs!
            let (turn_text, circle_fill) = match app.game_state.active_color {
                Color::White => ("White to Move", egui::Color32::WHITE),
                Color::Black => ("Black to Move", egui::Color32::BLACK),
            };
            
            ui.horizontal(|ui| {
                ui.heading("Active Turn:");
                ui.add_space(5.0);
                
                let (rect, _) = ui.allocate_exact_size(egui::vec2(20.0, 20.0), egui::Sense::hover());
                ui.painter().circle(
                    rect.center(), 
                    10.0, 
                    circle_fill, 
                    egui::Stroke::new(2.0, egui::Color32::WHITE) // Force a strict White border so Black is definitively visible on dark Egui backgrounds!
                );
                
                ui.add_space(5.0);
                ui.heading(egui::RichText::new(turn_text).color(egui::Color32::LIGHT_GRAY));
            });
            
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.checkbox(&mut app.flip_board, "Flip Board (Play as Black)");
            });
        });
        
        ui.add_space(10.0);

        let available = ui.available_size();
        
        // Reserve physical vertical space for the Native Navigation Controls at the bottom!
        let controls_height = 90.0; 
        let max_board_height = (available.y - controls_height).max(0.0);
        let margin = 25.0; // Render padding explicit for FIDE annotation letters!
        let board_size = (available.x.min(max_board_height) - margin * 2.0).max(0.0);
        
        if board_size > 0.0 {
            // Allocate physical area accounting for the external text margins
            let (rect, _response) = ui.allocate_exact_size(
                egui::vec2(board_size + margin * 2.0, board_size + margin * 2.0),
                egui::Sense::hover(),
            );

            let square_size = board_size / 8.0;
            let grid_start = rect.min + egui::vec2(margin, margin);
            
            // Draw File Vectors (A through H) across Top and Bottom geometries natively
            for col in 0..8 {
                let logical_col = if app.flip_board { 7 - col } else { col };
                let file_char = (b'a' + logical_col as u8) as char;
                let text = file_char.to_string();
                
                let top_rect = egui::Rect::from_min_max(
                    grid_start + egui::vec2(col as f32 * square_size, -margin),
                    grid_start + egui::vec2((col + 1) as f32 * square_size, 0.0)
                );
                let btm_rect = egui::Rect::from_min_max(
                    grid_start + egui::vec2(col as f32 * square_size, board_size),
                    grid_start + egui::vec2((col + 1) as f32 * square_size, board_size + margin)
                );
                
                ui.painter().text(top_rect.center(), egui::Align2::CENTER_CENTER, &text, egui::FontId::proportional(margin * 0.6), egui::Color32::LIGHT_GRAY);
                ui.painter().text(btm_rect.center(), egui::Align2::CENTER_CENTER, &text, egui::FontId::proportional(margin * 0.6), egui::Color32::LIGHT_GRAY);
            }
            
            // Draw Rank Vectors (1 through 8) descending down Left and Right geometries natively
            for row in 0..8 {
                let logical_row = if app.flip_board { 7 - row } else { row };
                let rank_char = (b'8' - logical_row as u8) as char;
                let text = rank_char.to_string();
                
                let lft_rect = egui::Rect::from_min_max(
                    grid_start + egui::vec2(-margin, row as f32 * square_size),
                    grid_start + egui::vec2(0.0, (row + 1) as f32 * square_size)
                );
                let rgt_rect = egui::Rect::from_min_max(
                    grid_start + egui::vec2(board_size, row as f32 * square_size),
                    grid_start + egui::vec2(board_size + margin, (row + 1) as f32 * square_size)
                );
                
                ui.painter().text(lft_rect.center(), egui::Align2::CENTER_CENTER, &text, egui::FontId::proportional(margin * 0.6), egui::Color32::LIGHT_GRAY);
                ui.painter().text(rgt_rect.center(), egui::Align2::CENTER_CENTER, &text, egui::FontId::proportional(margin * 0.6), egui::Color32::LIGHT_GRAY);
            }

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

                    let min = grid_start + egui::vec2(col as f32 * square_size, row as f32 * square_size);
                    let max = min + egui::vec2(square_size, square_size);
                    let square_rect = egui::Rect::from_min_max(min, max);
                    
                    let logical_row = if app.flip_board { 7 - row } else { row };
                    let logical_col = if app.flip_board { 7 - col } else { col };
                    let index = logical_row * 8 + logical_col;

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
                    let (white_heat, black_heat) = heat_map[logical_row][logical_col];
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
                                // Formulate True FIDE String BEFORE mutating active board geometry natively!
                                let algebraic_notation = chaiss_core::engine::notation::get_algebraic_notation(&app.game_state, sel_idx, index, None);
                                println!("Mathematical Move Resolved: {}", algebraic_notation);
                                
                                // 1. Push FIDE root to history on the very first active move!
                                if app.history_stack.is_empty() {
                                    app.history_stack.push(app.game_state.to_fen());
                                    if app.algebraic_history.is_empty() {
                                        app.algebraic_history.push("START".to_string());
                                    }
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
                                    
                                    let fen_snapshot = app.game_state.to_fen();
                                    app.history_stack.push(fen_snapshot.clone());
                                    
                                    app.view_cursor = app.history_stack.len() - 1;
                                    app.live_db_ply = app.history_stack.len();
                                    
                                    app.algebraic_history.push(algebraic_notation.clone());

                                    // Pass explicit FEN layout natively across Flume/Tokio cleanly!
                                    if let (Some(db), Some(game_id)) = (app.db_client.clone(), app.active_game_id) {
                                        let move_ply = app.live_db_ply as i64;
                                        let notation = algebraic_notation.clone();
                                        let fen_clone = fen_snapshot.clone();
                                                                                
                                        tokio::spawn(async move {
                                            let _ = db.log_move(game_id, move_ply, &fen_clone, &notation).await;
                                        });
                                    }
                                    
                                    // Seamless LLM Integration recursively natively mimicking Explicit Input 
                                    if !app.silence_llm_analysis {
                                        if let Some(tx) = &app.llm_tx {
                                            let payload = chaiss_core::llm::LlmPromptPayload {
                                                prompt: format!("The formal move `{}` was just executed physically on the board. Assess the structural geometry.", algebraic_notation),
                                                current_fen: fen_snapshot.clone(),
                                                ascii_board: app.game_state.to_ascii(),
                                                algebraic_history: app.algebraic_history.clone(),
                                                chat_history: app.chat_history.clone(),
                                                system_role: "Companion".to_string(), // Bound future dynamically!
                                            };
                                            let _ = tx.send(crate::app::LlmEvent::InferenceRequested(payload));
                                        }
                                    }
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

                    // 4. Mount pure SVG image explicitly into the generated matrix structure!
                    if let Some(piece) = app.game_state.board[index] {
                        let image_source = get_image_source_for_piece(&piece);
                        let img_size = egui::vec2(square_size * 0.85, square_size * 0.85);
                        
                        let image = egui::Image::new(image_source)
                            .fit_to_exact_size(img_size);
                            
                        // Inject SVG image dynamically geometrically locked across exact mathematical center matrices!
                        let img_rect = egui::Rect::from_center_size(square_rect.center(), img_size);
                        ui.put(img_rect, image);
                    }
                }
            }

            // Draw Checkmate / Stalemate overlay natively across absolute bounds!
            if let Some(status) = terminal_state {
                let overlay_color = egui::Color32::from_black_alpha(150);
                let full_board_rect = egui::Rect::from_min_max(grid_start, grid_start + egui::vec2(board_size, board_size));
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
        }
    });
}
