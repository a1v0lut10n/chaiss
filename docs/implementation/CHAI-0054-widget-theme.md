# CHAI-0054 — one widget theme through egui's StyleProvider

- **Ticket:** CHAI-0054
- **Branch:** `feature/CHAI-0054-widget-theme` (stacked on
  `bugfix/CHAI-0053-wsl-prefer-x11`)
- **Status:** planned; spike (step 1) done 2026-11-10
- **Origin:** egui 0.37's experimental theme plugin (emilk/egui#8153),
  raised with the CHAI-0052 upgrade.

## Premise

chaiss's widgets currently follow two styling regimes:

- **Hand-painted, on the aicogito "Feed + Desk" tokens** (`ui/theme.rs`):
  `prominent_button` paints the 28 px toolbar buttons itself (Create New
  Game, Resign, Undo Move), because the stock `Button` pulled per-state
  `ButtonStyle` padding in 0.36 and shifted its text on hover. That painter
  also registers its own AccessKit info and pointing-hand cursor.
  `primary_button` / `standard_button` (Confirm & Start, Cancel) set
  `.fill()` / `.stroke()` per call, which, per egui's docs, overrides all
  hover feedback.
- **Stock egui defaults:** the move-navigation buttons (`<< Start`,
  `< Prev Move`, `Next Move >`, `Live >>`), Send, both ComboBoxes and
  their items, three checkboxes, the prompt `TextEdit`, the chat
  separator. These use egui's default visuals, which in 0.37 are also
  roomier with larger corner radii (#8617, #8445), so the two families
  drift further apart.

chaiss also never sets a theme preference, so egui follows the OS. On a
light-mode desktop the stock widgets turn light while the token-painted
surfaces stay dark.

egui 0.37 (eframe feature `experimental`) adds `StyleProvider<S>`: one
object per widget style (`ButtonStyle`, `CheckboxStyle`,
`TextEditStyle`, `SeparatorStyle`) computes the style from the widget's
**classes** and **state** (inactive / hovered / active / noninteractive),
registered once with `Context::add_widget_theme`. Classes attach with
`Button::with_class` (public without the flag). Built-in classes already
in play: `Button::CLASS_SELECTED` (ComboBox items), `ComboBox::CLASS_OPEN`,
`CLASS_SMALL`, `CLASS_NO_FRAME`. Relevant mechanics, read from the
0.37.0 source:

- `DefaultStyle` builds the button frame with
  `apply_stroke_and_expansion_without_layout_shift`, and a provider controls
  `inner_margin` per state. A provider that returns one margin for all
  states cannot shift layout by construction.
- `Button::min_size` combines with the style's via `at_least`, so a
  per-call minimum width still works on top of a themed 28 px height.
- A `.fill()` / `.stroke()` set on the button wins over the provider. Themed
  buttons must not set them.
- `Visuals::interact_cursor` gives every stock interactive widget the
  pointing hand, replacing the painter's per-widget cursor.
- A widget's state comes from its previous-pass response, so hover
  styling lags one frame. The same holds for stock egui; there is no
  layout effect when margins are state-independent.

## Spike result (2026-11-10, step 1)

A throwaway `egui_kittest` harness (egui 0.37, `experimental`) drove a
`Button` through inactive → hovered → pressed (states confirmed from the
responses) and recorded its rect and its right-hand sibling's x:

| variant | natural width | `min_size.x = 110` |
|---|---|---|
| stock `DefaultStyle` | 82.53 × 20.00, stable | 110 × 20, stable |
| themed, no class | 92.53 × 25.00, stable | 110 × 25, stable |
| themed, `chaiss::toolbar` (15 pt, 28 px) | 102.75 × 28.00, stable | 110 × 28, stable |

**The 0.36 hover shift is gone in stock egui 0.37**, so the painter
no longer fixes a bug. The theme reaches the painter's geometry and
border-only hover with equal stability. The case for the work is what
the Risks section anticipated: one look across all widgets, dark
mode pinned, and the painter deleted. Pixel-level look is left to the
step 6 screenshots.

Also learned: **kittest panics on a glyph no installed font has**
(`No glyph for "⟲" (U+27F2) in Proportional`), since its harness has
the bundled fonts only and no system font provider. Tests that render
"⟲ Undo Move" or "⚐ Resign" must either add
`egui_system_font_provider::SystemFontProvider` to the harness context
or set `MissingGlyphPolicy::Tofu`. The provider keeps the test honest;
the tofu policy keeps it independent of the machine's fonts. Pick the
policy, since the tests assert geometry and roles, not glyphs.

## Goal

One theme, on the existing tokens, covering every stock widget chaiss
uses; delete the hand-painted button; pin dark mode.

## Action items

1. **Spike (throwaway, not merged).** An `egui_kittest` harness on egui
   0.37 with the `experimental` feature: register a provider with
   state-independent margins, render a 28 px classed `Button`, and assert
   its rect is identical inactive vs. hovered vs. pressed. Also confirm the
   unthemed 0.37 `Button` still shifts (or doesn't). If the stock button no
   longer shifts, the case rests on consistency and code removal; record
   that. Abort criterion: if a themed button cannot reach the painter's
   look (15 pt text, 28 px, border-only hover), stop and report.
2. **Feature flag.** `eframe = { version = "0.37", features =
   ["experimental"] }`, with a comment that the theme API may break on any
   egui minor, and a line in this plan's Consequences for the upgrade
   checklist.
3. **`ui/theme.rs`: `ChaissTheme`.** Keeps the tokens; adds a provider
   implementing:
   - `StyleProvider<ButtonStyle>`: start from `DefaultStyle` (so
     built-in classes keep working) and override fill, stroke, text colour,
     corner radius and a **state-independent** `inner_margin` from the
     tokens.
     - no class: the standard look (`BTN_BG` / `BTN_BORDER` →
       `BTN_BORDER_HOVER`, `BTN_TEXT`), which the navigation buttons and
       Send pick up untouched;
     - `chaiss::primary`: `BTN_PRIMARY_*`;
     - `chaiss::danger`: `BTN_BG`, `DANGER_BORDER*`, `DANGER_TEXT`;
     - `chaiss::toolbar` (combinable with the two above): 15 pt text,
       `min_size.y = TOOLBAR_BUTTON_H`;
     - `CLASS_SELECTED`: the chip family (`CHIP_BG` / `CHIP_BORDER`), as
       session cards already use for "active";
     - `ComboBox::CLASS_OPEN`: the hovered border, held.
   - `StyleProvider<TextEditStyle>`: `CARD_BG` fill, `CARD_BORDER` →
     `CHIP_BORDER_HOVER` when focused, `TEXT_FAINT` hint text.
   - `StyleProvider<CheckboxStyle>`: box in `BTN_BG` / `BTN_BORDER`, check
     stroke `ACCENT`.
   - `StyleProvider<SeparatorStyle>`: `CARD_BORDER`.
   - `ChaissTheme::install(ctx)`: registers all four, sets
     `ThemePreference::Dark`, and sets `interact_cursor =
     Some(PointingHand)` on both styles.
4. **Call sites.** `primary_button`, `standard_button`, `danger_button`,
   `toolbar_button`, `primary_toolbar_button` become thin builders
   returning a classed `Button` (keeping names and `min_w`, so call sites
   barely change). Delete `prominent_button`, its manual `widget_info` and
   cursor code; stock `Button` registers `Role::Button` with its label
   itself. `main.rs` calls `ChaissTheme::install` in the creator callback.
5. **Tests** (`egui_kittest` 0.37 as a dev-dependency, which is the egui
   project's own headless harness and needs no GPU):
   - layout stability: each toolbar button's rect is identical
     inactive / hovered / pressed (the regression the painter existed for);
   - class mapping: the provider's fill and stroke per class and state;
   - accessibility: toolbar buttons are found by label with `Role::Button`.
   egui 0.37's kittest fails on input widgets without an accessible name;
   if a harness reaches the prompt `TextEdit`, give it one
   (`Ui::label_inputs_by` or a hint), which is a fix in itself.
6. **Visual check under WSLg**, before and after, by screenshot:
   the bottom toolbar (egui-chrome pitfall 1: stroked buttons in a
   fixed-height bar must not clip; pitfall 5: buttons, heading and
   ComboBox share one centreline), the ComboBox popup, the new-game
   dialog, the chat input row.
7. Gates; plan updated to what was built; CHANGELOG; journal entry.

## Out of scope

- **Board painting** (squares, heat maps, arrows, coordinate labels):
  painter calls, not widgets; the theme does not reach them.
- **Chat role colours** (`LIGHT_BLUE` / `LIGHT_GREEN` / `GOLD` / orange
  on labels): labels have no `StyleProvider`. Moving them into named
  tokens is a small, separate cleanup.
- **Session cards** (`frame_with_corner_click`): a `Frame`, not a
  themed widget; already on the tokens.
- **A light theme.** chaiss's tokens are dark-only; this plan pins dark
  rather than designing a second palette.

## Risks

- **API churn.** `experimental` is explicitly allowed to break on any
  egui minor. Mitigation: everything theme-related lives in
  `ui/theme.rs`; the next egui upgrade checks it first.
- **The stock-button shift may be gone already** (step 1 decides). Then
  the gain is consistency, dark-mode pinning and ~70 fewer painter lines,
  not a bug fix.
- **One-frame hover lag**: cosmetic, shared with stock egui.

## Consequences (expected)

- Every button, ComboBox, checkbox and text field in chaiss draws from the
  same tokens, in dark mode regardless of the OS setting.
- Adding a styled button is a class, not a painter.
- egui upgrade checklist: re-run the theme tests first; `experimental`
  carries no semver promise.
