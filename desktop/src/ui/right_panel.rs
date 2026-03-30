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
                        // Primitive Algebraic heuristic testing geometry natively before passing to Neural networks!
                        let is_math_move = text.len() <= 7 && text.chars().all(|c| "KQRBNabcdefgh12345678-x+=#O0".contains(c));
                        
                        if is_math_move {
                            println!("Algebraic Move Captured: {}", text);
                            // TODO: Integrate Engine GameState.parse_algebraic_move(text) physically here!
                        } else {
                            if let Some(tx) = &app.llm_tx {
                                let _ = tx.send(crate::app::LlmEvent::ChatSubmitted(text));
                            }
                        }
                    }
                    app.prompt_buffer.clear();
                    response.request_focus(); // Maintain lock algebraically natively!
                }
            });
        });
}
