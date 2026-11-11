//! WSL: open the window through X11 instead of Wayland.
//!
//! WSLg's compositor (Weston 9.0.0 with an RDP backend) segfaults under the
//! client-side decorations winit draws on Wayland, and every Wayland client
//! loses its connection when WSLGd restarts it (chaiss then exits with
//! `Broken pipe`). Over XWayland, Windows draws the frame natively and the
//! crash does not occur. See docs/implementation/CHAI-0053-wsl-prefer-x11.md.

/// Environment variable that, set to anything but `0`, keeps Wayland on WSL.
pub const OPT_OUT_VAR: &str = "CHAISS_WAYLAND";

/// Whether to force winit's X11 backend: running under WSL (`WSL_DISTRO_NAME`
/// is non-empty), XWayland is available (`DISPLAY` is set), and the user has
/// not opted out through [`OPT_OUT_VAR`].
pub fn prefer_x11(wsl_distro: Option<&str>, display: Option<&str>, opt_out: Option<&str>) -> bool {
    let on_wsl = wsl_distro.is_some_and(|d| !d.is_empty());
    let has_x = display.is_some_and(|d| !d.is_empty());
    let opted_out = opt_out.is_some_and(|v| !v.is_empty() && v != "0");
    on_wsl && has_x && !opted_out
}

/// [`prefer_x11`] read from the process environment.
pub fn prefer_x11_from_env() -> bool {
    let var = |name| std::env::var(name).ok();
    prefer_x11(
        var("WSL_DISTRO_NAME").as_deref(),
        var("DISPLAY").as_deref(),
        var(OPT_OUT_VAR).as_deref(),
    )
}

#[cfg(test)]
mod tests {
    use super::prefer_x11;

    #[test]
    fn native_linux_keeps_wayland() {
        assert!(!prefer_x11(None, Some(":0"), None));
        assert!(!prefer_x11(Some(""), Some(":0"), None));
    }

    #[test]
    fn wsl_with_xwayland_uses_x11() {
        assert!(prefer_x11(Some("Ubuntu"), Some(":0"), None));
        assert!(prefer_x11(Some("Ubuntu"), Some(":0"), Some("")));
        assert!(prefer_x11(Some("Ubuntu"), Some(":0"), Some("0")));
    }

    #[test]
    fn wsl_without_display_leaves_winit_alone() {
        assert!(!prefer_x11(Some("Ubuntu"), None, None));
        assert!(!prefer_x11(Some("Ubuntu"), Some(""), None));
    }

    #[test]
    fn opt_out_keeps_wayland_on_wsl() {
        assert!(!prefer_x11(Some("Ubuntu"), Some(":0"), Some("1")));
        assert!(!prefer_x11(Some("Ubuntu"), Some(":0"), Some("yes")));
    }
}
