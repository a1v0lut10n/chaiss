use eframe::egui;

pub fn draw(ctx: &egui::Context, chat_input: &mut String) {
    #[allow(deprecated)]
    egui::SidePanel::right("chat_panel")
        .resizable(true)
        .min_width(280.0)
        .show(ctx, |ui| {
            ui.heading("LLM Chat & Analysis");
            ui.separator();

            // Reserve most space for the chat history, leaving room at bottom
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .max_height(ui.available_height() - 40.0) 
                .show(ui, |ui| {
                    ui.label("🤖 AI: Initialization complete, awaiting first move...");
                    ui.add_space(8.0);
                    ui.label("👤 Human: I will start with e4.");
                    ui.add_space(8.0);
                    ui.label("🤖 AI: A classic opening sequence. Black traditionally responds with e5 or c5.");
                });

            ui.separator();

            ui.horizontal(|ui| {
                let response = ui.add_sized(
                    [ui.available_width() - 50.0, 30.0],
                    egui::TextEdit::singleline(chat_input).hint_text("Enter prompt...")
                );
                
                if ui.button("Send").clicked() || (response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))) {
                    chat_input.clear();
                }
            });
        });
}
