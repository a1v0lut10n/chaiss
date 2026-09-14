use eframe::egui;

mod app;
mod ui;

use app::ChaissApp;

use std::sync::Arc;

fn load_icon() -> Option<Arc<egui::IconData>> {
    let image_data = include_bytes!("../assets/chaiss-logo.png");
    let image = image::load_from_memory(image_data).ok()?;
    let image = image.to_rgba8();
    Some(Arc::new(egui::IconData {
        width: image.width(),
        height: image.height(),
        rgba: image.into_raw(),
    }))
}

#[tokio::main]
async fn main() -> eframe::Result<()> {
    // Explicitly load .env contents mathematically into the OS environment vector array!
    dotenvy::dotenv().ok();
    // rustemo (colap's parser runtime) traces every parse step to stderr in
    // debug builds unless this is set; keep debug launches as quiet as release.
    if std::env::var_os("RUSTEMO_NOTRACE").is_none() {
        std::env::set_var("RUSTEMO_NOTRACE", "1");
    }
    // Overlay .chaiss.cola (literate or plain cola) on top; it wins per key.
    chaiss_core::cola_config::init();

    println!("Starting Chaiss Desktop...");
    chaiss_core::init();

    // Establish the native global connection pool inside the Tokio runtime!
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://chaiss.db".to_string());
    let db_client = Arc::new(
        chaiss_core::db::DbClient::new(&db_url)
            .await
            .expect("Failed to securely bind SQLite Database locally!"),
    );

    // Fetch previous mathematical SQL sessions before booting graphical frames synchronously!
    let initial_sessions = db_client
        .get_active_games()
        .await
        .unwrap_or_else(|_| Vec::new());

    let mut viewport = egui::ViewportBuilder::default()
        .with_inner_size([1200.0, 800.0])
        .with_min_inner_size([800.0, 600.0]);

    if let Some(icon) = load_icon() {
        viewport = viewport.with_icon(icon);
    }

    let native_options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    eframe::run_native(
        "Chaiss - AI Chess Board",
        native_options,
        Box::new(move |cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);

            // Mathematically bind the native Noto Emoji rendering architecture into the context array
            let mut fonts = egui::FontDefinitions::default();
            fonts.font_data.insert(
                "emoji".to_owned(),
                std::sync::Arc::new(egui::FontData::from_static(include_bytes!(
                    "../assets/NotoEmoji-Regular.ttf"
                ))),
            );
            if let Some(prop) = fonts.families.get_mut(&egui::FontFamily::Proportional) {
                prop.push("emoji".to_owned());
            }
            if let Some(mono) = fonts.families.get_mut(&egui::FontFamily::Monospace) {
                mono.push("emoji".to_owned());
            }
            cc.egui_ctx.set_fonts(fonts);

            Ok(Box::new(ChaissApp::new(cc, db_client, initial_sessions)))
        }),
    )
}
