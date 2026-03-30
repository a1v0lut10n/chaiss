# Unified API LLM Architecture Plan

The absolute priority is securely and efficiently bridging our UI state mathematically into robust non-blocking HTTP network streams physically invoking Frontier Intelligence pipelines (Gemini 3.1, GPT, vLLM intra-nets). Coupling this with the explicit Roles defined natively in the new `chaiss_ai_agents.md` document drastically evolves our application layout.

## User Review Required
Does replacing the purely standalone Algebraic text input natively with the **AI Chat window** completely align with your UI expectation? It is a brilliant consolidation—if the Engine parses your chat string as a mathematically perfect legal move (`exd5`), it immediately plays it automatically! 

## Proposed Architecture

### 1. Engine Evolution (`docs/design/chaiss_ai_agents.md`)
I have formally seeded the **Agent Network Architecture** documentation detailing exactly how *The Companion*, *The Opponent*, and *The Teacher* geometrically intercept `GameState` slices!

### 2. `llm` Integration (`core/Cargo.toml` & `core/src/llm.rs`)
I will inject the `llm` crate functionally into `chaiss_core`.
Because the network latency inherently delays evaluations for milliseconds to seconds at a time:
- The inference logic natively binds to non-blocking `tokio::spawn` runtime threads logically decoupled from the framerate.
- We allocate and configure flexible `llm` Client structures capable of executing requests flawlessly natively to Gemini API surfaces or local `vLLM` nodes automatically via configurable API keys!
- It physically streams text chunks autonomously, piping the byte buffers smoothly through `flume` UI event channels natively.

### 3. UI Chat Consolidation (`desktop/src/ui/right_panel.rs`)
I will fundamentally refactor the Egui Input logic physically mapping the `prompt_buffer`.
When you hit **Enter**:
1. It validates the string across the math bounds natively (`app.game_state.is_legal_move(&prompt)`).
2. If legal -> Execute physical Piece transformation -> Broadcast `[System] I played X` to the LLM Async thread.
3. If illegal -> Dispatch directly to the LLM Async thread physically asking for conversational feedback!

### 4. Flume Chat Events (`desktop/src/app.rs`)
We extend the existing DB events structurally natively mapping:
```rust
pub enum LlmEvent {
    TokenStreamed(String),
    InferenceFinished,
}
```

## Verification
- We compile the local LLM async architecture.
- We simulate keyboard strings into the chat box (e.g., `e4`).
- It parses as mathematical chess strings cleanly mapping database saves while concurrently booting the LLM thread dynamically!
