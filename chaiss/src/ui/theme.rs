//! Visual tokens and shared widgets adopted from the aicogito
//! "Feed + Desk" design language (aicogito `tokens.rs` / `widgets.rs`).
//! Only the subset chaiss uses lives here; keep values in sync by hand.
//!
//! [`ChaissTheme`] applies the tokens to every stock egui widget (buttons,
//! ComboBoxes, text fields, checkboxes, separators) through egui's
//! experimental `StyleProvider`; buttons choose a variant by class.

use eframe::egui::{
    self,
    class::{ClassName, HasClasses as _},
    text::LayoutJob,
    theme::{DefaultStyle, StyleProvider},
    widget_style::{
        ButtonStyle, CheckboxStyle, SeparatorStyle, StyleArgs, TextEditStyle, WidgetState,
    },
    Button, Color32, ComboBox, CornerRadius, Frame, Margin, Response, Stroke, TextFormat, Ui,
};

// ─── Surfaces ────────────────────────────────────────────────

/// Sidebar / roster background.
pub const SIDEBAR: Color32 = Color32::from_rgb(0x19, 0x1d, 0x23);

// ─── Cards & chips ───────────────────────────────────────────

pub const CARD_BG: Color32 = Color32::from_rgb(0x1d, 0x22, 0x2a);
pub const CARD_BORDER: Color32 = Color32::from_rgb(0x2a, 0x31, 0x3a);
pub const CARD_BORDER_HOVER: Color32 = Color32::from_rgb(0x3e, 0x49, 0x57);
/// Selected-card surface (the blue chip family).
pub const CHIP_BG: Color32 = Color32::from_rgb(0x1d, 0x2b, 0x3d);
pub const CHIP_BORDER: Color32 = Color32::from_rgb(0x2d, 0x47, 0x63);
pub const CHIP_BORDER_HOVER: Color32 = Color32::from_rgb(0x40, 0x64, 0x8b);

// ─── Text ────────────────────────────────────────────────────

pub const TEXT_PRIMARY: Color32 = Color32::from_rgb(0xd7, 0xdc, 0xe3);
pub const TEXT_BRIGHT: Color32 = Color32::from_rgb(0xee, 0xf1, 0xf5);
pub const TEXT_SECONDARY: Color32 = Color32::from_rgb(0x99, 0xa2, 0xad);
pub const TEXT_FAINT: Color32 = Color32::from_rgb(0x6b, 0x74, 0x80);

// ─── Accents ─────────────────────────────────────────────────

/// Active session, links.
pub const ACCENT: Color32 = Color32::from_rgb(0x62, 0xa0, 0xe0);
/// Destructive affordances (delete hover).
pub const DANGER: Color32 = Color32::from_rgb(0xc8, 0x32, 0x32);
/// Faint red border for danger buttons (red analog of `BTN_BORDER`).
pub const DANGER_BORDER: Color32 = Color32::from_rgb(0x6e, 0x3d, 0x3d);
pub const DANGER_BORDER_HOVER: Color32 = Color32::from_rgb(0x9a, 0x52, 0x52);
/// Soft red text for danger buttons (red analog of `BTN_TEXT`).
pub const DANGER_TEXT: Color32 = Color32::from_rgb(0xe2, 0xa9, 0xa9);

// ─── Buttons ─────────────────────────────────────────────────

pub const BTN_PRIMARY_BG: Color32 = Color32::from_rgb(0x24, 0x40, 0x5e);
pub const BTN_PRIMARY_BORDER: Color32 = Color32::from_rgb(0x3d, 0x6e, 0xa8);
pub const BTN_PRIMARY_BORDER_HOVER: Color32 = Color32::from_rgb(0x55, 0x93, 0xd6);
pub const BTN_PRIMARY_TEXT: Color32 = Color32::from_rgb(0xcf, 0xe2, 0xf5);
pub const BTN_BG: Color32 = Color32::from_rgb(0x26, 0x2d, 0x36);
pub const BTN_BORDER: Color32 = Color32::from_rgb(0x39, 0x42, 0x4e);
pub const BTN_BORDER_HOVER: Color32 = Color32::from_rgb(0x4f, 0x5b, 0x6b);
pub const BTN_TEXT: Color32 = Color32::from_rgb(0xcc, 0xd3, 0xdb);

// ─── Type scale & shape ──────────────────────────────────────

