use eframe::egui;

/// Height of the prompt field and its Send button.
const CHAT_INPUT_HEIGHT: f32 = 60.0;

/// The prompt field with its Send button, laid out right to left: the
/// button takes its natural width first (a hard-coded 50 px ran short of
/// "Send" plus the 0.37 button padding, so its right stroke drew past the
/// panel's content edge and was clipped), and the field gets the rest.
/// Returns (send clicked, Ctrl+Enter pressed in the field, the field).
pub fn chat_input_row(ui: &mut egui::Ui, prompt: &mut String) -> (bool, bool, egui::Response) {
    ui.push_id("chat_input_row", |ui| {
        ui.allocate_ui_with_layout(
            egui::vec2(ui.available_width(), CHAT_INPUT_HEIGHT),
            egui::Layout::right_to_left(egui::Align::Center),
            |ui| {
                let send_clicked = ui
                    .add_sized([0.0, CHAT_INPUT_HEIGHT], egui::Button::new("Send"))
                    .clicked();
                let response = ui.add_sized(
                    [ui.available_width(), CHAT_INPUT_HEIGHT],
                    egui::TextEdit::multiline(prompt)
                        .id_salt("chat_prompt")
                        .hint_text(
                            "Enter algebraic move or detailed prompt (Ctrl+Enter to send)...",
                        )
                        .desired_rows(3),
                );
                let enter_pressed = response.has_focus()
                    && ui.input(|i| i.key_pressed(egui::Key::Enter) && i.modifiers.command);
                (send_clicked, enter_pressed, response)
            },
        )
        .inner
    })
    .inner
}

