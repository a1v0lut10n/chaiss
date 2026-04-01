use eframe::egui;

pub fn draw(ctx: &egui::Context, app: &mut crate::app::ChaissApp) {
    #[allow(deprecated)]
    egui::SidePanel::right("chat_panel")
        .resizable(true)
        .min_width(280.0)
        .show(ctx, |ui| {
            ui.heading("LLM Chat & Analysis");
            ui.separator();

            // 1. Reserve exact geometric space at the bottom dynamically using TopBottomPanel!
            egui::TopBottomPanel::bottom("chat_input_panel")
                .frame(egui::Frame::none().inner_margin(egui::Margin::symmetric(0, 5)))
                .show_inside(ui, |ui| {
                    ui.checkbox(&mut app.silence_llm_analysis, "Silence AI Auto-Analysis for Rapid Testing");
                    ui.add_space(5.0);

                    let (send_clicked, enter_pressed, response) = ui.horizontal(|ui| {
                        let total_width = ui.available_width();
                        let btn_width = 50.0;
                        let spacing = ui.spacing().item_spacing.x;
                        let text_width = total_width - btn_width - spacing - 2.0;

                        let response = ui.add_sized(
                            [text_width, 60.0],
                            egui::TextEdit::multiline(&mut app.prompt_buffer)
                                .hint_text("Enter algebraic move or detailed prompt (Ctrl+Enter to send)...")
                                .desired_rows(3)
                        );
                        
                        let send_clicked = ui.add_sized([btn_width, 60.0], egui::Button::new("Send")).clicked();
                        let enter_pressed = response.has_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter) && i.modifiers.command);
                        
                        (send_clicked, enter_pressed, response)
                    }).inner;

                    if send_clicked || enter_pressed {
                        let text = app.prompt_buffer.trim().to_string();
                        if !text.is_empty() {
                            let is_likely_pgn = text.starts_with('[') || text.contains("1.") || text.contains("1-0") || text.contains("0-1") || text.contains("1/2-1/2");
                            let mut moves_applied = 0;
                            
                            // 1. Bulk PGN Injection Route
                            if is_likely_pgn {
                                let pgn_parsed = chaiss_core::engine::notation::parse_pgn_moves(&text);
                                for mv in pgn_parsed {
                                    if let Ok((from, to, promo)) = chaiss_core::engine::notation::parse_algebraic_move(&app.game_state, &mv) {
                                        let san_mapped = mv.clone();
                                        if app.history_stack.is_empty() {
                                            app.history_stack.push(app.game_state.to_fen());
                                            app.live_db_ply = 1;
                                        }
                                        
                                        app.game_state.apply_move(from, to, promo);
                                        let fen_snapshot = app.game_state.to_fen();
                                        
                                        app.history_stack.push(fen_snapshot.clone());
                                        app.live_db_ply += 1;
                                        app.view_cursor = app.history_stack.len() - 1;
                                        
                                        if app.algebraic_history.is_empty() {
                                            app.algebraic_history.push("START".to_string());
                                        }
                                        app.algebraic_history.push(mv.clone());

                                        if let (Some(client), Some(game_id)) = (app.db_client.clone(), app.active_game_id) {
                                            let move_ply = app.live_db_ply as i64;
                                            let fen_clone = fen_snapshot.clone();
                                            let san_clone = san_mapped.clone();
                                            tokio::spawn(async move {
                                                let _ = client.log_move(game_id, move_ply, &fen_clone, &san_clone).await;
                                            });
                                        }
                                        moves_applied += 1;
                                    } else {
                                        println!("Halting PGN Sequence mathematically at invalid geometry frame natively: {}", mv);
                                        break; 
                                    }
                                }
                                
                                if moves_applied > 0 {
                                    if !app.silence_llm_analysis {
                                        if let Some(tx) = &app.llm_tx {
                                            let payload = chaiss_core::llm::LlmPromptPayload {
                                                prompt: format!("I dynamically loaded `{}` structural moves natively from a PGN. Assess the resulting geometry organically.", moves_applied),
                                                current_fen: app.game_state.to_fen(),
                                                ascii_board: app.game_state.to_ascii(),
                                                algebraic_history: app.algebraic_history.clone(),
                                                chat_history: app.chat_history.clone(),
                                                system_role: "Companion".to_string(),
                                            };
                                            let _ = tx.send(crate::app::LlmEvent::InferenceRequested(payload));
                                        }
                                    }
                                    app.prompt_buffer.clear();
                                }
                            }
                            
                            // 2. Single Explicit Algebraic Node OR Raw Text Fallback
                            if moves_applied == 0 {
                                let is_single_token = text.split_whitespace().count() == 1;
                                
                                if is_single_token && chaiss_core::engine::notation::parse_algebraic_move(&app.game_state, &text).is_ok() {
                                    let (from, to, promo) = chaiss_core::engine::notation::parse_algebraic_move(&app.game_state, &text).unwrap();
                                println!("Algebraic Move Captured Structurally! from: {} to: {}", from, to);
                                
                                let san_mapped = text.clone();
                                
                                if app.history_stack.is_empty() {
                                    app.history_stack.push(app.game_state.to_fen());
                                    app.live_db_ply = 1;
                                }
                                
                                app.game_state.apply_move(from, to, promo);
                                let fen_snapshot = app.game_state.to_fen();
                                
                                app.history_stack.push(fen_snapshot.clone());
                                app.live_db_ply += 1;
                                app.view_cursor = app.history_stack.len() - 1;
                                
                                if app.algebraic_history.is_empty() {
                                    app.algebraic_history.push("START".to_string());
                                }
                                app.algebraic_history.push(text.clone());

                                if let (Some(client), Some(game_id)) = (app.db_client.clone(), app.active_game_id) {
                                    let move_ply = app.live_db_ply as i64;
                                    let fen_clone = fen_snapshot.clone();
                                    let san_clone = san_mapped.clone();
                                    tokio::spawn(async move {
                                        let _ = client.log_move(game_id, move_ply, &fen_clone, &san_clone).await;
                                    });
                                }
                                
                                if !app.silence_llm_analysis {
                                    if let Some(tx) = &app.llm_tx {
                                        let payload = chaiss_core::llm::LlmPromptPayload {
                                            prompt: format!("I played the formal move: {}. Please analyze this move conceptually.", text),
                                            current_fen: fen_snapshot.clone(),
                                            ascii_board: app.game_state.to_ascii(),
                                            algebraic_history: app.algebraic_history.clone(),
                                            chat_history: app.chat_history.clone(),
                                            system_role: "Companion".to_string(),
                                        };
                                        let _ = tx.send(crate::app::LlmEvent::InferenceRequested(payload));
                                    }
                                }
                                
                                app.prompt_buffer.clear();
                                
                            // 3. Raw LLM Context Prompt Stream
                            } else {
                                if let Some(tx) = &app.llm_tx {
                                    let payload = chaiss_core::llm::LlmPromptPayload {
                                        prompt: text.clone(),
                                        current_fen: app.game_state.to_fen(),
                                        ascii_board: app.game_state.to_ascii(),
                                        algebraic_history: app.algebraic_history.clone(),
                                        chat_history: app.chat_history.clone(),
                                        system_role: "Companion".to_string(),
                                    };
                                    let _ = tx.send(crate::app::LlmEvent::InferenceRequested(payload));
                                }
                                app.prompt_buffer.clear();
                            }
                            }
                        }
                        response.request_focus();  
                    }
                });

            // 2. Consume universally what physical rendering space remains dynamically natively!
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    if app.chat_history.is_empty() {
                        ui.label(egui::RichText::new("🤖 AI: Initialization complete, awaiting architecture definition...").italics());
                    } else {
                        for (idx, (role, msg)) in app.chat_history.iter().enumerate() {
                            ui.horizontal(|ui| {
                                if role == "User" {
                                    ui.label(egui::RichText::new("👤 User: ").color(egui::Color32::LIGHT_BLUE).strong());
                                } else {
                                    ui.label(egui::RichText::new("🤖 Agent: ").color(egui::Color32::LIGHT_GREEN).strong());
                                }
                            });
                            
                            // Natively route explicitly to the new geometric Markdown Caching Engine
                            ui.push_id(format!("chat_{}_{}", app.active_game_id.unwrap_or(0), idx), |ui| {
                                egui_commonmark::CommonMarkViewer::new()
                                    .show(ui, &mut app.markdown_cache, msg);
                            });
                                
                            ui.add_space(8.0);
                        }
                    }
                    if !app.live_llm_response.is_empty() {
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("🤖 Agent (Streaming): ").color(egui::Color32::LIGHT_GREEN).strong().italics());
                        });
                        
                        ui.push_id(format!("streaming_{}", app.active_game_id.unwrap_or(0)), |ui| {
                            egui_commonmark::CommonMarkViewer::new()
                                .show(ui, &mut app.markdown_cache, &app.live_llm_response);
                        });
                    }
                });
        });
}
