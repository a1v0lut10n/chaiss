//! Desktop integration: a freedesktop `.desktop` entry plus icon.
//!
//! On Wayland the compositor never takes a window icon from the window
//! itself (winit 0.30's Wayland backend drops it); it looks up the desktop
//! entry whose file id equals the window's app id and uses that entry's
//! icon. Without a match, KDE Plasma shows its generic Wayland icon and
//! GNOME a blank one. `main.rs` therefore sets [`APP_ID`] as the Wayland
//! app id, and [`install`] writes the matching `chaiss.desktop` and the
//! logo into the user's XDG data directory.

use std::path::{Path, PathBuf};
use std::{env, fs, io};

/// Desktop-file id and Wayland app id; the two must stay equal.
pub const APP_ID: &str = "chaiss";

const ICON_SVG: &str = include_str!("../assets/chaiss-logo.svg");

/// Where [`install`] wrote.
#[derive(Debug)]
pub struct Installed {
    pub desktop_file: PathBuf,
    pub icon_file: PathBuf,
}

/// The user's XDG data home: `$XDG_DATA_HOME`, else `~/.local/share`.
pub fn data_home() -> Option<PathBuf> {
    if let Some(dir) = env::var_os("XDG_DATA_HOME").filter(|d| !d.is_empty()) {
        return Some(PathBuf::from(dir));
    }
    env::var_os("HOME").map(|home| PathBuf::from(home).join(".local/share"))
}

/// Quotes one `Exec=` argument per the desktop-entry spec: `%` is a field
/// code and must be doubled; anything with whitespace or shell-reserved
/// characters is wrapped in double quotes with the inner specials escaped.
fn exec_arg(arg: &str) -> String {
    let escaped = arg.replace('%', "%%");
    let needs_quotes = escaped
        .chars()
        .any(|c| c.is_whitespace() || "\"'`$\\><~|&;*?#()".contains(c));
    if !needs_quotes {
        return escaped;
    }
    let mut quoted = String::from("\"");
    for c in escaped.chars() {
        if matches!(c, '"' | '`' | '$' | '\\') {
            quoted.push('\\');
        }
        quoted.push(c);
    }
    quoted.push('"');
    quoted
}

/// Renders the desktop entry for the executable at `exe`, launched with
/// `workdir` as its working directory (where `.env`, `.chaiss.cola` and
/// `chaiss.db` live).
pub fn desktop_entry(exe: &Path, workdir: &Path) -> String {
    format!(
        "[Desktop Entry]\n\
         Type=Application\n\
         Version=1.5\n\
         Name=Chaiss\n\
         GenericName=Chess Board\n\
         Comment=AI-assisted chess board\n\
         Exec={exec}\n\
         Path={path}\n\
         Icon={APP_ID}\n\
         Terminal=false\n\
         Categories=Game;BoardGame;\n\
         Keywords=chess;llm;\n\
         StartupWMClass={APP_ID}\n",
        exec = exec_arg(&exe.to_string_lossy()),
        path = workdir.to_string_lossy(),
    )
}

/// Writes `applications/chaiss.desktop` and
/// `icons/hicolor/scalable/apps/chaiss.svg` under `data_home`, replacing
/// earlier copies. Directories are created as needed.
pub fn install(data_home: &Path, exe: &Path, workdir: &Path) -> io::Result<Installed> {
    let applications = data_home.join("applications");
    let icons = data_home.join("icons/hicolor/scalable/apps");
    fs::create_dir_all(&applications)?;
    fs::create_dir_all(&icons)?;

    let desktop_file = applications.join(format!("{APP_ID}.desktop"));
    fs::write(&desktop_file, desktop_entry(exe, workdir))?;
    let icon_file = icons.join(format!("{APP_ID}.svg"));
    fs::write(&icon_file, ICON_SVG)?;

    Ok(Installed {
        desktop_file,
        icon_file,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;
    use std::sync::atomic::{AtomicUsize, Ordering};

    /// A scratch XDG data home, removed on drop.
    struct ScratchDataHome(PathBuf);

    impl ScratchDataHome {
        fn new() -> Self {
            static COUNTER: AtomicUsize = AtomicUsize::new(0);
            let dir = env::temp_dir().join(format!(
                "chaiss-desktop-entry-test-{}-{}",
                std::process::id(),
                COUNTER.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir_all(&dir).expect("create scratch data home");
            Self(dir)
        }
    }

    impl Drop for ScratchDataHome {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn entry_names_binary_workdir_and_app_id() {
        let text = desktop_entry(
            Path::new("/home/me/.cargo/bin/chaiss"),
            Path::new("/home/me/chess"),
        );
        assert!(text.starts_with("[Desktop Entry]\n"), "{text}");
        assert!(text.contains("Exec=/home/me/.cargo/bin/chaiss\n"), "{text}");
        assert!(text.contains("Path=/home/me/chess\n"), "{text}");
        assert!(text.contains("Icon=chaiss\n"), "{text}");
        assert!(text.contains("StartupWMClass=chaiss\n"), "{text}");
    }

    #[test]
    fn exec_argument_is_quoted_and_escaped_when_needed() {
        assert_eq!(exec_arg("/opt/chaiss/chaiss"), "/opt/chaiss/chaiss");
        assert_eq!(
            exec_arg("/opt/my chaiss/chaiss"),
            "\"/opt/my chaiss/chaiss\""
        );
        assert_eq!(exec_arg("/opt/100%/chaiss"), "/opt/100%%/chaiss");
        assert_eq!(exec_arg("/opt/a\"b/chaiss"), "\"/opt/a\\\"b/chaiss\"");
    }

    #[test]
    fn install_writes_entry_and_icon_into_xdg_layout() {
        let home = ScratchDataHome::new();
        let installed = install(
            &home.0,
            Path::new("/home/me/.cargo/bin/chaiss"),
            Path::new("/home/me/chess"),
        )
        .expect("install");

        assert_eq!(
            installed.desktop_file,
            home.0.join("applications/chaiss.desktop")
        );
        assert_eq!(
            installed.icon_file,
            home.0.join("icons/hicolor/scalable/apps/chaiss.svg")
        );
        let entry = fs::read_to_string(&installed.desktop_file).expect("read entry");
        assert!(entry.contains("Path=/home/me/chess\n"));
        let icon = fs::read_to_string(&installed.icon_file).expect("read icon");
        assert!(icon.contains("<svg"));

        // Re-installing replaces rather than fails.
        install(&home.0, Path::new("/elsewhere/chaiss"), Path::new("/tmp")).expect("reinstall");
        let entry = fs::read_to_string(&installed.desktop_file).expect("read entry");
        assert!(entry.contains("Exec=/elsewhere/chaiss\n"));

        // When the freedesktop validator is installed, the entry must pass it.
        if let Ok(output) = Command::new("desktop-file-validate")
            .arg(&installed.desktop_file)
            .output()
        {
            assert!(
                output.status.success(),
                "desktop-file-validate: {}{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }
}