pub fn draw(ui: &mut egui::Ui, app: &mut crate::app::ChaissApp) {
    egui::Panel::right("chat_panel")
        .resizable(true)
        .default_size(320.0)
        .size_range(280.0..=450.0)
        .show(ui, |ui| {

            // Explicitly increase font sizes inside the chat panel for High-Res screens natively!
            if let Some(body_style) = ui.style_mut().text_styles.get_mut(&egui::TextStyle::Body) {
                body_style.size = 16.0; // Base is typically 14
            }

            ui.heading("LLM Chat & Analysis");

            // Model selector: only shown when more than one backend has a key
            // configured. Session-scoped; the next request uses the selection.
            if app.llm_targets.len() > 1 {
                ui.add_space(2.0);
                let mut selected = app.selected_llm_index;
                egui::ComboBox::from_id_salt("llm_model_selector")
                    .width(ui.available_width() - 8.0)
                    .selected_text(app.selected_llm_target().to_string())
                    .show_ui(ui, |ui| {
                        for (idx, target) in app.llm_targets.iter().enumerate() {
                            ui.selectable_value(&mut selected, idx, target.to_string());
                        }
                    });
                app.selected_llm_index = selected;
            }
            ui.separator();

            // 1. Reserve exact geometric space at the bottom dynamically using TopBottomPanel!
            egui::Panel::bottom("chat_input_panel")
                .frame(egui::Frame::NONE.inner_margin(egui::Margin::symmetric(0, 5)))
                .show(ui, |ui| {
                    ui.checkbox(&mut app.silence_llm_analysis, "Silence AI Auto-Analysis for Rapid Testing");
                    ui.add_space(5.0);

                    // Right to left: the button takes its natural width first (a
                    // hard-coded 50 px ran short of "Send" plus the 0.37 button
                    // padding, so its right stroke drew past the panel's content
                    // edge and was clipped), and the text field gets the rest.
                    let (send_clicked, enter_pressed, response) =
                        chat_input_row(ui, &mut app.prompt_buffer);

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
                                                predictive_matrix_hotspots: app.game_state.extract_hottest_predictive_squares(&app.game_state.generate_predictive_matrix()),
                                                system_role: "Companion".to_string(),
                                                target: app.selected_llm_target(),
                                            };
                                            let _ = tx.send(crate::app::LlmEvent::InferenceRequested(payload));
                                        }
                                    }
                                    app.prompt_buffer.clear();
                                }
                            }

                            // 1b. Intercept Manual Resignation String Algebraically Natively
                            if moves_applied == 0 {
                                let lower_text = text.to_lowercase();
                                let mut manual_resign = None;
                                if lower_text == "1-0" || lower_text == "white wins" || lower_text == "black resigned" {
                                    manual_resign = Some(("1-0".to_string(), chaiss_core::engine::Color::White));
                                } else if lower_text == "0-1" || lower_text == "black wins" || lower_text == "white resigned" {
                                    manual_resign = Some(("0-1".to_string(), chaiss_core::engine::Color::Black));
                                }

                                if let Some((san_mapped, winner)) = manual_resign {
                                    println!("Manual Game Termination Captured: {:?} Wins", winner);
                                    app.game_state.manual_terminal_status = Some(chaiss_core::engine::GameEndStatus::Resignation(winner));

                                    if app.history_stack.is_empty() {
                                        app.history_stack.push(app.game_state.to_fen());
                                        app.live_db_ply = 1;
                                    }

                                    let fen_snapshot = app.game_state.to_fen();
                                    app.history_stack.push(fen_snapshot.clone());
                                    app.live_db_ply += 1;
                                    app.view_cursor = app.history_stack.len() - 1;

                                    if app.algebraic_history.is_empty() {
                                        app.algebraic_history.push("START".to_string());
                                    }
                                    app.algebraic_history.push(san_mapped.clone());

                                    if let (Some(client), Some(game_id)) = (app.db_client.clone(), app.active_game_id) {
                                        let move_ply = app.live_db_ply as i64;
                                        let fen_clone = fen_snapshot.clone();
                                        tokio::spawn(async move {
                                            let _ = client.log_move(game_id, move_ply, &fen_clone, &san_mapped).await;
                                        });
                                    }
                                    app.prompt_buffer.clear();
                                    moves_applied += 1; // Structurally blocks fallbacks safely
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
                                            prompt: format!("The formal move `{}` was just physically executed on the board. Please analyze the resulting structural geometry conceptually.", text),
                                            current_fen: fen_snapshot.clone(),
                                            ascii_board: app.game_state.to_ascii(),
                                            algebraic_history: app.algebraic_history.clone(),
                                            chat_history: app.chat_history.clone(),
                                            predictive_matrix_hotspots: app.game_state.extract_hottest_predictive_squares(&app.game_state.generate_predictive_matrix()),
                                            system_role: "Companion".to_string(),
                                            target: app.selected_llm_target(),
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
                                        predictive_matrix_hotspots: app.game_state.extract_hottest_predictive_squares(&app.game_state.generate_predictive_matrix()),
                                        system_role: "Companion".to_string(),
                                        target: app.selected_llm_target(),
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

            let panel_width = ui.available_width();
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    ui.set_max_width(panel_width - 16.0); // Account for scrollbar
                    ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Wrap);

                    if app.chat_history.is_empty() {
                        ui.label(egui::RichText::new("🤖 AI: Initialization complete, awaiting architecture definition...").italics());
                    } else {
                        for (idx, (role, msg)) in app.chat_history.iter().enumerate() {
                            ui.horizontal(|ui| {
                                if role == "User" {
                                    ui.label(egui::RichText::new("👤 User: ").color(egui::Color32::LIGHT_BLUE).strong());
                                } else if role == "Agent" {
                                    ui.label(egui::RichText::new("🤖 Agent: ").color(egui::Color32::LIGHT_GREEN).strong());
                                } else if role == "Engine" {
                                    ui.label(egui::RichText::new("⚙ Engine: ").color(egui::Color32::GOLD).strong());
                                } else {
                                    ui.label(egui::RichText::new("⚠️ System: ").color(egui::Color32::from_rgb(255, 140, 0)).strong());
                                }
                            });

                            // Natively route explicitly to the new geometric Markdown Caching Engine.
                            // The horizontal ScrollArea contains any content the sanitizer cannot
                            // wrap (e.g. long code-block lines): it scrolls inside the panel
                            // instead of inflating the panel's content width and corrupting the
                            // panel/central-panel boundary.
                            ui.push_id(format!("chat_{}_{}", app.active_game_id.unwrap_or(0), idx), |ui| {
                                ui.allocate_ui(egui::vec2(panel_width - 24.0, 0.0), |ui| {
                                    egui::ScrollArea::horizontal()
                                        .auto_shrink([false, true])
                                        .show(ui, |ui| {
                                            egui_commonmark::CommonMarkViewer::new()
                                                .show(ui, &mut app.markdown_cache, msg);
                                        });
                                });
                            });

                            ui.add_space(8.0);
                        }
                    }

                    if app.is_llm_thinking && app.live_llm_response.is_empty() {
                        ui.horizontal_wrapped(|ui| {
                            ui.spinner();
                            ui.add_space(5.0);
                            let status = if app.correction_count > 0 {
                                format!(
                                    "⚙ The engine rejected the predicted line; Agent is revising ({} of {})...",
                                    app.correction_count,
                                    crate::app::MAX_LINE_CORRECTIONS
                                )
                            } else {
                                "🤖 Agent is analyzing the geometrical pathways...".to_string()
                            };
                            ui.label(egui::RichText::new(status).italics().color(egui::Color32::LIGHT_GRAY));
                        });
                        ui.add_space(8.0);
                    }
                    if !app.live_llm_response.is_empty() {
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("🤖 Agent (Streaming): ").color(egui::Color32::LIGHT_GREEN).strong().italics());
                        });

                        ui.push_id(format!("streaming_{}", app.active_game_id.unwrap_or(0)), |ui| {
                            let sanitized_stream = crate::app::ChaissApp::sanitize_markdown(&app.live_llm_response);
                            ui.allocate_ui(egui::vec2(panel_width - 24.0, 0.0), |ui| {
                                egui::ScrollArea::horizontal()
                                    .auto_shrink([false, true])
                                    .show(ui, |ui| {
                                        egui_commonmark::CommonMarkViewer::new()
                                            .show(ui, &mut app.markdown_cache, &sanitized_stream);
                                    });
                            });
                        });
                    }
                });
        });
}

