# Chaiss

**Chaiss** is an intelligent, AI-assisted chessboard application built from the ground up in Rust using the modern `egui` immediate mode UI framework. Designed both for learning and deep exploration of chess, Chaiss integrates tightly with frontier Large Language Models (LLMs) to provide real-time guidance, context-aware analysis, and an environment to experiment with different lines.

## Key Features

- **Heat Map Targeting:** A unique visualization that highlights the targeted ranges of chess pieces. Overlapping attacks increase the intensity of the square's color while remaining distinctly distinguishable from the underlying light/dark squares.
- **LLM Integration:** Talk and analyze games with advanced AI models (like Gemini and ChatGPT) directly in the app. Utilizing the `llm` crate, interactions are configurable and context-aware.
- **Game & Player Persistence:** Name your players and your games. Relying on SQLite and `sqlx`, Chaiss allows you to save, load, and manage a library of your past chess explorations easily.
- **Dual Control Schemes:** Seamlessly move pieces via Mouse/Touch, or use intuitive keyboard-based algebraic notation highlighting.
- **Exploration Mode:** A sandboxed mode offering robust undo/redo capabilities to test "what if" scenarios before instantly reverting to the live game state.

## Project Structure (Planned)

- `src/` - Rust source code for the Chaiss application
- `docs/` - Requirements, design documents, and developer setup
- `assets/` - Fonts, piece images, and other resources

## Getting Started

1. **Clone the repository:**
   ```bash
   git clone <repository-url>
   cd chaiss
   ```
2. **Environment Configuration:**
   Copy the `.env.example` to `.env` and fill in your desired backend, model, and API keys.
3. **Run Locally:**
   ```bash
   cargo run --release
   ```

## Development and Contributions

For a detailed breakdown of the features, architectural choices, and upcoming roadmap, please refer to the `docs/requirements/chaiss_requirements.md` file.
