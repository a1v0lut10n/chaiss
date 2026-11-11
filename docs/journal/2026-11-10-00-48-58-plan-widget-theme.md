# Plan: one widget theme through egui's StyleProvider (CHAI-0054)

- **When:** 2026-11-10 00:48:58 local
- **Type:** plan
- **Project:** chaiss

## Context

chaiss styles widgets two ways: a hand-painted `prominent_button` on the
aicogito tokens (written because 0.36's stock Button shifted on hover),
and stock egui defaults for everything else (navigation buttons, Send,
ComboBoxes, checkboxes, the prompt TextEdit). No theme preference is set,
so stock widgets follow the OS into light mode. egui 0.37's experimental
`StyleProvider` can style all stock widgets from classes and state.

## Details

Agreed plan (docs/implementation/CHAI-0054-widget-theme.md): a
`ChaissTheme` providing Button/TextEdit/Checkbox/Separator styles from
the existing tokens; button classes `chaiss::primary`, `chaiss::danger`,
`chaiss::toolbar`, untagged = standard; margins state-independent; dark
mode pinned; `interact_cursor` = pointing hand. The theme helpers become
thin classed-Button builders and `prominent_button` is deleted. Tests on
egui_kittest (geometry stable across states, class → colours, Role::Button
by label); WSLg screenshots for bar clipping and row alignment.

Spike (done): stock 0.37 Button no longer shifts across inactive /
hovered / pressed; the themed toolbar button (28 px, 15 pt) is equally
stable. The work is consistency and code removal, not a bug fix. kittest
panics on missing glyphs; tests set `MissingGlyphPolicy::Tofu`.

Out of scope: board painting, chat role colours (labels), session cards,
a light palette. Risk: `experimental` has no semver promise; all theme
code stays in ui/theme.rs and is first on the next egui upgrade.

## Links

- Plan: docs/implementation/CHAI-0054-widget-theme.md
- Follows: 2026-11-09-22-48-51-execution-wsl-prefer-x11.md