#[cfg(test)]
mod tests {
    use super::chat_input_row;
    use eframe::egui::{Role, Vec2};
    use egui_kittest::{kittest::Queryable as _, Harness};
    use std::cell::Cell;
    use std::rc::Rc;

    /// The Send button must sit inside the row it is given, whatever width
    /// the theme's padding makes it: the old fixed 50 px pushed its right
    /// stroke past the panel's content edge, where the frame clipped it.
    #[test]
    fn send_button_stays_inside_the_row() {
        for width in [260.0_f32, 320.0, 480.0] {
            let right_edge = Rc::new(Cell::new(0.0_f32));
            let edge = Rc::clone(&right_edge);
            let mut prompt = String::new();
            let mut installed = false;
            let harness = Harness::builder()
                .with_size(Vec2::new(width, 120.0))
                .build_ui(move |ui| {
                    if !installed {
                        crate::ui::theme::ChaissTheme::install(ui.ctx());
                        installed = true;
                    }
                    edge.set(ui.max_rect().right());
                    chat_input_row(ui, &mut prompt);
                });
            let button = harness.get_by_role_and_label(Role::Button, "Send").rect();
            let field = harness.get_by_role(Role::MultilineTextInput).rect();
            assert!(
                button.right() <= right_edge.get() + 0.01,
                "width {width}: button ends at {} past the content edge {}",
                button.right(),
                right_edge.get()
            );
            assert!(
                field.right() <= button.left(),
                "width {width}: field {field:?} overlaps button {button:?}"
            );
            assert!(
                button.width() >= 40.0,
                "width {width}: button only {} wide",
                button.width()
            );
        }
    }
}
