# One widget theme through egui's StyleProvider (CHAI-0054)

- **When:** 2026-11-10 22:58:45 local
- **Type:** execution
- **Project:** chaiss

## Context

Executes the plan of 2026-11-10 (plan-widget-theme): stock egui widgets
were on egui defaults while chaiss's own buttons were hand-painted on the
aicogito tokens, and the app followed the OS into light mode.

## Details

`ChaissTheme` (ui/theme.rs, eframe `experimental`) provides Button,
TextEdit, Checkbox and Separator styles from the tokens. Button classes
`chaiss::primary` / `chaiss::danger` / `chaiss::toolbar`, untagged =
standard; frames built with a constant 1 px border and no hover
expansion; DefaultStyle's no-frame / hide-when-inactive / selected
classes mirrored for ComboBox items. Dark mode pinned; pointing-hand
interact cursor. `prominent_button` and the `min_w` arguments are gone
(the 0.36 hover shift no longer exists in 0.37).

Verified: gates clean; 5 egui_kittest tests (labelled buttons, 28 px
toolbar height, no movement on hover/press, class → tokens, geometry
state-independent), mutation-checked; WSLg before/after screenshots
show one look, no bar clipping, aligned top row.

## Links

- Plan: docs/implementation/CHAI-0054-widget-theme.md
- Plan entry: 2026-11-10-00-48-58-plan-widget-theme.md
