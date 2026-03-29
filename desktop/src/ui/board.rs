use eframe::egui;

pub fn draw(ctx: &egui::Context) {
    #[allow(deprecated)]
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Board Context");
        ui.add_space(10.0);

        let available = ui.available_size();
        let board_size = available.x.min(available.y);
        
        if board_size > 0.0 {
            // Allocate a perfectly square area in the center
            let (rect, _response) = ui.allocate_exact_size(
                egui::vec2(board_size, board_size),
                egui::Sense::hover(),
            );

            let square_size = board_size / 8.0;

            // Render checkerboard grid
            for row in 0..8 {
                for col in 0..8 {
                    let is_light = (row + col) % 2 == 0;
                    
                    let color = if is_light {
                        egui::Color32::from_rgb(240, 217, 181) // Light square
                    } else {
                        egui::Color32::from_rgb(181, 136, 99) // Dark square
                    };

                    let min = rect.min + egui::vec2(col as f32 * square_size, row as f32 * square_size);
                    let max = min + egui::vec2(square_size, square_size);
                    
                    ui.painter().rect_filled(
                        egui::Rect::from_min_max(min, max),
                        0.0,
                        color,
                    );
                }
            }
            
            // Note: Heatmap accumulation overlay will render over these squares later!
        }
    });
}
