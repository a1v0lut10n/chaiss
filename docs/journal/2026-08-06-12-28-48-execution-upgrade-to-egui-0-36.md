# Upgrade to egui 0.36

- **When:** 2026-08-06 12:28:48 local
- **Type:** execution
- **Project:** chaiss
- **Branch:** `feature/CHAI-0031-upgrade-to-egui-0-36`

## Context

egui 0.36.0 shipped 2026-08-05 with same-day releases of eframe, egui_extras,
and egui_commonmark 0.25.0 (which requires egui ^0.36), so the full dependency
stack could move together. First branch numbered from the local NEXT-TICKET
counter.

## Details

Version-bump-only upgrade: eframe/egui_extras 0.35→0.36, egui_commonmark
0.24→0.25; no source changes required. Verified with workspace tests
(18 passing), clippy, fmt, an `inspection`-feature build, and a live smoke
test via the egui MCP (board, heatmaps, arrows, markdown chat rendering,
session switching).

## Links

- Related entry: `2026-07-13-17-07-20-execution-upgrade-egui-0-35.md`
