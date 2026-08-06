//! Visual tokens and shared widgets adopted from the brainforge-app
//! "Feed + Desk" design language (brainforge-ui `tokens.rs` / `widgets.rs`).
//! Only the subset chaiss uses lives here; keep values in sync by hand.

use eframe::egui::{
    self, text::LayoutJob, Button, Color32, CornerRadius, Frame, Margin, Response, RichText,
    Stroke, TextFormat, Ui,
};

// ─── Surfaces ────────────────────────────────────────────────

/// Sidebar / roster background.
pub const SIDEBAR: Color32 = Color32::from_rgb(0x19, 0x1d, 0x23);

// ─── Cards & chips ───────────────────────────────────────────

pub const CARD_BG: Color32 = Color32::from_rgb(0x1d, 0x22, 0x2a);
pub const CARD_BORDER: Color32 = Color32::from_rgb(0x2a, 0x31, 0x3a);
/// Selected-card surface (the blue chip family).
pub const CHIP_BG: Color32 = Color32::from_rgb(0x1d, 0x2b, 0x3d);
pub const CHIP_BORDER: Color32 = Color32::from_rgb(0x2d, 0x47, 0x63);

// ─── Text ────────────────────────────────────────────────────

pub const TEXT_PRIMARY: Color32 = Color32::from_rgb(0xd7, 0xdc, 0xe3);
pub const TEXT_SECONDARY: Color32 = Color32::from_rgb(0x99, 0xa2, 0xad);
pub const TEXT_FAINT: Color32 = Color32::from_rgb(0x6b, 0x74, 0x80);

// ─── Accents ─────────────────────────────────────────────────

/// Active session, links.
pub const ACCENT: Color32 = Color32::from_rgb(0x62, 0xa0, 0xe0);
/// Destructive affordances (delete hover).
pub const DANGER: Color32 = Color32::from_rgb(0xc8, 0x32, 0x32);
/// Faint red border for danger buttons (red analog of `BTN_BORDER`).
pub const DANGER_BORDER: Color32 = Color32::from_rgb(0x6e, 0x3d, 0x3d);
/// Soft red text for danger buttons (red analog of `BTN_TEXT`).
pub const DANGER_TEXT: Color32 = Color32::from_rgb(0xe2, 0xa9, 0xa9);

// ─── Buttons ─────────────────────────────────────────────────

pub const BTN_PRIMARY_BG: Color32 = Color32::from_rgb(0x24, 0x40, 0x5e);
pub const BTN_PRIMARY_BORDER: Color32 = Color32::from_rgb(0x3d, 0x6e, 0xa8);
pub const BTN_PRIMARY_TEXT: Color32 = Color32::from_rgb(0xcf, 0xe2, 0xf5);
pub const BTN_BG: Color32 = Color32::from_rgb(0x26, 0x2d, 0x36);
pub const BTN_BORDER: Color32 = Color32::from_rgb(0x39, 0x42, 0x4e);
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

/// Primary button: `#24405e` fill, `#3d6ea8` border, `#cfe2f5` text.
pub fn primary_button(ui: &mut Ui, text: &str) -> Response {
    ui.add(
        Button::new(RichText::new(text).color(BTN_PRIMARY_TEXT))
            .fill(BTN_PRIMARY_BG)
            .stroke(Stroke::new(1.0, BTN_PRIMARY_BORDER))
            .corner_radius(CornerRadius::same(RADIUS_BUTTON)),
    )
}

/// Standard button: `#262d36` fill, `#39424e` border, `#ccd3db` text.
pub fn standard_button(ui: &mut Ui, text: &str) -> Response {
    ui.add(
        Button::new(RichText::new(text).color(BTN_TEXT))
            .fill(BTN_BG)
            .stroke(Stroke::new(1.0, BTN_BORDER))
            .corner_radius(CornerRadius::same(RADIUS_BUTTON)),
    )
}

/// Danger button ("Resign"): the standard button body with a faint red
/// border and soft red text, slightly larger for prominence.
pub fn danger_button(ui: &mut Ui, text: &str) -> Response {
    ui.add(
        Button::new(RichText::new(text).size(15.0).color(DANGER_TEXT))
            .fill(BTN_BG)
            .stroke(Stroke::new(1.0, DANGER_BORDER))
            .corner_radius(CornerRadius::same(RADIUS_BUTTON))
            .min_size(egui::vec2(0.0, 28.0)),
    )
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
