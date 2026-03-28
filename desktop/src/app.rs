use eframe::egui;

#[derive(Default)]
pub struct ChaissApp {
    // Central state holder
}

impl ChaissApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for ChaissApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ctx, |ui| {
            ui.heading("Chaiss - Native Desktop Scaffold");
            ui.label("Waiting for modular UI implementation...");
        });
    }

    fn ui(&mut self, _ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Stub to satisfy eframe 0.34 App trait
    }
}