pub const SIZE_SECONDARY: f32 = 12.5;
pub const SIZE_FINE: f32 = 11.5;
/// Section labels: uppercase mono, letter-spacing 0.09em.
pub const SIZE_SECTION_LABEL: f32 = 10.5;
pub const RADIUS_BUTTON: u8 = 4;
pub const RADIUS_CARD: u8 = 6;

/// An uppercase mono section label ("ACTIVE SESSIONS") with wide letter-spacing.
pub fn section_label(ui: &mut Ui, text: &str) {
    let mut job = LayoutJob::default();
    let mut format = TextFormat::simple(egui::FontId::monospace(SIZE_SECTION_LABEL), TEXT_FAINT);
    format.extra_letter_spacing = SIZE_SECTION_LABEL * 0.09;
    job.append(&text.to_uppercase(), 0.0, format);
    ui.label(job);
}

/// Top-bar button height: the `toolbar` class, and the analysis ComboBox
/// that sits beside those buttons (`board.rs`).
pub const TOOLBAR_BUTTON_H: f32 = 28.0;
/// Text size of `toolbar`-class buttons.
pub const SIZE_TOOLBAR: f32 = 15.0;

// ─── Widget theme ────────────────────────────────────────────
//
// Every stock button, text field, checkbox and separator is styled here,
// through egui's experimental `StyleProvider` (eframe feature
// `experimental`, which carries no semver promise: check this section
// first on every egui upgrade). Buttons pick a variant by class; an
// untagged button gets the standard look. Margins and stroke widths are
// the same in every state, so hovering never moves anything.

/// Primary action (blue): "Create New Game", "Confirm & Start".
pub const CLASS_PRIMARY: ClassName = ClassName::from_static("chaiss::primary");
/// Destructive action (red border and text): "Resign".
pub const CLASS_DANGER: ClassName = ClassName::from_static("chaiss::danger");
/// The prominent top-bar size (28 px, 15 pt); combines with the above.
pub const CLASS_TOOLBAR: ClassName = ClassName::from_static("chaiss::toolbar");

/// The chaiss widget theme. See [`ChaissTheme::install`].
#[derive(Debug, Clone, Copy)]
pub struct ChaissTheme;

impl ChaissTheme {
    /// Registers the theme for every themed widget style, pins dark mode
    /// (the tokens are dark-only, and egui would otherwise follow the OS)
    /// and gives interactive widgets the pointing-hand cursor.
    pub fn install(ctx: &egui::Context) {
        ctx.set_theme(egui::ThemePreference::Dark);
        ctx.all_styles_mut(|style| {
            style.visuals.interact_cursor = Some(egui::CursorIcon::PointingHand);
        });
        ctx.add_widget_theme::<ButtonStyle>(Self);
        ctx.add_widget_theme::<TextEditStyle>(Self);
        ctx.add_widget_theme::<CheckboxStyle>(Self);
        ctx.add_widget_theme::<SeparatorStyle>(Self);
    }
}

/// `(fill, border, text)` for a button with these classes in this state.
fn button_colors(args: &StyleArgs<'_>) -> (Color32, Color32, Color32) {
    let lit = matches!(args.state, WidgetState::Hovered | WidgetState::Active)
        || args.has_class(&ComboBox::CLASS_OPEN);
    let pick = |rest, hover| if lit { hover } else { rest };
    if args.has_class(&Button::CLASS_SELECTED) {
        (CHIP_BG, pick(CHIP_BORDER, CHIP_BORDER_HOVER), TEXT_BRIGHT)
    } else if args.has_class(&CLASS_PRIMARY) {
        (
            BTN_PRIMARY_BG,
            pick(BTN_PRIMARY_BORDER, BTN_PRIMARY_BORDER_HOVER),
            BTN_PRIMARY_TEXT,
        )
    } else if args.has_class(&CLASS_DANGER) {
        (
            BTN_BG,
            pick(DANGER_BORDER, DANGER_BORDER_HOVER),
            DANGER_TEXT,
        )
    } else {
        (BTN_BG, pick(BTN_BORDER, BTN_BORDER_HOVER), BTN_TEXT)
    }
}

