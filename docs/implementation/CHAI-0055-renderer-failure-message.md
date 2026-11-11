# CHAI-0055 — say what went wrong when the GPU renderer cannot start

- **Ticket:** CHAI-0055
- **Branch:** `bugfix/CHAI-0055-renderer-failure-message` (stacked on
  `feature/CHAI-0054-widget-theme`)
- **Status:** implemented (PR pending review)

## Premise

With two large models holding the GPU (genite-brain and flint-engine),
chaiss exited with

    Error: Wgpu(RequestDeviceError(RequestDeviceError { inner: Core(Device(Lost)) }))

That is `main` returning `eframe::run_native`'s error, printed in its
`Debug` form. eframe 0.37 renders through wgpu by default (the `glow`
feature is off), so a GPU that cannot hand out another device ends the
program before a window exists, and the one line it prints names
neither the cause nor a way around it.

## Action items

1. `main.rs`: keep `eframe::run_native`'s result, print
   `describe_run_failure(&err)` to stderr and exit 1, as the database
   open failure already does.
2. `describe_run_failure`: for `eframe::Error::Wgpu`, explain by
   variant. `RequestDeviceError`: the driver found a GPU but refused a
   device, usually because its memory is held by other programs, so
   close or shrink them. `RequestAdapterError` /
   `CustomNativeAdapterSelectionError`: no usable adapter for the
   selected backend, check the driver and any `WGPU_BACKEND` /
   `WGPU_ADAPTER_NAME`. Anything else: the renderer could not be set
   up. Every variant ends with the switches egui-wgpu reads from the
   environment (`WGPU_BACKEND=vulkan|gl`, `WGPU_POWER_PREF=low`) and the
   original error in both `Display` and `Debug` form. `Winit` /
   `WinitEventLoop`: no window; mention the display session and, on
   WSL, `libxkbcommon-x11-0`. `AppCreation`: the message as is.
3. Tests on the constructible variants (`NoSurfaceFormatsAvailable`,
   `AppCreation`); wgpu's `RequestDeviceError` has a private
   constructor.

## Verification

- [x] fmt, clippy `-D warnings`, workspace tests.
- [x] Forced failure on this machine: `WGPU_BACKEND=dx12 chaiss` (no
      DirectX on Linux) prints "chaiss could not start its GPU
      renderer", the generic cause, the switches and
      `WGPU error: Failed to create surface for any enabled backend`,
      exit code 1. The out-of-memory case itself needs a GPU that is
      full; its arm is covered by the variant match and the unit test's
      shape.

## Not done

- A fallback to the `glow` (OpenGL) renderer when wgpu fails. It would
  need eframe's `glow` feature and a second event loop in the same
  process, which winit does not allow; and a full GPU does not get
  emptier by switching API. The message points at `WGPU_BACKEND=gl`
  for the cases where a different driver path helps.
