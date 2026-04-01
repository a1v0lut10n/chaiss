use eframe::egui;

pub fn draw(ctx: &egui::Context, app: &mut crate::app::ChaissApp) {
    #[allow(deprecated)]
    egui::SidePanel::left("roster_panel")
        .resizable(true)
        .min_width(200.0)
        .show(ctx, |ui| {
            ui.heading("Game Roster");
            ui.add_space(10.0);
            
            if ui.button("Create New Game").clicked() {
                app.show_new_game_modal = true;
            }
            ui.add_space(20.0);

            ui.label("Active Sessions:");
            ui.separator();
            
            egui::ScrollArea::vertical().show(ui, |ui| {
                if app.active_sessions.is_empty() {
                    ui.label(egui::RichText::new("No active sessions natively tracked...").italics());
                } else {
                    for session in &app.active_sessions {
                        let is_active = app.active_game_id == Some(session.id);
                        let btn_label = format!("{} ({} vs {})", session.name, session.white_player, session.black_player);
                        
                        let rich_text = if is_active {
                            // Highlight dynamically playing instance natively in explicit Green!
                            egui::RichText::new(btn_label).strong().color(egui::Color32::from_rgb(100, 255, 100))
                        } else {
                            egui::RichText::new(btn_label)
                        };

                        ui.horizontal(|ui| {
                            let g_id = session.id;
                            let mut trigger_delete = false;
                            
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                // Draw mathematically right-aligned explicit 🗑 element natively!
                                let trash_btn = egui::Button::new(egui::RichText::new("🗑").color(egui::Color32::RED));
                                if ui.add_sized([24.0, 24.0], trash_btn).clicked() {
                                    trigger_delete = true;
                                }
                                
                                // Expand primary Loading string completely across remaining layout algebra cleanly
                                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                                    if ui.add_sized([ui.available_width(), 24.0], egui::Button::new(rich_text)).clicked() {
                                        if let (Some(db), Some(tx)) = (app.db_client.clone(), app.db_tx.clone()) {
                                            tokio::spawn(async move {
                                                if let Ok((root_fen, mut history, mut algebraic)) = db.load_game_history(g_id).await {
                                                    history.insert(0, root_fen);
                                                    algebraic.insert(0, "START".to_string());
                                                    let chat = db.load_chat_history(g_id).await.unwrap_or_default();
                                                    let _ = tx.send_async(crate::app::DbEvent::GameResumed { history, algebraic, chat, game_id: g_id }).await;
                                                }
                                            });
                                        }
                                    }
                                });
                            });
                            
                            if trigger_delete {
                                if let (Some(db), Some(tx)) = (app.db_client.clone(), app.db_tx.clone()) {
                                    tokio::spawn(async move {
                                        if db.delete_game(g_id).await.is_ok() {
                                            let _ = tx.send_async(crate::app::DbEvent::GameDeleted { game_id: g_id }).await;
                                        }
                                    });
                                }
                            }
                        });
                        ui.add_space(5.0);
                    }
                }
            });
        });

    if app.show_new_game_modal {
        egui::Window::new("Initialize Match")
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Game Name:");
                    ui.text_edit_singleline(&mut app.new_game_name);
                });
                ui.horizontal(|ui| {
                    let (rect, _) = ui.allocate_exact_size(egui::vec2(12.0, 12.0), egui::Sense::hover());
                    ui.painter().circle(rect.center(), 6.0, egui::Color32::WHITE, egui::Stroke::new(1.0, egui::Color32::WHITE));
                    ui.label("White Player:");
                    ui.text_edit_singleline(&mut app.white_player_name);
                });
                ui.horizontal(|ui| {
                    let (rect, _) = ui.allocate_exact_size(egui::vec2(12.0, 12.0), egui::Sense::hover());
                    ui.painter().circle(rect.center(), 6.0, egui::Color32::BLACK, egui::Stroke::new(1.0, egui::Color32::WHITE));
                    ui.label("Black Player:");
                    ui.text_edit_singleline(&mut app.black_player_name);
                });
                
                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    if ui.button("Cancel").clicked() {
                        app.show_new_game_modal = false;
                    }
                    if ui.button("Confirm & Start").clicked() {
                        app.show_new_game_modal = false;
                        println!("Invoking Async DB Threads for Game Creation!");
                        
                        if let (Some(db), Some(tx)) = (app.db_client.clone(), app.db_tx.clone()) {
                            let p1_name = app.white_player_name.clone();
                            let p2_name = app.black_player_name.clone();
                            let game_title = app.new_game_name.clone();
                            let start_fen = app.game_state.to_fen();
                            
                            // Spin non-blocking async DB generation safely outside 60FPS loop!
                            tokio::spawn(async move {
                                if let Ok(p1) = db.get_or_create_player(&p1_name).await {
                                    if let Ok(p2) = db.get_or_create_player(&p2_name).await {
                                        if let Ok(g_id) = db.create_game(&game_title, p1, p2, &start_fen).await {
                                            // Pipeline the resolution natively backwards to egui!
                                            let _ = tx.send_async(crate::app::DbEvent::GameCreated { game_id: g_id }).await;
                                        }
                                    }
                                }
                            });
                        }
                    }
                });
            });
    }
}