impl StyleProvider<ButtonStyle> for ChaissTheme {
    fn style(&mut self, args: &StyleArgs<'_>) -> ButtonStyle {
        // Start from egui's own style for what the theme does not decide
        // (gap, image tint, the default minimum height).
        let mut style: ButtonStyle = DefaultStyle.style(args);
        let toolbar = args.has_class(&CLASS_TOOLBAR);
        let (fill, border, text) = button_colors(args);

        let inner_margin = if toolbar {
            Margin::symmetric(12, 4)
        } else {
            args.style.spacing.button_padding.into()
        };
        let painted = Frame::new()
            .fill(fill)
            .stroke(Stroke::new(1.0, border))
            .corner_radius(CornerRadius::same(RADIUS_BUTTON))
            .inner_margin(inner_margin);

        // Mirror `DefaultStyle`'s frame classes, so ComboBox items and
        // frameless buttons keep working. A frame hidden while inactive is
        // the painted frame made invisible: same margins, no shift on hover.
        let has_frame = args.has_class(&Button::CLASS_FRAME)
            || (!args.has_class(&Button::CLASS_NO_FRAME) && args.style.visuals.button_frame);
        let layout = &mut style.atom_layout;
        layout.frame = if !has_frame {
            Frame::new()
        } else if args.has_class(&Button::CLASS_HIDE_FRAME_WHEN_INACTIVE)
            && args.state == WidgetState::Inactive
        {
            painted.invisible()
        } else {
            painted
        };
        layout.text_style.color = text;
        if toolbar {
            layout.text_style.font_id = egui::FontId::proportional(SIZE_TOOLBAR);
            layout.min_size.y = layout.min_size.y.max(TOOLBAR_BUTTON_H);
        }
        style
    }
}

impl StyleProvider<TextEditStyle> for ChaissTheme {
    fn style(&mut self, args: &StyleArgs<'_>) -> TextEditStyle {
        let mut style: TextEditStyle = DefaultStyle.style(args);
        if !args.has_class(&egui::TextEdit::CLASS_READ_ONLY) {
            let border = match args.state {
                WidgetState::Active => CHIP_BORDER_HOVER,
                WidgetState::Hovered => CARD_BORDER_HOVER,
                _ => CARD_BORDER,
            };
            // A 1 px border in every state (egui's dark default has none at
            // rest), applied so the stroke never changes the layout.
            style.atom_layout.frame = Frame::new()
                .fill(CARD_BG)
                .corner_radius(CornerRadius::same(RADIUS_BUTTON))
                .inner_margin(args.style.spacing.button_padding)
                .apply_stroke_and_expansion_without_layout_shift(Stroke::new(1.0, border), 0.0);
        }
        style.atom_layout.text_style.color = TEXT_PRIMARY;
        style.hint_text_color = TEXT_FAINT;
        style
    }
}

impl StyleProvider<CheckboxStyle> for ChaissTheme {
    fn style(&mut self, args: &StyleArgs<'_>) -> CheckboxStyle {
        let mut style: CheckboxStyle = DefaultStyle.style(args);
        let lit = matches!(args.state, WidgetState::Hovered | WidgetState::Active);
        style.checkbox_frame.fill = BTN_BG;
        style.checkbox_frame.stroke =
            Stroke::new(1.0, if lit { BTN_BORDER_HOVER } else { BTN_BORDER });
        style.check_stroke = Stroke::new(2.0, ACCENT);
        style.atom_layout.text_style.color = if lit { TEXT_BRIGHT } else { TEXT_PRIMARY };
        style
    }
}

impl StyleProvider<SeparatorStyle> for ChaissTheme {
    fn style(&mut self, args: &StyleArgs<'_>) -> SeparatorStyle {
        let mut style: SeparatorStyle = DefaultStyle.style(args);
        style.stroke = Stroke::new(1.0, CARD_BORDER);
        style
    }
}

// ─── Buttons ─────────────────────────────────────────────────

/// Primary button ("Confirm & Start").
pub fn primary_button(ui: &mut Ui, text: &str) -> Response {
    ui.add(Button::new(text).with_class(CLASS_PRIMARY))
}

/// Standard button ("Cancel"); the same look any untagged button gets.
pub fn standard_button(ui: &mut Ui, text: &str) -> Response {
    ui.add(Button::new(text))
}

/// Danger button at toolbar prominence ("⚐ Resign").
pub fn danger_button(ui: &mut Ui, text: &str) -> Response {
    ui.add(
        Button::new(text)
            .with_class(CLASS_DANGER)
            .with_class(CLASS_TOOLBAR),
    )
}

/// Standard button at toolbar prominence ("⟲ Undo Move").
pub fn toolbar_button(ui: &mut Ui, text: &str) -> Response {
    ui.add(Button::new(text).with_class(CLASS_TOOLBAR))
}

