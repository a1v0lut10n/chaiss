# CHAI-0046 — the window icon on Wayland: app id plus desktop entry

- **Ticket:** CHAI-0046
- **Branch:** `feature/CHAI-0046-wayland-desktop-entry`
- **Status:** implemented (PR pending review)

## Premise

After upgrading to Ubuntu 26.04 (KDE Plasma 6.6 on Wayland) the task bar
shows a white W on a yellow disc instead of the chaiss logo. That is
Breeze's `wayland` application icon, Plasma's fallback for a Wayland
window it cannot match to a desktop entry. The embedded icon passed via
`ViewportBuilder::with_icon` reaches X11 (`_NET_WM_ICON`), Windows and
macOS, but winit 0.30.13's Wayland `set_window_icon` is an empty
function, and no app id is set, so the compositor has nothing to match.

Newer upstream state, checked 2026-10-13: winit implemented the
`xdg-toplevel-icon-v1` protocol in PR #4288 (June 2025), shipped only
on the 0.31 line (0.31.0-beta.3, September 2026); the newest stable,
0.30.13 (March 2026), does not carry it. eframe 0.36.1, the newest
release, depends on winit `^0.30.13`. KWin 6.6.6 here advertises
`xdg_toplevel_icon_manager_v1`, so the embedded icon will reach Plasma
once eframe moves to winit 0.31; GNOME's mutter does not implement the
protocol and keeps needing the desktop entry. The entry is therefore
the right fix now and stays right later.

## Action items

1. `ViewportBuilder::with_app_id("chaiss")` in `main.rs`; the constant
   lives in the new `desktop_entry` module so the app id and the
   desktop-file id cannot drift apart. eframe persistence is not used,
   so the app id's effect on the storage directory is moot.
2. `chaiss/src/desktop_entry.rs`: `desktop_entry(exe, workdir)` renders
   the entry (Exec quoted per the spec, `Path=` the working directory
   where `.env`, `.chaiss.cola` and `chaiss.db` live, `Icon=chaiss`,
   `StartupWMClass=chaiss` for X11); `install(data_home, exe, workdir)`
   writes `applications/chaiss.desktop` and
   `icons/hicolor/scalable/apps/chaiss.svg`. The SVG is embedded from
   `chaiss/assets/chaiss-logo.svg` (copied from `static/`, which lies
   outside the packaged crate). The PNG is 242×256, not square, so only
   the scalable icon is installed.
3. `chaiss --install-desktop-entry`, with `--help` and `--version`;
   hand-rolled argument check, no new dependency. Exec points at
   `current_exe()`, Path at `current_dir()`, so the launcher starts the
   same binary in the same directory the user ran the command from.
4. Tests against a scratch XDG data home: entry content, Exec quoting,
   install layout and overwrite, and `desktop-file-validate` when it is
   on PATH.
5. README section "Desktop integration"; CHANGELOG `[Unreleased]`.

## Verification checklist

- [x] fmt / clippy `-D warnings` / `cargo test --workspace` clean.
- [x] On this Plasma 6.6 Wayland session: install the entry, launch
      chaiss, and the task bar shows the chaiss logo instead of the
      Wayland fallback (screenshot via spectacle).
- [x] `desktop-file-validate` passes on the installed entry.

## Out of scope

- Upgrading eframe/winit (the workspace rule upgrades egui minors
  deliberately; no released eframe fixes this yet).
- System-wide installation (`/usr/share`); `cargo install` is per user.
- Packaging (deb, flatpak), where the entry would ship in the package.
