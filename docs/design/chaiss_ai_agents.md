# Chaiss AI Agents Architecture

## 1. Core Concept
Instead of a single monolithic AI opponent, Chaiss dynamically orchestrates an extensible network of LLM "Agents". Each Agent leverages a discrete persona and rule-set using the Unified `llm` Rust crate interface, seamlessly integrating with the Right-Panel UI to bridge conversational analysis with physical GameState execution.

## 2. Defined Agent Roles

### 2.1 The Companion (Advisor)
- **Objective:** Teams up with the human user against a third-party opponent (local human or external entity).
- **Behavior:** Offers tactical analysis, spots blunders, predicts opponent structures, and suggests optimal algebraic lines. It does *not* auto-play moves.
- **Context Injection:** When the opponent plays, the application sends context: `[System] Black played Nf3`. When the user plays, it sends: `[System] I have played e5`.

### 2.2 The Opponent 
- **Objective:** Autonomously battles the human player. 
- **Behavior:** Highly competitive. Does not offer unsolicited advice. Analyzes the active context vectors and dynamically fires asynchronous physical moves.
- **Context Injection:** Egui structurally executes the LLM's returned algebraic string dynamically onto the physical Engine state!

### 2.3 The Teacher
- **Objective:** Pedagogical oversight.
- **Behavior:** Evaluates the math for *both* sides objectively. The human makes the moves physically for both White and Black. The Teacher parses positional dynamics, highlighting weaknesses and architectural theories.
- **Context Injection:** Conversational analysis structurally prompted by the user (e.g., "Was trading the Bishop for the Knight a blunder here?").

## 3. Keyboard Input Flow Consolidation (UI)
The standalone algebraic text box is officially **superseded** by the AI Chat Window physically! 
- **Algebraic Priority:** If the human types a legal algebraic string (e.g., `O-O` or `e4`), the Engine instantaneously mathematically matches it, plays the piece physically, and forwards a systemic broadcast to the LLM thread: `[System] I have played e4`.
- **Conversational Priority:** If the string is plain English, the Engine bypasses the Chess struct parser entirely and simply routes the string to the active LLM context as standard Chat sequence.

## 4. `llm` Crate Orchestration (Frontier API Networking)
Rather than aggressively blocking local CPU threads with quantized modeling, the architecture natively leverages the asynchronous `llm` crate to dynamically bridge Frontier AI networks (Gemini 3.1, ChatGPT, Claude) or localized intranet GPU clusters (vLLM).
- **Asynchronous Integrity:** Because the `llm` crate bindings are fundamentally asynchronous, we will pipe the API dispatches structurally inside `tokio::spawn` threads without ever locking the Graphical framerate computationally.
- **Flume Streaming:** API chunks received sequentially over the network stream are mathematically forwarded over `flume` channels straight to `app.rs`, resolving conversational UI flows instantaneously algebraically!