/// Primary button at toolbar prominence ("Create New Game").
pub fn primary_toolbar_button(ui: &mut Ui, text: &str) -> Response {
    ui.add(
        Button::new(text)
            .with_class(CLASS_PRIMARY)
            .with_class(CLASS_TOOLBAR),
    )
}

/// Repaints a widget's border in a brighter color — the subtle hover hint
/// shared by toolbar buttons and session cards. Painted over the resting
/// border after the widget, so it never affects layout.
pub fn paint_hover_border(ui: &Ui, rect: egui::Rect, radius: u8, color: Color32) {
    ui.painter().rect_stroke(
        rect,
        CornerRadius::same(radius),
        Stroke::new(1.0, color),
        egui::StrokeKind::Inside,
    );
}

/// Draws `frame` as a full-width clickable row and registers a "corner action"
/// (e.g. a delete icon) whose rect the body returns. The corner's click is
/// registered LAST so egui's z-order gives it hit-priority over the row —
/// no pointer-position disambiguation needed (brainforge's
/// `frame_with_corner_click`). Returns `(row_response, corner_response)`.
pub fn frame_with_corner_click(
    ui: &mut Ui,
    frame: Frame,
    id: egui::Id,
    add_contents: impl FnOnce(&mut Ui) -> egui::Rect,
) -> (Response, Response) {
    let mut corner_rect = egui::Rect::NOTHING;
    let row_response = frame
        .show(ui, |ui| {
            corner_rect = add_contents(ui);
        })
        .response
        .interact(egui::Sense::click());
    let corner_r = ui.interact(corner_rect, id.with("corner"), egui::Sense::click());
    (row_response, corner_r)
}

