# CHAI-0052 — upgrade the egui stack 0.36 → 0.37 for color emoji

- **Ticket:** CHAI-0052
- **Branch:** `feature/CHAI-0052-upgrade-to-egui-0-37`
- **Status:** done (executed 2026-11-09 against egui 0.37.0)
- **Origin:** egui 0.37.0 (2026-10-15), "Colored emojis!". Follows the
  0.35 → 0.36 upgrade in CHAI-0031.

## Premise

chaiss renders emoji in its chat transcript (🤖 👤 ⚙ ⚠️), on session
cards (🗑) and in the toolbar (⚐ ⟲). Since 0.1.3 it has bundled
`NotoEmoji-Regular.ttf` (880 KB, monochrome) and appended it to both
font families, because egui could only draw single-color outline glyphs.

egui 0.37 can draw any glyph the system has, colour emoji included:

- `eframe`'s default `system_fonts` feature installs
  `egui_system_font_provider`, which asks the OS (fontconfig,
  DirectWrite, CoreText via `fontique`) for a font covering a character
  that no configured font has, and enables epaint's `color_fonts`
  rasterizer for `COLR`, `sbix` and `CBDT` glyphs. (The provider's README
  still says emoji fonts are skipped; the 0.37.0 source accepts colour
  and bitmap glyphs, so the README is stale.)
- Configured fonts form the head of every fallback chain; discovered
  system fonts are appended after them. A configured monochrome emoji
  font therefore always wins, and colour never shows.
- egui's own monochrome emoji fonts moved behind
  `monochrome_emoji_fonts`, which is off by default.

Ecosystem: `egui_extras` 0.37.0 and `egui_commonmark` 0.26.0 (which
requires egui ^0.37) were released the same day. Nothing blocks the upgrade.

## Action items

1. `chaiss/Cargo.toml`: `eframe` / `egui_extras` 0.37,
   `egui_commonmark` 0.26.0; `cargo update` the three.
2. `chaiss/src/main.rs`: drop the bundled emoji font registration and
   `set_fonts` call; delete `chaiss/assets/NotoEmoji-Regular.ttf`.
3. API renames from the 0.37 breaking changes:
   - `egui::WidgetType::Button` → `egui::Role::Button`
     (`ui/theme.rs`, `prominent_button`'s `WidgetInfo`).
   - `egui::Id::new` → `egui::Id::unique` (`ui/left_panel.rs`, the
     session card's corner-click id).
   - `Ui::id` → `Ui::scope_id` (`ui/board.rs`, square interaction ids).
4. Gates: `cargo fmt --all -- --check`, `cargo clippy --workspace
   --all-targets -- -D warnings`, `cargo test --workspace`.
5. Visual check under WSLg that the chat and toolbar glyphs render in
   colour and nothing turns to tofu.

## What was built

As planned. The compiler reported only the `Ui::id` deprecation beyond
the renames already known from the changelog; `Panel` constructors,
`ComboBox::from_id_salt` and the `egui_commonmark` viewer compiled
unchanged.

Visual check: a throwaway eframe 0.37 probe (kept out of the repo)
rendered every icon string chaiss uses, at 14 and 24 pt, under WSLg with
`fonts-noto-color-emoji` installed. 🤖 👤 ⚙️ ⚠️ 🗑️ drew in colour from
Noto Color Emoji; ⚐, ⟲, the arrows and ♔♕♖♗♘♙ (not emoji) came from
system text fonts as before. No tofu.

## Consequences

- Emoji now come from the platform: Segoe UI Emoji on Windows, Apple
  Color Emoji on macOS, Noto Color Emoji on most Linux desktops. A
  Linux system without any emoji font shows tofu for 🤖 and friends;
  on Debian/Ubuntu (including WSL) the fix is
  `apt install fonts-noto-color-emoji`.
- The binary is 880 KB lighter.
- Building on a fresh WSL/Ubuntu needs `libssl-dev` (pre-existing:
  `llm` → `reqwest` → `native-tls`), unrelated to this upgrade.

## Out of scope

The experimental `StyleProvider` theme plugin (eframe `experimental`
feature) could replace `theme.rs`'s hand-painted `prominent_button` with
classed stock `Button`s. It gets its own ticket.
