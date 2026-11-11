# CHAI-0056 — the Send button's right border was clipped

- **Ticket:** CHAI-0056
- **Branch:** `bugfix/CHAI-0056-send-button-clip` (stacked on
  `bugfix/CHAI-0055-renderer-failure-message`)
- **Status:** implemented (PR pending review)

## Premise

Field observation on the egui 0.37 build (2026-11-11, Wayland): the
chat pane's Send button showed its label in full but lost its right
border. The prompt row gave the text field
`available width − 50 px − spacing − 2 px` and then added the button
at 50 px; `add_sized` widens a button to its natural size, and "Send" at
the chat pane's 16 px body text plus the 0.37 button padding is 53.5
px. The button therefore ended 1.5 px past the panel's content edge and
the frame clipped its stroke. The egui-chrome skill lists this family
(stroked chrome against a bar edge); the cause here is the width
arithmetic, not a missing margin.

## Action items

1. `chat_input_row(ui, prompt)` in `right_panel.rs`: a fixed-height
   row (`CHAT_INPUT_HEIGHT`) laid out right to left under an explicit
   `push_id`. The button goes first at its natural width, the text field
   takes `available_width()`. The field carries `id_salt("chat_prompt")`
   so its id does not depend on the button's width.
2. A headless `egui_kittest` test renders the row at three widths with
   the chaiss theme installed and asserts that the button ends inside the
   content rect and that the field ends before the button.

## Verification

- [x] fmt, clippy `-D warnings`, workspace tests (one new).
- [x] Live on this machine through the inspection session: the button
      now spans x 1438.5–1492 in a 1500 pt window with the full border
      drawn (before: 1440–1493.5, right stroke cut).
