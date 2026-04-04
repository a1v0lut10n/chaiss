# Implementing Visual AI Continuations

This design formalizes how we will mathematically mutate the LLM's raw text generation explicitly into glowing, transparent analytical arrows organically mapped across the physical Egui chessboard.

## Objective Core Workflow
To bridge "chat text" into "interactive visualizations", we need to bind the Gemini Model's explicit response pattern structurally to yield mathematical arrays at the end of its prompt, parse those dynamically in the application layer, convert them algebraically to physical geometry coordinates, and render transparent arrows traversing the tiles!

## User Review Required

> [!WARNING]
> Parsing explicit JSON structures via a streaming pipeline is notoriously brittle since elements pop in organically token-by-token. 
> **My Proposal:** Instead of raw JSON, we strictly command the LLM to format its responses organically but **ALWAYS** append a specific diagnostic tag at the physical end of its conceptual analysis: `### PREDICTIVE MATRIX: e4, e5, Nf3, Nc6`. 
> This provides standard readable chat output for the user natively, while allowing us to effortlessly hook the Regex/String parser when the inference stream finally technically terminates!

> [!TIP]
> To visualize the depth of the AI's hypothesized continuation smoothly, we will mathematically alter the Alpha tracking (Transparency) of the arrows dynamically over sequence length. (e.g., the 1st move is densely colored, the 2nd is 60% transparent, the 3rd is 30% transparent).

## Proposed Changes

---

### Prompt Architecture Modifications

#### [MODIFY] [llm.rs](file:///home/hansvw/Projects/Aivolution/chaiss/core/src/llm.rs)
- Embellish the `System Prompt` to inherently command formatting: 
*"At the mathematical conclusion of your analysis, you MUST provide exactly one hypothesized continuation line up to 4 ply deep, formatted distinctly exactly like this: `### PREDICTIVE MATRIX: e4, e5, Nf3, Nc6`."*

### Application State & Parsing

#### [MODIFY] [app.rs](file:///home/hansvw/Projects/Aivolution/chaiss/desktop/src/app.rs)
- Expand `ChaissApp` explicitly to possess `pub ai_predictive_arrows: Vec<(usize, usize)>`.
- During the `LlmEvent::InferenceFinished` hook:
  1. Recursively search `self.live_llm_response` for the `### PREDICTIVE MATRIX:` flag.
  2. If detected, natively clone the current `game_state`.
  3. Loop synchronously across the provided algebraic array strings natively invoking `chaiss_core::engine::notation::parse_algebraic_move(&mut temp_state, &move_str)` to definitively extract the `(from, to)` tile indices mathematically.
  4. Embed and store the compiled scalar tuples securely inside `ai_predictive_arrows`! 
- Clear the arrays instantly upon the physical `apply_move` or new session loads cleanly.

### Visual Arrow Rendering

#### [MODIFY] [board.rs](file:///home/hansvw/Projects/Aivolution/chaiss/desktop/src/ui/board.rs)
- Render loop addition: Following the dynamic square/piece rendering hooks, evaluate `app.ai_predictive_arrows` structurally.
- Iterate the `(from, to)` vector indices locally mapping mathematically to physical `<egui::Pos2>` center points across the bounding rects.
- Synthesize transparent directional pointers actively leveraging `ui.painter().arrow(origin, vector, Stroke)`.

## Verification Plan

### Manual Sandbox Verification
- Launch the GUI.
- Ask the AI actively (via Prompt window): *"What is your structural continuation plan here natively?"*
- Wait for inference generation completion.
- Verify that immediately upon completion, brilliant visually scaled arrows cleanly span the board geometrically matching the AI's exact text layout cleanly!
