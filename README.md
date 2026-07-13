<p align="center">
  <img src="static/chaiss-logo.svg" alt="Chaiss Logo" width="300">
</p>


# Chaiss

**Chaiss** is an intelligent, AI-assisted chessboard application built from the ground up in Rust using the modern `egui` immediate mode UI framework. Designed both for learning and deep exploration of chess, Chaiss integrates tightly with frontier Large Language Models (LLMs) to provide real-time guidance, context-aware analysis, and an environment to experiment with different lines.

## Motivation

This project was born out of two distinct drivers:
1. **The Vibe Coding Experiment:** A desire to experiment working in a "vibe coding" fashion, efficiently juggling multiple projects at once.
2. **The Chess Match Reality Check:** I found myself spending far too much time consulting Gemini for my next best move against my niece's boyfriend, only to realize that LLMs (both Gemini, and ChatGPT even more so) suffered from terrible inherent board vision and memory limitations.

Within mere hours of spelling out that I wanted to use my favorite language, my favorite UI framework within that language, and my favorite crate for interacting with LLMs, I had a working prototype. Chaiss significantly improved my moves simply by being able to persistently and accurately present the true current board state to the LLM backend!

## Key Features

- **Heat Map Targeting:** A unique visualization that highlights the targeted ranges of chess pieces. Overlapping attacks increase the intensity of the square's color while remaining distinctly distinguishable from the underlying light/dark squares.
- **LLM Integration:** Talk and analyze games with advanced AI models (like Gemini and ChatGPT) directly in the app. Utilizing the `llm` crate, interactions are configurable and context-aware.
- **Game & Player Persistence:** Name your players and your games. Relying on SQLite and `sqlx`, Chaiss allows you to save, load, and manage a library of your past chess explorations easily.
- **Dual Control Schemes:** Seamlessly move pieces via Mouse/Touch, or use intuitive keyboard-based algebraic notation highlighting.
- **Exploration Mode:** A sandboxed mode offering robust undo/redo capabilities to test "what if" scenarios before instantly reverting to the live game state.

## Project Structure

- `chaiss-core/` - Rust source code for backend logic, database integrations (`sqlx`), and models.
- `chaiss/` - Rust source code for the `egui`-based frontend application. Includes UI components and an `assets/` directory.
- `docs/` - Requirements, design documents, and developer journals.
- `tools/` - Sandbox and helper scripts for repository maintenance.

## Getting Started

1. **Clone the repository:**
   ```bash
   git clone <repository-url>
   cd chaiss
   ```
2. **Environment Configuration:**
   Copy the provided `.env.example` to `.env` and fill in your desired backend, model, and API keys.
   ```bash
   cp .env.example .env
   ```
3. **Run Locally:**
   Thanks to the pre-processed `sqlx` cache and auto-executing built-in Rust migrations, the application can be built and initialized purely offline and is 100% cross-platform. Just run:
   ```bash
   cargo run --release
   ```

## User Guide

For detailed instructions on how to use the dual-control mechanisms, launch a match, parse raw PGN text directly into the structural Egui modules, and securely interrogate the AI backend, please see the explicit **[Chaiss User Guide](docs/user_guide.md)**.

## UI Inspection via MCP

Chaiss is built on `egui` 0.35, which ships an **inspection protocol**
(`egui_inspection`) that exposes a running app's live UI (AccessKit) tree over a
local port. Chaiss wires this to the [`egui-mcp`](https://github.com/rerun-io/kittest_inspector)
server so an AI agent — e.g. Claude Code — can **read and drive the running UI**
through the Model Context Protocol.

**One-time setup** (installs the `egui-mcp` server binary; macOS and Ubuntu/Debian
today, Windows planned):

```bash
tools/scripts/setup-egui-mcp.sh          # add --global to also register at Claude user scope
```

The repository's committed `.mcp.json` registers the `egui` server for Claude
Code automatically once the binary is on your `PATH` (via `~/.cargo/bin`).

**Run Chaiss with inspection enabled** (exposes the UI on `127.0.0.1:5719`):

```bash
tools/scripts/run-chaiss-inspect.sh
# equivalent to:
EGUI_INSPECTION=1 cargo run -p chaiss --features inspection
```

Inspection is a build-time opt-in (the `inspection` feature on `chaiss`, which
enables `eframe/inspection`); normal and release builds are unaffected. With the
app running, the `egui` MCP tools in Claude can connect and interact with it.

> **macOS note:** keep the Chaiss window **foregrounded and visible** while using
> the MCP tools. An occluded or minimized window stops painting, so the AccessKit
> tree collapses to a minimal snapshot and `screenshot` times out ("the app is
> not painting; bring its window to the foreground"). Bring the window to the
> front and the full widget tree and screenshots work.

## Development and Contributions

For a detailed breakdown of the features, architectural choices, and upcoming roadmap, please refer to the `docs/requirements/chaiss_requirements.md` file.

## Acknowledgments

- Thanks to **Emil Ernerfeldt** (@emilk) and contributors for the [`egui`](https://github.com/emilk/egui) framework. Its immediate-mode rendering made rapid prototyping straightforward for a weekend project.
- Thanks to **graniet** (@graniet) for the [`llm`](https://github.com/graniet/llm) crate, whose unified API layer connects Chaiss to frontier models like Gemini, OpenAI, and Anthropic.
- Thanks to [`flume`](https://github.com/zesterer/flume) for its cloneable MPMC channels, which fit the immediate-mode model well: worker threads send messages and request a repaint, and the UI consumes whatever is ready on the next frame via non-blocking receives.
- Thanks to the **Google DeepMind** team and the **Gemini 3.1 Pro** model, used via the **Antigravity IDE** during development.
