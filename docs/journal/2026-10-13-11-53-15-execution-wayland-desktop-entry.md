# Window icon on Wayland

- **When:** 2026-10-13 11:53:15 local
- **Type:** execution
- **Project:** chaiss

## Context

After the Ubuntu 26.04 upgrade, Plasma 6.6 on Wayland showed its
generic Wayland icon (Breeze's white W on yellow) for chaiss instead
of the logo. Wayland compositors take a window's icon from the desktop
entry matched by the window's app id; winit 0.30 drops the embedded
icon on Wayland, and chaiss set no app id, so nothing matched.

## Details

CHAI-0046. The viewport carries the app id `chaiss`; a new
`desktop_entry` module renders and installs the matching
`chaiss.desktop` (Exec = this binary, Path = the current directory,
where `.env`, `.chaiss.cola` and `chaiss.db` live) and the logo
as a scalable hicolor icon into the XDG data home, exposed as
`chaiss --install-desktop-entry` with `--help` and `--version`.
The flag is Linux-only: macOS and Windows get the icon natively from
eframe's app-icon setter, which the change does not touch. Verified
on this Plasma session (task bar shows the logo; the installed entry
passes `desktop-file-validate`). PR #64.

Upstream, checked 2026-10-13: winit's `xdg-toplevel-icon-v1` support
(PR #4288) exists only on the 0.31 beta line; eframe 0.36.1 pins winit
0.30.13; KWin 6.6 advertises the protocol, GNOME's mutter does not. So
the embedded icon will reach Plasma once eframe moves to winit 0.31,
and the desktop entry stays the right fix for GNOME regardless.

## Lesson

A window icon is a window-system feature, not an application one: on
Wayland the compositor decides from the desktop entry, so an app id
and an installed entry belong to the app from the start.

## Links

- Plan: `docs/implementation/CHAI-0046-wayland-desktop-entry.md`
- winit issue: https://github.com/rust-windowing/winit/issues/3859
