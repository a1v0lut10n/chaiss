use eframe::egui;

mod app;
mod desktop_entry;
mod ui;
#[cfg(target_os = "linux")]
mod wsl;

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

/// The winit event-loop hook: on WSL, force the X11 backend (see `wsl`).
#[cfg(target_os = "linux")]
fn event_loop_hook() -> Option<eframe::EventLoopBuilderHook> {
    if !wsl::prefer_x11_from_env() {
        return None;
    }
    println!(
        "WSL detected: opening the window through X11 (WSLg's Wayland compositor crashes \
         under client-drawn title bars). Needs libxkbcommon-x11-0; set {}=1 to use Wayland.",
        wsl::OPT_OUT_VAR
    );
    Some(Box::new(|builder| {
        use winit::platform::x11::EventLoopBuilderExtX11;
        builder.with_x11();
    }))
}

#[cfg(not(target_os = "linux"))]
fn event_loop_hook() -> Option<eframe::EventLoopBuilderHook> {
    None
}

const USAGE: &str = "\
Usage: chaiss [OPTION]

Run without options to open the chess board, reading .env, .chaiss.cola and
chaiss.db from the current directory.

  --install-desktop-entry  Linux only: write a launcher
                           (~/.local/share/applications/chaiss.desktop) and icon for
                           this binary, with the current directory as its working
                           directory; on Wayland this is what gives the window its icon
  -V, --version            print the version
  -h, --help               print this help

On WSL the window opens through X11 (needs libxkbcommon-x11-0), since WSLg's
Wayland compositor crashes under client-drawn title bars; set CHAISS_WAYLAND=1
to use Wayland anyway.
";

/// Writes the desktop entry and icon for this executable, working in the
/// current directory. Exits the process either way, since nothing else runs.
fn install_desktop_entry() -> ! {
    if !cfg!(target_os = "linux") {
        eprintln!(
            "--install-desktop-entry applies to Linux desktops only; on macOS and Windows \
             the embedded icon is set natively and needs no launcher file."
        );
        std::process::exit(2)
    }
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
        event_loop_builder: event_loop_hook(),
        ..Default::default()
    };

    let outcome = eframe::run_native(
        "Chaiss - AI Chess Board",
        native_options,
        Box::new(move |cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            ui::theme::ChaissTheme::install(&cc.egui_ctx);

            // No bundled emoji font: eframe's `system_fonts` provider finds the
            // platform's color emoji font (Segoe UI Emoji, Apple Color Emoji,
            // Noto Color Emoji) on demand. A configured monochrome emoji font
            // would sit ahead of it in every fallback chain and win.

            Ok(Box::new(ChaissApp::new(cc, db_client, initial_sessions)))
        }),
    );
    if let Err(err) = outcome {
        eprintln!("{}", describe_run_failure(&err));
        std::process::exit(1);
    }
    Ok(())
}

/// What to tell the user when the window or its renderer could not be
/// brought up. `main` returning the error would print its `Debug` form
/// (`Error: Wgpu(RequestDeviceError(... Device(Lost)))`), which says
/// nothing about the likely cause: the GPU had no memory left for another
/// device because other programs (large models, a second GPU-heavy app)
/// hold it.
fn describe_run_failure(err: &eframe::Error) -> String {
    let detail = format!("{err} ({err:?})");
    match err {
        eframe::Error::Wgpu(wgpu_err) => {
            let cause = match wgpu_err {
                eframe::egui_wgpu::WgpuError::RequestDeviceError(_) => {
                    "The graphics driver found a GPU but refused to open a device on it. \
                     The usual reason is that the GPU's memory is already taken by other \
                     programs (a running model, another GPU-heavy application): close or \
                     shrink them and start chaiss again."
                }
                eframe::egui_wgpu::WgpuError::RequestAdapterError(_)
                | eframe::egui_wgpu::WgpuError::CustomNativeAdapterSelectionError(_) => {
                    "No usable GPU adapter was found for the selected backend. Check that \
                     the graphics driver is installed and that any WGPU_BACKEND / \
                     WGPU_ADAPTER_NAME setting names something this machine has."
                }
                _ => {
                    "The GPU renderer could not be set up. Check the graphics driver, and \
                     that no other program has exhausted the GPU."
                }
            };
            format!(
                "chaiss could not start its GPU renderer.\n\n{cause}\n\n\
                 To try another path: WGPU_BACKEND=vulkan or WGPU_BACKEND=gl selects the \
                 backend, WGPU_POWER_PREF=low prefers an integrated GPU where there is one.\n\n\
                 Renderer error: {detail}"
            )
        }
        eframe::Error::Winit(_) | eframe::Error::WinitEventLoop(_) => format!(
            "chaiss could not open a window.\n\nCheck that a display session is available \
             (DISPLAY / WAYLAND_DISPLAY) and, on WSL, that libxkbcommon-x11-0 is installed.\n\n\
             Window error: {detail}"
        ),
        eframe::Error::AppCreation(_) => format!("chaiss failed while starting up: {detail}"),
        #[allow(unreachable_patterns)]
        _ => format!("chaiss could not start: {detail}"),
    }
}

#[cfg(test)]
mod run_failure_tests {
    use super::describe_run_failure;

    #[test]
    fn a_renderer_setup_failure_names_the_gpu_and_the_switches() {
        let err = eframe::Error::Wgpu(eframe::egui_wgpu::WgpuError::NoSurfaceFormatsAvailable);
        let text = describe_run_failure(&err);
        assert!(
            text.starts_with("chaiss could not start its GPU renderer."),
            "{text}"
        );
        assert!(text.contains("WGPU_BACKEND"), "{text}");
        assert!(text.contains("NoSurfaceFormatsAvailable"), "{text}");
    }

    #[test]
    fn app_creation_failures_keep_their_message() {
        let err = eframe::Error::AppCreation("database went away".into());
        let text = describe_run_failure(&err);
        assert!(text.contains("database went away"), "{text}");
        assert!(!text.contains("GPU"), "{text}");
    }
}
