# Upgrade Chaiss to egui 0.35

- **When:** 2026-07-13 17:07:20 CEST
- **Type:** execution
- **Project:** chaiss

## Context
Branch `feature/CHAI-28-upgrade-to-egui-0-35` targets egui 0.35. The motivating
benefit is 0.35's inspection protocol, which lets an external MCP server read and
drive a running egui UI — wired up separately (see the follow-up entry). This
entry covers the version bump and source-level API migration. The same upgrade
was completed in the sibling aiquill-workspace project, whose approach we mirror.

## Details
Version bumps (`chaiss/Cargo.toml`; confirmed egui-0.35-compatible on crates.io):
- `eframe` / `egui_extras`: 0.34.1 → **0.35**.
- `egui_commonmark`: 0.23.0 → **0.24.0** (0.24 is the release built for egui 0.35).
- `chaiss-core` has no egui dependencies — untouched.

API migrations required by egui 0.35 (all confined to the `chaiss` crate):
- **`eframe::App::update(ctx, …)` → `App::ui(ui, …)`.** Panels now render into a
  root `Ui` supplied by eframe. Folded the old `update` body into `ui` and removed
  the previous 0.34-compat `ui` stub (`app.rs`). `ctx` was only used to forward the
  three panel draws, so the draws now take `&mut egui::Ui`.
- **`SidePanel` / `TopBottomPanel` / `CentralPanel` merged into a unified `Panel`**
  shown with `.show(ui, …)`. `min_width`/`max_width` → `.size_range(min..=max)`;
  `CentralPanel::default().show(ui, …)` retained (`left_panel.rs`, `right_panel.rs`,
  `board.rs`). Dropped the now-unneeded `#[allow(deprecated)]` wrappers.
- **`ComboBox::from_id_source` → `from_id_salt`** (`board.rs`).
- **`Frame::none()` → `Frame::NONE`** (`right_panel.rs`).
- Floating `Window::show` still takes `&Context`; obtained via `ui.ctx()` for the
  New-Game modal (`left_panel.rs`).

Verification:
- `cargo fmt --check`, `cargo clippy --workspace --all-targets`, and
  `cargo check --workspace --all-targets` → all clean.
- Runtime smoke test: app boots, loads games from the DB, and renders the
  roster / board / chat panes without panic.

## Links
- Branch: `feature/CHAI-28-upgrade-to-egui-0-35`
- Reference: aiquill-workspace commit `7fce3d3` (same upgrade).
- Follow-up: enable the egui MCP inspection server (next entry).
