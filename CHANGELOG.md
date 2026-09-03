# Changelog

All notable changes to this project are documented in this file. Entries are
derived from the development journal in `docs/journal/` and the release
history. Versions cover both workspace crates (`chaiss` and `chaiss-core`),
which are released in lockstep.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.1] - 2026-09-03

### Fixed

- LLM responses containing markdown pipe tables no longer corrupt the chat
  panel layout. egui_commonmark renders tables as a grid whose cells never
  wrap, so a table with sentence-long cells forced the panel's content far
  wider than the panel and clipped the text on both sides beyond what the
  splitter could repair. Tables are now reflowed into nested bullet lists
  (one bullet per row, header-labelled sub-bullets per column) — applied to
  streamed responses and to stored chat history on game resume — and each
  message renders inside a horizontal scroll area so any remaining
  unwrappable content scrolls within the panel.

## [0.3.0] - 2026-08-30

### Added

- `LLM_MODEL` environment variable overrides the per-backend default model
  without a rebuild (documented in `.env.example`).
- `GEMINI_API_KEY` is accepted as an alias for `GOOGLE_API_KEY`, matching the
  name Google's own documentation uses.

### Changed

- Default Google model updated from `gemini-3.5-flash` to `gemini-3.7-flash`
  (GA, August 2026).
- LLM backend failures now surface as plain-language messages in the chat
  panel — high demand / unavailable (503), rate limit or quota (429), rejected
  API key (401/403), unknown model (404), provider internal errors (5xx), and
  network failures each get a distinct explanation naming the model and
  provider. The raw backend error is logged to the console for
  troubleshooting instead of being shown verbatim.
- `chaiss-core`: `stream_llm_response` now returns a typed `LlmError`
  (`user_message` + `detail`) instead of a boxed `dyn Error`; the new
  `classify_backend_error` helper is public.

## [0.2.0] - 2026-08-06

### Added

- Board orientation is persisted per game: the "Flip Board (Play as Black)"
  setting is stored in the database and restored when a session is resumed
  (databases from earlier versions are migrated automatically).
- Formal move undo: an "⟲ Undo Move" button reverts the last committed move
  (or a mistaken resignation) in the database and resynchronizes the app;
  useful when shadowing a game played on a real board or chess.com.
- Horizontal move-history strip in the sandbox navigation: the currently
  displayed move is shown bright and slightly magnified at the center, with
  past and future moves fanning outward progressively smaller and dimmer;
  the visible count adapts to the window width.
- Local `NEXT-TICKET` branch numbering convention (`docs/NEXT-TICKET`),
  replacing Jira-issued ticket numbers.

### Changed

- Upgraded the UI stack to egui/eframe 0.36, with `egui_extras` 0.36 and
  `egui_commonmark` 0.25 aligned.
- Session roster restyled as two-line cards with right-aligned delete icons
  and an accent-highlighted active session, adopting a shared design-token
  theme (`ui/theme.rs`); key buttons (Create New Game, Resign, Undo Move)
  use themed 28px variants with stable layout and subtle hover hints, and
  the Analysis Overlay dropdown matches their height.
- Predictive-matrix arrows are colored by side — transparent blue for
  White's predicted moves, red for Black's, matching the analysis overlay's
  color language — with transparency increasing for deeper plies. They are
  drawn only at the live position, not in exploration views.
- Sandbox labels simplified: the checkbox and status banner now read
  "Exploration mode".

### Fixed

- Buttons no longer shrink or shift their label on hover (replaced stateful
  egui buttons with directly painted equivalents).
- The session cards' delete-icon hover tint (and pointer cursor) now
  actually shows; egui's hover arbitration inside the scroll area never
  flagged those responses.
- Predictive arrows were rendered over-bright due to a premultiplied-alpha
  misuse; they now blend correctly.

## [0.1.7] - 2026-07-17

### Fixed

- LLM chat responses containing LaTeX math markup (e.g.
  `$$\text{7. d4 \quad exd4}$$`, as emitted by newer Gemini models) rendered
  verbatim in the chat panel — dollar signs, backslashes and braces included.
  The markdown sanitizer now rewrites complete math spans as bold plain text:
  `\text{}`-style groups are unwrapped, spacing macros become spaces, and
  common TeX symbols map to their Unicode equivalents. Partially streamed
  spans, fenced code blocks, escaped dollars, and ordinary dollar amounts are
  deliberately left untouched.

### Changed

- The LLM system prompt now instructs the model to respond in plain Markdown
  only and to avoid LaTeX/math notation, writing chess moves as plain
  algebraic notation.

## [0.1.6] - 2026-07-13

### Changed

- Upgraded the UI stack to egui/eframe 0.35, with `egui_extras` and
  `egui_commonmark` aligned to matching versions.

### Added

- Opt-in `inspection` cargo feature: launching with `EGUI_INSPECTION=1`
  exposes the live UI tree to the egui MCP server, enabling agent-driven UI
  inspection, automation, and screenshots during development.

## [0.1.5] - 2026-06-20

### Added

- Auto-retry for incomplete LLM responses: an answer that arrives without its
  concluding predictive-matrix line triggers a bounded automatic re-request.

### Fixed

- More robust markdown sanitization of streamed LLM output, including
  auto-closing unbalanced code fences so partial streams render cleanly.

## [0.1.4] - 2026-06-17

### Fixed

- Right-panel chat text no longer clips/overflows its container.

### Changed

- Default Google model updated from `gemini-3.1-pro-preview` to
  `gemini-3.5-flash`.
- SVG logo adapts its colors for dark mode; the cyan AI outline was thickened
  to fully cover the inner text border.
- Established `cargo fmt` and `cargo clippy` as enforced workspace
  requirements.

## [0.1.3] - 2026-05-20

### Fixed

- Emoji rendering: NotoEmoji is embedded as an explicit font fallback so
  emoji (e.g. the robot token in chat) display correctly.

## [0.1.2] - 2026-04-18

### Changed

- Crate metadata and packaging optimization for crates.io.

## [0.1.1] - 2026-04-18

### Changed

- Packaging alignment for crates.io publication.

## [0.1.0] - 2026-04-18

Initial public release, split into the `chaiss` desktop application and the
headless `chaiss-core` engine crate.

### Added

- Pure chess engine: raycasting-based move generation, full legality
  validation (checks, pins), castling, promotion, en passant, and terminal
  evaluation (checkmate and stalemate), fully decoupled from the UI.
- Three-pane egui desktop interface with SVG vector piece rendering, board
  flipping (Black perspective), and dual-tone gradient highlighting of board
  pressure.
- SQLite persistence via sqlx with cross-platform, pure-Rust database
  initialization and migrations; database work runs off the render loop over
  a flume channel bridge.
- Unified LLM integration (Google, OpenAI, Anthropic) with streaming chat
  responses and persistent per-game chat context.
- Algebraic notation parser for move entry via chat, PGN sequence loading for
  importing full games, and rapid board initialization from custom positions.
- Predictive-matrix heat map and visual AI continuation arrows rendered on
  the board from LLM analysis.
- Game session roster with creation, deletion, resumption, manual
  resignation, and a non-destructive sandbox exploration mode for reviewing
  history.
