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
                for i in 1..=5 {
                    if ui.button(format!("Human vs Model (Match {})", i)).clicked() {
                        // Switch active game logic
                    }
                    ui.add_space(5.0);
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
                    ui.label("White Player:");
                    ui.text_edit_singleline(&mut app.white_player_name);
                });
                ui.horizontal(|ui| {
                    ui.label("Black Player:");
                    ui.text_edit_singleline(&mut app.black_player_name);
                });
                
                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    if ui.button("Cancel").clicked() {
                        app.show_new_game_modal = false;
                    }
                    if ui.button("Confirm & Start").clicked() {
                        // Connect to DbClient natively here!
                        app.show_new_game_modal = false;
                        println!("New Game Captured: {}", app.new_game_name);
                    }
                });
            });
    }
}
