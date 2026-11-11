# On WSL, open the window through X11 (CHAI-0053)

- **When:** 2026-11-09 22:48:51 local
- **Type:** execution
- **Project:** chaiss

## Context

The first run on a Windows machine through WSLg, while verifying CHAI-0052,
exited at startup with `Broken pipe` / `WinitEventLoop(ExitFailure(1))`.

## Details

WSLg's Weston 9.0.0 segfaults (`signal 11` in /mnt/wslg/stderr.log) under
the client-side decorations winit draws on Wayland; WSLGd restarts it and
every Wayland client is disconnected. Not an egui 0.37 regression: main on
0.36 fails the same way. A bare eframe probe crashed ~1 in 5 runs, 0 in 8
without decorations; chaiss on X11 ran clean.

Under WSL with DISPLAY set, chaiss now hands winit's X11 backend to
eframe's event-loop hook (`wsl::prefer_x11`, unit-tested), printing one
line; `CHAISS_WAYLAND=1` opts out. Adds a Linux-only `winit` 0.30
dependency (eframe doesn't re-export the X11 builder trait). Needs
`libxkbcommon-x11-0` on WSL.

Verified: gates clean; three 30 s WSLg runs, no Weston segfault.

## Links

- Plan: docs/implementation/CHAI-0053-wsl-prefer-x11.md
- Builds on: 2026-11-09-22-22-04-execution-upgrade-to-egui-0-37.md
