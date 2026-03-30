use eframe::egui;

pub fn draw(ctx: &egui::Context, app: &mut crate::app::ChaissApp) {
    #[allow(deprecated)]
    egui::SidePanel::right("chat_panel")
        .resizable(true)
        .min_width(280.0)
        .show(ctx, |ui| {
            ui.heading("LLM Chat & Analysis");
            ui.separator();

            // Structurally enumerate dynamic chat arrays natively tracking AI and User Personas explicitly!
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .max_height(ui.available_height() - 40.0) 
                .show(ui, |ui| {
                    if app.chat_history.is_empty() {
                        ui.label(egui::RichText::new("🤖 AI: Initialization complete, awaiting architecture definition...").italics());
                    } else {
                        for (role, msg) in &app.chat_history {
                            if role == "User" {
                                ui.label(egui::RichText::new(format!("👤 {}: {}", role, msg)).color(egui::Color32::LIGHT_BLUE));
                            } else {
                                ui.label(egui::RichText::new(format!("🤖 {}: {}", role, msg)).color(egui::Color32::LIGHT_GREEN));
                            }
                            ui.add_space(4.0);
                        }
                    }
                    
                    // Render dynamically streaming Async tokens geographically live!
                    if !app.live_llm_response.is_empty() {
                        ui.label(egui::RichText::new(format!("🤖 Agent: {}▌", app.live_llm_response)).color(egui::Color32::LIGHT_GREEN));
                    }
                });

            ui.separator();

            ui.horizontal(|ui| {
                let response = ui.add_sized(
                    [ui.available_width() - 50.0, 30.0],
                    egui::TextEdit::singleline(&mut app.prompt_buffer).hint_text("Enter algebraic move or API prompt...")
                );
                
                let send_clicked = ui.button("Send").clicked();
                let enter_pressed = response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));

                if send_clicked || enter_pressed {
                    let text = app.prompt_buffer.trim().to_string();
                    if !text.is_empty() {
                        // Dynamically test if it rigorously maps against formal Chess Engine bounds precisely!
                        if let Ok((from, to, promo)) = chaiss_core::engine::notation::parse_algebraic_move(&app.game_state, &text) {
                            println!("Algebraic Move Captured Structurally! from: {} to: {}", from, to);
                            
                            // Re-calculate live structural history natively
                            let san_mapped = text.clone();
                            
                            if app.history_stack.is_empty() {
                                // Initialize root matrix identically to `board.rs` natively!
                                app.history_stack.push(app.game_state.to_fen());
                                app.live_db_ply = 1;
                            }
                            
                            // Immediately execute the move physically!
                            app.game_state.apply_move(from, to, promo);
                            let fen_snapshot = app.game_state.to_fen();
                            
                            app.history_stack.push(fen_snapshot.clone());
                            app.live_db_ply += 1;
                            app.view_cursor = app.history_stack.len() - 1;
                            
                            if app.algebraic_history.is_empty() {
                                app.algebraic_history.push("START".to_string());
                            }
                            app.algebraic_history.push(text.clone());

                            // Synchronize DB and natively broadcast exactly what the physical layer mathematically rendered directly to the AI loop!
                            if let (Some(client), Some(game_id)) = (app.db_client.clone(), app.active_game_id) {
                                let move_ply = app.live_db_ply as i64;
                                let fen_clone = fen_snapshot.clone();
                                let san_clone = san_mapped.clone();
                                tokio::spawn(async move {
                                    let _ = client.log_move(game_id, move_ply, &fen_clone, &san_clone).await;
                                });
                            }
                            
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
                            
                            app.prompt_buffer.clear();
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
                    response.request_focus(); // Maintain lock algebraically natively!
                }
            });
        });
}
