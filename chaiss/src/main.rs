use eframe::egui;

mod app;
mod desktop_entry;
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

const USAGE: &str = "\
Usage: chaiss [OPTION]

Run without options to open the chess board, reading .env, .chaiss.cola and
chaiss.db from the current directory.

  --install-desktop-entry  write a launcher (~/.local/share/applications/chaiss.desktop)
                           and icon for this binary, with the current directory as
                           its working directory; on Wayland this is what gives the
                           window its icon
  -V, --version            print the version
  -h, --help               print this help
";

/// Writes the desktop entry and icon for this executable, working in the
/// current directory. Exits the process either way, since nothing else runs.
fn install_desktop_entry() -> ! {
    let result = (|| -> std::io::Result<desktop_entry::Installed> {
        let exe = std::env::current_exe()?;
        let workdir = std::env::current_dir()?;
        let data_home = desktop_entry::data_home().ok_or_else(|| {
            std::io::Error::other(
                "neither XDG_DATA_HOME nor HOME is set; cannot locate the data directory",
            )
        })?;
        desktop_entry::install(&data_home, &exe, &workdir)
    })();
    match result {
        Ok(installed) => {
            println!(
                "Wrote {}\nWrote {}\nLaunchers pick the entry up on their own; if the icon does not \
                 appear, run `update-desktop-database` on that applications directory.",
                installed.desktop_file.display(),
                installed.icon_file.display()
            );
            std::process::exit(0)
        }
        Err(err) => {
            eprintln!("Could not install the desktop entry: {err}");
            std::process::exit(1)
        }
    }
}

#[tokio::main]
async fn main() -> eframe::Result<()> {
    match std::env::args().nth(1).as_deref() {
        None => {}
        Some("--install-desktop-entry") => install_desktop_entry(),
        Some("-V" | "--version") => {
            println!("chaiss {}", env!("CARGO_PKG_VERSION"));
            return Ok(());
        }
        Some("-h" | "--help") => {
            print!("{USAGE}");
            return Ok(());
        }
        Some(other) => {
            eprint!("chaiss: unrecognised option `{other}`\n\n{USAGE}");
            std::process::exit(2)
        }
    }

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
    // Resolved through the cola overlay (loaded by `cola_config::init` above,
    // which must keep running before this point), then the environment.
    let db_url = chaiss_core::cola_config::config_value("DATABASE_URL")
        .unwrap_or_else(|| "sqlite://chaiss.db".to_string());
    let db_client = match chaiss_core::db::DbClient::new(&db_url).await {
        Ok(client) => Arc::new(client),
        Err(err) => {
            eprintln!("{}", chaiss_core::db::describe_open_failure(&db_url, &err));
            std::process::exit(1);
        }
    };

    // Fetch previous mathematical SQL sessions before booting graphical frames synchronously!
    let initial_sessions = db_client
        .get_active_games()
        .await
        .unwrap_or_else(|_| Vec::new());

    // The app id is what Wayland compositors match against the installed
    // `chaiss.desktop` (see `desktop_entry`) to find the window icon; the
    // embedded icon below serves X11, Windows and macOS, where winit 0.30
    // can hand it to the window system directly.
    let mut viewport = egui::ViewportBuilder::default()
        .with_app_id(desktop_entry::APP_ID)
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
