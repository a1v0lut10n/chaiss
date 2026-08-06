use crate::ui::theme;
use eframe::egui;

pub fn draw(ui: &mut egui::Ui, app: &mut crate::app::ChaissApp) {
    // egui 0.35 shows floating windows against the Context; grab an owned handle
    // (cheap Arc clone) before the panel borrows `ui` mutably.
    let ctx = ui.ctx().clone();

    egui::Panel::left("roster_panel")
        .resizable(true)
        .size_range(200.0..=f32::INFINITY)
        .frame(
            egui::Frame::new()
                .fill(theme::SIDEBAR)
                .inner_margin(egui::Margin::same(12)),
        )
        .show(ui, |ui| {
            ui.heading("Game Roster");
            ui.add_space(10.0);

            if theme::primary_button(ui, "Create New Game").clicked() {
                app.show_new_game_modal = true;
            }
            ui.add_space(20.0);

            theme::section_label(ui, "Active Sessions");
            ui.add_space(6.0);

            egui::ScrollArea::vertical().show(ui, |ui| {
                if app.active_sessions.is_empty() {
                    ui.label(
                        egui::RichText::new("No active sessions natively tracked...")
                            .italics()
                            .color(theme::TEXT_FAINT),
                    );
                } else {
                    for session in &app.active_sessions {
                        let is_active = app.active_game_id == Some(session.id);
                        let g_id = session.id;

                        let mut trash_hovered = false;
                        let (card_res, trash_res) = theme::frame_with_corner_click(
                            ui,
                            theme::session_card_frame(is_active),
                            egui::Id::new(("session_card", g_id)),
                            |ui| {
                                ui.set_width(ui.available_width());
                                let mut trash_rect = egui::Rect::NOTHING;
                                ui.horizontal(|ui| {
                                    ui.with_layout(
                                        egui::Layout::right_to_left(egui::Align::Center),
                                        |ui| {
                                            // Paint faint; the corner response repaints it
                                            // red on hover after hit-testing below.
                                            let trash = ui.label(
                                                egui::RichText::new("🗑")
                                                    .size(theme::SIZE_SECONDARY)
                                                    .color(theme::TEXT_FAINT),
                                            );
                                            trash_rect = trash.rect;

                                            let name_color = if is_active {
                                                theme::ACCENT
                                            } else {
                                                theme::TEXT_PRIMARY
                                            };
                                            ui.with_layout(
                                                egui::Layout::left_to_right(egui::Align::Center),
                                                |ui| {
                                                    ui.add(
                                                        egui::Label::new(
                                                            egui::RichText::new(&session.name)
                                                                .strong()
                                                                .color(name_color),
                                                        )
                                                        .truncate(),
                                                    );
                                                },
                                            );
                                        },
                                    );
                                });
                                ui.add_space(2.0);
                                ui.add(
                                    egui::Label::new(
                                        egui::RichText::new(format!(
                                            "{} vs {}",
                                            session.white_player, session.black_player
                                        ))
                                        .size(theme::SIZE_FINE)
                                        .color(theme::TEXT_SECONDARY),
                                    )
                                    .truncate(),
                                );
                                trash_rect
                            },
                        );

                        if trash_res.hovered() {
                            trash_hovered = true;
                            ui.painter().text(
                                trash_res.rect.center(),
                                egui::Align2::CENTER_CENTER,
                                "🗑",
                                egui::FontId::proportional(theme::SIZE_SECONDARY),
                                theme::DANGER,
                            );
                        }

                        if card_res.hovered() || trash_res.hovered() {
                            ctx.set_cursor_icon(egui::CursorIcon::PointingHand);
                        }

                        if trash_res.clicked() {
                            if let (Some(db), Some(tx)) = (app.db_client.clone(), app.db_tx.clone())
                            {
                                tokio::spawn(async move {
                                    if db.delete_game(g_id).await.is_ok() {
                                        let _ = tx
                                            .send_async(crate::app::DbEvent::GameDeleted {
                                                game_id: g_id,
                                            })
                                            .await;
                                    }
                                });
                            }
                        } else if card_res.clicked() && !trash_hovered {
                            if let (Some(db), Some(tx)) = (app.db_client.clone(), app.db_tx.clone())
                            {
                                tokio::spawn(async move {
                                    if let Ok((root_fen, mut history, mut algebraic)) =
                                        db.load_game_history(g_id).await
                                    {
                                        history.insert(0, root_fen);
                                        algebraic.insert(0, "START".to_string());
                                        let chat =
                                            db.load_chat_history(g_id).await.unwrap_or_default();
                                        let flip_board =
                                            db.get_flip_board(g_id).await.unwrap_or(false);
                                        let _ = tx
                                            .send_async(crate::app::DbEvent::GameResumed {
                                                history,
                                                algebraic,
                                                chat,
                                                game_id: g_id,
                                                flip_board,
                                            })
                                            .await;
                                    }
                                });
                            }
                        }

                        ui.add_space(8.0);
                    }
                }
            });
        });

    if app.show_new_game_modal {
        egui::Window::new("Initialize Match")
            .collapsible(false)
            .resizable(false)
            .show(&ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Game Name:");
                    ui.text_edit_singleline(&mut app.new_game_name);
                });
                ui.horizontal(|ui| {
                    let (rect, _) =
                        ui.allocate_exact_size(egui::vec2(12.0, 12.0), egui::Sense::hover());
                    ui.painter().circle(
                        rect.center(),
                        6.0,
                        egui::Color32::WHITE,
                        egui::Stroke::new(1.0, egui::Color32::WHITE),
                    );
                    ui.label("White Player:");
                    ui.text_edit_singleline(&mut app.white_player_name);
                });
                ui.horizontal(|ui| {
                    let (rect, _) =
                        ui.allocate_exact_size(egui::vec2(12.0, 12.0), egui::Sense::hover());
                    ui.painter().circle(
                        rect.center(),
                        6.0,
                        egui::Color32::BLACK,
                        egui::Stroke::new(1.0, egui::Color32::WHITE),
                    );
                    ui.label("Black Player:");
                    ui.text_edit_singleline(&mut app.black_player_name);
                });

                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    if theme::standard_button(ui, "Cancel").clicked() {
                        app.show_new_game_modal = false;
                    }
                    if theme::primary_button(ui, "Confirm & Start").clicked() {
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
                                        if let Ok(g_id) =
                                            db.create_game(&game_title, p1, p2, &start_fen).await
                                        {
                                            // Pipeline the resolution natively backwards to egui!
                                            let _ = tx
                                                .send_async(crate::app::DbEvent::GameCreated {
                                                    game_id: g_id,
                                                })
                                                .await;
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
