# CHAI-0053 — on WSL, open the window through X11

- **Ticket:** CHAI-0053
- **Branch:** `bugfix/CHAI-0053-wsl-prefer-x11` (stacked on
  `feature/CHAI-0052-upgrade-to-egui-0-37`)
- **Status:** done (executed 2026-11-09)
- **Origin:** the first run of chaiss on a Windows machine through WSLg,
  while verifying CHAI-0052.

## Premise

On WSL2, WSLg shows Linux windows on the Windows desktop through its own
Weston 9.0.0 compositor (RDP backend, `rdprail` shell), which offers both
a Wayland socket and XWayland. winit prefers Wayland, and on Wayland it
draws client-side decorations (a title bar and borders built from
`wl_subsurface`s backed by `wl_shm` buffers), since that Weston offers no
server-side decorations.

That Weston segfaults under those decorations. `/mnt/wslg/stderr.log`
records `weston … terminated with signal 11`. WSLGd restarts it, and every
Wayland client loses its connection: chaiss prints `Io error: Broken pipe`
(or `Connection reset by peer`) and exits with
`WinitEventLoop(ExitFailure(1))`.

Evidence, gathered with a throwaway eframe probe and chaiss itself:

- `main` (egui 0.36) fails the same way: not an egui 0.37 regression.
- A bare eframe window crashes about one run in five; chaiss repaints
  continuously and stays open, so it hits the crash almost immediately.
- The same probe with `with_decorations(false)`: 8/8 runs clean.
- chaiss on X11 (XWayland, with Windows drawing the native frame through
  RDP-RAIL): 3/3 runs clean for 30 s each.

A frameless window cannot be moved or resized, so turning decorations
off is no fix. X11 is the route other WSLg GUI apps rely on in practice.

## Action items

1. `chaiss/src/wsl.rs`: `prefer_x11(wsl_distro, display, opt_out)`, a
   pure decision: true when `WSL_DISTRO_NAME` is non-empty, `DISPLAY`
   is set (XWayland is up), and `CHAISS_WAYLAND` is not set to a
   non-empty value other than `0`. Unit-test the decision table.
2. `main.rs` (Linux only): when it holds, set
   `NativeOptions::event_loop_builder` to a hook calling winit's
   `EventLoopBuilderExtX11::with_x11()`, and print one line saying so,
   naming the `libxkbcommon-x11-0` prerequisite and the opt-out.
3. `chaiss/Cargo.toml`: `winit = "0.30"` (`x11` feature) for Linux
   targets only. eframe does not re-export winit or its platform
   extension traits; Cargo unifies this with eframe's own winit 0.30, so
   no second copy is built. Clearing `WAYLAND_DISPLAY` at runtime
   instead would mutate the environment after tokio's worker threads
   have started.
4. README (Linux / WSL note) and `--help` usage: the WSL behaviour, the
   package, the opt-out.
5. Gates: fmt, clippy `-D warnings`, tests; run chaiss on WSLg without
   any environment tweaks.

## What was built

As planned, with one shape change: the hook comes from an
`event_loop_hook()` function (a Linux body and a non-Linux `None`)
rather than mutating `NativeOptions` behind `cfg`, which would have
needed `#[allow(unused_mut)]` on other targets.

Verified: fmt, clippy `-D warnings` and tests clean (4 new decision-table
tests in `wsl`). On WSLg, `./target/debug/chaiss` with no environment
tweaks printed the X11 notice and ran 30 s three times with no Weston
segfault (counted in `/mnt/wslg/stderr.log`). The non-Linux `None` arm
was not compiled here (no Windows target installed).

## Consequences

- WSL runs need `libxkbcommon-x11-0` (winit loads it at runtime and
  panics without it). Native Linux, Windows and macOS are unaffected.
- On WSL the window gets a native Windows title bar instead of winit's
  drawn one.
- If a future WSLg fixes its Weston, `CHAISS_WAYLAND=1` restores
  Wayland without a rebuild; the default can then be dropped.
