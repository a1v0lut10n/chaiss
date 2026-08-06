//! Visual tokens and shared widgets adopted from the aicogito
//! "Feed + Desk" design language (aicogito `tokens.rs` / `widgets.rs`).
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
pub const CARD_BORDER_HOVER: Color32 = Color32::from_rgb(0x3e, 0x49, 0x57);
/// Selected-card surface (the blue chip family).
pub const CHIP_BG: Color32 = Color32::from_rgb(0x1d, 0x2b, 0x3d);
pub const CHIP_BORDER: Color32 = Color32::from_rgb(0x2d, 0x47, 0x63);
pub const CHIP_BORDER_HOVER: Color32 = Color32::from_rgb(0x40, 0x64, 0x8b);

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

/// Top-bar button height, shared by [`danger_button`] and [`toolbar_button`].
pub const TOOLBAR_BUTTON_H: f32 = 28.0;

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

/// Shared body for the prominent (28px) buttons, painted directly instead of
/// via `egui::Button`: the stock button pulls per-state `ButtonStyle` padding,
/// so its layout (and the text inside it) shifts by a couple of pixels on
/// hover. Here the allocation is fixed at `max(min_w, text) x 28`, the galley
/// is centered in it every frame, and hover changes only the border color.
fn prominent_button(
    ui: &mut Ui,
    text: &str,
    min_w: f32,
    fill: Color32,
    text_color: Color32,
    border: Color32,
    border_hover: Color32,
) -> Response {
    let galley = ui.painter().layout_no_wrap(
        text.to_owned(),
        egui::FontId::proportional(15.0),
        text_color,
    );
    let width = min_w.max(galley.size().x + 24.0);
    let (rect, response) =
        ui.allocate_exact_size(egui::vec2(width, TOOLBAR_BUTTON_H), egui::Sense::click());
    if ui.is_rect_visible(rect) {
        let border_color = if response.hovered() {
            border_hover
        } else {
            border
        };
        ui.painter().rect(
            rect,
            CornerRadius::same(RADIUS_BUTTON),
            fill,
            Stroke::new(1.0, border_color),
            egui::StrokeKind::Inside,
        );
        ui.painter()
            .galley(rect.center() - galley.size() / 2.0, galley, text_color);
    }
    if response.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, ui.is_enabled(), text));
    response
}

/// Danger button ("Resign"): the standard button body with a faint red
/// border and soft red text, slightly larger for prominence. `min_w` pins
/// the allocated width (see [`toolbar_button`]).
pub fn danger_button(ui: &mut Ui, text: &str, min_w: f32) -> Response {
    prominent_button(
        ui,
        text,
        min_w,
        BTN_BG,
        DANGER_TEXT,
        DANGER_BORDER,
        DANGER_BORDER_HOVER,
    )
}

/// Standard-colored button at [`danger_button`]'s prominence. `min_w` pins
/// the allocated width (pass a value comfortably above the natural text
/// width): egui re-lays hovered buttons out slightly narrower, which
/// otherwise shifts everything to the right of the button by a couple of
/// pixels on every hover.
pub fn toolbar_button(ui: &mut Ui, text: &str, min_w: f32) -> Response {
    prominent_button(
        ui,
        text,
        min_w,
        BTN_BG,
        BTN_TEXT,
        BTN_BORDER,
        BTN_BORDER_HOVER,
    )
}

/// Primary-colored button at the same prominence ("Create New Game").
pub fn primary_toolbar_button(ui: &mut Ui, text: &str, min_w: f32) -> Response {
    prominent_button(
        ui,
        text,
        min_w,
        BTN_PRIMARY_BG,
        BTN_PRIMARY_TEXT,
        BTN_PRIMARY_BORDER,
        BTN_PRIMARY_BORDER_HOVER,
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
