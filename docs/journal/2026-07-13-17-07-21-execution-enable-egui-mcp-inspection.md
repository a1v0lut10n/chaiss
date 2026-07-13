# Enable egui 0.35 UI inspection via an MCP server

- **When:** 2026-07-13 17:07:21 CEST
- **Type:** execution
- **Project:** chaiss

## Context
egui 0.35's headline benefit for Chaiss is its inspection protocol
(egui_inspection): a running app can expose its live AccessKit UI tree so an
external agent can read and drive it. We want Chaiss inspectable from Claude via
the egui-mcp server, installable with one script on macOS and Ubuntu now (Windows
to follow). Mirrors aiquill-workspace's enablement.

## Details
Findings (verified against eframe/egui_inspection 0.35):
- eframe's `inspection` feature pulls in `egui_inspection` + `accesskit`; eframe
  auto-attaches the plugin from the `EGUI_INSPECTION` env var and enables
  AccessKit itself — no app code changes required. The app listens on
  127.0.0.1:5719.
- The MCP bridge is `egui-mcp` from rerun-io/kittest_inspector.

Changes:
- `chaiss/Cargo.toml`: add opt-in `inspection = ["eframe/inspection"]` feature
  (normal/release builds unaffected).
- `.mcp.json` (repo root): register the `egui` MCP server so Claude Code offers it.
- `tools/scripts/setup-egui-mcp.sh`: install the egui-mcp binary; macOS +
  Ubuntu/Debian detection, Windows manual-steps notice, `--force`/`--global` flags.
- `tools/scripts/run-chaiss-inspect.sh`: launch with `EGUI_INSPECTION=1` and the
  inspection feature.
- README: new "UI Inspection via MCP" section, including a macOS foregrounding note.

Verification:
- `cargo check -p chaiss --features inspection` → egui_inspection 0.35 resolved;
  default `cargo check --workspace` unaffected.
- End-to-end MCP test against a running `run-chaiss-inspect.sh` instance (drove the
  egui-mcp stdio server as an MCP client): `initialize` → 15 tools listed;
  `attach` connected and identified the peer as "Chaiss - AI Chess Board" on
  127.0.0.1:5719; `status` = connected.
- Caveat (macOS): the window must be foregrounded/painting. While occluded, the
  AccessKit tree collapses to a minimal snapshot — observed here (the
  background-launched window returned only the collapsed root), matching the
  README's foregrounding note.

## Links
- Branch: `feature/CHAI-28-upgrade-to-egui-0-35`
- Reference: aiquill-workspace commit `e543e12` (same enablement).
- Upstream: https://github.com/rerun-io/kittest_inspector (egui_mcp)
- Predecessor: the egui 0.35 upgrade entry.
