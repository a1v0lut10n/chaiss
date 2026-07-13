#!/usr/bin/env bash
# SPDX-License-Identifier: Apache-2.0
#
# Run Chaiss with egui inspection enabled.
#
# Builds chaiss with the `inspection` feature (eframe/inspection) and sets
# EGUI_INSPECTION so eframe attaches the egui_inspection plugin, exposing the
# live UI tree on 127.0.0.1:5719 for the egui-mcp server.
#
# Any extra arguments are forwarded to `cargo run` (e.g. --release).

set -euo pipefail

cd "$(dirname "$0")/../.."

# Default to enabling inspection; allow an explicit EGUI_INSPECTION override
# (e.g. EGUI_INSPECTION=127.0.0.1:6000) to be respected.
export EGUI_INSPECTION="${EGUI_INSPECTION:-1}"

echo "🔎 Launching Chaiss with inspection (EGUI_INSPECTION=$EGUI_INSPECTION, port 5719)"
exec cargo run -p chaiss --features inspection "$@"
