use eframe::egui;

mod app;
mod ui;

use app::ChaissApp;

use std::sync::Arc;

#[tokio::main]
async fn main() -> eframe::Result<()> {
    // Explicitly load .env contents mathematically into the OS environment vector array!
    dotenvy::dotenv().ok();
    
    println!("Starting Chaiss Desktop...");
    chaiss_core::init();
    
    // Establish the native global connection pool inside the Tokio runtime!
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://chaiss.db".to_string());
    let db_client = Arc::new(chaiss_core::db::DbClient::new(&db_url).await.expect("Failed to securely bind SQLite Database locally!"));

    // Fetch previous mathematical SQL sessions before booting graphical frames synchronously!
    let initial_sessions = db_client.get_active_games().await.unwrap_or_else(|_| Vec::new());

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Chaiss - AI Chess Board",
        native_options,
        Box::new(move |cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(ChaissApp::new(cc, db_client, initial_sessions)))
        }),
    )
}
