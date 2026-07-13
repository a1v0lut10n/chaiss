#!/usr/bin/env bash
# SPDX-License-Identifier: Apache-2.0
#
# Install and enable the egui MCP inspection server for Chaiss.
#
# egui 0.35 ships an inspection protocol (egui_inspection): when the app is built
# with the `eframe/inspection` feature and launched with EGUI_INSPECTION=1, it
# exposes its live UI (AccessKit) tree on 127.0.0.1:5719. The `egui-mcp` server
# bridges that to any MCP client (e.g. Claude Code / Claude Desktop) so an agent
# can read and drive the running UI.
#
# This script installs the `egui-mcp` server binary. The server itself is enabled
# for this repository by the committed .mcp.json at the repo root — Claude Code
# will offer the `egui` server automatically once the binary is on PATH.
#
# Supported today: macOS and Ubuntu/Debian. Windows support is planned (see the
# windows_notice function below).
#
# Usage:
#   tools/scripts/setup-egui-mcp.sh            # install if missing, then print next steps
#   tools/scripts/setup-egui-mcp.sh --force    # reinstall even if already present
#   tools/scripts/setup-egui-mcp.sh --global   # also register with Claude at user scope
#                                              # (`claude mcp add`), not just this repo

set -euo pipefail

readonly EGUI_MCP_GIT="https://github.com/rerun-io/kittest_inspector"
readonly EGUI_MCP_CRATE="egui_mcp"
readonly EGUI_MCP_BIN="egui-mcp"
readonly INSPECTION_PORT="5719"

FORCE=0
GLOBAL=0
for arg in "$@"; do
  case "$arg" in
    --force) FORCE=1 ;;
    --global) GLOBAL=1 ;;
    -h|--help) sed -n '3,26p' "$0"; exit 0 ;;
    *) echo "❌ Unknown argument: $arg (try --help)"; exit 2 ;;
  esac
done

echo "♟️  Chaiss · egui MCP inspection setup ♟️"
echo "========================================"

# --- Platform detection --------------------------------------------------------
windows_notice() {
  cat <<'EOF'
🪟 Windows is not automated yet.

The pieces all work on Windows (egui uses AccessKit natively there), but this
installer is currently macOS/Ubuntu only. To set up manually for now:

  1. cargo install --git https://github.com/rerun-io/kittest_inspector egui_mcp
  2. Ensure %USERPROFILE%\.cargo\bin is on PATH (egui-mcp.exe lives there).
  3. The repo's .mcp.json registers the server for Claude Code automatically.
  4. Launch the app with inspection:
         set EGUI_INSPECTION=1
         cargo run -p chaiss --features inspection

A tools/scripts/setup-egui-mcp.ps1 PowerShell port is planned.
EOF
}

OS="$(uname -s 2>/dev/null || echo unknown)"
case "$OS" in
  Darwin) PLATFORM="macOS" ;;
  Linux)
    PLATFORM="Linux"
    if [ -r /etc/os-release ]; then
      # shellcheck disable=SC1091
      . /etc/os-release
      case "${ID:-}${ID_LIKE:-}" in
        *debian*|*ubuntu*) PLATFORM="Ubuntu/Debian" ;;
        *) echo "⚠️  Detected Linux '${ID:-unknown}', not Debian/Ubuntu — proceeding, but system deps are your responsibility." ;;
      esac
    fi
    ;;
  MINGW*|MSYS*|CYGWIN*|Windows_NT) windows_notice; exit 1 ;;
  *) echo "❌ Unsupported platform: $OS"; exit 1 ;;
esac
echo "🖥️  Platform: $PLATFORM"

# --- Prerequisites -------------------------------------------------------------
if ! command -v cargo >/dev/null 2>&1; then
  echo "❌ cargo not found. Install the Rust toolchain first: https://rustup.rs/"
  exit 1
fi
echo "🦀 cargo: $(cargo --version)"

if ! command -v cc >/dev/null 2>&1; then
  echo "⚠️  No C compiler (cc) found — cargo install may fail to build native deps."
  if [ "$PLATFORM" = "Ubuntu/Debian" ]; then
    echo "    Install build tooling:  sudo apt-get install -y build-essential pkg-config"
  elif [ "$PLATFORM" = "macOS" ]; then
    echo "    Install command line tools:  xcode-select --install"
  fi
fi

# --- Install the egui-mcp server ----------------------------------------------
if command -v "$EGUI_MCP_BIN" >/dev/null 2>&1 && [ "$FORCE" -eq 0 ]; then
  echo "✅ $EGUI_MCP_BIN already installed at: $(command -v "$EGUI_MCP_BIN")"
  echo "   (re-run with --force to reinstall)"
else
  echo "📦 Installing $EGUI_MCP_BIN from $EGUI_MCP_GIT ..."
  install_args=(--git "$EGUI_MCP_GIT" "$EGUI_MCP_CRATE")
  [ "$FORCE" -eq 1 ] && install_args+=(--force)
  cargo install "${install_args[@]}"
  echo "✅ Installed: $(command -v "$EGUI_MCP_BIN" || echo "$EGUI_MCP_BIN (ensure ~/.cargo/bin is on PATH)")"
fi

# --- Enable for Claude ---------------------------------------------------------
REPO_ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
if [ -f "$REPO_ROOT/.mcp.json" ]; then
  echo "🔌 Project MCP config present: $REPO_ROOT/.mcp.json"
  echo "   Claude Code will offer the 'egui' server when you open this repo."
else
  echo "⚠️  $REPO_ROOT/.mcp.json is missing — the server won't be registered for this repo."
fi

if [ "$GLOBAL" -eq 1 ]; then
  if command -v claude >/dev/null 2>&1; then
    if claude mcp list 2>/dev/null | grep -q '^egui\b'; then
      echo "✅ 'egui' already registered with Claude at user scope."
    else
      echo "🔗 Registering 'egui' with Claude at user scope ..."
      claude mcp add egui "$EGUI_MCP_BIN"
    fi
  else
    echo "⚠️  --global requested but the 'claude' CLI was not found; skipping user-scope registration."
  fi
fi

# --- Next steps ----------------------------------------------------------------
cat <<EOF

🎉 Done.

Next steps:
  1. Launch Chaiss with inspection enabled (exposes the UI on 127.0.0.1:${INSPECTION_PORT}):
         tools/scripts/run-chaiss-inspect.sh
     (equivalent to: EGUI_INSPECTION=1 cargo run -p chaiss --features inspection)

  2. In Claude Code, approve/use the 'egui' MCP server. Its tools let an agent
     read the live widget tree and drive the running Chaiss UI.

Tip: the app must be running with inspection enabled before the egui-mcp tools
can connect.
EOF
