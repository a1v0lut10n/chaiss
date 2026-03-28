use eframe::egui;

mod app;
mod ui;

use app::ChaissApp;

#[tokio::main]
async fn main() -> eframe::Result<()> {
    println!("Starting Chaiss Desktop...");
    chaiss_core::init();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Chaiss - AI Chess Board",
        native_options,
        Box::new(|cc| Ok(Box::new(ChaissApp::new(cc)))),
    )
}