/// The card frame for a session row: chip-blue when active, neutral otherwise.
pub fn session_card_frame(is_active: bool) -> Frame {
    let (fill, border) = if is_active {
        (CHIP_BG, CHIP_BORDER)
    } else {
        (CARD_BG, CARD_BORDER)
    };
    Frame::new()
        .fill(fill)
        .stroke(Stroke::new(1.0, border))
        .corner_radius(CornerRadius::same(RADIUS_CARD))
        .inner_margin(Margin::symmetric(10, 8))
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui::{class::Classes, epaint::text::MissingGlyphPolicy, Rect, Role, Vec2};
    use egui_kittest::{kittest::Queryable as _, Harness};

    type Helper = fn(&mut Ui, &str) -> Response;

    /// Every themed button helper, with the label chaiss gives it.
    const BUTTONS: [(&str, Helper); 5] = [
        ("Confirm & Start", primary_button),
        ("Cancel", standard_button),
        ("⚐ Resign", danger_button),
        ("⟲ Undo Move", toolbar_button),
        ("Create New Game", primary_toolbar_button),
    ];

    /// A harness showing all of [`BUTTONS`] in one row, as the toolbar does.
    fn button_row() -> Harness<'static> {
        let mut installed = false;
        Harness::builder()
            .with_size(Vec2::new(900.0, 100.0))
            .build_ui(move |ui| {
                if !installed {
                    // The harness has the bundled fonts only: draw ⚐ and ⟲ as
                    // boxes instead of panicking. These tests assert geometry.
                    ui.ctx().set_missing_glyph_policy(MissingGlyphPolicy::Tofu);
                    ChaissTheme::install(ui.ctx());
                    installed = true;
                }
                ui.horizontal(|ui| {
                    for (label, add) in BUTTONS {
                        add(ui, label);
                    }
                });
            })
    }

    fn rects(harness: &Harness<'_>) -> Vec<Rect> {
        BUTTONS
            .iter()
            .map(|(label, _)| harness.get_by_role_and_label(Role::Button, label).rect())
            .collect()
    }

    #[test]
    fn every_helper_is_a_labelled_button() {
        let mut harness = button_row();
        harness.run();
        for (label, _) in BUTTONS {
            harness.get_by_role_and_label(Role::Button, label);
        }
    }

    #[test]
    fn toolbar_buttons_are_toolbar_height() {
        let mut harness = button_row();
        harness.run();
        for label in ["⚐ Resign", "⟲ Undo Move", "Create New Game"] {
            let height = harness
                .get_by_role_and_label(Role::Button, label)
                .rect()
                .height();
            assert_eq!(height, TOOLBAR_BUTTON_H, "{label}");
        }
    }

    /// Hovering or pressing any button must not move or resize it, nor
    /// anything else in its row: the reason `prominent_button` once existed.
    #[test]
    fn hovering_or_pressing_never_moves_the_row() {
        for (label, _) in BUTTONS {
            let mut harness = button_row();
            harness.run();
            let resting = rects(&harness);
            let center = harness
                .get_by_role_and_label(Role::Button, label)
                .rect()
                .center();

            harness.hover_at(center);
            harness.run();
            assert_eq!(rects(&harness), resting, "hovering {label}");

            harness.drag_at(center);
            harness.run();
            assert_eq!(rects(&harness), resting, "pressing {label}");
        }
    }

    /// The provider's button style for `classes` in each state.
    fn button_styles(classes: &[ClassName]) -> Vec<(WidgetState, ButtonStyle)> {
        let classes = classes.iter().fold(Classes::default(), |acc, class| {
            acc.with_class(class.clone())
        });
        let states = [
            WidgetState::Inactive,
            WidgetState::Hovered,
            WidgetState::Active,
        ];
        let mut out = Vec::new();
        let mut harness = Harness::new_ui(|ui| {
            out = states
                .iter()
                .map(|&state| {
                    let args = StyleArgs {
                        classes: &classes,
                        state,
                        stack: ui.stack(),
                        style: ui.style(),
                        ctx: ui.ctx(),
                    };
                    (state, ChaissTheme.style(&args))
                })
                .collect();
        });
        harness.run();
        drop(harness);
        out
    }

    /// `(fill, border at rest, border lit, text)` for each variant.
    #[test]
    fn classes_map_to_tokens() {
        let cases: [(&[ClassName], Color32, Color32, Color32, Color32); 4] = [
            (&[], BTN_BG, BTN_BORDER, BTN_BORDER_HOVER, BTN_TEXT),
            (
                &[CLASS_PRIMARY],
                BTN_PRIMARY_BG,
                BTN_PRIMARY_BORDER,
                BTN_PRIMARY_BORDER_HOVER,
                BTN_PRIMARY_TEXT,
            ),
            (
                &[CLASS_DANGER, CLASS_TOOLBAR],
                BTN_BG,
                DANGER_BORDER,
                DANGER_BORDER_HOVER,
                DANGER_TEXT,
            ),
            (
                &[Button::CLASS_SELECTED],
                CHIP_BG,
                CHIP_BORDER,
                CHIP_BORDER_HOVER,
                TEXT_BRIGHT,
            ),
        ];
        for (classes, fill, rest, lit, text) in cases {
            for (state, style) in button_styles(classes) {
                let frame = style.atom_layout.frame;
                let border = if state == WidgetState::Inactive {
                    rest
                } else {
                    lit
                };
                assert_eq!(frame.fill, fill, "{classes:?} {state:?}");
                assert_eq!(
                    frame.stroke,
                    Stroke::new(1.0, border),
                    "{classes:?} {state:?}"
                );
                assert_eq!(
                    style.atom_layout.text_style.color, text,
                    "{classes:?} {state:?}"
                );
            }
        }
    }

    /// Margins and stroke widths are state-independent for every variant,
    /// including the frame a ComboBox item hides while inactive.
    #[test]
    fn frame_geometry_is_state_independent() {
        let variants: [&[ClassName]; 5] = [
            &[],
            &[CLASS_PRIMARY, CLASS_TOOLBAR],
            &[CLASS_DANGER, CLASS_TOOLBAR],
            &[Button::CLASS_SELECTED],
            &[Button::CLASS_HIDE_FRAME_WHEN_INACTIVE],
        ];
        for classes in variants {
            let styles = button_styles(classes);
            let (_, first) = &styles[0];
            for (state, style) in &styles {
                let (a, b) = (&first.atom_layout, &style.atom_layout);
                assert_eq!(
                    a.frame.inner_margin, b.frame.inner_margin,
                    "{classes:?} {state:?}"
                );
                assert_eq!(
                    a.frame.outer_margin, b.frame.outer_margin,
                    "{classes:?} {state:?}"
                );
                assert_eq!(
                    a.frame.stroke.width, b.frame.stroke.width,
                    "{classes:?} {state:?}"
                );
                assert_eq!(a.min_size, b.min_size, "{classes:?} {state:?}");
            }
        }
        // ...and the hidden frame really is hidden at rest.
        let (_, inactive) = &button_styles(&[Button::CLASS_HIDE_FRAME_WHEN_INACTIVE])[0];
        assert_eq!(inactive.atom_layout.frame.fill, Color32::TRANSPARENT);
        assert_eq!(
            inactive.atom_layout.frame.stroke.color,
            Color32::TRANSPARENT
        );
    }
}
