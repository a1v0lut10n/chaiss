# Reflow LLM markdown tables to fix the chat panel blow-out

- **When:** 2026-09-03 15:41:43 local
- **Type:** execution
- **Project:** chaiss
- **Ticket:** CHAI-0038

## Context

A "compare 7...Nxd5 vs 7...Ne4" question made Gemini answer with a markdown
pipe table whose cells hold full sentences. egui_commonmark lays tables out
as an `egui::Grid` whose cells never wrap, so the message demanded ~1700+
points of content width inside the 450-point chat panel. That inflated
minimum width desynchronized the chat panel from the central panel — the
board area painted over the chat's left edge while the table ran off the
right — and no splitter position could repair it, because the width demand
came from the content itself.

## Details

Two-layer fix:

- `sanitize_markdown` gained a `reflow_tables` step that rewrites pipe
  tables into nested bullet lists — one top-level bullet per row (its first
  cell), with `header: cell` sub-bullets for the remaining columns. It only
  triggers on a real header + `|---|` separator pair, skips ``` code fences
  (ASCII boards keep their pipes), honors escaped `\|`, and converts rows as
  they arrive during streaming so a raw table never reaches the grid
  renderer. Applied to streamed responses and — new — to chat history loaded
  on game resume, so already-affected games render correctly without
  touching the database.
- Each chat message now renders inside a horizontal `ScrollArea`
  (`auto_shrink([false, true])`), so any remaining unwrappable content
  (e.g. long code-block lines) scrolls within the panel instead of inflating
  its width and corrupting the panel boundary.

Verified live via the egui MCP loop against the affected "Modern" game: the
comparison matrix renders as a readable bulleted list, the panel boundary is
correct, and the ASCII boards are untouched. Four new unit tests cover the
reflow (full table, partially streamed row, pipes in code/prose, escaped
pipes); fmt, clippy `-D warnings`, and all 12 chaiss tests pass.

## Links

- PR: https://github.com/a1v0lut10n/chaiss/pull/44
- Related entry: `2026-08-30-11-13-44-execution-release-0-3-0.md`
