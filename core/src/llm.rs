use flume::Sender;
use futures::StreamExt;
use llm::{
    builder::{LLMBackend, LLMBuilder},
    chat::ChatMessage,
};

#[derive(Clone, Debug)]
pub struct LlmPromptPayload {
    pub prompt: String,
    pub current_fen: String,
    pub ascii_board: String,
    pub algebraic_history: Vec<String>,
    pub chat_history: Vec<(String, String)>,
    pub predictive_matrix_hotspots: Vec<String>,
    pub system_role: String,
}

// Orchestrate mathematically generic non-blocking HTTP REST streaming logic directly interacting with Gemini 3.1!
pub async fn stream_llm_response(payload: LlmPromptPayload, tx: Sender<String>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let api_key = std::env::var("GOOGLE_API_KEY").unwrap_or_else(|_| "TESTKEY".to_string());
    
    // Validate we actually have a test key or an env var mapped, else error gracefully without crashing!
    if api_key == "TESTKEY" {
        let _ = tx.send_async("\n[System Warning: No GOOGLE_API_KEY exported natively in your terminal! Please restart your desktop client with `GOOGLE_API_KEY=your_key_here cargo run --bin desktop` to leverage Gemini natively.]".to_string()).await;
        return Ok(());
    }

    let llm = LLMBuilder::new()
        .backend(LLMBackend::Google)
        .api_key(api_key.clone())
        .model("gemini-3.1-pro-preview") // Frontier Gemini architecture structurally mapping rigorous mathematical constraints
        .max_tokens(8000)
        .temperature(0.7)
        .build()
        .map_err(|e| format!("Failed LLM Build: {:?}", e))?;

    let fen_parts: Vec<&str> = payload.current_fen.split_whitespace().collect();
    let is_white_turn = fen_parts.get(1).map_or(true, |&p| p == "w");
    let active_color = if is_white_turn { "WHITE" } else { "BLACK" };
    
    // 1. Build context mathematically formatting history and explicit ASCII layouts securely
    let formatted_history: String = payload.algebraic_history.iter().enumerate().map(|(i, mov)| format!("{}. {}", i, mov)).collect::<Vec<_>>().join("\n");
    let mut futuristic_foresight = String::new();
    if !payload.predictive_matrix_hotspots.is_empty() {
        futuristic_foresight = format!(
            "\n\nCRITICAL CONTEXT INJECTION:\nThe Rust Engine's 2nd-Order Predictive Matrix natively resolved that the following squares will become the MOST densely contested structural targets 1-ply into the future: {}\nIncorporate this absolute mathematical foresight organically into your conceptual strategic analysis!",
            payload.predictive_matrix_hotspots.join(", ")
        );
    }

    let system_prompt = format!(
        "You are Chaiss, an advanced Chess {} mathematically bound to geometrical analysis.\n\n\
        Current FEN String:\n{}\n\n\
        Structural ASCII Board Matrix:\n{}\n\n\
        Full Explicit Match Algebraic Sequence:\n{}\n\n\
        The geometry currently dictates it is {}'s turn to move. \
        Critically evaluate physical piece interactions natively, recognize structural blunders explicitly, and predict future hostile pressure correctly. Focus your analysis purely geometrically tracking explicit pawn structure and piece coordination sequentially over time. The user provides algebraic prompts.{}",
        payload.system_role,
        payload.current_fen,
        payload.ascii_board,
        formatted_history,
        active_color,
        futuristic_foresight
    );

    // 2. Synthesize Context Matrix iteratively mimicking continuous API session strings cleanly
    let mut messages = vec![ChatMessage::user().content(&system_prompt).build()];
    
    // Inject a dummy acknowledgment so Gemini mathematically anchors the System constraints before our formal chat!
    messages.push(ChatMessage::assistant().content("System Context Acknowledged. I am mathematically bound to the supplied FEN bounds.").build());
    
    for (role, content) in payload.chat_history {
        if role == "User" {
            messages.push(ChatMessage::user().content(&content).build());
        } else {
            messages.push(ChatMessage::assistant().content(&content).build());
        }
    }
    
    // Inject the final active mathematical Prompt 
    messages.push(ChatMessage::user().content(&payload.prompt).build());

    let mut stream = llm.chat_stream(&messages).await.map_err(|e| format!("Chat Stream err: {}", e))?;
    
    while let Some(result) = stream.next().await {
        match result {
            Ok(token) => {
                let _ = tx.send_async(token).await;
            }
            Err(e) => {
                let _ = tx.send_async(format!("\n\n[Network Stream Disconnected Abruptly: {}]", e)).await;
                break;
            }
        }
    }
    
    Ok(())
}
