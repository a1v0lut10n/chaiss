# Upgrade to egui 0.37 with colour emoji (CHAI-0052)

- **When:** 2026-11-09 22:22:04 local
- **Type:** execution
- **Project:** chaiss

## Context

egui 0.37.0 (2026-10-15) renders any system glyph, colour emoji included,
via eframe's default `system_fonts` provider and epaint's `color_fonts`
rasterizer. chaiss had bundled monochrome NotoEmoji since 0.1.3.

## Details

eframe/egui_extras 0.37, egui_commonmark 0.26 (same-day release). Dropped
the bundled NotoEmoji-Regular.ttf: configured fonts head every fallback
chain, so it would always beat the system colour font. API renames:
`WidgetType`→`Role`, `Id::new`→`Id::unique`, `Ui::id`→`Ui::scope_id`;
nothing else broke. The provider's README claim that emoji fonts are
skipped is stale against the 0.37.0 source.

Verified: fmt/clippy/test clean; a throwaway eframe probe under WSLg with
fonts-noto-color-emoji rendered 🤖 👤 ⚙️ ⚠️ 🗑️ in colour and ⚐ ⟲ ♔ from
system text fonts, no tofu. WSLg displays the window natively; a fresh
WSL needs libssl-dev (pre-existing, via llm → reqwest → native-tls).

Deferred: the experimental StyleProvider theme plugin as its own ticket.

## Links

- Plan: docs/implementation/CHAI-0052-upgrade-to-egui-0-37.md
- Previous upgrade: 2026-08-06-12-28-48-execution-upgrade-to-egui-0-36.md
