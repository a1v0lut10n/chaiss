# CHAI-0057 — chat emoji still render monochrome on a Linux desktop

- **Ticket:** CHAI-0057
- **Branch:** `bugfix/CHAI-0057-tinted-emoji`
- **Status:** open (ticket claimed 2026-11-11, work not started)

## Premise

Field observation on the egui 0.37 build (2026-11-11, Linux, KDE on
Wayland): the chat transcript's 👤 and 🤖 take the colour of their
label (light blue, light green). They are drawn as outline glyphs and
tinted as text, so they are monochrome on this machine. CHAI-0052's
check under WSLg showed them in colour from Noto Color Emoji, and the
0.7 changelog promises colour emoji on Linux once
`fonts-noto-color-emoji` is installed. Here it is installed, is the only
emoji font fontconfig knows, and is the first match for the generic
`emoji` family.

What the build does: chaiss configures no fonts of its own since
CHAI-0052. eframe's default `system_fonts` feature installs
`egui_system_font_provider` 0.37, which turns on epaint's `color_fonts`
and asks `fontique` (through fontconfig) for a font covering any
character the configured fonts lack. egui's `monochrome_emoji_fonts`
feature is off. epaint 0.37 does not tint colour glyphs. So for these
two characters the provider delivered a face that epaint rasterised as
a plain outline.

Two leads, found with fontconfig on this machine:

1. **🤖 (U+1F916) has a second home.** The Source Code Pro family
   installed under `/usr/share/fonts/googlefonts/` carries an outline
   glyph for it, and `fc-match -s "sans-serif:charset=1f916"` ranks
   Source Code Pro ahead of Noto Color Emoji. If fontique follows the
   same ranking, the provider picks a monospace text font for the
   robot, and the glyph is correctly monochrome. The WSLg image had no
   such font, which is why it showed colour.
2. **👤 (U+1F464) has only one home**, Noto Color Emoji, so the fallback
   cannot have gone elsewhere. The installed file is a `CBDT`/`CBLC`
   bitmap font with no `COLR` table. Either the provider accepts the
   face but epaint draws its bitmap strikes as a coverage mask, which
   then gets tinted, or the observation of 👤 was wrong and only 🤖
   was affected. The first thing to do is look again, one glyph at a
   time.

## Action items

1. Reproduce with a throwaway eframe 0.37 probe (as in CHAI-0052),
   rendering 👤 🤖 ⚙ ⚠️ 🗑 at 14 and 24 pt, each in its own label with a
   strong tint, and record for each whether it is tinted. Log which
   face the provider chose (`RUST_LOG` for `egui_system_font_provider`
   and `fontique`, or a patched provider if it does not log).
2. For 🤖: confirm the Source Code Pro lead by hiding that family from
   fontconfig for one run (`FONTCONFIG_FILE` with a `<rejectfont>`
   rule). If the robot turns colour, the question becomes whether the
   provider should prefer an emoji-presentation font for characters
   whose default presentation is emoji (`unicode-properties` already
   exposes `Emoji_Presentation`). That is an upstream conversation with
   `egui_system_font_provider`, and chaiss may want a local preference
   in the meantime.
3. For 👤: check epaint 0.37's handling of `CBDT` strikes against a
   `COLR` emoji font (e.g. Twemoji COLR, or Noto Color Emoji's COLRv1
   build from Google Fonts) on the same machine. If only the bitmap
   build is tinted, report it to egui with the probe.
4. Decide the fix for chaiss: install an emoji preference through the
   provider's API if it has one; otherwise load the system's emoji font
   by path at startup when fontconfig finds one (`emoji` generic
   family) and put it ahead of the system fallback; or document the
   limitation in the user guide next to the `fonts-noto-color-emoji`
   note.
5. Gates: `cargo fmt --all -- --check`, `cargo clippy --workspace
   --all-targets -- -D warnings`, `cargo test --workspace`.

## Verification

- [ ] Probe shows which face is chosen for each of the five glyphs on
      this machine, and the result is recorded here.
- [ ] 👤 and 🤖 draw in colour in the chat transcript on this Linux
      desktop, with Source Code Pro still installed.
- [ ] WSLg still draws them in colour (no regression from a local
      preference).
- [ ] A Linux machine without any emoji font still shows the documented
      tofu, not a crash.
