use eframe::egui;

pub fn draw(ctx: &egui::Context) {
    #[allow(deprecated)]
    egui::SidePanel::left("roster_panel")
        .resizable(true)
        .min_width(200.0)
        .show(ctx, |ui| {
            ui.heading("Game Roster");
            ui.add_space(10.0);
            
            if ui.button("Create New Game").clicked() {
                // Logic hook for game creation modal
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
}
