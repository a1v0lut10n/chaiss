use flume::Sender;
use futures::StreamExt;
use llm::{
    builder::{LLMBackend, LLMBuilder},
    chat::ChatMessage,
};

// Orchestrate mathematically generic non-blocking HTTP REST streaming logic directly interacting with Gemini 1.5/2.0+!
pub async fn stream_llm_response(prompt: &str, tx: Sender<String>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let api_key = std::env::var("GOOGLE_API_KEY").unwrap_or_else(|_| "TESTKEY".to_string());
    
    // Validate we actually have a test key or an env var mapped, else error gracefully without crashing!
    if api_key == "TESTKEY" {
        let _ = tx.send_async("\n[System Warning: No GOOGLE_API_KEY exported natively in your terminal! Please restart your desktop client with `GOOGLE_API_KEY=your_key_here cargo run --bin desktop` to leverage Gemini natively.]".to_string()).await;
        return Ok(());
    }

    let llm = LLMBuilder::new()
        .backend(LLMBackend::Google)
        .api_key(api_key.clone())
        .model("gemini-2.0-flash") // Default to optimal Google architecture explicitly mapped via the 1.3.7 crate!
        .max_tokens(1000)
        .temperature(0.7)
        .build()
        .map_err(|e| format!("Failed LLM Build: {:?}", e))?;

    let messages = vec![ChatMessage::user().content(prompt).build()];
    let mut stream = llm.chat_stream(&messages).await.map_err(|e| format!("Chat Stream err: {}", e))?;
    
    while let Some(Ok(token)) = stream.next().await {
        let _ = tx.send_async(token).await;
    }
    
    Ok(())
}
